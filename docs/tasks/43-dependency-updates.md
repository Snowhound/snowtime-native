# 081.43: Dependency updates, deno_core 0.413 and V8 150.4

Status: in-progress

Move to the newest releases that can affect performance, and measure them against the
`5a5cb9b` baseline. Checked on 2026-10-22:

| Crate        | Was     | Now     | Note |
| ------------ | ------- | ------- | ---- |
| `deno_core`  | 0.405.0 | 0.413.0 | With `deno_web` 0.291 and `deno_webidl` 0.260 |
| `v8`         | 149.4.0 | 150.4.0 | Through `deno_core`'s new `deno_v8` facade, which requires `^150.4`; the newest `v8`, 152.2, has no `deno_core` release yet |
| `tokio`      | 1.53.2  | 1.54.1  | |
| `hyper`      | 1.11.1  | 1.12.0  | |
| `aws-lc-rs`  | 1.18.1  | 1.18.2  | |

rusqlite 0.40.2 and axum 0.8.9 are current.

Task 081.18 found no win from V8 150.4 (`deno_core` 0.412) in microbenchmarks, and its
synthetic render failed calibration. This is the first measurement of V8 150.4 on the real
bundle through the performance gate.

`deno_v8` also offers an optional QuickJS backend behind `deno_core`'s `quickjs` feature.
It isn't tried here: the rendering spike already rejected QuickJS, which needs Intl
polyfills and rendered too slowly ([native rendering](../architecture/native-rendering.md#rejected)).

## Changes

- `crates/render/Cargo.toml`: `deno_core`, `deno_web`, and `deno_webidl` bumped together.
- `crates/render/vendor/deno_fetch`: `20_headers.js`, `21_formdata.js`, and `22_body.js`
  from deno_fetch 0.284.0, the release for `deno_web` 0.291. The 0.276.0 scripts failed
  the snapshot (`webidl.createAsyncIterableConverter is not a function`). The local change
  to `22_http_client.js` stays; upstream didn't change that file.
- `crates/server/Cargo.toml`: `aws-lc-sys` 0.45 → 0.46, the version `aws-lc-rs` 1.18.2
  needs. With both, cargo-deny refused the duplicate and `licenses/generate.sh` found two
  copies; the scrypt tests pass on 0.46.
- `deploy/v8-archive.sh`: SHA-256 for the 150.4.0 Linux archives (x86_64 `f48762ca…`,
  aarch64 `539e2838…`), matching GitHub's published asset digests. The 149.4.0 entries
  stay while CI builds a base commit that locks them.

## Acceptance criteria

- [x] The bar passes (2026-10-22, the second run; the first refused the duplicate
      `aws-lc-sys`).
- [ ] The performance gate against `5a5cb9b` on a quiet machine, with render CPU and peak
      RSS compared with 081.42's run (`2026-10-22T10-20-55.436Z`).
- [ ] CI passes, including the image build with the new V8 archive.
