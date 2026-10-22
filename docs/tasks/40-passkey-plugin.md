# 081.40: Passkeys through better-auth-rs

Status: done (2026-10-11): passkeys stay app code; the library's plugin matched 29 of 151 steps, 44 with the clock worked around

Check whether better-auth-rs's passkey plugin can replace the app's passkey module
(`crates/server/src/auth/passkeys.rs`, 927 lines, from 081.28). 081.39 kept the module
because alpha.3's plugin refused registrations without user verification; upstream fixed
that in `f4470b0a` (2026-09-30), before the pin, with a dual-server compat test. Whether
the plugin answers Snowtime's flows byte for byte hasn't been tried. Work on a branch from
`main` after 081.39 merges, the way 081.39 tried organizations: mount, compare, count,
then decide with the maintainer.

## Steps

1. Turn on the fork's `passkey` feature for `better-auth-api` and `better-auth`. Run
   cargo-deny and `cargo tree -i` for ring and `rsa`; the feature adds `webauthn-rs-core`
   and `serde_cbor_2` from the library's side.
2. Implement `PasskeyStore` in `auth/library/store.rs` on the app's `passkey` table, in
   place of the refusals. The spike's adapter rebuilt webauthn-rs's credential from the
   stored COSE key, credential ID, counter, device type, and backup flag, with no extra
   column ([auth spike](auth-spike.md)). Snowtime stores no registration UV history, so
   imported rows use the unverified, preferred policy.
