//! Personal API keys (src/server/auth/api-keys.server.ts): the rules Settings calls, and the
//! check that signs a JSON API request in with a key (docs/architecture/auth.md, "API keys").
//! better-auth-rs's api-key plugin creates a key, as only it generates one; listing, revoking,
//! and checking a request's key read and change its rows directly. Either backend accepts the
//! other's keys.
use super::login_domains;
use super::schemas::{ApiKey, CreateApiKeyInput, CreatedApiKey, RevokeApiKeyInput, RevokedApiKey};
use crate::limits::API_KEYS_PER_USER;
use crate::rate_limit::{API_KEY_REQUESTS, MemoryStore, WRITES_PER_USER};
use crate::schemas::{Timestamp, js_whitespace};
use crate::{Code, Error, Key, Result, refuse};
use axum::http::{HeaderMap, header};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use better_auth::{BetterAuth, server_api::CreateKeyOptions};
use rusqlite::{Connection, OptionalExtension};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

pub(crate) const PREFIX: &str = "snow_";
const DAY_SECONDS: i64 = 24 * 60 * 60;
// How often a key's last use is saved: Settings shows it, and no check depends on it.
const LAST_USE_PRECISION_MS: i64 = 60_000;

// The key in `Authorization: Bearer <key>`, or None when the request has none: the pattern
// /^Bearer\s+(\S+)\s*$/i over the header as fetch's Headers.get gives it, repeated fields
// joined with ", " and each byte one Latin-1 character.
pub(crate) fn bearer_key(headers: &HeaderMap) -> Option<String> {
    let mut value = String::new();
    for (i, field) in headers.get_all(header::AUTHORIZATION).iter().enumerate() {
        if i > 0 {
            value.push_str(", ");
        }
        value.extend(field.as_bytes().iter().map(|&b| char::from(b)));
    }
    let rest = value
        .get(..6)
        .filter(|scheme| scheme.eq_ignore_ascii_case("bearer"))
        .map(|_| &value[6..])?;
    let key = rest.trim_start_matches(js_whitespace);
    if key.len() == rest.len() {
        return None;
    }
    let (key, tail) = key.split_at(key.find(js_whitespace).unwrap_or(key.len()));
    (!key.is_empty() && tail.chars().all(js_whitespace)).then(|| key.to_owned())
}

/// The hash the plugin stores for a key: SHA-256 as unpadded base64url (its defaultKeyHasher).
pub(crate) fn hash_key(key: &str) -> String {
    URL_SAFE_NO_PAD.encode(Sha256::digest(key.as_bytes()))
}

// The plugin's permissions, `{"api":[...]}`, with `write` implying `read`. Rows hold what a
// server wrote; anything unreadable counts as read, the least access.
fn access_of(permissions: Option<&str>) -> &'static str {
    let api = permissions
        .and_then(|p| serde_json::from_str::<serde_json::Value>(p).ok())
        .and_then(|p| p.get("api").cloned());
    let write = match api {
        Some(serde_json::Value::Array(scopes)) => scopes.iter().any(|s| s == "write"),
        // Array and string share JavaScript's includes.
        Some(serde_json::Value::String(scopes)) => scopes.contains("write"),
        _ => false,
    };
    if write { "write" } else { "read" }
}

/// The user's keys, newest first, without their hashes.
pub fn list_api_keys(
    db: &Connection,
    user_id: &str,
    _: crate::schemas::Empty,
) -> Result<Vec<ApiKey>> {
    Ok(crate::sql!(
        "select id, name, permissions, created_at, expires_at, last_request from api_key where api_key.reference_id = ",
        user_id,
        " order by api_key.created_at desc"
    )
    .query(db, |row| {
        Ok(ApiKey {
            id: row.get(0)?,
            name: row.get::<_, Option<String>>(1)?.unwrap_or_default(),
            access: access_of(row.get::<_, Option<String>>(2)?.as_deref()),
            created_at: row.get(3)?,
            expires_at: row.get(4)?,
            last_used_at: row.get(5)?,
        })
    })?)
}

/// Checks that the user may have another key, which `issue_api_key` then creates.
pub fn check_api_key_room(
    db: &Connection,
    user_id: &str,
    input: CreateApiKeyInput,
) -> Result<CreateApiKeyInput> {
    let total: i64 = crate::sql!(
        "select count(*) from api_key where api_key.reference_id = ",
        user_id
    )
    .query_row(db, |row| row.get(0))?;
    if total >= API_KEYS_PER_USER {
        return refuse(Code::LimitReached, Key::ApiKeyLimit);
    }
    Ok(input)
}

