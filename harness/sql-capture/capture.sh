#!/bin/sh
# Records the app's SQL per route while its conformance suite runs, with the porting kit's
# sql-capture tool at the revision harness/kit.sh names (PORTING_KIT overrides it with a
# checkout). Applies app.patch and the kit's libsql adapter in app/ for the run, reverts
# both, and writes each route's SQL to sql/. The capture stays in temp/sql-capture/.
set -eu
kit=${PORTING_KIT:-$("$(dirname "$0")/../kit.sh")}
cd "$(dirname "$0")/../.."
root=$(pwd)
if ! git -C app diff --quiet; then
  echo "app/ has changes; commit or revert them first" >&2
  exit 1
fi
out=$root/temp/sql-capture
mkdir -p "$out"
rm -f "$out/app.jsonl"
adapter=app/src/server/sql-capture.server.ts
trap 'git -C app checkout -q -- . && rm -f "$adapter"' EXIT
git -C app apply ../harness/sql-capture/app.patch
cp "$kit/tools/sql-capture/libsql.ts" "$adapter"
status=0
(cd app && SQL_CAPTURE="$out/app.jsonl" bun test ./conformance/*.conformance.ts) || status=$?
bun "$kit/tools/sql-capture/cli.ts" route-parity.json --capture "$out/app.jsonl" --out sql
if [ "$status" -ne 0 ]; then echo "the conformance suite failed; the capture may be partial" >&2; fi
exit "$status"
