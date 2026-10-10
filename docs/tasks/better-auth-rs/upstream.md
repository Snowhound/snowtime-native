# better-auth-rs: upstream drafts (081.39)

Drafts only. Nothing here has been filed or pushed; each waits for Kait's approval. The
fork is the local branch `snowtime` of `better-auth-rs`, on `master` at `9f91cd7`
(2026-09-30). Upstream's compatibility harness pins better-auth `1.7.6`; the app runs
`1.7.7`. Pull requests 1–6 below were drafted at sign-out; the fork also carries the OAuth fixes
listed at the end, not yet drafted. Commit IDs changed with a rebase on 2026-10-10; see
`git log 9f91cd7..snowtime` in the fork.

## Pull requests

### 1. TLS: a `rustls-no-provider` feature (`c63e889d`)

`better-auth-api` refuses to build without `native-tls` or `rustls`, and `rustls` turns
on reqwest's `rustls-tls`, which brings in ring and selects it as reqwest's provider.
An app that installs its own process-wide provider (aws-lc-rs) and shares reqwest with
the library then gets ring in its own HTTP client too. The patch adds
`rustls-no-provider` (reqwest's `rustls-tls-webpki-roots-no-provider`) to the API and
root crates and accepts it in both `compile_error!` checks.

### 2. Error bodies in better-auth's key order (`13ba81f6`, `4edc777a`)

better-auth's `APIError` serializes `{"message": ..., "code": ...}`.
`ErrorCodeMessageResponse` and `CodeMessageResponse` declare `code` first, and four
handlers build bodies with `json!`, which sorts keys without serde_json's
`preserve_order`. Byte-level clients and caches see different bodies from the two
servers. The patch reorders both structs and builds the four `json!` bodies from
`ErrorCodeMessageResponse`.

### 3. Cookies in better-call's attribute order (`29611ee8`)

better-call's `serializeCookie` writes `name=value; Max-Age; Domain; Path; Expires;
HttpOnly; Secure; SameSite`, and Better Auth sets no `Expires` on its auth cookies. The
library builds cookies with the `cookie` crate, which writes `HttpOnly; SameSite; Path;
Max-Age` and adds `Expires` beside `Max-Age`. The patch serializes in better-call's
order and omits `Expires`. A `__Secure-` or `__Host-` name still forces `Secure`.

### 4. Sign-out expires a received `session_data` cookie twice (`02de70fc`, `02f25a4f`)

On sign-out, better-auth expires `session_data` once, then `createSessionStore(...).clean()`
expires every received cache cookie, the unchunked one included, so a request carrying
`session_data` gets two identical expiries before the chunks. The library skipped the
unchunked name in `cookie_cache::clear`, and `finish_response` keeps only the last
`Set-Cookie` per name. The patch includes the unchunked name and lets
`finish_response` keep an identical repeat in place; a later different value for the
same name still replaces the earlier ones, which keeps the 2FA safeguard.

### 5. Malformed JSON before origin checks (`25889963`)

better-call's router parses the body (`getBody`) before any endpoint middleware, so a
malformed JSON body answers 400 `BAD_REQUEST` "Invalid JSON in request body" even from a
foreign origin. The library runs CSRF and origin middleware first and answers 403. The
patch checks bodies whose content type matches better-call's
`application/(…+)?json` before the middleware chain.

### 6. JWT crypto backend as a feature (`c4cefdbd`)

The workspace pins `jsonwebtoken` with `rust_crypto`, which depends on `rsa` 0.9 and so
on RUSTSEC-2023-0071 (Marvin attack, no fixed release). cargo-deny and cargo-audit fail
for any app that denies unfixed advisories. The patch moves the backend to features:
`jwt-rust-crypto` (default, unchanged behavior) or `jwt-aws-lc-rs`.

## Issue: OAuth state follows the pre-1.7 protocol

Not patched: matching it rewrites the state module and the start and callback paths for
both strategies, more than a small fork patch, so 081.39 stopped here. Checked against
the installed `better-auth` 1.7.7 (`dist/state.mjs`, `dist/oauth2/state.mjs`):

| Behavior                       | better-auth 1.7.7                                   | better-auth-rs `9f91cd7`                   |
| ------------------------------ | --------------------------------------------------- | ------------------------------------------ |
| State parameter                | 32 random characters `[A-Za-z0-9_-]`                | UUID v4                                    |
| Verification identifier        | `auth-state:<state>`                                | `oauth:<state>`                            |
| Stored payload                 | includes `oauthState`, checked against the callback | no `oauthState`                            |
| State cookie (database)        | `state`, HMAC-signed value, `Max-Age=300`           | `state`, HS256 JWT `{state, exp, iat}`     |
| State cookie (cookie strategy) | payload encrypted with a purpose-derived key        | payload as an HS256 JWT                    |
| Missing verification row       | `state_mismatch`                                    | `please_restart_the_process`               |
| Cookie or state mismatch       | `state_mismatch`, to the flow's `errorURL`          | `state_mismatch`, to the default error URL |
| PKCE code verifier             | 128 characters                                      | 43 alphanumeric characters                 |

A flow started on one server can't finish on the other, and rows written by better-auth
aren't found. Smaller gaps seen while reading, not yet verified against a running flow:

- Requested scopes replace the provider's defaults and come first; better-auth appends
  them after the defaults.
- A `databaseHooks` refusal (`APIError` with a code) redirects with its code and message
  in better-auth (`error=EMAIL_UNVERIFIED&error_description=...`); the library
  stringifies store errors into `error=<message>` and keeps the code only for banned
  users.

## OAuth fixes in the fork, to draft

- State as better-auth 1.7 stores and checks it (the issue above), cookie strategy still
  a JWT rather than the encrypted payload.
- Zod-order validation of `/sign-in/social`, `/link-social`, and the POST callback.
- Built-in providers' default scopes before requested ones; scopes merged on relink and
  kept on sign-in.
- `list-accounts` key order; `UNAUTHORIZED` from signed-out account and link calls
  (`require_session_middleware`); `email_does_not_match`.
- GitHub's token exchange follows redirects (`OAuthProvider::follow_token_redirects`).
- A database hook's `APIError` redirects with its code and message.
- `unlink-account` requires a fresh session (`require_fresh_session_middleware`, a day by
  default as in better-auth).
- `time::now` and `set_clock`, in place of every `Utc::now()`.

## API key fix in the fork, to draft

- The plugin reads a key's creation time before its expiry and passes it in
  `CreateApiKey::created_at` (`e2595fb2`), as better-auth's insert reads `createdAt` before
  `expiresAt`; the store used to read its own clock after the expiry, so a key's lifetime
  could come out a millisecond short. The compat server sets `follow_token_redirects`
  (`796d9d91`).
