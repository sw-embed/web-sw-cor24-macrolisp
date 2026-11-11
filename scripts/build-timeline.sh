#!/usr/bin/env bash
set -euo pipefail

# Build the Lisp timeline (lisp-timeline/, forked from sw-apl's timeline by
# way of the Forth one) into pages/lisp-timeline/, published at
# https://sw-embed.github.io/web-sw-cor24-macrolisp/lisp-timeline/.
# It is its own WebAssembly, built by wasm-pack from
# lisp-timeline/data/timeline.json; Trunk does not know about it, and
# build-pages.sh / build-all.sh leave pages/lisp-timeline/ alone.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
SRC="$PROJECT_DIR/lisp-timeline"
OUT="$PROJECT_DIR/pages/lisp-timeline"

echo "=== Testing the timeline ==="
( cd "$SRC" && cargo test --quiet )

echo "=== Building pages/lisp-timeline/ ==="
mkdir -p "$OUT"
rsync -a --delete --exclude='timeline/' "$SRC/site/" "$OUT/"
( cd "$SRC/crates/lisp-timeline-web" && \
  wasm-pack build --release --target web --no-pack --out-dir "$OUT/timeline" )
# wasm-pack writes a .gitignore and type declarations; neither belongs in pages/.
rm -f "$OUT/timeline/.gitignore" "$OUT"/timeline/*.d.ts

echo "=== Done ==="
echo "Timeline built in: $OUT"
