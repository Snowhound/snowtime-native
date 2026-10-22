# 081.41: The kit's first crates, from the auth code

Status: todo

Start the porting kit's crate extraction (081.05, the kit's `docs/architecture.md`, "What
goes in") with the auth code that the kit's auth recipe marks generic
(`recipes/auth/better-auth.md`, "Reference code"). snowtime-native becomes the first
consumer: it depends on the crates and keeps only Snowtime's policy. The design plans this
step for once the port is complete and split, which it is.

## Scope

From `crates/server/src/`:

| Code                       | Lines | Generic part                                                                                           | Stays in snowtime-native                                         |
| -------------------------- | ----: | ------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------- |
| `lane.rs`                  |   234 | The whole lane: admission bounded by count and time, refusal, restart budget                            | The lane sizes                                                   |
| `auth/library/store.rs`    | 1,080 | The store traits on `rusqlite` over lanes, column mapping, `f64` binds, `'null'` metadata, refusing stores | `create_user` and `create_session` hooks: login domains, unverified sign-ups |
| `auth/library/transaction.rs` | 129 | Async transactions on a dedicated writer with their own deadline                                      |                                                                  |
| `auth/library/bridge.rs`   |   179 | Host request in, library answer out; a lane's refusal answered as the host's 503                        | The client-address header's name                                 |
| `auth/cookie.rs`           |   109 | better-call's signed cookies                                                                           |                                                                  |
| `auth/session.rs`          |   280 | The session row and its per-request check, with renewal                                               | Expiry and renewal settings                                      |
| `auth/password.rs`         |    90 | Better Auth's scrypt hashes on AWS-LC                                                                  |                                                                  |

The store reaches the database only through the host's lanes (`App::write_gate` and
`read_gate`), so the lane comes out first, or the store takes a small trait that the app
implements. The lane is in the design's crate list on its own.

## Questions for the maintainer before the first commit

1. **Where the crates live and how snowtime-native builds them.** The kit's repository has
   no remote, and its rules say not to push. A path dependency (`../porting-kit/crates/…`)
   builds on the Mac but not in CI or the images, the problem the fork had before it was
   pushed. Options: a private remote for the kit, a git subtree or vendored copy in
   snowtime-native until the kit is published, or the crates kept in snowtime-native's
   workspace under `kit/` and moved later.
2. **Crate boundaries.** For example `kit-lane`, `better-auth-sqlite` (store, transaction,
   bridge), and `better-auth-session` (cookie, session check, scrypt), or a single crate
   with features. Fewer crates are simpler now; separate ones match the design's rule that
   each crate plugs in alone.
3. **Names and license**, since the crates are meant to be published one day.

## Answers

Answered 2026-10-22:

1. The crates live in the kit's repository, published as github.com/kaitk/porting-kit
   under its working name. snowtime-native depends on them by git URL at an exact
   commit, as it pins the better-auth-rs fork.
2. Open, waiting for a second opinion from Codex. The proposal: three crates in one
   workspace.
   - `<kit>-lane`: `admission.rs` and `lane.rs`, on tokio alone, returning its own
     refusal error that the app answers as its 503 or 500.
   - `better-auth-session`: cookie, session check, and scrypt, with no better-auth-rs
     dependency, so it also serves the hand-ported auth. The session check takes a
     `&Connection` instead of the app's `Gate`.
   - `better-auth-rs-sqlite`: store, transaction, and bridge, reaching lanes through a
     small trait.

   Couplings the scope table doesn't show: `lane.rs` needs `admission.rs`, both answer
   with the app's `Response` and `wire::failure`, and `lane.rs` and `password.rs` count
   into `crate::bench`.
3. Crate names follow the kit's final name. The kit and its crates are MIT OR Apache-2.0,
   like better-auth-rs, and snowtime-native moves to the same.

better-auth-rs moved to v1.0.0-alpha.5 after an 828-commit rewrite since the pinned
`9f91cd7`, and all seven upstream pull requests now conflict. The store is extracted only
after a check against alpha.5, since its traits have probably changed.

## Steps

1. Record the maintainer's answers here and in the kit's `docs/decisions.md`.
2. Extract the lane, with its tests, and switch snowtime-native to it. Run the bar.
3. Extract the store, transaction, and bridge behind traits for what the app supplies:
   the lanes, the user and session hooks, and the client-address header. Move the store
   and transaction tests with them. Run the bar and the library's compatibility harness.
4. Extract the cookie, session check, and scrypt, with Snowtime's settings as
   configuration. Run the bar.
5. Run the performance gate against the `5a5cb9b` baseline: extraction shouldn't change
   CPU, memory, or binary size beyond noise.
6. Update the kit's auth recipe to point at the crates instead of snowtime-native's files.

## Acceptance criteria

- [ ] The three questions answered by the maintainer and recorded.
- [ ] The crates build on their own with their tests, and none depends on Snowtime, Solid,
      or Start.
- [ ] snowtime-native uses them, and its remaining auth code is Snowtime's policy and the
      areas that stay app code. Lines moved and lines left recorded here.
- [ ] The bar unchanged: conformance 66/570/13, `compare.ts` every call byte-equal,
      hardening 17, `page-compare.ts`, server, host, and render tests, cargo-deny,
      cargo-audit, licenses, and CI.
- [ ] The performance gate passes against the `5a5cb9b` baseline.
- [ ] The kit's recipe and architecture updated: crate names, versions, and what the app
      supplies to each.
