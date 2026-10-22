#!/bin/sh
# Checks the generated input validation (crates/server/src/inputs) with the porting kit's
# schema generator at the revision it was generated with, then compares its verdicts with
# the app's. Pass --write to regenerate instead. Needs the app built (its Paraglide
# messages) and Bun.
set -eu
kit_revision=1801d782e747919f50568eea5a4409217bb97a57
kit=${TMPDIR:-/tmp}/porting-kit-$kit_revision
if [ ! -d "$kit" ]; then
  git clone -q https://github.com/kaitk/porting-kit "$kit"
  git -C "$kit" checkout -q "$kit_revision"
fi
cd "$(dirname "$0")/.."
if [ "${1:-}" = --write ]; then
  bun "$kit/tools/schema-gen/cli.ts" schema-gen.json
  exit
fi
bun "$kit/tools/schema-gen/cli.ts" schema-gen.json --check
bun "$kit/tools/schema-gen/differential.ts" schema-gen.json