/// Creates a key through the plugin's server API and returns it, the only time it appears;
/// the row keeps its hash.
pub(crate) async fn issue_api_key(
    auth: &BetterAuth<super::library::Schema>,
    user_id: String,
    input: CreateApiKeyInput,
) -> std::result::Result<CreatedApiKey, crate::http::Response> {
    let expires_in = match input.lifetime.as_str() {
        "30d" => Some(30 * DAY_SECONDS),
        "90d" => Some(90 * DAY_SECONDS),
        "1y" => Some(365 * DAY_SECONDS),
        _ => None,
    };
    let scopes = if input.access == "write" {
        vec!["read".into(), "write".into()]
    } else {
        vec!["read".into()]
    };
    let options = CreateKeyOptions {
        name: Some(input.name),
        expires_in: expires_in.map(|seconds| seconds as f64),
        permissions: Some(HashMap::from([("api".into(), scopes)])),
        ..Default::default()
    };
    let created =
        super::library::call(async { auth.api_keys()?.create(&user_id, options).await }).await?;
    Ok(CreatedApiKey {
        id: created.api_key.id,
        key: created.key,
    })
}

/// Deletes one of the user's own keys.
pub fn revoke_api_key(
    db: &Connection,
    user_id: &str,
    input: RevokeApiKeyInput,
) -> Result<RevokedApiKey> {
    let deleted = crate::sql!(
        "delete from api_key where (api_key.id = ",
        &input.id,
        " and api_key.reference_id = ",
        user_id,
        ") returning id"
    )
    .query(db, |row| row.get::<_, String>(0))?;
    if deleted.is_empty() {
        return refuse(Code::NotFound, Key::ApiKeyNotFound);
    }
    Ok(RevokedApiKey { id: input.id })
}

// Refusals only a key's requests get, with English messages and no key.
fn refusal(status: u16, code: &'static str, message: &'static str) -> Error {
    Error::Auth {
        status,
        code,
        message,
    }
}
pub(crate) fn not_allowed() -> Error {
    refusal(403, "FORBIDDEN", "API keys cannot make this call.")
}

/// The key's last use, due to be saved.
#[derive(Debug, PartialEq)]
pub(crate) struct LastUse {
    id: String,
    at: i64,
}

/// The user a key signs in, from one read of the key and its user (keyChecker). A write
/// also counts toward the user's write rate, which the session's calls share. `LastUse` is
/// set when the saved last use is missing or a minute old.
pub(crate) fn key_user(
    db: &Connection,
    key: &str,
    write: bool,
    rate_limits: &MemoryStore,
    allowed_domains: &[String],
    now: i64,
) -> Result<(String, Option<LastUse>)> {
    let found = db
        .prepare_cached(
            "select api_key.id, api_key.reference_id, user.email, api_key.enabled, api_key.expires_at, api_key.permissions, api_key.last_request
             from api_key inner join user on user.id = api_key.reference_id where api_key.key = ?1",
        )?
        .query_row([hash_key(key)], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)? != 0,
                row.get::<_, Option<Timestamp>>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, Option<Timestamp>>(6)?,
            ))
        })
        .optional()?;
    let Some((id, user_id, email, true, expires_at, permissions, last_used_at)) = found else {
        return Err(refusal(401, "UNAUTHENTICATED", "Invalid API key."));
    };
    if expires_at.is_some_and(|at| at.0 <= now) {
        return Err(refusal(401, "UNAUTHENTICATED", "API key expired."));
    }
    if write && access_of(permissions.as_deref()) != "write" {
        return Err(refusal(403, "FORBIDDEN", "API key is read-only."));
    }
    // A key outlives neither its domain's removal from ALLOWED_LOGIN_DOMAINS nor a changed
    // address.
    if !login_domains::allowed(allowed_domains, &email) {
        return Err(refusal(401, "UNAUTHENTICATED", "Email domain not allowed."));
    }
    // Both count, as Promise.all consumes both, before either refuses.
    let per_key = rate_limits.consume(&format!("api-key:{id}"), API_KEY_REQUESTS, now);
    let per_user = !write || rate_limits.consume(&format!("write:{user_id}"), WRITES_PER_USER, now);
    if !per_key {
        return Err(refusal(429, "RATE_LIMITED", "Too many requests."));
    }
    if !per_user {
        return refuse(Code::RateLimited, Key::RateLimited);
    }
    let due = last_used_at.is_none_or(|at| now - at.0 >= LAST_USE_PRECISION_MS);
    Ok((user_id, due.then_some(LastUse { id, at: now })))
}

/// Saves a key's last use. A failure only leaves Settings showing an older one, so it is
/// logged and never fails the call.
pub(crate) fn save_last_use(db: &Connection, last_use: &LastUse) {
    let saved = crate::sql!(
        "update api_key set last_request = ",
        last_use.at,
        " where api_key.id = ",
        &last_use.id
    )
    .execute(db);
    if let Err(error) = saved {
        eprintln!("[api-key] Saving an API key's last use failed: {error}");
    }
}

#[cfg(test)]
mod tests;
