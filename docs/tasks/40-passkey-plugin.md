# 081.40: Passkeys through better-auth-rs

Status: todo

Check whether better-auth-rs's passkey plugin can replace the app's passkey module
(`crates/server/src/auth/passkeys.rs`, 927 lines, from 081.28). 081.39 kept the module
because alpha.3's plugin refused registrations without user verification; upstream fixed
that in `f4470b0a` (2026-09-30), before the pin, with a dual-server compat test. Whether
the plugin answers Snowtime's flows byte for byte hasn't been tried. Work on a branch from
`main` after 081.39 merges, the way 081.39 tried organizations: mount, compare, count,
then decide with Kait.

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
5. Stop and report to Kait with the count and the list before fixing anything in the fork.

## Acceptance criteria

- [ ] The experiment's result recorded here: steps matched out of the flow's total, each
      difference classified as format or behavior, and the fork fixes it would take.
- [ ] Kait's decision recorded: the library's plugin or the app's module.
- [ ] If the plugin: `passkeys.rs` removed, the bar unchanged (conformance 66/570/13,
      `compare.ts` every call byte-equal, `passkey-compare.ts`, hardening 17,
      `page-compare.ts`, server, host, and render tests, CI), each fork fix with a test on
      its own branch and offered upstream, and the lines removed and added, the binary
      size, and `THIRD_PARTY_LICENSES` recorded.
- [ ] If the module: the experiment saved as a patch in `temp/081.40/`, as
      `temp/081.39/org-experiment.patch` was.
- [ ] The kit's auth recipe (`recipes/auth/better-auth.md`) updated with the passkey
      verdict and its evidence.
