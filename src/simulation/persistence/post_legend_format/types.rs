use super::super::SimulationThermalRelation;

/// Explicit stored or runtime words; absent older fields remain unknown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendFormatInputs {
    /// Raw B8; primary modes 1/2 select f/g, all others select e.
    pub mode_raw: Option<i32>,
    /// Raw BC; width uses signed32 wrapping addition of seven.
    pub precision_raw: Option<i32>,
    /// Raw C4; alternate selection treats every nonzero value as enabled.
    pub alternate_enabled_raw: Option<i32>,
    /// Raw C8; alternate selection uses f for zero, g otherwise.
    pub alternate_selector_raw: Option<i32>,
}

/// Explicit local call facts, without inferring a pointer or value from archives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendFormatRuntime {
    /// None is unknown; Some(false) is a known null output-string pointer.
    pub output_pointer_present: Option<bool>,
    /// Original binary32 input, including NaN payloads and the sign bit.
    pub value_bits: Option<u32>,
}

/// A skipped alternate operation leaves the native output string untouched.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendFormatSelection {
    /// Constructed the primary printf-style string.
    Primary,
    /// Selected and constructed an alternate printf-style string.
    Alternate,
    /// Known null output pointer bypassed every stored word and value read.
    OutputPointerNull,
    /// Nonzero raw mode bypassed the alternate branch.
    ModeNotZero,
    /// Zero raw C4 bypassed the alternate branch.
    AlternateDisabled,
    /// Absolute binary32 magnitude was outside the inclusive native interval.
    MagnitudeOutsideInterval,
}

/// Validation scope names the floating environment assumption of alternate selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendFormatCheckMode {
    /// Exact signed32 word arithmetic and ASCII string construction.
    ExactWordArithmetic,
    /// Exact local selection with masked FP exceptions and DAZ disabled.
    ExactMaskedExceptionsDazOff,
}

/// Complete local validation coverage, without invoking a native formatter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendFormatCoverage {
    /// Explicit arithmetic/selection mode.
    pub check_mode: SimulationLegendFormatCheckMode,
    /// Bits 0..3 identify consumed B8, BC, C4 and C8 respectively.
    pub consumed_member_mask: u8,
    /// Whether pointer presence was required and inspected.
    pub pointer_checked: bool,
    /// Whether the original binary32 value was required and inspected.
    pub value_checked: bool,
    /// Constructed ASCII byte count, zero when no string was produced.
    pub output_ascii_bytes: usize,
    /// False; this read/projection operation never edits retained archive bytes.
    pub wire_edited: bool,
    /// Exact local construction/selection has zero achieved pattern error.
    pub achieved_pattern_error_bits: u32,
}

/// All supplied parents survive independently of the subset read by a branch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendFormatHistory {
    /// Original explicit words, including unconsumed optional values.
    pub inputs: SimulationLegendFormatInputs,
    /// Original runtime facts; primary construction has no runtime argument.
    pub runtime: Option<SimulationLegendFormatRuntime>,
    /// Derived for a constructed string; None when no output is produced.
    pub output_relation: Option<SimulationThermalRelation>,
}

/// Boundaries returned alongside every completed local result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendFormatDiagnostic {
    /// SolidWorks and CString target code were not executed.
    NativeNotExecuted,
    /// Construction does not establish successful printf/MFC numeric rendering.
    NativeFormattingNotModeled,
    /// Localization and numeric display output remain outside this operation.
    LocaleNotModeled,
    /// Actual invocation, output pointer and input value are not inferred.
    RuntimeNotInferred,
}
