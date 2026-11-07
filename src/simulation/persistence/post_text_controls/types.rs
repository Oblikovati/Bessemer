use super::super::SimulationPostStyleText;

/// Raw stored display controls and complete counted text without inferred runtime state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationPostTextControls {
    /// Style23C; any nonzero value selects runtime primary text.
    pub primary_enabled_raw: i32,
    /// Style240; any nonzero value selects all seven runtime line positions.
    pub seven_lines_enabled_raw: i32,
    /// All counted code units, including units after an embedded NUL.
    pub stored_text: SimulationPostStyleText,
}

/// Explicit current CString units; None is unknown and Some(empty) is known empty.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationPostTextRuntime {
    /// Current primary CString, which may differ from the latest stored payload.
    pub primary: Option<SimulationPostStyleText>,
    /// The seven runtime members at250 through280, without invented wire fields.
    pub lines: [Option<SimulationPostStyleText>; 7],
}

/// Exact local source of selected text, without a font or UI interpretation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPostTextOrigin {
    /// A local empty literal selected by a zero display control.
    EmptyLiteral,
    /// Explicit current primary CString at248.
    RuntimePrimary,
    /// Explicit runtime line position0 through6.
    RuntimeLine(u8),
}

/// One known selection; raw runtime units are retained without NUL truncation or Unicode repair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationPostTextSelection {
    /// Source identity, including a line's position.
    pub origin: SimulationPostTextOrigin,
    /// Owned immutable units selected for the local consumer.
    pub text: SimulationPostStyleText,
}

/// Distinguishes an actual positive-count assignment from count0 preservation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPostTextAssignmentKind {
    /// Assigned the prefix before the first zero unit of a positive-count payload.
    AssignedPrefix,
    /// Count0 skipped assignment and retained the known previous CString.
    PreservedPrevious,
}

/// Completion concerns known local strings only, not native text output or layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPostTextCompletion {
    /// Every resulting selected string has zero code units.
    CompletedEmpty,
    /// At least one resulting string contains code units.
    CompletedText,
}

/// Exact local units and explicit resource policy; no font/layout validation is implied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationPostTextCoverage {
    /// Original stored count retained for provenance, even when selection does not read it.
    pub retained_wire_code_units: usize,
    /// Units in the known local output strings.
    pub result_code_units: usize,
    /// Reached CString dependencies in this stage; composed assignment retains separate coverage.
    pub required_runtime_strings: usize,
    /// Supplied per-operation bound; assignment also bounds its counted input.
    pub maximum_code_units: usize,
    /// True; only assignment/selection of raw units is covered.
    pub local_units_only: bool,
    /// Exact code-unit operations have zero achieved pattern error.
    pub achieved_pattern_error_units: usize,
}

/// Exclusions returned with every completed local assignment or selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPostTextDiagnostic {
    /// SolidWorks and CString target code were not executed.
    NativeNotExecuted,
    /// Raw units are not promoted into validated Unicode scalar values.
    UnicodeNotValidated,
    /// Empty strings do not establish skipped font, measurement or output work.
    LayoutNotModeled,
    /// Runtime string production and UI labels are not inferred.
    RuntimeProductionNotInferred,
}
