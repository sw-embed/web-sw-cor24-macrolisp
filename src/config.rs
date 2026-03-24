/// Prelude tier determines which pre-compiled REPL binary to load.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PreludeTier {
    Bare,
    Minimal,
    Standard,
    Full,
    /// Scheme-flavored prelude (not yet compiled — uses Full as fallback)
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
            Self::Scheme => "Scheme-flavored: #t/#f, define-fn, let*, equal? (incomplete)",
        }
    }

    pub fn assembly(self) -> &'static str {
        match self {
            Self::Bare => include_str!("../asm/repl-bare.s"),
            Self::Minimal => include_str!("../asm/repl-minimal.s"),
            Self::Standard => include_str!("../asm/repl-standard.s"),
            // Scheme falls back to Full until a dedicated binary exists
            Self::Full | Self::Scheme => include_str!("../asm/repl-full.s"),
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

/// Stack size configuration for the COR24 EBR region.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StackSize {
    /// 3 KB — matches MachXO hardware default
    ThreeKb,
    /// 8 KB — full EBR window, needed for deep recursion
    EightKb,
}

impl StackSize {
    pub fn initial_sp(self) -> u32 {
        match self {
            Self::ThreeKb => 0xFEEC00,
            Self::EightKb => 0xFF0000,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::ThreeKb => "3 KB",
            Self::EightKb => "8 KB",
        }
    }

    pub fn bytes(self) -> u32 {
        match self {
            Self::ThreeKb => 3072,
            Self::EightKb => 8192,
        }
    }

    pub const ALL: [StackSize; 2] = [Self::ThreeKb, Self::EightKb];
}
