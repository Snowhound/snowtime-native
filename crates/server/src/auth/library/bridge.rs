//! The boundary between the host and better-auth-rs: the host's request in, the library's
//! answer out, and the host's own refusal when a database lane turned the library away.
use super::Schema;
use crate::http::{Request, Response};
use axum::{
    body::Body,
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::IntoResponse,
};
use better_auth::BetterAuth;
use better_auth_core::{AuthError, AuthRequest, HttpMethod};
use std::{cell::RefCell, collections::HashMap};

/// The header the library reads the client's address from. The bridge sets it from the
/// host's resolved address (`CLIENT_IP_HEADER`) and drops any value the client sent.
pub(super) const CLIENT_IP: &str = "x-snowtime-client-ip";

tokio::task_local! {
    static REFUSAL: RefCell<Option<Response>>;
}

/// Keeps a lane's refusal, so the bridge answers with it (503 and `Retry-After`) in place
/// of the 500 the library makes of a store error.
pub(super) fn refused(response: Response) -> AuthError {
    let status = response.status;
    let _ = REFUSAL.try_with(|slot| {
        slot.borrow_mut().get_or_insert(response);
    });
    AuthError::internal(format!("database lane refused: {status}"))
}

/// Runs one of the library's server operations, answering a lane's refusal with the host's
/// own and any other error as the library's HTTP API would.
pub(crate) async fn call<T>(
    operation: impl Future<Output = Result<T, AuthError>>,
) -> Result<T, Response> {
    let (result, refusal) = REFUSAL
        .scope(RefCell::new(None), async {
            let result = operation.await;
            (result, REFUSAL.with(|slot| slot.borrow_mut().take()))
        })
        .await;
    if let Some(refusal) = refusal {
        return Err(refusal);
    }
    result.map_err(|error| {
        let answer = error.to_auth_response();
        crate::wire::WireResponse {
            status: answer.status,
            body: answer.body,
        }
        .into()
    })
}

pub(crate) async fn serve(
    auth: &BetterAuth<Schema>,
    request: Request,
    headers: &HeaderMap,
) -> axum::response::Response {
    let request = auth_request(request, headers);
    let (answer, refusal) = REFUSAL
        .scope(RefCell::new(None), async {
            let answer = auth.handle_request(request).await;
            (answer, REFUSAL.with(|slot| slot.borrow_mut().take()))
        })
        .await;
    if let Some(refusal) = refusal {
        return refusal.into_response();
    }
    let answer = answer.unwrap_or_else(|error| error.to_auth_response());
    let mut response = axum::response::Response::new(Body::from(answer.body));
    *response.status_mut() =
        StatusCode::from_u16(answer.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    for (name, value) in answer.headers.iter() {
        if let (Ok(name), Some(value)) = (HeaderName::try_from(name.as_str()), latin1(value)) {
            response.headers_mut().append(name, value);
        }
    }
    response
}

/// A header value as fetch's ByteString writes it: one Latin-1 byte per character.
fn latin1(value: &str) -> Option<HeaderValue> {
    let bytes: Option<Vec<u8>> = value.chars().map(|c| u8::try_from(c as u32).ok()).collect();
    HeaderValue::from_bytes(&bytes?).ok()
}

/// The app's `encodeCallbackUrls` hook: Better Auth writes callback URLs into Latin-1
/// `Location` headers, so wider characters are percent-encoded before the state stores
/// them. The body's other bytes and key order stay as sent.
fn encode_callback_urls(path: &str, body: &mut Vec<u8>) {
    if path != "/api/auth/sign-in/social" && path != "/api/auth/link-social" {
        return;
    }
    let Ok(mut fields) = serde_json::from_slice::<
        indexmap::IndexMap<String, Box<serde_json::value::RawValue>>,
    >(body) else {
        return;
    };
    let mut changed = false;
    for field in ["callbackURL", "errorCallbackURL", "newUserCallbackURL"] {
        let Some(value) = fields
            .get(field)
            .and_then(|raw| serde_json::from_str::<String>(raw.get()).ok())
            .filter(|value| value.chars().any(|c| c as u32 > 255))
        else {
            continue;
        };
        let mut encoded = String::new();
        for character in value.chars() {
            if character as u32 <= 255 {
                encoded.push(character);
            } else {
                for byte in character.encode_utf8(&mut [0; 4]).bytes() {
                    encoded.push_str(&format!("%{byte:02X}"));
                }
            }
        }
        if let Ok(raw) = serde_json::value::to_raw_value(&encoded) {
            fields.insert(field.to_owned(), raw);
            changed = true;
        }
    }
    if changed && let Ok(rewritten) = serde_json::to_vec(&fields) {
        *body = rewritten;
    }
}

fn auth_request(mut request: Request, headers: &HeaderMap) -> AuthRequest {
    encode_callback_urls(&request.path, &mut request.body);
    let method = match request.method.as_str() {
        "POST" => HttpMethod::Post,
        "PUT" => HttpMethod::Put,
        "DELETE" => HttpMethod::Delete,
        "PATCH" => HttpMethod::Patch,
        "OPTIONS" => HttpMethod::Options,
        "HEAD" => HttpMethod::Head,
        _ => HttpMethod::Get,
    };
    let mut fields: HashMap<String, String> = HashMap::new();
    for (name, value) in headers {
        let (name, Ok(value)) = (name.as_str(), value.to_str()) else {
            continue;
        };
        if name == CLIENT_IP {
            continue;
        }
        let separator = if name == "cookie" { "; " } else { ", " };
        fields
            .entry(name.to_owned())
            .and_modify(|joined| {
                joined.push_str(separator);
                joined.push_str(value);
            })
            .or_insert_with(|| value.to_owned());
    }
    // AuthCall turns a form body into JSON, as Better Auth reads either.
    if fields
        .get("content-type")
        .is_some_and(|value| value.starts_with("application/x-www-form-urlencoded"))
    {
        fields.insert("content-type".into(), "application/json".into());
    }
    if let Some(ip) = request.client_ip {
        fields.insert(CLIENT_IP.into(), ip);
    }
    let query = request
        .query
        .as_deref()
        .map(|query| {
            form_urlencoded::parse(query.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_default();
    let body = (!request.body.is_empty()).then_some(request.body);
    AuthRequest::from_parts(method, request.path, fields, body, query)
}
