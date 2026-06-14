#!/usr/bin/env bash
set -euo pipefail

# Build pages/ for GitHub Pages deployment.
# Run this before committing pages/ changes.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
TML_DIR="$PROJECT_DIR/../sw-cor24-macrolisp"

# Resolve tc24r: an explicit $TC24R wins, else the sibling release build,
# else whatever is on PATH (the shared toolchain installs one). This lets the
# script run in clones that don't have ../sw-cor24-x-tinyc checked out.
TC24R="${TC24R:-$PROJECT_DIR/../sw-cor24-x-tinyc/components/cli/target/release/tc24r}"
if [[ ! -x "$TC24R" ]]; then
  if command -v tc24r >/dev/null 2>&1; then
    TC24R="$(command -v tc24r)"
    echo "warning: ../sw-cor24-x-tinyc not checked out; falling back to PATH tc24r" >&2
    echo "         ($TC24R). A different tc24r version may regenerate asm/repl-*.s" >&2
    echo "         that diverges from the committed baseline — review 'git diff asm/'" >&2
    echo "         and do NOT commit a regen unless it is intentional and verified." >&2
  else
    echo "error: tc24r not found at $TC24R and not on PATH" >&2
    echo "       check out ../sw-cor24-x-tinyc or put tc24r on PATH" >&2
    exit 1
  fi
fi
echo "Using tc24r: $TC24R"

# 1. Recompile all REPL variants from sw-cor24-macrolisp
echo "=== Compiling REPL variants ==="
for v in bare minimal standard full scheme; do
  echo "  repl-$v..."
  "$TC24R" "$TML_DIR/src/repl-$v.c" -I "$TML_DIR/src" -o "$PROJECT_DIR/asm/repl-$v.s"
done

# 2. Build WASM into dist/ (gitignored), then sync to pages/
#    rsync --exclude preserves .nojekyll in pages/ across rebuilds
echo "=== Building pages/ ==="
cd "$PROJECT_DIR"
mkdir -p pages
touch pages/.nojekyll
trunk build --release --public-url /web-sw-cor24-macrolisp/ -d dist
rsync -a --delete --exclude='.nojekyll' dist/ pages/

echo "=== Done ==="
echo "Pages built in: $PROJECT_DIR/pages/"
echo "To preview locally: ./scripts/serve.sh"
echo "To deploy: git add pages/ && git commit && git push"
