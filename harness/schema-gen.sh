#!/bin/sh
# Checks the generated input validation (crates/server/src/inputs) with the porting kit's
# schema generator at the revision harness/kit.sh names, then compares its verdicts with
# the app's. Pass --write to regenerate instead. Needs the app built (its Paraglide
# messages) and Bun.
set -eu
kit=$("$(dirname "$0")/kit.sh")
cd "$(dirname "$0")/.."
if [ "${1:-}" = --write ]; then
  bun "$kit/tools/schema-gen/cli.ts" schema-gen.json
  exit
fi
bun "$kit/tools/schema-gen/cli.ts" schema-gen.json --check
bun "$kit/tools/schema-gen/differential.ts" schema-gen.json
