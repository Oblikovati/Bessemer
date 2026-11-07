use super::*;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct FieldValueSpan {
    pub full: (usize, usize),
    pub wide: (usize, usize),
}

/// Immutable complete version-prefixed field values under an explicit runtime layout.
#[derive(Clone, Debug)]
pub struct FlowFieldValues {
    pub(super) wire: TableWire,
    pub(super) options: FlowFieldValueOptions,
    pub(super) spans: Arc<[FieldValueSpan]>,
    pub(super) table_words: usize,
    pub(super) wide_words: usize,
}

/// Borrowed global-object position; width is supplied and zero-width positions are retained.
#[derive(Clone, Copy, Debug)]
pub struct FlowFieldObject<'a> {
    pub(super) owner: &'a FlowFieldValues,
    pub(super) ordinal: usize,
    pub(super) width: FlowFieldCount,
    pub(super) span: &'a FieldValueSpan,
}

/// Borrowed region with stored width and explicit row/storage context.
#[derive(Clone, Copy, Debug)]
pub struct FlowFieldRegion<'a> {
    pub(super) owner: &'a FlowFieldValues,
    pub(super) ordinal: usize,
    pub(super) context: FlowFieldRegionContext,
    pub(super) span: &'a FieldValueSpan,
}

/// Exact physical check mode keeps the version prefix outside body decompression.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowFieldValueCheckMode {
    /// Four raw version bytes plus complete raw body through EOF.
    ExactRawVersionedBody,
    /// Four raw version bytes plus one complete checksum-valid zlib body and decoded EOF.
    ExactZlibVersionedBody,
}

/// Complete byte accounting and actual serialized values, without runtime acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowFieldValueCoverage {
    /// Complete version/body check mode.
    pub mode: FlowFieldValueCheckMode,
    /// Always four raw version bytes.
    pub stored_version_bytes: usize,
    /// Remaining stored body bytes, including codec framing.
    pub stored_body_bytes: usize,
    /// Complete decoded body, excluding version.
    pub decoded_body_bytes: usize,
    /// Four logical version bytes plus decoded body.
    pub decoded_payload_bytes: usize,
    /// Physical width-table words in legacy layouts.
    pub width_table_words: usize,
    /// Global-object positions, including zero-width positions.
    pub objects: usize,
    /// Region positions, including inactive/empty regions.
    pub regions: usize,
    /// Actual raw wide patterns.
    pub wide_words: usize,
    /// Legacy-only check presence, including wildcard or empty check lists.
    pub expected_width_table_checked: Option<bool>,
    /// True only for a checked region layout.
    pub expected_region_count_checked: bool,
    /// True only for a checked region layout.
    pub expected_region_widths_checked: bool,
    /// Exact serialized error, without approximation.
    pub achieved_error: usize,
}

/// Required exclusions accompanying exact serialized field coverage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowFieldValueDiagnostic {
    /// Native global-object/region layout is supplied, not inferred.
    LayoutContextAssumed,
    /// Native runtime context is not executed.
    NativeContextNotExecuted,
    /// Native allocation and storage are not simulated.
    NativeStorageNotReproduced,
    /// Raw patterns have no assigned engineering column names or units.
    EngineeringColumnsNotResolved,
    /// Selected region numeric data is not automatically bound to global record 102.
    SelectedNumericBindingNotEstablished,
    /// Native object-form and allocation flags are not validated from widths alone.
    NativeObjectFormsNotValidated,
    /// Caller body transport does not reconstruct its compression owner.
    OwnerCompressionContextNotReconstructed,
    /// Native application acceptance was not executed.
    NativeApplicationNotChecked,
}

/// Completed isolated existing-field edit with typed checkpoints and physical history.
#[derive(Clone, Debug)]
pub struct FlowFieldValueEdit {
    pub(super) source: FlowFieldValues,
    pub(super) result: FlowFieldValues,
    pub(super) carrier_edit: FlowCarrierEdit,
}
