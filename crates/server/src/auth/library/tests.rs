use super::*;
use crate::{Config, Limits};
use better_auth_core::{error::AuthError, store::*, types::*};
use chrono::Utc;
use std::time::{Duration, Instant};

// The store holds a weak reference, so each test keeps its app.
fn store(deadline: Duration) -> (Arc<App>, LaneStore) {
    let app = App::open_with_limits(
        Config {
            database_path: ":memory:".into(),
            app_url: "http://localhost:3100".into(),
            secret: "library-secret-of-at-least-32-characters".into(),
            password_enabled: true,
            production: false,
            sign_in_page: Default::default(),
            client_ip_header: None,
            rate_limit: false,
            oauth: vec![],
        },
        0,
        Limits {
            hashes: 1,
            queue_timeout: Duration::from_millis(30),
            max_waiting: 32,
        },
    )
    .unwrap();
    crate::migrations::migrate(
        &mut app.db(),
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../app/drizzle"),
    )
    .unwrap();
    let store = LaneStore::new(Arc::downgrade(&app), vec!["example.com".into()], deadline);
    (app, store)
}

// The writer stays bounded while an async callback holds a transaction (adoption gate 3).
#[tokio::test]
async fn transaction_commit_error_deadline_and_cancellation() {
    let (_app, store) = store(Duration::from_millis(80));
    let at = Instant::now();
    let result: Result<String, _> = transaction::<Schema, _, _>(&store, |tx| {
        Box::pin(async move {
            let u = tx
                .create_user(
                    CreateUser::new()
                        .with_email("commit@example.com")
                        .with_name("Commit")
                        .with_email_verified(true),
                )
                .await?;
            let account = tx
                .create_account(CreateAccount {
                    user_id: u.id.clone(),
                    account_id: u.id.clone(),
                    provider_id: "credential".into(),
                    access_token: None,
                    refresh_token: None,
                    id_token: None,
                    access_token_expires_at: None,
                    refresh_token_expires_at: None,
                    scope: None,
                    password: Some("spike-hash".into()),
                })
                .await?;
            assert_eq!(account.password.as_deref(), Some("spike-hash"));
            let session = tx
                .create_session(CreateSession {
                    user_id: u.id.clone(),
                    expires_at: Utc::now() + chrono::Duration::days(30),
                    ip_address: None,
                    user_agent: None,
                    impersonated_by: None,
                    active_organization_id: None,
                })
                .await?;
            assert_eq!(session.user_id, u.id);
            Ok(u.id)
        })
    })
    .await;
    assert!(
        store
            .get_user_by_id(&result.unwrap())
            .await
            .unwrap()
            .is_some()
    );
    let failed: Result<(), _> = transaction::<Schema, _, _>(&store, |tx| {
        Box::pin(async move {
            tx.create_user(
                CreateUser::new()
                    .with_email("rollback@example.com")
                    .with_name("Rollback")
                    .with_email_verified(true),
            )
            .await?;
            Err(AuthError::bad_request("test rollback"))
        })
    })
    .await;
    assert!(failed.is_err());
    assert!(
        store
            .get_user_by_email("rollback@example.com")
            .await
            .unwrap()
            .is_none()
    );
    let task_store = store.clone();
    let timed_at = Instant::now();
    let timed = tokio::spawn(async move {
        transaction::<Schema, (), _>(&task_store, |tx| {
            Box::pin(async move {
                tx.create_user(
                    CreateUser::new()
                        .with_email("timeout@example.com")
                        .with_name("Timeout")
                        .with_email_verified(true),
                )
                .await?;
                tokio::time::sleep(Duration::from_secs(60)).await;
                Ok(())
            })
        })
        .await
    });
    tokio::time::sleep(Duration::from_millis(15)).await;
    assert!(store.get_user_by_email("commit@example.com").await.is_err());
    assert!(timed.await.unwrap().is_err());
    println!(
        "80ms transaction deadline released writer in {:?}",
        timed_at.elapsed()
    );
    assert!(
        store
            .get_user_by_email("timeout@example.com")
            .await
            .unwrap()
            .is_none()
    );
    let task_store = store.clone();
    let (sent, received) = tokio::sync::oneshot::channel();
    let cancel = tokio::spawn(async move {
        transaction::<Schema, (), _>(&task_store, move |tx| {
            Box::pin(async move {
                tx.create_user(
                    CreateUser::new()
                        .with_email("cancel@example.com")
                        .with_name("Cancel")
                        .with_email_verified(true),
                )
                .await?;
                sent.send(()).unwrap();
                tokio::time::sleep(Duration::from_secs(60)).await;
                Ok(())
            })
        })
        .await
    });
    received.await.unwrap();
    cancel.abort();
    tokio::time::sleep(Duration::from_millis(15)).await;
    assert!(
        store
            .get_user_by_email("cancel@example.com")
            .await
            .unwrap()
            .is_none()
    );
    println!(
        "commit/error/80ms deadline/cancellation: {:?}",
        at.elapsed()
    );
}
