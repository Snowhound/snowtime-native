# 081.50: What leaving deno_core for the bare `v8` crate would take

Status: done (2026-10-22): read-only audit; the recommendation is to stay on deno_core

Cloudflare is buying Deno (announced 2026-10-09), and deno_core gets only security releases
until about October 2027. This audit scopes moving the render crate to the bare `v8` crate,
without benchmarks. Versions: deno_core 0.413.0, deno_ops 0.289.0, serde_v8 0.322.0,
deno_web 0.291.0, deno_webidl 0.260.0, deno_error 0.7.1, v8 150.4.0.

## What the pages use

The isolate registers 288 ops. A host built with deno_core's op metrics hook (a local patch,
not committed) rendered `harness/page-compare.ts`'s 59 pages, all byte-equal. They call 22
ops:

| From | Ops |
| --- | --- |
| The render crate | `op_send` (async), `op_head`, `op_chunk`, `op_render_encode` |
| deno_core | `op_run_microtasks`, `op_drain_pending_rejections`, `op_timer_now`, `op_timer_schedule`, `op_timer_track`, `op_timer_untrack`, `op_encode`, `op_decode`, `op_structured_clone`, `op_is_any_array_buffer`, `op_is_array_buffer`, `op_is_shared_array_buffer`, `op_is_typed_array`, `op_is_proxy` |
| deno_web | `op_url_parse_with_base`, `op_url_get_serialization`, `op_url_parse_search_params`, `op_url_stringify_search_params` |

The handoff's 360 ops weren't recorded with a version; 288 is the count at deno_core
0.413.

The bootstrap (`adapter/bootstrap.js`) loads 21 scripts, with their dependencies:
23,227 lines from deno_web (17,497), deno_webidl (1,693), and the vendored deno_fetch
scripts (4,037). They name 53 ops, 2 of them in the fetch scripts and registered by no
extension. Loading them needs only an op table where unknown names are undefined; the
pages call 4 of the 53. The scripts also
rely on deno_core's own JS: primordials (650 lines), infra (551), core (1,324), and
timers (434).

## deno_core's API and its `v8` crate equivalent

| The render crate uses | With the bare `v8` crate |
| --- | --- |
| `snapshot::create_snapshot` in `build.rs` | `v8::Isolate::snapshot_creator` with the ops' external references, `set_default_context`, `create_blob` |
| `JsRuntime::new` with the snapshot, extensions, and `CreateParams` | `v8::Isolate::new` with `snapshot_blob` and the same external references; `Context::from_snapshot` |
| `extension!` and `#[op2]` (`op_head`, `op_chunk` fast, `op_render_encode`) | `FunctionTemplate`s on an ops object, the state behind `v8::External` |
| `#[serde]` arguments and results, `ToJsBuffer` | Reading objects by hand, or `v8::json`; a `Uint8Array` over a backing store from the `Vec` |
| The async `op_send` | A `PromiseResolver` per call, resolved when the Tokio future finishes |
| `OpState` (`put`, `borrow`, `take`) | A Rust struct the renderer owns, reached from callbacks through the isolate's slot |
| `execute_script` and `JsError` | `Script::compile` and `run` under a `TryCatch`, with the message and stack formatted |
| `resolve` and `with_event_loop_promise` | A loop: microtask checkpoint, pending op futures, due timers, until the promise settles; a promise-reject callback for unhandled rejections |
| Core timers (4 ops and `02_timers.js`) | A timer heap holding `Global<Function>`s |
| `runtime_activity_stats_factory` (leftover ops and timers) | Counts of pending op promises and active timers |
| `add_near_heap_limit_callback`, `thread_safe_handle`, `terminate_execution` | The same, from `v8::Isolate` |
| `JsErrorBox` | `throw_exception` with `v8::Exception::error` |
| `serde_v8::to_utf8` | `String::write_utf8_v2` |
| The inspector (`profile.rs`, profiling only) | `v8::inspector` with a client and channel (deno_core's is 2,356 lines), or drop it |
| `Deno.core.loadExtScript`, `core.print`, `__bootstrap` | A shim: an ops object, a script loader for the IIFE-style scripts, `print`, and a copy of the primordials |

## Estimate

Keep deno_web's JavaScript (MIT) and reimplement what the pages reach, rather than
replacing the web APIs with polyfills. A polyfill for streams alone is thousands of lines
and would change what the pages do; the vendored scripts already render the 59 pages
byte-equal. Then:

- Rust: isolate, snapshot, ops binding, and errors, about 500 lines; the event loop,
  async calls, timers, and leftover-work counts, about 300; the 14 core ops and 4 URL ops,
  about 300 (structured clone over `ValueSerializer`, URL parsing on `ada-url`, already an
  optional dependency).
- JavaScript: a shim of about 150 lines, plus the vendored scripts and primordials, about
  26,000 lines kept as they are.
- The inspector: drop it, or about 300 more lines.
- Checks: `page-compare.ts` 59 of 59 byte-equal, the crate's tests, and then the
  performance gate.

That is about 1,100 to 1,400 lines of new Rust. These are estimates from reading the code,
not measured.

## The `v8` crate is Deno's too

`v8` (rusty_v8) is published from github.com/denoland/rusty_v8, so the move shrinks what
the port depends on from Deno from seven crates to one thin binding, but doesn't remove
the dependency. rusty_v8 mostly tracks Chrome's V8 releases, so a fork is cheaper to keep
than one of deno_core's runtime. The switch conditions follow from that:

- a V8 security fix that reaches `v8` but not deno_core within about a month;
- deno_core releases stopping before about October 2027;
- no fork of deno_core (are-we-dodo-yet, dodo) publishing crates by about Q2 2027;
- and a new condition: `v8` itself not getting V8's security releases, which no move
  inside Rust's V8 bindings fixes.

## Acceptance criteria

- [x] The ops the 59 compared pages call, of the isolate's registered ops.
- [x] A map from each deno_core API the render crate uses to its `v8` equivalent.
- [x] The web-API layer's size, and the choice between polyfills and deno_web's
      JavaScript with its ops reimplemented.
- [x] A scoped estimate and the switch conditions.
