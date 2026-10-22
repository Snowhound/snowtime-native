# 081.49: Dates as Node reads them

Status: done (2026-10-22): 16,763 of 16,763 verdicts match, with no exclusion

The app's transport revives a date field with `new Date(text)`, which on Node accepts V8's
legacy forms beyond ISO 8601: `2026-9-24`, `2026-02-30` (rolled over to March 2), `-1`,
`1.5`, `ABC-123`. `Timestamp::parse` read ISO 8601 only, so `schema-gen.json` excluded
those six values from the differential test (task 081.45). It also read a date-time
without a zone as UTC, where Node reads local time.

## Changes

- The kit's schema generator writes `inputs/js_date.rs` beside the runtime: V8 13.6's
  `Date.parse` (Node 24), ported under V8's BSD license, whose notice
  `THIRD_PARTY_LICENSES` already carries for the render crate's V8. The kit checks it
  against Node on 40,069 strings in two zones (kit task 003, kit `2024eb8`).
- `Timestamp::parse` calls it, with a local time read in the process's time zone through
  `mktime`. Node takes the offset from before a DST change for a time the change skips or
  repeats; `mktime` follows the platform. That matters only outside UTC, and production
  runs in UTC.
- `schema-gen.json` has no exclusions; the differential test compares 16,763 inputs, with
  the app's verdicts from Node, the app's production runtime, rather than Bun (kit
  `7e8e86c`). CI installs Node 24, whose `Date.parse` `js_date.rs` ports.
- `harness/kit.sh` takes `PORTING_KIT`, a kit checkout to use instead of the pinned clone
  while the kit changes beside the port.

## Acceptance criteria

- [x] The differential test matches every verdict without the date exclusion.
- [x] The server's tests and the conformance suite pass (the OAuth file fails locally on
      `main` too; task 081.48).
