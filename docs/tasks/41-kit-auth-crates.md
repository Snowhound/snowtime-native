# 081.41: The kit's first crates, from the auth code

Status: todo

Start the porting kit's crate extraction (081.05, the kit's `docs/architecture.md`, "What
goes in") with the auth code that the kit's auth recipe marks generic
(`recipes/auth/better-auth.md`, "Reference code"). snowtime-native becomes the first
consumer: it depends on the crates and keeps only Snowtime's policy. The design plans this
step for once the port is complete and split, which it is.

## Scope

From `crates/server/src/`, after 081.42 restores 081.28's hand-port. The kit's task 002
reviews the seams and the behavior each extraction must keep.

| Crate                       | Code                                                                 | Stays in snowtime-native                                   |
| --------------------------- | -------------------------------------------------------------------- | ---------------------------------------------------------- |
| `<kit>-lane`                | `admission.rs` (190) and `lane.rs` (234): admission bounded by count and time, refusal, cancellation | Lane sizes, the 503 and 500 bodies, benchmark counters |
| `<kit>-better-auth-session` | `auth/cookie.rs` (109), `auth/session.rs` (280), `auth/password.rs` (90) | Expiry and renewal settings, login domains, user-agent bounds |
| `<kit>-better-auth-compat`  | Sign-in (472), sign-out (183), OAuth (1,370), API keys (273), passkeys (927), organization writes (1,711), the error page (486), and their schemas (1,194) | HTTP composition, Snowtime's roles and validation, the providers' configuration |

The lane's restart budget belongs to the renderer (081.17), not to `lane.rs`.

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
   commit, never by a path to a sibling checkout.
2. Three crates in one workspace, as in the scope table: the lane, Better Auth sessions,
   and Better Auth compatibility. The third was first planned as a better-auth-rs storage
   adapter; on 2026-10-22 the port went back to its own auth code (081.42), and the kit
   owns its Better Auth compatibility code, with better-auth-rs only as a reference
   (porting-kit `docs/decisions.md`). Codex reviewed the boundaries and agreed; its
   review is the kit's task 002.

   Couplings the scope table doesn't show: `lane.rs` needs `admission.rs`, both answer
   with the app's `Response` and `wire::failure`, and `lane.rs` and `password.rs` count
   into `crate::bench`. The session check returns a result the app acts on when renewal
   needs the writer, instead of taking the app's `Gate`.
3. Crate names follow the kit's final name. The kit and its crates are MIT OR Apache-2.0,
   and snowtime-native moved to the same.

## Steps

1. Record the maintainer's answers here and in the kit's `docs/decisions.md`.
2. Extract the lane, with its tests, and switch snowtime-native to it. Run the bar.
3. Extract the cookie, session check, and scrypt, with Snowtime's settings as
   configuration. Run the bar.
4. Extract the auth handlers into the compatibility crate one area at a time, behind what
   the app supplies: execution on its lanes, its hooks, and its HTTP composition. Run the
   bar after each area.
5. Run the performance gate against the `5a5cb9b` baseline: extraction shouldn't change
   CPU, memory, or binary size beyond noise.
6. Update the kit's auth recipe to point at the crates instead of snowtime-native's files.

## Acceptance criteria

- [x] The three questions answered by the maintainer and recorded (2026-10-22).
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
