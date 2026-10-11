# Native auth library

The port's authentication is app code in `crates/server/src/auth/`. The app's
`docs/architecture/auth.md` holds the rules it follows. This file records the choice of a
Rust auth library.

Question 5's [2026-10-06 spike](../tasks/auth-spike.md) keeps
better-auth-rs as the target, with an app-owned `rusqlite` store on the host's lanes.
Its SeaORM adapter is rejected because its sqlx pool bypasses admission and the
single writer. Async transaction callbacks need a separate writer-lifetime deadline;
the lane's admission deadline alone does not bound them.

Kait chose better-auth-rs on 2026-10-10 ([081.39](../tasks/39-better-auth-rs.md)). It
serves sign-out, OAuth, and API key creation, pinned to the fork
[`kaitk/better-auth-rs`](https://github.com/kaitk/better-auth-rs) at an exact commit
while its fixes wait upstream. Organizations, invitations, profile writes, passkeys,
password sign-in, and the per-request cookie check stay app code: the library's
organization roles and passkey verification policy differ from Better Auth's, and it
hashes passwords with Argon2 where Better Auth uses scrypt.
