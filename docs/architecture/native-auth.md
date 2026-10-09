# Native auth library

The port's authentication is app code in `crates/server/src/auth/`. The app's
`docs/architecture/auth.md` holds the rules it follows. This file records the choice of a
Rust auth library.

Question 5's [2026-10-06 spike](../tasks/081-native-backend/auth-spike.md) keeps
better-auth-rs as the target, with an app-owned `rusqlite` store on the host's lanes.
Its SeaORM adapter is rejected because its sqlx pool bypasses admission and the
single writer. Async transaction callbacks need a separate writer-lifetime deadline;
the lane's admission deadline alone does not bound them.

Production adoption remains open. Published `1.0.0-alpha.3` lacks public server-only
scoped API-key calls and refuses passkey registrations without user verification
that TypeScript accepts. Its session cookies also carry raw tokens. The spike proves
storage and supplies a signed-cookie boundary and app policies; it does not mount
new production routes. A pinned release or reviewed patch must pass the upstream
compatibility harness and the app's HTTP conformance suite before mounting it.
The native per-request cookie check and AWS-LC password sign-in stay in app code.
