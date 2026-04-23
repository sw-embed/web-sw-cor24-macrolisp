# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Web UI for Tiny Macro Lisp on COR24. Browser-based Lisp REPL running on the COR24 emulator via Rust + Yew + WebAssembly.

## Sibling path dependencies

`Cargo.toml` and `scripts/build-all.sh` reference these projects by **relative path** (`../<name>`), so they must be checked out as siblings of this repo:

- `../sw-cor24-macrolisp` — the Lisp implementation in C. `src/demos.rs` pulls demo sources from `../../sw-cor24-macrolisp/demos/*.l24` via `include_str!`.
- `../sw-cor24-emulator` — COR24 assembler + emulator crates (`cor24-emulator`, `cor24-isa`) used as path dependencies.
- `../sw-cor24-tinyc` — COR24 C compiler (planned rename: `sw-cor24-x-tinyc`). `build-all.sh` invokes `<tinyc>/components/cli/target/release/tc24r` to recompile the REPL variants from C to `asm/repl-*.s`. If the rename lands, update the path in `scripts/build-all.sh`.

## Build

Edition 2024 for any Rust code. Never suppress warnings.

**Always use scripts, never run `trunk` directly:**

```bash
./scripts/serve.sh          # Dev server on port 9135 (Trunk.toml)
./scripts/build-all.sh      # Recompile macrolisp (tc24r) + build pages/
./scripts/build-pages.sh    # Release build to pages/ (skips tc24r recompile)
./scripts/deploy.sh         # Rebuild if stale, commit pages/, push
cargo clippy -- -D warnings
cargo fmt --all
```

`build-all.sh` does two things, in order:
1. Recompiles all 5 REPL variants (bare/minimal/standard/full/scheme) from `../sw-cor24-macrolisp/src/repl-*.c` via `tc24r`, writing `asm/repl-*.s`.
2. Builds WASM into `dist/` (gitignored), then `rsync`s to `pages/` (the `--exclude='.nojekyll'` preserves that file across rebuilds).

**Before committing `pages/` changes, always run `./scripts/build-all.sh`.** The `pages/` directory is committed and deployed via GitHub Actions (`.github/workflows/pages.yml`, which just uploads the `pages/` artifact — CI does not rebuild).

## Architecture

**Pipeline.** `repl-<tier>.c` → (`tc24r`, at build time) → `asm/repl-<tier>.s` → (`cor24_emulator::Assembler`, at browser startup) → bytes loaded into `EmulatorCore` at address 0 → CPU runs the REPL → user Lisp flows in via UART RX, output via UART TX.

**Prelude tiers** (`src/config.rs`). `PreludeTier::{Bare, Minimal, Standard, Full, Scheme}` maps to one of the `asm/repl-*.s` files via `include_str!`. `Standard` uses `asm/repl-snapshot.s` plus a pre-compiled heap snapshot (`snapshots/standard.snap`, loaded at `0x080000`) for ~10× faster startup — other tiers eval the prelude live.

**Main component** (`src/repl.rs`). One Yew component, `Repl`, drives everything:
- **Tick loop**: self-rescheduling `Msg::Tick` via `gloo::timers::Timeout(25ms)` runs `BATCH_SIZE = 50_000` instructions per frame. CPU gauge is an EWMA of the fraction of budget consumed (`ALPHA = 0.15`).
- **UART feed** (`send_input` + Tick): enqueues bytes into `uart_tx_queue`, feeds one byte per poll of status register `0xFF0101` (bit 0 = busy). After feeding `\n`, a `feed_cooldown = 10` (≈250 ms) suppresses the "at prompt" heuristic so spin-loops (delay, poll) don't look idle just because the last output was `"> "`.
- **"At prompt" idle detection**: requires `feed_cooldown == 0` AND empty queue AND `output.ends_with("> ")`. If you change how the REPL prints its prompt, update this check.
- **Memory gauges** read runtime globals by assembly label (`_heap_next`, `_sym_count`, `_str_pool_next`), captured from the assembler's symbol table at load time. Stack depth = `initial_sp - sp` (reg 4). Totals (`HEAP_SIZE=32768`, `MAX_SYMBOLS=512`, `STR_POOL_SIZE=8192`) are hardcoded and must match the C side.
- **Stack size** (`StackSize::{ThreeKb, EightKb}`): 3 KB maps hardware default (`SP = 0xFEEC00`), 8 KB is full EBR (`SP = 0xFF0000`). Demos in `src/demos.rs` declare which they need.
- **View modes**: `Cli` (single-line input, Enter to eval, interleaved transcript) vs `Split` (textarea overlay on output, Shift-Enter to eval). Loading a demo auto-switches to Split.
- **Hardware**: `set_uart_tx_busy_cycles(0)` makes TX instant; `is_led_on()` reads D2 (active-low); `set_button_pressed()` toggles S2.

**Adding a demo.** Append a `Demo` to `DEMOS` in `src/demos.rs`, grouped by prelude, alphabetical within group. `source` is an `include_str!` from `../../sw-cor24-macrolisp/demos/*.l24` — the file must exist in the sibling repo.

## Devgroup workflow (this host)

This clone is part of a coordinator-relay setup — **do not push**.

- `main` and `dev` are coordinator-only. You never commit to or push either.
- New work: `dg-new-feature <slug>` (or `dg-new-fix <slug>`) — creates `feat/<slug>` based on `origin/dev` (falls back to `origin/main` until `origin/dev` exists).
- When ready: `dg-mark-pr` renames `feat/<slug>` → `pr/<slug>`. The coordinator (mike) scans local clones for `pr/*` refs and relays them into `dev` on GitHub. The branch name is the contract — no PR API, no tickets.
- Rebase is fine on your own `feat/*`; never rewrite history on `dev` or `main`.
- Other helpers on `$SCRIPTROOT` PATH: `dg-env`, `dg-policy`, `dg-list-pr`, `dg-reap`, `dg-install-agents`, `dg-agentrail-retrofit`, `onboarding`.
- Full policy: `/disk1/github/softwarewrighter/devgroup/docs/branching-pr-strategy.md`.
