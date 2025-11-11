#!/usr/bin/env bash
set -euo pipefail
ART=release
NAME=irig106_timecodes
VER=${1:-v0.1.0}
OUT="$ART/${NAME}-${VER}"
rm -rf "$OUT" && mkdir -p "$OUT"

cargo build --release
cp -r target/release "$OUT/bin"
mkdir -p "$OUT/meta"
git rev-parse HEAD > "$OUT/meta/git_commit.txt" || true
cargo cyclonedx -o "$OUT/meta/sbom.cdx.json" || true
( cd "$OUT/.." && find "${NAME}-${VER}" -type f -print0 | sort -z | xargs -0 sha256sum ) > "$ART/manifest.sha256"
( cd "$ART" && tar --format=ustar --owner=0 --group=0 -cf "${NAME}-${VER}.tar" "${NAME}-${VER}" )
gzip -n "$ART/${NAME}-${VER}.tar"
echo "Release at $ART/${NAME}-${VER}.tar.gz"
