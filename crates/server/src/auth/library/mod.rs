//! Better Auth's own routes through better-auth-rs (task 081.39), on an app-owned rusqlite
//! store on the host's lanes. The router sends a route here once the library serves it;
//! passkeys, password sign-in, and the per-request cookie check stay app code.
mod bridge;
mod policy;
mod providers;
mod store;
#[cfg(test)]
mod tests;
mod transaction;

use crate::http::App;
use better_auth::{
    AuthConfig, BetterAuth,
    plugins::{AccountManagementPlugin, OAuthPlugin, SessionManagementPlugin},
};
use better_auth_core::{
    middleware::{CsrfConfig, RateLimitConfig},
    schema::AuthSchema,
    wire::*,
};
pub(crate) use bridge::serve;
use std::{sync::Arc, time::Duration};
pub use store::LaneStore;

/// How long an async transaction callback may hold the writer, apart from the lane's
/// admission deadline (adoption gate 3 in docs/tasks/auth-spike.md).
const TRANSACTION_DEADLINE: Duration = Duration::from_secs(5);

pub(crate) async fn build(app: &Arc<App>) -> BetterAuth<Schema> {
    // The library follows the host's clock, which PERF_NOW moves in bench builds.
    let _ = better_auth_core::time::set_clock(|| {
        chrono::DateTime::from_timestamp_millis(crate::clock::now()).unwrap_or_default()
    });
    let mut config = AuthConfig::new(app.config.secret.clone())
        .base_url(app.config.app_origin())
        .base_path("/api/auth");
    if app.session.secure {
        config.session.cookie_name = "__Secure-better-auth.session_token".into();
    }
    config.advanced.ip_address.headers = vec![bridge::CLIENT_IP.into()];
    config.session.expires_in = chrono::Duration::days(30);
    let oauth = providers::providers(&app.config)
        .into_iter()
        .fold(OAuthPlugin::new(), |plugin, (id, provider)| {
            plugin.add_provider(&id, provider)
        });
    let store = LaneStore::new(
        Arc::downgrade(app),
        app.session.allowed_domains.clone(),
        TRANSACTION_DEADLINE,
    );
    BetterAuth::new(config)
        .store(store)
        // The host's rate limits (auth/rate_limit.rs) wrap these routes.
        .rate_limit(RateLimitConfig {
            enabled: false,
            ..Default::default()
        })
        .csrf(CsrfConfig::new())
        .plugin(policy::Policy {
            app: Arc::downgrade(app),
        })
        .plugin(SessionManagementPlugin::new())
        .plugin(oauth)
        .plugin(AccountManagementPlugin::new())
        .build()
        .await
        .expect("better-auth-rs accepts the app's configuration")
}

pub struct Schema;
impl AuthSchema for Schema {
    type User = UserView;
    type Session = LaneSession;
    type Account = AccountView;
    type Verification = VerificationView;
}

// Snowtime has no session.active column: a row is active until deleted or expired.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct LaneSession(SessionView);
impl std::ops::Deref for LaneSession {
    type Target = SessionView;
    fn deref(&self) -> &SessionView {
        &self.0
    }
}
impl better_auth_core::AuthSession for LaneSession {
    fn id(&self) -> std::borrow::Cow<'_, str> {
        better_auth_core::AuthSession::id(&self.0)
    }
    fn expires_at(&self) -> chrono::DateTime<chrono::Utc> {
        better_auth_core::AuthSession::expires_at(&self.0)
    }
    fn token(&self) -> &str {
        better_auth_core::AuthSession::token(&self.0)
    }
    fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        better_auth_core::AuthSession::created_at(&self.0)
    }
    fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        better_auth_core::AuthSession::updated_at(&self.0)
    }
    fn ip_address(&self) -> Option<&str> {
        better_auth_core::AuthSession::ip_address(&self.0)
    }
    fn user_agent(&self) -> Option<&str> {
        better_auth_core::AuthSession::user_agent(&self.0)
    }
    fn user_id(&self) -> std::borrow::Cow<'_, str> {
        better_auth_core::AuthSession::user_id(&self.0)
    }
    fn impersonated_by(&self) -> Option<&str> {
        better_auth_core::AuthSession::impersonated_by(&self.0)
    }
    fn active_organization_id(&self) -> Option<&str> {
        better_auth_core::AuthSession::active_organization_id(&self.0)
    }
    fn active(&self) -> bool {
        true
    }
}
