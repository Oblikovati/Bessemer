use super::*;

/// Independent aggregate limits. Zero collection bounds admit empty collections.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowConditionalItemOptions {
    /// Maximum stored payload bytes, including any raw prefix.
    pub maximum_stored_bytes: usize,
    /// Maximum decoded bytes; record128 excludes its four raw version bytes.
    pub maximum_decoded_bytes: usize,
    /// Total presence positions, including absent objects.
    pub maximum_entries: usize,
    /// Total present items, including inactive items.
    pub maximum_items: usize,
    /// Total header groups and item groups, including empty groups.
    pub maximum_groups: usize,
    /// Total legacy and modern subrecords.
    pub maximum_subrecords: usize,
    /// Total raw vector values, after block multiplication; includes byte vectors.
    pub maximum_vector_values: usize,
    /// Total stored values, including scalar members and structural fields.
    pub maximum_values: usize,
    /// Total scope descriptors, including the root.
    pub maximum_scopes: usize,
    /// Total field descriptors, including present empty vectors.
    pub maximum_fields: usize,
}

/// Immutable complete conditional-item record with exact field and scope coverage.
#[derive(Clone, Debug)]
pub struct FlowConditionalItems {
    pub(super) wire: TableWire,
    pub(super) format: FlowConditionalFormat,
    pub(super) options: FlowConditionalItemOptions,
    pub(super) fields: Arc<[ConditionalFieldSpan]>,
    pub(super) scopes: Arc<[ConditionalScopeSpan]>,
    pub(super) totals: [usize; 6],
    pub(super) scale: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ConditionalScopeSpan {
    pub kind: FlowConditionalScopeKind,
    pub parent: Option<usize>,
    pub ordinal: usize,
    pub interval: (usize, usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ConditionalFieldSpan {
    pub scope: usize,
    pub kind: FlowConditionalFieldKind,
    pub encoding: FlowConditionalEncoding,
    pub layout: FlowConditionalFieldLayout,
}

/// Scope identity bound to one original occurrence and revision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowConditionalScopeId {
    pub(super) record: FlowRecordId,
    pub(super) ordinal: usize,
}

/// Field identity bound to one original occurrence and revision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowConditionalFieldId {
    pub(super) record: FlowRecordId,
    pub(super) ordinal: usize,
}

/// Borrowed complete physical scope and ancestry.
#[derive(Clone, Copy, Debug)]
pub struct FlowConditionalScope<'a> {
    pub(super) owner: &'a FlowConditionalItems,
    pub(super) ordinal: usize,
    pub(super) span: &'a ConditionalScopeSpan,
}

/// Borrowed existing scalar/vector descriptor, with checked typed access.
#[derive(Clone, Copy, Debug)]
pub struct FlowConditionalField<'a> {
    pub(super) owner: &'a FlowConditionalItems,
    pub(super) ordinal: usize,
    pub(super) span: &'a ConditionalFieldSpan,
}

/// Exact physical and decoded coverage, with no runtime or engineering credit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowConditionalCoverage {
    /// Explicit codec/prefix check mode.
    pub mode: FlowConditionalCheckMode,
    /// Complete stored payload bytes.
    pub stored_bytes: usize,
    /// Raw prefix bytes outside compression, zero or four.
    pub stored_prefix_bytes: usize,
    /// Complete decoded bytes through EOF, excluding any raw prefix.
    pub decoded_bytes: usize,
    /// Patterns per counted wide-vector element: one, nine or six.
    pub wide_patterns_per_vector_element: usize,
    /// Complete physical presence-position count.
    pub entries: usize,
    /// Complete present-item count.
    pub items: usize,
    /// Complete header/item-group count.
    pub groups: usize,
    /// Complete subrecord count.
    pub subrecords: usize,
    /// Total physical vector values, including bytes.
    pub vector_values: usize,
    /// Total stored values, including framing.
    pub values: usize,
    /// Complete descriptor count, including empty fields.
    pub fields: usize,
    /// Complete scope count, including empty scopes.
    pub scopes: usize,
    /// Zero serialized approximation error.
    pub achieved_error: usize,
}

/// Completed isolated edit with both validated views and full carrier history.
#[derive(Clone, Debug)]
pub struct FlowConditionalItemEdit {
    pub(super) source: FlowConditionalItems,
    pub(super) result: FlowConditionalItems,
    pub(super) carrier_edit: FlowCarrierEdit,
}
