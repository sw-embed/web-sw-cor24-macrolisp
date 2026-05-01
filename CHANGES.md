# Changelog

## 2026-05-01

### Sync from sw-cor24-macrolisp
- Regenerate `asm/repl-full.s` from sw-cor24-macrolisp@f6459e4 — picks up tail-recursive `range`, `repeat`, `take`, `zip`, `flatten`. Functional Toolkit peak C-stack drops from ~3.26 KB to ~2.24 KB (~31%) and now fits in the 3 KB EBR default. Functional Toolkit demo's default stack reverted to `ThreeKb`.
- Rebuild `pages/` bundle.

### Build
- `rustfmt` cleanup in `src/repl.rs` (one wrapped method call); rebuild `pages/`. Closes #1 (Functional Toolkit stack overflow on web).

### Demos
- Tail-Call Optimization demo: drop redundant 8 KB stack override → default 3 KB. Demo source has always documented "Stack: 3 KB (default)"; the larger setting was historical caution. Dropdown loses its `[8K]` suffix as a side-effect.

## 2026-04-30

### UI
- Add `StackSize::SixteenKbSram` — 16 KB stack relocated into SRAM (0x0FC000..0x100000). Implemented by runtime-patching `_start` in the loaded assembly to load `sp = 0x100000` before calling `_main`. Diagnostic option for distinguishing C-stack budget overflows from true unbounded recursion. Bound checks remain active (overflow at 0x0FC000).
- Functional Toolkit demo now defaults to 16 KB SRAM stack — overflows reported at 3 KB and 8 KB EBR.

### Sync from sw-cor24-macrolisp
- Regenerated all `asm/repl-*.s` snapshots from sw-cor24-macrolisp@278bdba (picks up the REPL/compiler drain-loop fix that allows multiple forms on one line).

## 2026-04-25

### Demos

- Add "Fuzzy + Percent" demo (Standard): `(if≈% α ≈ β ± ρ % then X else Y)` — an infix macro combining fuzzy-eq and percent-tol (Unicode ≈/± and ASCII `if~=%` twins; pure integer math; gensym-hygienic). Extended with `aif≈%` anaphoric variant that binds `it` to the signed delta and yields 0 when within tolerance or `it` otherwise — composes through reduce/+/abs for drift aggregation.
- README: demo count bumped to 35.

## 2026-04-24

### Demos

- `1d199f5` Refresh `fuzzy-eq` embedded source for sibling's new Unicode-glyph section: `(if≈ a ≈ b ± e then X else Y)` using real math symbols (≈ U+2248, ± U+00B1) alongside the ASCII `if~=` variant.
- `c581cd2` Add "Power DSL (Glyphs)" and "Power DSL (Typed + iff)" demos (Standard). Initial form used DOS-style box corners (┌│└) and `iff` as a tail argument on `•`.
- `5342217` Fix Power DSL demos: swap box corners (┌│└) for Unicode curly-brace pieces (⎧⎨⎩); move `iff` inside `loop` (between `times` and the body brace) so it visually tracks the loop control; define `assert` locally so the iff variant runs on the cached Standard snapshot (which predates the prelude's built-in `assert`).
- `ed5fb00` Refactor Power DSL: the curly brace `⎧⎨⎩` is the macro. `loop`/`iff` live inside the brace on the middle row, `•` body reduces to the brace expression. Added DSL-parsing-fragility notes (positional binding silently breaks on token reorder).
- `9cf7d6c` Match updated `docs/fix.txt` layout: relocate the per-step combination (operand, operator) to the `⎩` row; middle row (`⎨`) carries only loop ctrl + `iff`. Flattened form: `(• Power (n : ℤ e : ℤ) → (p : ℤ) ← (⎧ 1 ⎨ loop e times iff e is positive ⎩ n *))`.

### UI

- `8fe36b6` Drag-resize the Split-view input overlay. Invisible hover-lit handles on the top edge, left edge, and top-left corner grow/shrink the panel (clamped 20%–95% of the main area). Uses Pointer Events with `setPointerCapture` so drags survive the cursor leaving the handle — no global listeners needed.

### Docs

- `d706cba` CLAUDE.md: add a "Changelog discipline" section requiring a `CHANGES.md` entry on every commit; backfill this 2026-04-24 section.
- `3c921af` CHANGES.md: one bullet per commit so the day's iteration history reads in order.
- Footer "Changes" link now points to this repo's CHANGES.md (was: sibling `sw-cor24-macrolisp`).

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
