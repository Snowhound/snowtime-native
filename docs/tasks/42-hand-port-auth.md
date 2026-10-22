# 081.42: Auth back on the hand-port

Status: done (2026-10-22): auth back on the 081.28 hand-port, the bar and the gate passing

Return snowtime-native's auth to the hand-port of task 081.28 (tag `native-auth-081.28`),
which passed the full bar with no better-auth-rs. better-auth-rs and the fork leave the
port. The decision, 2026-10-22, follows a review of 081.39 and 081.40 and of
better-auth-rs v1.0.0-alpha.5:

- The library didn't reduce the code the port owns. Auth went from 8,246 lines to 8,476,
  plus a fork diff of +970/−320. The binary grew 4.7%, and peak RSS grew 3–10%.
- Upstream hasn't answered pull requests #119–#125 or issue #126 (filed 2026-10-10), and
  all seven now conflict. Upstream released alpha.4 and alpha.5 after 828 commits, with
  larger store traits (`store/mod.rs` net +2,286 lines).
- alpha.5 links the system ICU C++ library unconditionally (`rust_icu_*` in
  `better-auth-core`), which every build and image would have to supply.
- Probes on alpha.5 found #121 and #123 fixed upstream, #120 and #122 partly fixed, and
  #126 rewritten but still storing the raw state where Better Auth 1.7.7 stores
  `auth-state:<state>`. None of #119, #124, or #125's build features are upstream.

A second opinion from Codex reached the same recommendation: own the Better Auth
compatibility code, in snowtime-native and as the kit's default. Codex's corrections to
the records: alpha.5 hashes with scrypt by default, not Argon2; passkey verification still
needs upkeep as browsers and authenticators change, so it doesn't have zero churn.

## Approach

The tag is an ancestor of `main`, and the code changed since it only for 081.39's library
adoption. So `crates/`, `Cargo.toml`, `Cargo.lock`, `deny.toml`, `THIRD_PARTY_LICENSES`,
and `harness/lines.ts` come back from the tag as they were. The only thing kept from later
commits is the license line in `Cargo.toml`. The app pin moved from `54ae90f` to `981832f`
in the meantime, past docs-only commits, so the restored code needs no app changes.

Since the tag, `main` changed only docs and the license outside the files above:
`harness/scaling/README.md` keeps the 2026-10-22 path cleanup.

## Results

The bar on 2026-10-22, on the Mac: fmt, clippy with and without `bench`, server, host,
and render tests, the bench build, conformance (13 files pass), `compare.ts` ("Every call
answers the same"), and hardening (17 checks) all pass on the restored code.

`cargo deny` and `cargo audit` first failed on advisories published after `main` last
ran them, which fail `main` too: RUSTSEC-2026-0346 to 0349 (rustls 0.23.45),
RUSTSEC-2026-0382 (rustls-webpki 0.103.15), and RUSTSEC-2026-0381 (webpki-roots 1.0.9).
`cargo update` to rustls 0.23.46, rustls-webpki 0.103.16, and webpki-roots 1.0.10 cleared
them, and a second full run passed every step, licenses included.

The performance gate (`perf-gate.ts --build-current`, then the gate with the `mac`
profile against `5a5cb9b`) passed all 32 metrics on its fourth run,
`app/perf/.cache/native-gate/runs/2026-10-22T10-20-55.436Z`:

- The binary is 80,729,920 bytes, the same as the baseline. Load-test CPU is 0.590 ms
  against 0.590, and load-test peak RSS is 48.9 MiB against 48.7.
- Render-harness peak RSS is 227 MB against 212 (+7%), inside its 15-MB band. It was
  +7% in the third run too. Auth isn't on the render path, so look for the cause there.

The Mac wasn't idle: other builds kept the load average between 13 and 33. Two runs were
excluded for 503s from the report budget, one on the baseline binary and one on the
current one (`…T10-09-38.134Z`, `…T10-11-57.239Z`). The third run flagged load CPU at
+9.9%, which the fourth didn't repeat with the same binaries (`…T10-18-44.300Z`). Running
the gate needs `deploy/load/app.patch` applied in `app/` first, as the README says.

## Acceptance criteria

- [x] The bar passes on the restored code: fmt, clippy (with and without `bench`), server,
      host, and render tests, conformance 66/570/13, `compare.ts` every call byte-equal,
      hardening 17, cargo-deny (with all features too), cargo-audit, and licenses.
- [x] No better-auth-rs or fork left in `Cargo.toml`, `Cargo.lock`, or `deny.toml`.
- [x] The performance gate passes against the `5a5cb9b` baseline.
- [x] Records updated:
  - 081.39 and 081.40 marked as superseded;
  - 081.41's crate plan follows the owned compatibility crates;
  - the kit's auth recipe and decisions.
- [x] The upstream pull requests and issue stay open (2026-10-22). Upstream may close them;
      if they are closed without a reply, closing them from our side, with a note, comes
      later.
