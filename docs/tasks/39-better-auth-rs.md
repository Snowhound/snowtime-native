# 081.39: Auth through better-auth-rs, with passkeys beside it

Status: stopped at OAuth, awaiting Kait (2026-10-10)

Move the port's auth onto better-auth-rs, the intended library ([question 5](README.md),
Kait, 2026-10-06), on a branch, and measure what it costs. This applies the kit's auth
recipe ([05](05-porting-recipes.md), Kait, 2026-10-08): the library serves the flows,
its gaps are fixed in a fork and offered upstream, what it lacks goes in a crate beside
it, and 081.28's hand-ported flows are the fallback for gaps.

Passkeys are such a gap. The library's passkey plugin requires user verification, so a
registration TypeScript accepts fails there ([spike](auth-spike.md), adoption gate 2).
Here the app's passkey module (`crates/server/src/auth/passkeys.rs`, 081.28, on
`webauthn-rs-core` with TypeScript's preferred policy) stays beside the library (Kait,
2026-10-10), and the policy fix is offered upstream. The per-request signed-cookie check
and AWS-LC scrypt sign-in also stay app code. This is a separate bug from snowtime's task
092, whose patch answers a malformed registration with 400 instead of Better Auth's 500.

Published `1.0.0-alpha.3` targets better-auth 1.6.29; the library's `master` targets 1.7.6
since 2026-09-30, with typed server-only API-key operations (adoption gate 1); the app
runs 1.7.7. The spike's store and ceremony tests are at `ae0ec17` in snowtime, reachable
from the tag `archive/081-native-poc`, before 081.37 removed them.

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
      unmounted, and an issue or pull request upstream for the verification policy. The
      cookie check and AWS-LC scrypt sign-in stay app code.
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

## Progress, 2026-10-10

Branch `081.39-better-auth-rs`. Sign-out moved to the library with the bar unchanged;
OAuth stopped at a library gap larger than a small fork patch, as the task's rules
require. Organizations, invitations, and API keys haven't started, and the performance
gate hasn't run.

### Pin and fork

`Cargo.toml` names better-auth-rs `master` at `9f91cd7` (2026-09-30), whose
compatibility harness pins better-auth `1.7.6`. A workspace `[patch]` points it at the
local fork (`../better-auth-rs`, branch `snowtime`, eight commits on `9f91cd7`), so
`Cargo.lock` records path packages until Kait approves pushing the fork; then the patch
names the fork's URL and commit. CI can't build the branch before that. The library's
own compatibility harness hasn't run at the fork's head. Pull requests and the OAuth
issue are drafted in [better-auth-rs/upstream.md](better-auth-rs/upstream.md), none filed.

The library adds no SeaORM or sqlx. Two fork features keep the host's crypto: reqwest
without ring (`rustls-no-provider`), and JWTs on aws-lc-rs (`jwt-aws-lc-rs`), which also
drops `rsa` and its unfixed RUSTSEC-2023-0071 advisory. `deny.toml` allows the
better-auth-rs git source; `THIRD_PARTY_LICENSES` is regenerated.

### Sessions and sign-out (`5abac81`)

- `auth/library/`: the spike's `rusqlite` store on master's traits, reads on a reader's
  lane where the host runs readers, writes on the writer's. The store holds a weak
  reference to `App`, which builds the library on first use. Two-factor, device codes,
  and passkeys refuse in the store; their plugins aren't mounted.
- The bridge passes the host's request, with the host's resolved client address, and
  answers a lane refusal with the host's 503 and `Retry-After` instead of the library's
  500 (adoption gate 3; `sign_out_answers_the_writer_lanes_refusal`). The spike's
  transaction test passes on master: commit, rollback, an 80 ms writer deadline, and
  cancellation.
- `POST /api/auth/sign-out` goes to the library's session plugin. Five fork fixes made
  it byte-equal: error-body key order, cookie attribute order without `Expires`, the
  repeated `session_data` expiry, and malformed JSON refused before the origin check.
  The library's rate limiting is off; the host's limits wrap `/api/auth` as before.
- The per-request cookie check, scrypt sign-in, and passkeys stay app code.

The bar at `5abac81`: Clippy with and without `bench`, server tests (140, both ways),
host and render tests, conformance 66 tests with 570 assertions in 13 files, `compare.ts`
1,490 calls byte-equal, `hardening-compare.ts` 17, cargo-deny both ways, and cargo-audit.

| Auth code                          | Lines |
| ---------------------------------- | ----: |
| Replaced: sign-out handler         |    82 |
| Replaced: sign-out body schema     |    42 |
| Kept: callback-URL checks, moved   |   101 |
| Added: `library/` (incl. 176 test) | 1,733 |

The store is most of the added lines and serves every later area.

The release `bench` binary on macOS arm64, stripped, grows from 73,952,656 bytes at
`47b74fc` to 76,260,240 (+2.2 MiB, 3.1%). The library brings second copies of `rand`,
`sha2`, `hmac`, and `base64`, plus `clap`, `serde_yaml`, `totp-rs`, and `aes-gcm` from
plugins and a CLI the port doesn't mount. The gate's Linux sizes are pending its run.

### OAuth: stopped

The library's OAuth state follows the pre-1.7 protocol: UUID states under `oauth:`, a
JWT state cookie, no `oauthState`, other error codes, and a 43-character verifier. The
comparison's flow fails at its first check (state format), and better-auth's rows and
cookies don't interoperate. Matching 1.7.7 means rewriting the state module and the
start and callback paths for both strategies. Two smaller gaps sit behind it: scope
order and hook refusals' redirect codes. Details are in the
[OAuth issue draft](better-auth-rs/upstream.md#issue-oauth-state-follows-the-pre-17-protocol).
The provider wiring tried (Google and Microsoft profiles from the ID token, Microsoft's
Graph photo, the fake provider's transport) is saved, uncommitted, in
`temp/081.39/oauth-experiment.patch`.

Kait to choose: fix the state layer in the fork and offer it upstream, keep 081.28's
OAuth beside the library, or wait for upstream.
