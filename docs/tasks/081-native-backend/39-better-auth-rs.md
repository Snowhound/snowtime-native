# 081.39: Auth through better-auth-rs, with passkeys beside it

Status: todo

Move the port's auth onto better-auth-rs, the intended library ([question 5](README.md),
Kait, 2026-10-06), on a branch, and measure what it costs. Three parts stay app code
(Kait, 2026-10-10): the passkey module, the per-request signed-cookie check, and AWS-LC
scrypt sign-in. The passkey module (`crates/server/src/auth/passkeys.rs`, 081.28) uses
`webauthn-rs-core` with TypeScript's preferred user-verification policy; the library's
passkey plugin requires verification, so a registration TypeScript accepts fails there
([spike](auth-spike.md), adoption gate 2). Keeping the module beside the library removes
that gate. Published `1.0.0-alpha.3` targets better-auth 1.6.29; the library's `master`
targets 1.7.6 since 2026-09-30, with typed server-only API-key operations (adoption gate
1); the app runs 1.7.7. Branch `081-auth-spike` in snowtime holds the spike's store and
ceremony tests.

## Acceptance criteria

- [ ] better-auth-rs pinned to an exact commit in `Cargo.lock`, with the better-auth
      version its compatibility harness pins recorded here.
- [ ] Storage through the spike's app-owned `rusqlite` store on the host's lanes: no
      SeaORM or sqlx pool, and async transaction callbacks bounded by a writer-lifetime
      deadline as well as the lane's admission deadline (adoption gate 3).
- [ ] The library serves the auth routes the app uses except passkeys: sessions and
      sign-out, Google and GitHub OAuth (Microsoft may drop if it costs work, Kait,
      2026-10-02), organizations and invitations, and API keys through its server-only
      operations, with its HTTP key endpoints closed.
- [ ] Passkeys stay in `auth/passkeys.rs` on the same tables, with the library's plugin
      unmounted. The cookie check and AWS-LC scrypt sign-in stay app code.
- [ ] The app's policies hold: name checks, `ALLOWED_LOGIN_DOMAINS`, refusing unverified
      sign-ups, and the rate-limit rules, as library hooks or around it.
- [ ] The bar passes unchanged: conformance (66 tests, 570 assertions, 13 files),
      `compare.ts` with every call byte-equal, `hardening-compare.ts` (17), `page-compare.ts`,
      the server, host, and render tests, and CI. The library's own compatibility harness
      passes at the pin (adoption gate 4).
- [ ] Performance: `perf-gate.ts --build-current`, then the gate with the `mac` profile
      against the baseline frozen at `5a5cb9b` on 2026-10-10; the table and any flags are
      recorded here. Then the full kit baseline (README, "Prepare once, then keep the
      baseline").
- [ ] Recorded here: the auth code the library replaced and what stays, in lines; binary
      size; upstream gaps found, with issues or pull requests filed. The porting kit gets a
      recipe either way: the library where it fits, an app-owned recipe where it doesn't.
