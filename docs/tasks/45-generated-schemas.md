# 081.45: Generated input schemas

Status: done (2026-10-22): the bar and the differential test pass on the generated code

Generate the input structs and their validation from the app's valibot schemas, with the
porting kit's schema generator (kit task 003, `tools/schema-gen`), instead of keeping them
by hand. This port proves the generator before it enters the kit.

## Changes

- `schema-gen.json` names the app's schema modules, the input exports (`Input$`), the
  probe that reads a message's key (`issueMessage`), the transport's revival of dates and
  query booleans, and the port's own types: `Day` for `IsoDate`, `Unit`, `EntryRow`,
  `DayCursor`, and the issue links as `Option<String>`.
- `crates/server/src/inputs/` is generated: the 28 input structs, one validator per schema
  node, and the runtime they share. Validators follow valibot's order: entries in order,
  each pipe step, unions and variants, defaults, trimming.
- Each domain's `schemas.rs` keeps its outputs and constants, re-exports its inputs, and
  holds the 16 hooks the generator can't translate: the checks across fields, the time zone
  and duration pattern checks, the day check, and the issue links' transform.
- `crate::schemas::decode` maps the generated issue to `Error::Invalid`; the `Field` table,
  `check_field`, the English message table, and the query-boolean deserializers are gone.
  The messages now come from the app at generation.
- `Patch` moves to the generated runtime. Where the schema allows absent and null, the
  input is a `Patch` as the schema types say, also in `CreateEntryInput` and
  `StartTimerInput`, whose rules read it with `into_option` or `value`. Optional fields
  that can't be null, such as an update's dates, are `Option`.
- `examples/schema-run.rs` answers the differential test's inputs.

Hand-maintained schema code went from 2,649 to 1,593 lines; 878 of what remains is
`auth/schemas.rs`, mostly Better Auth's Zod checks and outputs, which no valibot schema
describes. The generated code is 3,621 lines, the config 66.

## Differential test

`bun <kit>/tools/schema-gen/differential.ts schema-gen.json` sends 16,375 inputs, built
from the schemas, through the app's own `decode` and through the generated Rust: all
verdicts match (key, parameters, and message, or the normalized value).

The first run found two differences:

- valibot's `object` takes an array, whose keys are its indexes, and reports the missing
  key; the generator answered "Expected Object". Fixed in the generator.
- The app revives dates with `new Date(text)`, whose legacy parser accepts `2026-9-24`,
  `2026-02-30` (rolled over to 2 March), `Sep 24 2026`, `2026/09/24`, and `ABC-123`;
  `Timestamp::parse` refuses them. The hand-written port refused them too, and the app's
  client sends ISO strings, but an API key's client may not. Left out of the test with that
  reason in `schema-gen.json`. Many apps revive dates this way, so the kit's runtime will
  match the engine (kit task 003).

## Acceptance criteria

- [x] The generated validators replace the hand-written tables, and the port builds,
      passes clippy, and passes its tests.
- [x] The differential test matches every verdict, with the date forms above left out and
      recorded.
- [x] The bar passes at `40d9d35`: all 67 conformance tests (572 assertions, 13 files),
      all 1,490 calls byte for byte, and the 17 hardening checks.
- [x] The maintainer decides on V8's legacy date forms (2026-10-22): the kit's runtime gets
      a port of V8's `Date.parse` (kit task 003); until then the port refuses them, as
      before, with the exclusion recorded in `schema-gen.json`.
- [x] CI checks that the generated files are current at the kit revision
      `harness/schema-gen.sh` names, and runs the differential test.
