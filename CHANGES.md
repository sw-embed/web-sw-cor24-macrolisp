# Changes

## 2026-03-28

### LED & Hardware

- Fix inverted LED display: use `is_led_on()` API (active-low: register 0 = lit)
- Rename `led_on` → `led_lit` with active-low documentation
- Add "Switch LED" demo: poll S2 to light D2

### Demos

- Add "String Formatting" demo (format with ~a interpolation)
- Fix demo ordering: alphabetical within each prelude group

## 2026-03-26

### Features

- Add instruction trace panel: toggle button captures and displays executed instructions
- Add "Iteration Patterns" demo (Full prelude)

## 2026-03-25

### Deployment

- Add favicon.ico (fix 404 console error)

### Emulator

- Snapshot-accelerated prelude loading (~10x faster Standard prelude startup)
- Recompile all 5 REPL variants from latest tml24c

## 2026-03-24

### Build & Deployment

- Fix build: trunk outputs to `dist/` (gitignored), rsync to `pages/` preserving `.nojekyll`
- Use `--release` for production builds (removes trunk WebSocket/live-reload code)
- Add `scripts/build-all.sh` (recompile tml24c + trunk release build + rsync)
- Add `scripts/serve.sh` (dev server on port 9135)
- Fix GitHub Pages deployment with `.nojekyll` and `--public-url /web-tml24c/`

### Demos

- Add 21 embedded demos across 4 prelude tiers (Minimal, Standard, Full, Scheme)
- Standard: Blink LED, Bottles of Beer, Bottles (Functional), Continuations,
  Error Handling, Fixed-Point Arithmetic, Macros, Metaprogramming, Multi-body Forms,
  Multi-line Input, Mutation, Parameters, Quasiquote, Restartable Conditions,
  Strings, Variadic Functions
- Full: Anaphoric Macros, Bottles (Trampoline), Functional Toolkit,
  Lazy Sequences, Threading Macros, Utility Functions
- Minimal: Tail-Call Optimization
- Scheme: Scheme Demo
- Demo dropdown grouped by prelude tier with auto-switching

### UI

- CPU load gauge (EWMA of instructions/batch) — idle/running/pegged with color
- 4 live memory gauges: Heap, Symbols, String Pool, Stack
- Hardware panel: LED D2 indicator, clickable Switch S2
- Dual view modes: CLI (single-line prompt) and Split (multi-line editor overlay)
- Animated spinner during evaluation
- Unified CLI transcript (interleaved input echo + output)
- Pause/Resume, Reset, Clear buttons
- 5 prelude tiers: Bare, Minimal, Standard, Full, Scheme
- 2 stack sizes: 3 KB and 8 KB

### Emulator

- Recompile all 5 REPL variants from latest tml24c (continuations, parameters,
  restartable conditions, multi-body forms, functional toolkit, iterative eval_list)
- 32K heap runtime
- UART TX busy cycles set to 0 for instant output in browser
- Instruction budget on UART feeding loop to prevent browser hangs
- 25ms tick interval (40 fps)
- Poll-before-feed UART pattern matching cor24-run CLI

## 2026-03-23

### Initial Release

- Yew/WASM scaffold with cor24-rs emulator backend
- Interactive Lisp REPL running COR24 emulator in the browser
- Multi-binary prelude switching (Bare/Minimal/Standard/Full/Scheme)
- GitHub Pages deployment via committed `pages/` directory
