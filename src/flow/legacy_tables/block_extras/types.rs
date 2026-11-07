use super::*;

/// Explicit runtime assumptions and independent limits, each normalized before parsing.
#[derive(Clone, Copy, Debug)]
pub struct FlowBlockExtraOptions {
    /// Caller-selected raw/zlib transport.
    pub compression: FlowLegacyTableTransport,
    /// Required explicit enabled-state assumption.
    pub enabled: FlowBlockExtrasEnabled,
    /// Required external leaf count; no tree-order correspondence is inferred.
    pub leaf_count: FlowLeafCount,
    /// Complete decoded-byte limit, at least four.
    pub maximum_decoded_bytes: usize,
    /// Positive maximum physical blocks.
    pub maximum_blocks: usize,
    /// Positive maximum scope descriptors, including the record itself.
    pub maximum_entities: usize,
    /// Positive maximum field descriptors, including present empty arrays.
    pub maximum_fields: usize,
    /// Positive total stored scalar/array elements, including counts and version.
    pub maximum_values: usize,
}

/// Immutable complete legacy record-408 fields and physical scope descriptors.
#[derive(Clone, Debug)]
pub struct FlowBlockExtras {
    pub(super) wire: TableWire,
    pub(super) options: FlowBlockExtraOptions,
    pub(super) entities: Arc<[ExtraEntitySpan]>,
    pub(super) fields: Arc<[ExtraFieldSpan]>,
    pub(super) values: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ExtraIndexSource {
    Derived(i32),
    Stored(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ExtraEntitySpan {
    pub(super) entity: FlowBlockExtraEntity,
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) index: Option<ExtraIndexSource>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ExtraFieldSpan {
    pub(super) entity: FlowBlockExtraEntity,
    pub(super) kind: FlowBlockExtraFieldKind,
    pub(super) encoding: FlowBlockExtraEncoding,
    pub(super) start: usize,
    pub(super) count: usize,
    pub(super) stride: usize,
}

/// Revision-bound field identity, also bound to the external leaf-count interpretation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowBlockExtraFieldId {
    pub(super) record: FlowRecordId,
    pub(super) leaf_count: FlowLeafCount,
    pub(super) ordinal: usize,
}

/// Borrowed complete field descriptor and checked element access.
#[derive(Clone, Copy, Debug)]
pub struct FlowBlockExtraField<'a> {
    pub(super) owner: &'a FlowBlockExtras,
    pub(super) ordinal: usize,
    pub(super) span: &'a ExtraFieldSpan,
}

/// Borrowed physical scope, with stored/derived nested indices kept distinct.
#[derive(Clone, Copy, Debug)]
pub struct FlowBlockExtraScope<'a> {
    pub(super) owner: &'a FlowBlockExtras,
    pub(super) span: &'a ExtraEntitySpan,
}

/// Complete physical layout; mixed version-eight fields have twelve-byte strides.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowBlockExtraFieldLayout {
    /// First element's decoded byte offset, or the empty array's offset.
    pub first_byte: usize,
    /// Complete number of elements, including a present zero count.
    pub element_count: usize,
    /// Width of one stored element.
    pub element_bytes: usize,
    /// Distance between consecutive elements.
    pub stride_bytes: usize,
}

/// Complete exact serialized coverage, separately retaining caller assumptions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowBlockExtraCoverage {
    /// Exact raw or complete checksum-valid zlib check mode.
    pub mode: FlowLegacyTableCheckMode,
    /// Complete stored payload bytes.
    pub stored_bytes: usize,
    /// Complete decoded payload bytes through exact EOF.
    pub decoded_bytes: usize,
    /// Explicit supplied leaf count.
    pub leaf_count: FlowLeafCount,
    /// Explicit enabled-state assumption.
    pub enabled: FlowBlockExtrasEnabled,
    /// Total physical scopes, including record, blocks, bodies and nested items.
    pub entities: usize,
    /// Total field descriptors, including empty arrays.
    pub fields: usize,
    /// Total scalar/array elements, including structural fields.
    pub values: usize,
    /// Exact serialized error, without approximation.
    pub achieved_error: usize,
}

/// Required exclusions from complete serialized coverage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowBlockExtraDiagnostic {
    /// Enabled state was supplied by the caller, not reconstructed from its owner.
    EnabledContextNotReconstructed,
    /// Leaf sequence has not been assigned to tree traversal order.
    TreeOrderNotAssigned,
    /// Native object references remain raw stored words.
    NativeObjectReferencesNotResolved,
    /// Engineering roles of raw scalars and tables remain unresolved.
    EngineeringRolesNotResolved,
    /// Native runtime state was not reconstructed.
    NativeRuntimeNotReproduced,
    /// Native application acceptance was not executed.
    NativeApplicationNotChecked,
}

/// Completed isolated field edit, carrying full physical history and both checkpoints.
#[derive(Clone, Debug)]
pub struct FlowBlockExtraEdit {
    pub(super) source: FlowBlockExtras,
    pub(super) result: FlowBlockExtras,
    pub(super) carrier_edit: FlowCarrierEdit,
}
