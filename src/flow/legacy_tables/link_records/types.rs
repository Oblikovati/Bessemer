use super::*;

/// Explicit transport, optional reference range and independent resource limits.
#[derive(Clone, Copy, Debug)]
pub struct FlowLinkRecordOptions {
    /// Raw or complete RFC 1950 stream, supplied by the caller.
    pub compression: FlowLegacyTableTransport,
    /// Optional reference bound; -1 remains a sentinel even for a zero bound.
    pub global_pointer_count: Option<FlowGlobalPointerCount>,
    /// Maximum decoded bytes, at least 52 for an empty version-zero record.
    pub maximum_decoded_bytes: usize,
    /// Positive maximum number of physical entries.
    pub maximum_entries: usize,
    /// Positive total dynamic elements across all variable, conditional and tail arrays.
    pub maximum_array_elements: usize,
}

/// Immutable complete legacy record-402 checkpoint and physical field spans.
#[derive(Clone, Debug)]
pub struct FlowLinkRecords {
    pub(super) wire: TableWire,
    pub(super) options: FlowLinkRecordOptions,
    pub(super) entries: Arc<[EntrySpan]>,
    pub(super) tail: LinkTailSpan,
    pub(super) array_elements: usize,
    pub(super) variable_integers: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct EntrySpan {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) variable_count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct LinkTailSpan {
    pub(super) start: usize,
    pub(super) discarded: (usize, usize),
    pub(super) control: Option<usize>,
    pub(super) conditional: (usize, usize),
    pub(super) numeric: Option<NumericSpan>,
    pub(super) final_tail: Option<FinalSpan>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct NumericSpan {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) count: usize,
    pub(super) indices: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct FinalSpan {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) count: usize,
    pub(super) width: usize,
}

/// Borrowed exact physical link entry, including stored and converted code bytes.
#[derive(Clone, Copy, Debug)]
pub struct FlowLinkEntry<'a> {
    pub(super) owner: &'a FlowLinkRecords,
    pub(super) ordinal: usize,
    pub(super) span: &'a EntrySpan,
}

/// Borrowed signed words retaining the exact decoded interval.
#[derive(Clone, Copy, Debug)]
pub struct FlowLinkIntegers<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) start: usize,
}

/// Borrowed complete link tail, without reconstructing runtime state.
#[derive(Clone, Copy, Debug)]
pub struct FlowLinkTail<'a> {
    pub(super) owner: &'a FlowLinkRecords,
}

/// Borrowed numeric tail, distinguishing stored indices from runtime defaults.
#[derive(Clone, Copy, Debug)]
pub struct FlowLinkNumericTail<'a> {
    pub(super) owner: &'a FlowLinkRecords,
    pub(super) span: &'a NumericSpan,
}

/// Borrowed final tail with its version-selected physical element width.
#[derive(Clone, Copy, Debug)]
pub struct FlowLinkFinalTail<'a> {
    pub(super) owner: &'a FlowLinkRecords,
    pub(super) span: &'a FinalSpan,
}

/// Exact serialized coverage, retaining external context separately from native acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowLinkRecordCoverage {
    /// Complete raw or checksum-valid compressed check mode.
    pub mode: FlowLegacyTableCheckMode,
    /// Complete stored bytes.
    pub stored_bytes: usize,
    /// Complete decoded bytes through exact EOF.
    pub decoded_bytes: usize,
    /// Physical entries in wire order.
    pub entries: usize,
    /// Variable integers in entry bodies only.
    pub variable_integers: usize,
    /// Total dynamic array elements consumed against the shared array budget.
    pub array_elements: usize,
    /// Optional caller-supplied bound; sentinel -1 is legal independently.
    pub pointer_range: Option<FlowGlobalPointerCount>,
    /// Exact serialized error, without approximation.
    pub achieved_error: usize,
}

/// Required exclusions from complete serialized link coverage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowLinkRecordDiagnostic {
    /// Native object pointers and derived runtime structures were not reproduced.
    NativeRuntimeNotReproduced,
    /// Engineering roles remain unresolved.
    EngineeringRolesNotResolved,
    /// Compression context was supplied, rather than recovered from the native owner.
    OwnerCompressionContextNotReconstructed,
    /// Native application acceptance was not executed.
    NativeApplicationNotChecked,
}

/// Completed isolated link edit with full carrier history and both checkpoints.
#[derive(Clone, Debug)]
pub struct FlowLinkRecordEdit {
    pub(super) source: FlowLinkRecords,
    pub(super) result: FlowLinkRecords,
    pub(super) carrier_edit: FlowCarrierEdit,
}
