# web-sw-cor24-macrolisp

Web UI for [sw-cor24-macrolisp](https://github.com/sw-embed/sw-cor24-macrolisp) on COR24. Browser-based Lisp REPL running on the COR24 emulator via Rust, Yew, and WebAssembly.

Write Lisp expressions, select a prelude tier, and evaluate — all in the browser.

**[Live Demo](https://sw-embed.github.io/web-sw-cor24-macrolisp/)**

![web-sw-cor24-macrolisp screenshot](images/screenshot.png?ts=1774674005000)

## Features

- **5 prelude tiers**: Bare, Minimal, Standard, Full, Scheme
- **17 embedded demos**: TCO, macros, lazy sequences, error handling, anaphoric macros, bottles of beer, LED blink, and more — grouped by prelude in the dropdown
- **Dual view modes**: CLI (Enter to eval, interleaved output) and Split (semi-transparent overlay for multi-line code)
- **Configurable stack**: 3 KB (hardware default) or 8 KB (full EBR)
- **Floating hardware panel**: LED D2, Switch S2 (clickable), with live memory gauges for heap, symbols, string pool, and stack (green/yellow/red)
- **Pause/Resume/Reset/Clear** controls
- **COR24 emulator**: Full 24-bit RISC CPU running in WebAssembly with instant UART TX

## Provenance

Forked from [sw-vibe-coding/web-tml24c](https://github.com/sw-vibe-coding/web-tml24c) as part of the COR24 ecosystem consolidation under [sw-embed](https://github.com/sw-embed).

## Related

- [sw-cor24-macrolisp](https://github.com/sw-embed/sw-cor24-macrolisp) — The Lisp implementation (C)
- [sw-cor24-tinyc](https://github.com/sw-embed/sw-cor24-tinyc) — COR24 C compiler (Rust)
- [sw-cor24-emulator](https://github.com/sw-embed/sw-cor24-emulator) — COR24 emulator (Rust)
- [sw-cor24-project](https://github.com/sw-embed/sw-cor24-project) — COR24 ecosystem hub

## Development

```bash
./scripts/serve.sh              # dev server with hot reload on port 9135
./scripts/build-all.sh          # recompile macrolisp + build pages/ for GitHub Pages
./scripts/build-pages.sh        # release build to pages/ for GitHub Pages
cargo clippy -- -D warnings     # lint
cargo fmt --all                 # format
```

## License

MIT © 2026 Michael A. Wright
