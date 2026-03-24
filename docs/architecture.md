# Architecture: web-tml24c

## Decision: Rust/Yew with cor24-rs library (not Emscripten)

### Approach

Use **cor24-rs** (assembler + emulator) as a Rust library dependency. The tml24c
C interpreter is pre-compiled to COR24 assembly via tc24r, then assembled and
executed in the browser-based COR24 emulator. Lisp I/O flows through the
emulated UART.

### Why not Emscripten?

Compiling tml24c C code directly to WASM via Emscripten would work but defeats
the purpose: users want to see Lisp running *on COR24*. The cor24-rs emulator
provides instruction tracing, register visibility, and memory inspection — all
essential for the Phase 3 execution trace feature.

### Why not a Rust rewrite of the Lisp core?

Unnecessary. The C code compiles to COR24 via tc24r already. A Rust rewrite
would duplicate effort and diverge from the canonical tml24c implementation.

## Pipeline

```
Build time:
  tml24c (C) --[tc24r]--> tml24c.s (COR24 assembly)

Browser runtime:
  tml24c.s --[cor24-rs assembler]--> binary
  binary   --[cor24-rs emulator]-->  running COR24 CPU
  User Lisp input --> UART RX --> interpreter --> UART TX --> output display
```

## Stack

- **Frontend**: Yew (Rust WASM framework), same as cor24-rs
- **Build tool**: Trunk (WASM bundler)
- **CPU/Assembler**: cor24-emulator crate (path dependency to cor24-rs)
- **ISA types**: cor24-isa crate

## Project Structure

```
web-tml24c/
├── Cargo.toml          # Workspace root
├── Trunk.toml          # WASM build config
├── index.html          # Entry point
├── src/
│   ├── main.rs         # Yew app entry
│   └── repl.rs         # REPL component
├── docs/
│   └── architecture.md # This file
└── asm/
    └── tml24c.s        # Pre-compiled Lisp interpreter (from tc24r)
```

## Component Design

### REPL Component

- Text input area for Lisp expressions
- Eval button (sends input to emulated UART)
- Output area (reads from emulated UART output buffer)
- The COR24 emulator runs the tml24c interpreter binary
- Each eval: write Lisp text to UART RX, run CPU until output appears on UART TX

### Future: Execution Trace (Phase 3)

The cor24-rs `TraceBuffer` provides a ring buffer of the last 200 executed
instructions with before/after register state. This will power the visual
execution trace showing how the Lisp evaluator steps through COR24 instructions.
