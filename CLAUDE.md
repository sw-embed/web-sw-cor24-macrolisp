# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Web UI for Tiny Macro Lisp on COR24. Browser-based Lisp REPL running on the COR24 emulator.

## Related Projects

- `~/github/sw-embed/sw-cor24-macrolisp` -- The Lisp implementation (C)
- `~/github/sw-embed/sw-cor24-x-tinyc` -- COR24 C compiler (Rust)
- `~/github/sw-embed/sw-cor24-emulator` -- COR24 assembler and emulator (Rust)

## Build

Edition 2024 for any Rust code. Never suppress warnings.

**Always use scripts, never run trunk directly:**

```bash
./scripts/serve.sh          # Dev server on port 9135
./scripts/build-all.sh      # Recompile macrolisp + build pages/ for GitHub Pages
./scripts/build-pages.sh    # Release build to pages/ for GitHub Pages
```

`build-all.sh` does two things:
1. Recompiles all 5 REPL variants (bare/minimal/standard/full/scheme) from sw-cor24-macrolisp via tc24r
2. Builds WASM into `dist/` (gitignored), then rsyncs to `pages/` (preserving `.nojekyll`)

**Before committing pages/ changes, always run `./scripts/build-all.sh`.**

The `pages/` directory is committed and deployed via GitHub Actions (`.github/workflows/pages.yml`).
Do NOT run `trunk build` or `trunk serve` directly — use the scripts.
