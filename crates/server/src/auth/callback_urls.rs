//! Better Auth's checks of the callback URLs a request names (originCheck's callbackURL,
//! redirectTo, errorCallbackURL, and newUserCallbackURL), for the routes still app code.
use super::sign_in::refusal;
use crate::http::{Request, Response};
use serde_json::Value;

pub(super) fn truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(value) => *value,
        Value::Number(value) => value.as_f64() != Some(0.0),
        Value::String(value) => !value.is_empty(),
        _ => true,
    }
}

pub(super) fn check_urls(
    request: &Request,
    body: Option<&Value>,
    origin: &str,
) -> Result<(), Response> {
    for (field, label, code, message) in [
        (
            "callbackURL",
            "callbackURL",
            "INVALID_CALLBACK_URL",
            "Invalid callbackURL",
        ),
        (
            "redirectTo",
            "redirectURL",
            "INVALID_REDIRECT_URL",
            "Invalid redirectURL",
        ),
        (
            "errorCallbackURL",
            "errorCallbackURL",
            "INVALID_ERROR_CALLBACK_URL",
            "Invalid errorCallbackURL",
        ),
        (
            "newUserCallbackURL",
            "newUserCallbackURL",
            "INVALID_NEW_USER_CALLBACK_URL",
            "Invalid newUserCallbackURL",
        ),
    ] {
        let query = request.query.as_deref().and_then(|query| {
            form_urlencoded::parse(query.as_bytes())
                .find(|(name, _)| name == field)
                .map(|(_, value)| Value::String(value.into_owned()))
        });
        let value = body
            .and_then(|body| body.get(field))
            .filter(|value| truthy(value))
            .or(if field == "callbackURL" {
                query.as_ref()
            } else {
                None
            });
        let Some(value) = value else {
            continue;
        };
        let Some(value) = value.as_str() else {
            return Err(Response {
                status: 400,
                body: serde_json::to_vec(
                    &serde_json::json!({"message": format!("Invalid {label}: expected a string")}),
                )
                .unwrap(),
                set_cookies: Vec::new(),
                server_timing: None,
            });
        };
        if value.is_empty() {
            continue;
        }
        let path = value
            .split(['?', '#'])
            .next()
            .unwrap_or(value)
            .to_lowercase();
        let relative = value.starts_with('/')
            && !value.starts_with("//")
            && !value.contains('\\')
            && !value
                .chars()
                .any(|c| c <= '\u{1f}' || ('\u{7f}'..='\u{9f}').contains(&c))
            && !path.contains("%2f")
            && !path.contains("%5c");
        let trusted = if value.starts_with('/') {
            relative
        } else {
            url::Url::parse(value).is_ok_and(|url| url.origin().ascii_serialization() == origin)
        };
        if !trusted {
            return Err(refusal(403, code, message));
        }
    }
    Ok(())
}
