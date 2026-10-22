#!/bin/sh
# Prints a checkout of the porting kit at the revision this port uses, cloning it once.
# The kit's tools (schema-gen, route-parity, sql-capture) run from it; moving the revision
# is a port change like any other dependency update.
set -eu
# PORTING_KIT names a checkout to use instead, while changing the kit beside the port.
if [ -n "${PORTING_KIT:-}" ]; then
  echo "$PORTING_KIT"
  exit
fi
kit_revision=7e8e86cbb69ab43678e383d06d9e7b22c59585d1
kit=${TMPDIR:-/tmp}/porting-kit-$kit_revision
if [ ! -d "$kit" ]; then
  git clone -q https://github.com/kaitk/porting-kit "$kit"
  git -C "$kit" checkout -q "$kit_revision"
fi
echo "$kit"
