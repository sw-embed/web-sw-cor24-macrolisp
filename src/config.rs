/// Prelude tier determines which pre-compiled REPL binary to load.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PreludeTier {
    Bare,
    Minimal,
    Standard,
    Full,
    /// Scheme-flavored prelude: #t/#f, define-fn, let*, equal?, even?/odd?
    Scheme,
}

impl PreludeTier {
    pub fn label(self) -> &'static str {
        match self {
            Self::Bare => "Bare",
            Self::Minimal => "Minimal",
            Self::Standard => "Standard",
            Self::Full => "Full",
            Self::Scheme => "Scheme",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Bare => "No prelude — raw primitives only",
            Self::Minimal => "6 comparison functions",
            Self::Standard => "~40 functions: list ops, macros, strings, I/O",
            Self::Full => "~65 functions: standard + lazy, threading, anaphora",
            Self::Scheme => "Scheme-flavored: #t/#f, define-fn, let*, equal?, even?/odd?",
        }
    }

    pub fn assembly(self) -> &'static str {
        match self {
            Self::Bare => include_str!("../asm/repl-bare.s"),
            Self::Minimal => include_str!("../asm/repl-minimal.s"),
            // Standard uses snapshot-capable REPL binary
            Self::Standard => include_str!("../asm/repl-snapshot.s"),
            Self::Full => include_str!("../asm/repl-full.s"),
            Self::Scheme => include_str!("../asm/repl-scheme.s"),
        }
    }

    /// Returns the pre-compiled heap snapshot for this prelude tier, if available.
    /// Loading a snapshot at 0x080000 gives ~10x faster startup vs eval_str.
    pub fn snapshot(self) -> Option<&'static [u8]> {
        match self {
            Self::Standard => Some(include_bytes!("../snapshots/standard.snap")),
            _ => None,
        }
    }

    pub const ALL: [PreludeTier; 5] = [
        Self::Bare,
        Self::Minimal,
        Self::Standard,
        Self::Full,
        Self::Scheme,
    ];
}

/// Stack size configuration.
///
/// `ThreeKb` and `EightKb` use the COR24 EBR region (8 KB hardware cap).
/// `SixteenKbSram` is a diagnostic option: it relocates the stack into SRAM
/// at 0x100000 (top of SRAM, grows down to 0x0FC000) by patching the loaded
/// assembly's `_start` to load `sp = 0x100000`. Use it to determine whether
/// a demo overflows due to true unbounded recursion or just C-stack budget.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StackSize {
    /// 3 KB — matches MachXO hardware default
    ThreeKb,
    /// 8 KB — full EBR window, needed for deep recursion
    EightKb,
    /// 16 KB — relocated into SRAM (0x0FC000..0x100000). Diagnostic only.
    SixteenKbSram,
}

impl StackSize {
    pub fn initial_sp(self) -> u32 {
        match self {
            Self::ThreeKb => 0xFEEC00,
            Self::EightKb => 0xFF0000,
            Self::SixteenKbSram => 0x100000,
        }
    }

    /// Lower bound for stack-overflow detection.
    pub fn lower_bound(self) -> u32 {
        match self {
            Self::ThreeKb | Self::EightKb => 0xFEE000,
            Self::SixteenKbSram => 0x0FC000,
        }
    }

    /// True if the loaded assembly needs `_start` patched to load SP into a
    /// non-default region (i.e. SRAM).
    pub fn requires_relocated_sp(self) -> bool {
        matches!(self, Self::SixteenKbSram)
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::ThreeKb => "3 KB",
            Self::EightKb => "8 KB",
            Self::SixteenKbSram => "16 KB SRAM",
        }
    }

    pub fn bytes(self) -> u32 {
        match self {
            Self::ThreeKb => 3072,
            Self::EightKb => 8192,
            Self::SixteenKbSram => 16384,
        }
    }

    pub const ALL: [StackSize; 3] = [Self::ThreeKb, Self::EightKb, Self::SixteenKbSram];
}
