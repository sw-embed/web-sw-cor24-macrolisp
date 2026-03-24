# web-tml24c

Web UI for [Tiny Macro Lisp](https://github.com/sw-vibe-coding/tml24c) on COR24. Browser-based Lisp REPL running on the COR24 emulator via Rust, Yew, and WebAssembly.

Write Lisp expressions, select a prelude tier, and evaluate — all in the browser.

**[Live Demo](https://sw-vibe-coding.github.io/web-tml24c/)**

![web-tml24c screenshot](images/screenshot.png?ts=1711310400000)

## Features

- **5 prelude tiers**: Bare, Minimal, Standard, Full, Scheme (incomplete)
- **17 embedded demos**: TCO, macros, lazy sequences, error handling, bottles of beer, LED blink, and more
- **Configurable stack**: 3 KB (hardware default) or 8 KB (full EBR)
- **Hardware widgets**: LED D2 visualization
- **COR24 emulator**: Full 24-bit RISC CPU running in WebAssembly

## Related

- [tml24c](https://github.com/sw-vibe-coding/tml24c) — The Lisp implementation (C)
- [tc24r](https://github.com/sw-vibe-coding/tc24r) — COR24 C compiler (Rust)
- [cor24-rs](https://github.com/sw-embed/cor24-rs) — COR24 assembler and emulator (Rust)
- [web-tc24r](https://github.com/sw-vibe-coding/web-tc24r) — Web UI for tc24r compiler

## Development

```bash
trunk serve                                              # dev server on port 9135
trunk build --release --public-url /web-tml24c/ -d pages # production build
```

## License

MIT
