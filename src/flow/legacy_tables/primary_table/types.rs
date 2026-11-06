use super::*;

/// Explicit class and independent aggregate resource limits. Zero admits only empty collections.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowPrimaryTableOptions {
    /// Required explicit class selection.
    pub profile: FlowPrimaryTableProfile,
    /// Complete raw payload bound, at least eight bytes.
    pub maximum_decoded_bytes: usize,
    /// Total physical items.
    pub maximum_items: usize,
    /// Total typed elements across both groups of every item.
    pub maximum_elements: usize,
    /// Total ragged rows, including empty rows.
    pub maximum_rows: usize,
    /// Total sixteen-pattern blocks.
    pub maximum_wide_blocks: usize,
    /// Total scopes, including the root and empty scopes.
    pub maximum_scopes: usize,
    /// Total fields, including empty strided columns and structural fields.
    pub maximum_fields: usize,
    /// Total stored scalar values, including framing.
    pub maximum_values: usize,
}

/// Complete immutable raw primary table, with no unresolved trailing bytes.
#[derive(Clone, Debug)]
pub struct FlowPrimaryTable {
    pub(super) wire: TableWire,
    pub(super) options: FlowPrimaryTableOptions,
    pub(super) scopes: Arc<[PrimaryScopeSpan]>,
    pub(super) fields: Arc<[PrimaryFieldSpan]>,
    pub(super) totals: [usize; 5],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct PrimaryScopeSpan {
    pub scope: FlowPrimaryScope,
    pub interval: (usize, usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct PrimaryFieldSpan {
    pub scope: FlowPrimaryScope,
    pub kind: FlowPrimaryFieldKind,
    pub encoding: FlowPrimaryEncoding,
    pub layout: FlowPrimaryFieldLayout,
}

/// Complete column layout, including zero-length columns and serialized strides.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowPrimaryFieldLayout {
    /// Decoded position of the first element, or the empty row's column origin.
    pub first_byte: usize,
    /// Physical number of column elements.
    pub element_count: usize,
    /// Serialized distance between elements; twelve for factory three.
    pub stride_bytes: usize,
}

/// Field identity bound to a physical occurrence and source revision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowPrimaryFieldId {
    pub(super) record: FlowRecordId,
    pub(super) ordinal: usize,
}

/// Borrowed checked descriptor of an existing column or scalar.
#[derive(Clone, Copy, Debug)]
pub struct FlowPrimaryField<'a> {
    pub(super) owner: &'a FlowPrimaryTable,
    pub(super) ordinal: usize,
    pub(super) span: &'a PrimaryFieldSpan,
}

/// Complete scope descriptor with all physical parent ordinals.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowPrimaryScopeView {
    /// Physical ancestry key.
    pub scope: FlowPrimaryScope,
    /// Exact complete decoded interval.
    pub payload_interval: (usize, usize),
}

/// Exact full framing and value accounting, without engineering acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowPrimaryCoverage {
    /// Always exact raw framing through EOF.
    pub mode: FlowLegacyTableCheckMode,
    /// Complete stored and decoded raw bytes.
    pub payload_bytes: usize,
    /// Physical item count.
    pub items: usize,
    /// Total typed elements.
    pub elements: usize,
    /// Total rows, including empty rows.
    pub rows: usize,
    /// Total sixteen-pattern blocks.
    pub wide_blocks: usize,
    /// Total stored values, including structural fields.
    pub values: usize,
    /// Complete scope count.
    pub scopes: usize,
    /// Complete field descriptor count.
    pub fields: usize,
    /// Zero serialized approximation error.
    pub achieved_error: usize,
}

/// Completed isolated edit with both validated checkpoints and full physical history.
#[derive(Clone, Debug)]
pub struct FlowPrimaryTableEdit {
    pub(super) source: FlowPrimaryTable,
    pub(super) result: FlowPrimaryTable,
    pub(super) carrier_edit: FlowCarrierEdit,
}
