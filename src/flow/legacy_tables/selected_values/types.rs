use super::*;
/// Explicit map assumption and independent actual descriptor bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowSelectionContextOptions {
    /// Required caller declaration; reused nonempty native maps are outside this profile.
    pub assumption: FlowSelectionMapAssumption,
    /// Actual link/map entries; zero admits complete empty inputs.
    pub maximum_entries: usize,
    /// Actual selected ordinals; zero admits any genuinely empty selection.
    pub maximum_selected_values: usize,
}
/// Immutable fresh selection retaining both complete input revisions.
#[derive(Clone, Debug)]
pub struct FlowSelectionContext {
    pub(super) links: FlowLinkRecords,
    pub(super) states: FlowByteStates,
    pub(super) options: FlowSelectionContextOptions,
    pub(super) dense_indices: Arc<[Option<usize>]>,
    pub(super) ordinals: Arc<[usize]>,
}
/// Complete supplied-input framing and exact fresh-map coverage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowSelectionContextCoverage {
    /// Every physical input entry is checked, without sampling.
    pub mode: FlowSelectionCheckMode,
    /// Complete link framing/transport mode supplied to the typed link reader.
    pub link_mode: FlowLegacyTableCheckMode,
    /// Complete state framing, always exact zlib.
    pub state_mode: FlowLegacyTableCheckMode,
    /// Actual physical link/map count.
    pub links: usize,
    /// Actual supplied state bytes, possibly zero with nonempty links.
    pub state_bytes: usize,
    /// Actual selected ordinals in physical order.
    pub selected_values: usize,
    /// Actual absent entries; absence differs from a selected zero value.
    pub absent_entries: usize,
    /// Caller-declared native map assumption.
    pub assumption: FlowSelectionMapAssumption,
    /// Zero serialized approximation error.
    pub achieved_error: usize,
}
/// Independent complete raw-payload and actual wide-value bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowSelectedValueOptions {
    /// Complete raw bytes; zero admits a complete empty selection.
    pub maximum_payload_bytes: usize,
    /// Actual selected raw wide patterns; zero admits a complete empty selection.
    pub maximum_values: usize,
}
/// Immutable complete headerless selected values and their retained input context.
#[derive(Clone, Debug)]
pub struct FlowSelectedValues {
    pub(super) wire: TableWire,
    pub(super) context: FlowSelectionContext,
    pub(super) options: FlowSelectedValueOptions,
}
/// Raw-value identity bound to the value revision, input revisions and declared dialect.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowSelectedValueId {
    pub(super) record: FlowRecordId,
    pub(super) links: FlowRecordId,
    pub(super) states: FlowRecordId,
    pub(super) transport: FlowLegacyTableTransport,
    pub(super) assumption: FlowSelectionMapAssumption,
    pub(super) index: usize,
}
/// Exact headerless payload coverage under an explicit retained fresh selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowSelectedValueCoverage {
    /// Always complete exact raw framing through EOF.
    pub mode: FlowLegacyTableCheckMode,
    /// Complete raw payload bytes; no version/count prefix is stored.
    pub payload_bytes: usize,
    /// Input physical link count.
    pub links: usize,
    /// Actual raw patterns, derived from the complete input context.
    pub selected_values: usize,
    /// Input entries without a selected value.
    pub absent_entries: usize,
    /// Actual stored scalars, equal to the raw pattern count.
    pub stored_values: usize,
    /// Zero serialized approximation error.
    pub achieved_error: usize,
}
