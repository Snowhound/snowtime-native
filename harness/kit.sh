#!/bin/sh
# Prints a checkout of the porting kit at the revision this port uses, cloning it once.
# The kit's tools (schema-gen, route-parity) run from it; moving the revision is a port
# change like any other dependency update.
set -eu
kit_revision=c6b8fce8f8eeccbc14a63648b93e86f03c720859
kit=${TMPDIR:-/tmp}/porting-kit-$kit_revision
if [ ! -d "$kit" ]; then
  git clone -q https://github.com/kaitk/porting-kit "$kit"
  git -C "$kit" checkout -q "$kit_revision"
fi
echo "$kit"
