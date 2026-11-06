use super::*;

/// Explicit mode and independent aggregate read/edit bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowLoadStateOptions {
    /// Required caller-selected load-mode assumption.
    pub mode: FlowLoadMode,
    /// Maximum complete raw payload bytes, at least sixteen.
    pub maximum_payload_bytes: usize,
    /// Maximum physical items.
    pub maximum_items: usize,
    /// Total raw UTF16 units across names and descriptions.
    pub maximum_text_units: usize,
    /// Total dynamic array values and fixed mode-tail patterns; excludes fixed member tuples.
    pub maximum_array_values: usize,
    /// Total stored values, including framing, members and UTF16 units.
    pub maximum_values: usize,
    /// Total field descriptors, including present empty arrays.
    pub maximum_fields: usize,
    /// Total physical scopes, including the root and empty strings.
    pub maximum_scopes: usize,
}

/// Immutable complete load-state payload under one explicit mode context.
#[derive(Clone, Debug)]
pub struct FlowLoadState {
    pub(super) wire: TableWire,
    pub(super) options: FlowLoadStateOptions,
    pub(super) fields: Arc<[LoadFieldSpan]>,
    pub(super) scopes: Arc<[LoadScopeSpan]>,
    pub(super) texts: Arc<[LoadTextSpan]>,
    pub(super) totals: [usize; 4],
    pub(super) width: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct LoadFieldSpan {
    pub scope: FlowLoadScope,
    pub kind: FlowLoadFieldKind,
    pub encoding: FlowLoadEncoding,
    pub layout: FlowLoadFieldLayout,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct LoadScopeSpan {
    pub scope: FlowLoadScope,
    pub interval: (usize, usize),
}
#[derive(Clone, Copy, Debug)]
pub(super) struct LoadTextSpan {
    pub scope: FlowLoadScope,
    pub interval: (usize, usize),
    pub units_field: usize,
    pub prefix_units: usize,
}

/// Field identity bound to occurrence, revision and the external mode interpretation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowLoadFieldId {
    pub(super) record: FlowRecordId,
    pub(super) mode: FlowLoadMode,
    pub(super) ordinal: usize,
}
/// Borrowed exact scalar/vector descriptor and checked values.
#[derive(Clone, Copy, Debug)]
pub struct FlowLoadField<'a> {
    pub(super) owner: &'a FlowLoadState,
    pub(super) ordinal: usize,
    pub(super) span: &'a LoadFieldSpan,
}
/// Complete immutable physical scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowLoadScopeView {
    /// Physical ancestry.
    pub scope: FlowLoadScope,
    /// Complete decoded byte interval.
    pub payload_interval: (usize, usize),
}
/// Borrowed lossless stored UTF16 with a separately projected first-NUL prefix.
#[derive(Clone, Copy, Debug)]
pub struct FlowLoadText<'a> {
    pub(super) owner: &'a FlowLoadState,
    pub(super) span: &'a LoadTextSpan,
}

/// Complete serialized accounting, without native owner/runtime acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowLoadCoverage {
    /// Exact framing under the declared context.
    pub check_mode: FlowLoadCheckMode,
    /// External mode assumption.
    pub load_mode: FlowLoadMode,
    /// Complete raw payload bytes.
    pub payload_bytes: usize,
    /// Physical item count.
    pub items: usize,
    /// Stored shared width, including unused large widths on empty records.
    pub array_width: usize,
    /// Complete stored UTF16 units.
    pub text_units: usize,
    /// Dynamic array and mode-tail values actually present.
    pub array_values: usize,
    /// All stored values, including framing and fixed tuples.
    pub values: usize,
    /// Complete field descriptor count.
    pub fields: usize,
    /// Complete physical scope count.
    pub scopes: usize,
    /// Complete name/description count.
    pub texts: usize,
    /// Presence of the fixed mode tail.
    pub mode_tail_present: bool,
    /// Zero serialized approximation error.
    pub achieved_error: usize,
}
/// Completed isolated edit with both validated views and full physical history.
#[derive(Clone, Debug)]
pub struct FlowLoadStateEdit {
    pub(super) source: FlowLoadState,
    pub(super) result: FlowLoadState,
    pub(super) carrier_edit: FlowCarrierEdit,
}