3. Mount `PasskeyPlugin` with the app's options (`passkey({ rpID, rpName: 'Snowtime',
   origin })` in `app/src/server/auth/better-auth.server.ts`, the rest defaults) and route
   the six `/api/auth/passkey/*` routes to the library.
4. Run `harness/passkey-compare.ts`, conformance, and `compare.ts`. Count the differing
   steps and sort them into format differences (key order, messages, cookie attributes),
   which are fork fixes, and behavior differences (accepted or refused ceremonies,
   counters, stored rows), which keep the module.
5. Stop and report to the maintainer with the count and the list before fixing anything in
   the fork.

## Acceptance criteria

- [x] The experiment's result recorded here: steps matched out of the flow's total, each
      difference classified as format or behavior, and the fork fixes it would take.
- [x] The maintainer's decision recorded: the library's plugin or the app's module.
- [ ] (Not applicable: the module stays.) If the plugin: `passkeys.rs` removed, the bar unchanged (conformance 66/570/13,
      `compare.ts` every call byte-equal, `passkey-compare.ts`, hardening 17,
      `page-compare.ts`, server, host, and render tests, CI), each fork fix with a test on
      its own branch and offered upstream, and the lines removed and added, the binary
      size, and `THIRD_PARTY_LICENSES` recorded.
- [x] If the module: the experiment saved as a patch in `temp/081.40/`, as
      `temp/081.39/org-experiment.patch` was.
- [x] The kit's auth recipe (`recipes/auth/better-auth.md`) updated with the passkey
      verdict and its evidence.

## Experiment, 2026-10-11

Branch `081.40-passkey-plugin` from `main` at `8286201`, fork at `1508ab79`. Saved as
`temp/081.40/passkey-experiment.patch`; the scripts and logs are in `temp/081.40/`.

- The `passkey` feature adds no crates: `webauthn-rs-core` 0.5.5 and `serde_cbor_2` were
  already in the graph through `passkeys.rs`, so `Cargo.lock` gains two dependency edges
  and `THIRD_PARTY_LICENSES` doesn't change. No ring, no `rsa`; cargo-deny passes both
  ways.
- `PasskeyStore` on the `passkey` table (+79 lines net) rebuilds webauthn-rs's credential
  from the row on every read, with no extra column. A row better-auth wrote signs in
  through the plugin with its counter stored.
- The plugin is mounted with `rpID`, `rpName: 'Snowtime'`, and `origin` as in
  `better-auth.server.ts`, and the six routes go to the library. `update-passkey` stays
  unrouted, as before.

### Result

The flow has 151 steps: 141 in `passkey-flow.ts` for three algorithms, and 10 fixtures.
`passkey-compare.ts` stops at its first step, so `temp/081.40/scripts/passkey-debug.ts`
runs the same steps without stopping at a mismatch. The baseline binary matches 151 of
151.

| Run                                                            | Same of 151 |
| -------------------------------------------------------------- | ----------: |
| The experiment as the bar runs it                              |          29 |
| Both servers on `localhost` instead of `127.0.0.1`             |          29 |
| Also the challenge's expiry on the library's clock (fork patch) |          44 |

Two harness-only causes hide everything else, and one more is the port's own:

- The plugin refuses an IP address as the RP ID (`build_webauthn` uses `Url::domain`),
  and the harness serves on `127.0.0.1`, so every options call answers 500. Better Auth
  accepts it. Browsers don't, so production doesn't hit this, but the bar does.
- The challenge cookie is a JWT whose `exp` the plugin sets with the library's clock
  (`time::now`) and `jsonwebtoken` checks against the system clock. Under `PERF_NOW` every
  challenge is eleven days expired. `oauth/state.rs`, `admin`, and `cookie_cache.rs`
  check JWT expiry the same way. The third run patches `decode_challenge_cookie` in a
  scratch worktree (`temp/081.40/fork-clock-measurement.patch`); the fork is unchanged.
- The store's `consume_verification*` compared expiry with `Utc::now()`, not the host's
  clock, which no route had called until now. Fixed in the experiment.

Outside passkeys nothing moved: `compare.ts` without its passkey section has 1,339 calls
byte-equal (1,490 less the 151). Conformance passes 65 of 66 tests with 433 of 570
assertions: the passkey contract fails at its first registration options, on the RP ID.
Server tests pass (128).

### The 107 differing steps

A step can have more than one cause, so the rows add up to more than the total. Counted
once each, as behavior if any behavior difference touches them, 83 steps differ in format
and 24 in behavior.

| Kind     | Steps touched | Difference                                                                                                         | Fork fix                                                     |
| -------- | ------------: | ------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------ |
| Format   |    47 | Key order: options (`challenge`, `rp`, `user` first), passkeys (`id` first), sessions, descriptors                 | Serialize in Better Auth's order                             |
| Format   |    22 | A refusal's status or message: 500 for a bad attestation (400), 403 for another user's key or a removed key (401), 400 for a bad signature (401), 500 for an authentication challenge used to register (400 `CHALLENGE_NOT_FOUND`), `{"message":"Unauthorized"}` for another user's deletion (empty 401) | Statuses and messages as the TypeScript plugin answers       |
| Format   |    12 | Validation messages in serde's words, not Zod's, and `response` checked after the challenge                         | Zod-order validation, as done for OAuth                      |
| Format   |     2 | Signed out: `AUTHENTICATION_REQUIRED` for `UNAUTHORIZED`                                                           | The session middleware's error                               |
| Behavior |    12 | The passkey name isn't trimmed, so the stored row keeps the spaces                                                 | Trim the name                                                |
| Behavior |     4 | The new session's `ipAddress` is stored empty: the plugin reads `x-forwarded-for`, not the configured header. The body also adds the user's `username`, `role`, `banned`, and other fields, and `impersonatedBy` | Use the configured IP headers; the app's user fields          |
| Behavior |     2 | A credential registered without backup eligibility that later asserts it is refused at discoverable sign-in (`allow_backup_eligible_upgrade` is off there); Better Auth accepts it | Allow the upgrade, as Better Auth does                       |
| Behavior |     2 | The counter listing after that refusal (cascade)                                                                   | —                                                            |
| Behavior |     2 | No fresh-session check: a day-old session gets registration options                                               | `freshSessionMiddleware` on both registration routes          |
| Behavior |     3 | A challenge the TypeScript server issued isn't accepted (better-call signed cookie and its JSON state, against a JWT cookie and webauthn-rs state) | Store and sign challenges as Better Auth does                |
| Behavior |     1 | `createSession` on registration is ignored                                                                         | Implement it                                                 |
| Behavior |     1 | An invalid `authenticatorAttachment` is accepted and echoed                                                        | Validate the enum                                            |

44 of 151 steps match. The 24 behavior steps come from seven causes and one cascade. Not
counted: the store writes the backup flags the plugin passes after sign-in, where Better
Auth updates only the counter. The upgrade refusal hides this in the flow, and the store
could write only the counter.

Each behavior difference has a contained fix, mostly a few lines in the plugin. The
challenge format is the largest, the same kind of rewrite as OAuth state, and it matters
only for ceremonies in flight when a deployment switches backends (five minutes).

### Lines

The experiment adds 127 lines and removes 32. Moving to the plugin would remove about
1,025: `passkeys.rs` (927, about 280 of them tests), `passkey_issues` (70), the passkey
name bound and its test (about 18), and the glue in `http.rs`, `auth/mod.rs`, and a
schema test. The fork fixes above aren't written. By the OAuth work's measure (eleven
fixes and a state rewrite, about 900 lines), they come to several hundred lines in the
fork plus tests: format fixes in the handlers and their wire types, the behavior fixes,
and the challenge format.

### Decision

The maintainer chose the app's module (2026-10-11): `passkeys.rs` stays, the plugin stays
unmounted, and the store keeps refusing passkey calls. The plugin would move about 1,000
lines from the port into the fork rather than remove them, passkeys have no providers or
protocol churn for upstream to maintain, and upstream hasn't answered the seven pull
requests yet. Reconsider once upstream merges those and the OAuth state issue is settled;
the experiment is in `temp/081.40/passkey-experiment.patch`.

Kept from the experiment: the store's `consume_verification` and
`consume_verification_by_identifier` compare expiry with the library's clock instead of
`Utc::now()`, so they follow `PERF_NOW` as the rest of the store does. Not filed: the
fork's JWT expiry checks ignore `time::set_clock` (passkey challenges, OAuth cookie state,
admin, the session cookie cache); it matters only under a test clock, and the port mounts
none of those paths.
