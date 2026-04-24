# Changelog

## 2026-04-24

### Demos

- Add "Power DSL (Glyphs)" demo (Standard): Power expressed as `(• power (N E) → (P) ← (loop E times ┌ 1 │ * N └))` via two glyph macros
- Add "Power DSL (Typed + iff)" demo (Standard): extends the glyph DSL with `n : ℤ` type annotations and `iff e is positive` → `(assert (positive? e))` preconditions
- Update fuzzy-eq demo: pulls sibling's new Unicode-glyph section (`if≈ a ≈ b ± e then X else Y`) into the embedded source

## 2026-04-23

### Demos

- Add "Fuzzy Equality" demo (Standard): `if~=` DSL macro mirroring `a ≈ b ± e` notation
- Add "Infix Syntax" demo (Standard): binary `$` macro and variadic left-to-right `infix` macro
- Add "Percent Tolerance" demo (Standard): hygienic `if-close?` macro for percentage-tolerance comparisons
- Add "Power (Integer Exponent)" demo (Standard): tail-recursive N^E with 22-bit fixnum wrap caveat
- Add "Reduce" demo (Full): reduce/fold patterns — sum, product, reverse, max, min, join

### Build

- Fix `build-all.sh` tc24r path: `sw-cor24-tinyc` → `sw-cor24-x-tinyc` (was stale after the `x-` rename)

## 2026-03-30 — Fork Migration

- Forked from [sw-vibe-coding/web-tml24c](https://github.com/sw-vibe-coding/web-tml24c)
- Renamed package to `web-sw-cor24-macrolisp`
- Updated path deps to `../sw-cor24-emulator` (was `../../sw-embed/cor24-rs`)
- Updated demo include paths to `../sw-cor24-macrolisp/demos/` (was `../../tml24c/demos/`)
- Updated GitHub links to `sw-embed/web-sw-cor24-macrolisp`
- Updated `build-all.sh` paths to use `../sw-cor24-macrolisp` and `../sw-cor24-x-tinyc`
- Added `build-pages.sh` script
- Removed `.agentrail/` and `.claude/` directories
- Updated README with ecosystem links and provenance

## 2026-03-29

### Bug Fixes

- Fix prompt detection for looping demos: REPL falsely idled when evaluating
  long-running expressions with no UART output (delay spin loops, polling).
  Added feed cooldown — 10 ticks after last UART feed before checking prompt.
- Skip comment lines (`;`) in UART feed to avoid wasting cycles and `nil` spam
- Rebuild from latest tml24c with variadic arithmetic fix (`+`, `*`, `-` now
  accept 2+ arguments)

### UI

- Shift-Enter in Split mode triggers Eval (plain Enter inserts newline)

### Demos

- Add "Switch LED" demo: mirror S2 switch to LED D2 in a polling loop
- Add variadic arithmetic examples to Variadic Functions demo
- Add reduce+range section to Functional Toolkit demo

## 2026-03-28

### LED & Hardware

- Fix inverted LED display: use `is_led_on()` API (active-low: register 0 = lit)
- Rename `led_on` → `led_lit` with active-low documentation

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
