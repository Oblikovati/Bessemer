use super::*;

/// Independent bounds on complete raw bytes and totals across both physical groups.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowPairedStateOptions {
    /// Maximum complete raw bytes, at least four.
    pub maximum_payload_bytes: usize,
    /// Total actual index words across both groups; zero admits empty groups.
    pub maximum_index_words: usize,
    /// Total actual wide patterns; zero admits absent and present-empty wide arrays.
    pub maximum_wide_words: usize,
}
/// Immutable complete record417, without opaque tails or per-element allocations.
#[derive(Clone, Debug)]
pub struct FlowPairedState {
    pub(super) wire: TableWire,
    pub(super) options: FlowPairedStateOptions,
    pub(super) groups: [PairedGroupSpan; 2],
    pub(super) totals: [usize; 2],
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct PairedGroupSpan {
    pub interval: (usize, usize),
    pub indices: (usize, usize),
    pub wides: Option<(usize, usize)>,
}
/// Borrowed complete group in physical wire order, with distinct optional wide-array presence.
#[derive(Clone, Copy, Debug)]
pub struct FlowPairedStateGroup<'a> {
    pub(super) owner: &'a FlowPairedState,
    pub(super) ordinal: usize,
    pub(super) span: &'a PairedGroupSpan,
}
/// Exact serialized validation and physical array counts, separate from engineering acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowPairedStateCoverage {
    /// Always exact raw framing through EOF.
    pub mode: FlowLegacyTableCheckMode,
    /// Complete raw payload bytes.
    pub payload_bytes: usize,
    /// Always two complete physical groups.
    pub groups: usize,
    /// Total actual index words.
    pub index_words: usize,
    /// Total actual wide patterns.
    pub wide_words: usize,
    /// Number of present wide arrays, including present-empty arrays.
    pub present_wide_arrays: usize,
    /// Zero serialized approximation error.
    pub achieved_error: usize,
}
/// Exclusions accompanying every complete paired-state view and edit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowPairedStateDiagnostic {
    /// Stored index patterns and wide patterns have no assigned engineering roles.
    EngineeringRolesNotResolved,
    /// Native runtime allocation and population were not reconstructed.
    NativeRuntimeNotReproduced,
    /// No acceptance check was performed in SolidWorks.
    NativeApplicationNotChecked,
}
/// Completed isolated edit with immutable typed checkpoints and full physical history.
#[derive(Clone, Debug)]
pub struct FlowPairedStateEdit {
    pub(super) source: FlowPairedState,
    pub(super) result: FlowPairedState,
    pub(super) carrier_edit: FlowCarrierEdit,
}
