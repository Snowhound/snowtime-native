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
        if let (Ok(name), Ok(value)) = (
            HeaderName::try_from(name.as_str()),
            HeaderValue::try_from(value.as_str()),
        ) {
            response.headers_mut().append(name, value);
        }
    }
    response
}

fn auth_request(request: Request, headers: &HeaderMap) -> AuthRequest {
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
