use super::*;
/// Independent bounds on complete raw payload, physical row descriptors and actual wide values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowTaggedStateOptions {
    /// Maximum complete raw bytes, at least four.
    pub maximum_payload_bytes: usize,
    /// Total row descriptors, including zero-tag rows; zero admits the empty record.
    pub maximum_rows: usize,
    /// Total actual raw wide values across all present rows; zero admits empty arrays.
    pub maximum_wide_words: usize,
}
/// Immutable complete raw version-nine record, retaining the separate leading tag block.
#[derive(Clone, Debug)]
pub struct FlowTaggedState {
    pub(super) wire: TableWire,
    pub(super) options: FlowTaggedStateOptions,
    pub(super) rows: Arc<[TaggedRowSpan]>,
    pub(super) wide_words: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct TaggedRowSpan {
    pub interval: (usize, usize),
    pub values: Option<(usize, usize)>,
}
/// Borrowed physical row, with its tag outside the row-body interval.
#[derive(Clone, Copy, Debug)]
pub struct FlowTaggedStateRow<'a> {
    pub(super) owner: &'a FlowTaggedState,
    pub(super) ordinal: usize,
    pub(super) span: &'a TaggedRowSpan,
}
/// Exact complete raw framing and physical field counts, separate from native acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowTaggedStateCoverage {
    /// Always exact raw framing through EOF.
    pub mode: FlowLegacyTableCheckMode,
    /// Complete raw payload bytes.
    pub payload_bytes: usize,
    /// Actual physical rows, including zero-body rows.
    pub rows: usize,
    /// Complete leading tag-block bytes, equal to row count.
    pub tag_bytes: usize,
    /// Nonzero-tag rows with stored width prefixes, including present-empty arrays.
    pub present_rows: usize,
    /// Actual stored wide patterns.
    pub wide_words: usize,
    /// Signed tail presence for positive row counts.
    pub tail_present: bool,
    /// All stored scalars, including version, count, tag bytes, widths and tail.
    pub stored_values: usize,
    /// Zero serialized approximation error.
    pub achieved_error: usize,
}
/// Qualifications accompanying every complete tagged-state result and edit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowTaggedStateDiagnostic {
    /// Native tag/context acceptance was not reconstructed.
    NativeContextValidationNotReproduced,
    /// Native runtime state was not populated.
    NativeRuntimeStateNotReproduced,
    /// Raw wide columns have no assigned engineering roles.
    EngineeringColumnsNotAssigned,
    /// No acceptance check was performed in SolidWorks.
    NativeApplicationNotChecked,
}
/// Completed isolated edit with immutable source/result checkpoints and physical carrier history.
#[derive(Clone, Debug)]
pub struct FlowTaggedStateEdit {
    pub(super) source: FlowTaggedState,
    pub(super) result: FlowTaggedState,
    pub(super) carrier_edit: FlowCarrierEdit,
}
