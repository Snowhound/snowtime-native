# 081.39: Auth through better-auth-rs, with passkeys beside it

Status: in progress (2026-10-10): sign-out, OAuth, and API key creation on the library; the gate passed except binary size; the choice between better-auth-rs and an 081.28 kit crate is Kait's

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
local fork (`../better-auth-rs`, branch `snowtime`, 20 commits on `9f91cd7`), so
`Cargo.lock` records path packages until Kait approves pushing the fork; then the patch
names the fork's URL and commit. CI can't build the branch before that. At the fork's
head (`796d9d91`) the library's compatibility harness passes: the dual-server client suite
against better-auth 1.7.6 (`full_client_compat` and `configuration_client_compat`) and the
workspace tests with Axum, SeaORM, and Redis (669). It runs without devenv once both
compat directories have their packages installed. Pull requests and the OAuth
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

### OAuth (`433d762`)

Kait chose to fix the library's OAuth in the fork (2026-10-10). Sign-in, linking, the
callback, `list-accounts`, and `unlink-account` go to the library's OAuth and account
plugins. The fork rewrote the state layer to better-auth 1.7's protocol and fixed eleven
smaller gaps; [upstream.md](better-auth-rs/upstream.md) lists them. App code beside the
library: `library/providers.rs` (Google's and Microsoft's ID-token profiles, Microsoft's
Graph photo, the fake provider's transport), `library/policy.rs` (the login-domain
middleware and the app's input bounds), the store's hook refusals (`EMAIL_UNVERIFIED`,
`LOGIN_DOMAIN_NOT_ALLOWED`), and the bridge's Latin-1 headers and callback-URL encoding.
The library follows the host's clock through the fork's `time::set_clock`, so `PERF_NOW`
moves it as it moves the app. The bar is unchanged: conformance 66/570/13, `compare.ts`
1,490 byte-equal (128 OAuth calls), hardening 17. The native `oauth.rs` (1,370 lines) and
its validators are removed.

### Organizations: kept as app code

Mounting the library's organization plugin matched 38 of 141 auth-writes steps. Beyond
key order and validation messages, its role and permission behavior differs from
better-auth's: a member could remove themselves, an unknown role was saved, an admin
couldn't rename the organization. Kait chose (2026-10-10) to keep 081.28's organization,
invitation, and profile writes as app code beside the library, measure the whole port,
and then decide between better-auth-rs and promoting 081.28's port to a kit crate. The
attempt is saved in `temp/081.39/org-experiment.patch`; `temp/081.39/scripts/writes-debug.ts`
prints every differing auth-writes step even when the flow throws.

### API keys

The app uses the plugin only to create a key, through its server API, because only the
plugin generates one; listing, revoking, and the per-request key check read and change
the rows directly (`api-keys.server.ts`). The port does the same: `POST /api/v1/api-keys`
counts the user's keys on the writer's lane, then calls `auth.api_keys().create` with the
app's `apiKeyOptions` (`library/mod.rs`), which takes the writer again for its insert and
the expired-key cleanup. `verify_api_key` isn't used, as the app doesn't use
`verifyApiKey`: it would add two writes to every keyed request. No route reaches the
plugin's HTTP endpoints, as `apiKeyDisabledPaths` closes them in the app.

`AsUser::then_library` (`http.rs`) runs a rule's checks on a lane and hands its result to
the library; `library::call` answers a lane's refusal with the host's 503. The store
binds the library's `f64` integers, writes a missing metadata as the text `'null'`, and
deletes keys that expired strictly before now, each as better-auth's adapter and plugin do.
One fork fix: the plugin reads the creation time before the expiry and passes it in
`CreateApiKey`, as the TypeScript plugin's insert does, so a 30-day key's lifetime is never
a millisecond short.

The bar is unchanged: conformance 66/570/13, `compare.ts` 1,490 byte-equal (the API key
rows column by column included), hardening 17, server tests 128 both ways, host 36,
render 18, Clippy and cargo-deny both ways, cargo-audit. `api_keys.rs` drops from 273 to
249 lines (key generation, the insert, and the expired-key cleanup); the library side
adds about 100.

### Before measuring

The host builds the library at startup (`App::start_auth`), so a configuration it
refuses stops the server instead of failing each request. `THIRD_PARTY_LICENSES` is
unchanged: the API key plugin was already in the graph. Since `47b74fc`, the server's
auth code grows from 8,246 to 8,665 lines (tests included): `library/` adds 2,208,
`callback_urls.rs` 101 moved out of the removed code, and `oauth.rs` (1,370),
`sign_out.rs` (183), and most of the OAuth and sign-out schemas (about 300) are gone. The
stripped release `bench` binary on macOS arm64 is 78,072,464 bytes, up 4,119,808 (5.6%)
from `47b74fc`. The bar is unchanged after both: conformance 66/570/13, `compare.ts` 1,490,
server 128, host 36, Clippy both ways.

### Performance gate

`perf-gate.ts --build-current` at `11220ac` (fork `796d9d91`), then the gate with the `mac`
profile against the baseline frozen at `5a5cb9b`; evidence in
`app/perf/.cache/native-gate/runs/2026-10-10T11-13-07.546Z`. The images can't reach the fork
beside the repository, so the gate exports its committed tree and passes it as a named
build context, which `deploy/Dockerfile` and `adapter/bench/Dockerfile` copy to
`/better-auth-rs` (an empty stage otherwise).

Every render, API, and load metric passes. The one flag is binary size, which doesn't vary
between runs, so it wasn't repeated:

| Metric                       |   Baseline |    Current | Delta | Band | Gate       |
| ---------------------------- | ---------: | ---------: | ----: | ---: | ---------- |
| render/timer/cpu_ms          |     14.939 |     14.696 | -1.6% | 1.49 | pass       |
| render/year/p95_ms           |     27.285 |     30.128 | 10.4% | 5.71 | pass       |
| render/\*/peak_rss_mb        |    212.184 |    230.910 |  8.8% | 18.7 | pass       |
| api/session/cpu_ms           |      0.120 |      0.130 |  8.3% | 0.02 | pass       |
| api/entries/cpu_ms           |      0.430 |      0.440 |  2.3% | 0.02 | pass       |
| load/cpu_ms                  |      0.698 |      0.706 |  1.2% | 0.04 | pass       |
| load/peak_rss_mib            |     48.852 |     50.914 |  4.2% |  4.0 | pass       |
| static/binary_bytes          | 80,729,920 | 84,924,952 |  5.2% | 5.0% | REGRESSION |
| static/render_bundle_bytes   |  2,001,714 |  2,001,714 |  0.0% |      | pass       |

The rows left out are within 6% and their bands. The render harness's peak RSS rises
by about 19 MB in all four pages, inside the band; the load server's by 2 MiB. The size
comes from the library's plugins and CLI the port doesn't mount (above). The full kit
baseline hasn't run.

### Rollback

The tag `native-auth-081.28` marks `47b74fc`, the port before this task: every auth flow
hand-ported in 081.28, no better-auth-rs dependency, and the full bar passing (conformance
66/570/13, `compare.ts` 1,490 byte-equal, hardening 17). To return to it, branch from the
tag, or revert this task's merge, and drop the `[patch]` and better-auth-rs entries from
`Cargo.toml`. The organization, invitation, and profile writes there are the same code
this branch keeps. Forward-port any app features added since by rerunning the bar.

### Next

1. Kait decides: keep better-auth-rs, accepting the 5.2% binary size (or trim the
   library's unmounted plugins behind features in the fork), or promote 081.28's port
   to a kit crate. Organizations, invitations, and profile writes stay app code either way.
2. Nothing is pushed, here or in the fork, until Kait approves (2026-10-10).
