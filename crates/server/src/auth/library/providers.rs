//! The app's OAuth providers for better-auth-rs, as installed Better Auth 1.7.7 reads
//! them: Google's and Microsoft's profiles come from the ID token of the HTTPS token
//! exchange, Microsoft's photo from Graph, and GitHub's from its user and email APIs.
use crate::config::{Config, OAuthProvider as Configured};
use async_trait::async_trait;
use base64::{
    Engine,
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
};
use better_auth::plugins::oauth::{
    OAuthProvider, OAuthUserInfo, OAuthUserInfoHandler, OAuthUserInfoRequest, OAuthUserInfoResponse,
};
use serde_json::{Value, json};
use std::sync::Arc;

pub(super) fn providers(config: &Config) -> Vec<(String, OAuthProvider)> {
    config
        .oauth
        .iter()
        .map(|provider| (provider.id.clone(), provider_of(provider)))
        .collect()
}

fn provider_of(configured: &Configured) -> OAuthProvider {
    let (id, secret) = (&configured.client_id, &configured.client_secret);
    match configured.id.as_str() {
        "google" => OAuthProvider {
            token_url: transport_url("https://oauth2.googleapis.com/token"),
            user_info_url: None,
            map_user_info: None,
            get_user_info: Some(Arc::new(IdToken { microsoft: false })),
            ..OAuthProvider::google(id, secret)
        },
        "github" => OAuthProvider::github_with_endpoints(
            id,
            secret,
            "https://github.com/login/oauth/authorize",
            &transport_url("https://github.com/login/oauth/access_token"),
            &transport_url("https://api.github.com/user"),
            &transport_url("https://api.github.com/user/emails"),
        ),
        _ => {
            let tenant = &configured.tenant;
            OAuthProvider {
                auth_url: format!(
                    "https://login.microsoftonline.com/{tenant}/oauth2/v2.0/authorize"
                ),
                token_url: transport_url(&format!(
                    "https://login.microsoftonline.com/{tenant}/oauth2/v2.0/token"
                )),
                user_info_url: None,
                scopes: ["openid", "profile", "email", "User.Read", "offline_access"]
                    .map(String::from)
                    .to_vec(),
                authorization_params: Vec::new(),
                map_user_info: None,
                get_user_info: Some(Arc::new(IdToken { microsoft: true })),
                ..OAuthProvider::google(id, secret)
            }
        }
    }
}

/// In bench builds, `OAUTH_FAKE_PROVIDER` sends the server's provider calls through the
/// conformance suite's fake provider.
fn transport_url(endpoint: &str) -> String {
    #[cfg(feature = "bench")]
    if let Ok(base) = std::env::var("OAUTH_FAKE_PROVIDER") {
        let url = url::Url::parse(&base).expect("fake provider URL");
        assert!(url.scheme() == "http" && url.host_str() == Some("127.0.0.1"));
        return format!(
            "{base}/proxy?url={}",
            form_urlencoded::byte_serialize(endpoint.as_bytes()).collect::<String>()
        );
    }
    endpoint.into()
}

fn jwt_claims(token: &str) -> Option<Value> {
    let parts = token.split('.').collect::<Vec<_>>();
    if parts.len() != 3 {
        return None;
    }
    serde_json::from_slice(&URL_SAFE_NO_PAD.decode(parts[1]).ok()?).ok()
}

/// The redirect flow trusts the HTTPS token exchange, as installed Better Auth does.
struct IdToken {
    microsoft: bool,
}

#[async_trait]
impl OAuthUserInfoHandler for IdToken {
    async fn get_user_info(
        &self,
        request: OAuthUserInfoRequest,
    ) -> Result<OAuthUserInfoResponse, String> {
        let token = request.id_token.as_deref().ok_or("missing id token")?;
        let mut data = jwt_claims(token).ok_or("malformed id token")?;
        if self.microsoft
            && let Some(access) = request.access_token.as_deref()
            && let Some(photo) = graph_photo(access).await
        {
            data["picture"] = json!(format!(
                "data:image/jpeg;base64, {}",
                STANDARD.encode(photo)
            ));
        }
        let subject = &data[if self.microsoft { "oid" } else { "sub" }];
        let id = subject
            .as_str()
            .map(str::to_owned)
            .or_else(|| (!self.microsoft).then(|| subject.as_i64().map(|v| v.to_string()))?)
            .filter(|id| !id.trim().is_empty())
            .ok_or("missing subject")?;
        let email = data["email"].as_str().unwrap_or_default().to_owned();
        let email_verified = if self.microsoft {
            data["email_verified"] == true
                || data["xms_edov"] == true
                || data["tid"] == "9188040d-6c67-4c5b-b112-36a304b66dad"
                || ["verified_primary_email", "verified_secondary_email"]
                    .iter()
                    .any(|k| {
                        data[k]
                            .as_array()
                            .is_some_and(|a| a.contains(&json!(email)))
                    })
        } else {
            data["email_verified"] == true
        };
        Ok(OAuthUserInfoResponse {
            user: OAuthUserInfo {
                id,
                email,
                name: data["name"].as_str().map(str::to_owned),
                image: data["picture"].as_str().map(str::to_owned),
                email_verified,
            },
            data,
        })
    }
}

async fn graph_photo(access_token: &str) -> Option<Vec<u8>> {
    let response = reqwest::Client::new()
        .get(transport_url(
            "https://graph.microsoft.com/v1.0/me/photos/48x48/$value",
        ))
        .bearer_auth(access_token)
        .send()
        .await
        .ok()?;
    if !response.status().is_success() {
        return None;
    }
    Some(response.bytes().await.ok()?.to_vec())
}
