//! The app's Better Auth hooks (`hooks.before` in better-auth.server.ts), as a plugin
//! mounted first: it runs after the library's origin checks and before every handler.
use super::Schema;
use crate::http::App;
use async_trait::async_trait;
use better_auth_core::{
    AuthContext, AuthPlugin, AuthRequest, AuthResponse, AuthResult, AuthRoute, BeforeRequestAction,
};
use std::sync::{Arc, Weak};

pub(super) struct Policy {
    pub(super) app: Weak<App>,
}

#[async_trait]
impl AuthPlugin<Schema> for Policy {
    fn name(&self) -> &'static str {
        "snowtime-policy"
    }
    fn routes(&self) -> Vec<AuthRoute> {
        Vec::new()
    }
    async fn before_request(
        &self,
        req: &AuthRequest,
        _ctx: &AuthContext<Schema>,
    ) -> AuthResult<Option<BeforeRequestAction>> {
        if let Some(issues) = bounded_input(req) {
            return Ok(Some(BeforeRequestAction::Respond(validation_error(
                &issues,
            ))));
        }
        // loginDomainMiddleware lets a refused session sign out or read itself.
        if req.path == "/sign-out" || req.path == "/get-session" {
            return Ok(None);
        }
        let Some(app) = self.app.upgrade() else {
            return Ok(None);
        };
        let cookie = req.headers.get("cookie").cloned();
        match Arc::clone(&app).login_domain_check(cookie).await {
            Ok(()) => Ok(None),
            Err(refusal) if refusal.status == 503 => Err(super::bridge::refused(refusal)),
            Err(refusal) => Ok(Some(BeforeRequestAction::Respond(
                AuthResponse::text(refusal.status, String::from_utf8_lossy(&refusal.body))
                    .with_header("content-type", "application/json"),
            ))),
        }
    }
    async fn on_request(
        &self,
        _req: &AuthRequest,
        _ctx: &AuthContext<Schema>,
    ) -> AuthResult<Option<AuthResponse>> {
        Ok(None)
    }
}

/// The app's own input bounds on the OAuth flows (task 081.31), where Better Auth has
/// none: `additionalData`, which the state stores, and the number of scope issues
/// reported, so a large body can't make a larger answer.
fn bounded_input(req: &AuthRequest) -> Option<String> {
    if req.path != "/sign-in/social" && req.path != "/link-social" {
        return None;
    }
    let body: serde_json::Value = serde_json::from_slice(req.body.as_deref()?).ok()?;
    let invalid_scopes: Vec<String> = body["scopes"]
        .as_array()
        .into_iter()
        .flatten()
        .enumerate()
        .filter(|(_, scope)| !scope.is_string())
        .take(SCOPE_ISSUES + 1)
        .map(|(index, scope)| {
            format!(
                "[body.scopes.{index}] Invalid input: expected string, received {}",
                kind(scope)
            )
        })
        .collect();
    if invalid_scopes.len() > SCOPE_ISSUES {
        return Some(invalid_scopes[..SCOPE_ISSUES].join("; "));
    }
    super::super::bounds::json_issue(&body["additionalData"], "additionalData")
}

const SCOPE_ISSUES: usize = super::super::bounds::SCOPE_ISSUES;

fn kind(value: &serde_json::Value) -> &'static str {
    match value {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "boolean",
        serde_json::Value::Number(_) => "number",
        serde_json::Value::String(_) => "string",
        serde_json::Value::Array(_) => "array",
        serde_json::Value::Object(_) => "object",
    }
}

fn validation_error(message: &str) -> AuthResponse {
    AuthResponse::json(
        400,
        &better_auth_core::ErrorCodeMessageResponse {
            message: message.to_owned(),
            code: Some("VALIDATION_ERROR".to_owned()),
        },
    )
    .unwrap_or_else(|_| AuthResponse::text(400, message))
}

#[cfg(test)]
mod tests {
    use super::*;
    use better_auth_core::HttpMethod;
    use serde_json::{Value, json};

    fn request(body: Value) -> AuthRequest {
        let mut req = AuthRequest::new(HttpMethod::Post, "/sign-in/social");
        req.body = Some(body.to_string().into_bytes());
        req
    }

    #[test]
    fn scope_issues_stop_at_thirty_two() {
        let issues = bounded_input(&request(
            json!({"provider":"google","scopes":vec![Value::Null; 200_000]}),
        ))
        .unwrap();
        assert_eq!(issues.split("; ").count(), 32);
        assert!(issues.ends_with("[body.scopes.31] Invalid input: expected string, received null"));
        let few = json!({"provider":"google","scopes":vec![Value::Null; 32]});
        assert_eq!(bounded_input(&request(few)), None);
    }

    #[test]
    fn additional_data_is_bounded() {
        let big = json!({"provider":"google","additionalData":{"v":"a".repeat(4089)}});
        assert!(bounded_input(&request(big)).is_some());
        let fits = json!({"provider":"google","additionalData":{"v":"a".repeat(4088)}});
        assert_eq!(bounded_input(&request(fits)), None);
    }
}
