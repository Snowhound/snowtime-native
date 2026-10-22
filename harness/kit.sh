#!/bin/sh
# Prints a checkout of the porting kit at the revision this port uses, cloning it once.
# The kit's tools (schema-gen, route-parity, sql-capture) run from it; moving the revision is a port
# change like any other dependency update.
set -eu
kit_revision=f5de0e699ae575d001c61717d8daed5b3e074077
kit=${TMPDIR:-/tmp}/porting-kit-$kit_revision
if [ ! -d "$kit" ]; then
  git clone -q https://github.com/kaitk/porting-kit "$kit"
  git -C "$kit" checkout -q "$kit_revision"
fi
echo "$kit"
