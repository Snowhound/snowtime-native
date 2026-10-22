#!/bin/sh
# Records the port's SQL per request while the conformance suite runs against it, built with
# the sql-capture feature, and compares it per route with the app's capture from capture.sh,
# using the porting kit's sql-capture tool at the revision harness/kit.sh names
# (PORTING_KIT overrides it with a checkout). Writes temp/sql-capture/compare.md.
set -eu
kit=${PORTING_KIT:-$("$(dirname "$0")/../kit.sh")}
cd "$(dirname "$0")/../.."
out=temp/sql-capture
if [ ! -s "$out/app.jsonl" ]; then
  echo "no app capture; run harness/sql-capture/capture.sh first" >&2
  exit 1
fi
cargo build -q -p snowtime-host --features sql-capture
rm -f "$out/port.jsonl"
status=0
SQL_CAPTURE="$(pwd)/$out/port.jsonl" bun harness/conformance.ts target/debug/snowtime-axum || status=$?
bun "$kit/tools/sql-capture/cli.ts" route-parity.json --compare "$out/app.jsonl" "$out/port.jsonl" \
  --json "$out/compare.json" > "$out/compare.md"
tail -n 1 "$out/compare.md"
if [ "$status" -ne 0 ]; then echo "the conformance suite failed; the capture may be partial" >&2; fi
exit "$status"
