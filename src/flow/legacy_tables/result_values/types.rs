use super::*;
/// Explicit caller declaration; native objects are not instantiated or inspected.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowImsFreshObjects {
    /// Interpret the value stream as a load into fresh definition objects.
    FreshDeclared,
}
/// Validated IMS-version-two/allocation-zero definitions with exact revision provenance.
#[derive(Clone, Debug)]
pub struct FlowImsValueContext {
    pub(super) definitions: FlowResultDefinitions,
    pub(super) declaration: FlowImsFreshObjects,
}
/// Independent whole-body, object, shared-word and BL row/word bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowImsValueOptions {
    /// Complete inflated bytes, at least one for an empty context.
    pub maximum_decoded_bytes: usize,
    /// Actual objects derived from the retained definitions; zero admits an empty context.
    pub maximum_objects: usize,
    /// Total raw-wide patterns across shared arrays; zero admits present-empty arrays.
    pub maximum_shared_values: usize,
    /// Total actual BL rows, independently of their 18-column width.
    pub maximum_bl_rows: usize,
    /// Total raw-wide patterns across all BL rows, independently of shared arrays.
    pub maximum_bl_values: usize,
}
/// Physical shared-array position, without assigning engineering roles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowImsValueArray {
    /// Conditional first shared array.
    First,
    /// Mandatory main shared array.
    Main,
    /// Conditional trailing shared array.
    Last,
}
/// Raw nonzero-presence selector; stored values are retained without boolean normalization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowImsValueFlag {
    /// Controls the first shared array.
    First,
    /// Controls BL rows.
    Bl,
    /// Controls the trailing shared array.
    Last,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ValueInterval {
    pub start: usize,
    pub end: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ValueSpan {
    pub body: ValueInterval,
    pub identifier: i32,
    pub flags: [usize; 3],
    pub arrays: [Option<ValueInterval>; 3],
    pub bl: Option<ValueInterval>,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct ValueTotals {
    pub shared: usize,
    pub bl_rows: usize,
    pub bl_values: usize,
}
/// Immutable complete values with both value-carrier and exact definition checkpoints.
#[derive(Clone, Debug)]
pub struct FlowImsValues {
    pub(super) wire: TableWire,
    pub(super) context: FlowImsValueContext,
    pub(super) options: FlowImsValueOptions,
    pub(super) objects: Arc<[ValueSpan]>,
    pub(super) totals: ValueTotals,
}
/// Borrowed physical IMS value object, preserving duplicate inherited identifiers.
#[derive(Clone, Copy, Debug)]
pub struct FlowImsValue<'a> {
    pub(super) owner: &'a FlowImsValues,
    pub(super) ordinal: usize,
    pub(super) span: &'a ValueSpan,
}
/// Borrowed complete BL body with exactly 18 uninterpreted wide patterns per row.
#[derive(Clone, Copy, Debug)]
pub struct FlowImsBlRows<'a> {
    pub(super) owner: &'a FlowImsValues,
    pub(super) span: ValueInterval,
}
/// Complete exact serialized coverage with explicit retained definition provenance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowImsValueCoverage {
    /// One checksum-valid zlib stream and complete decoded EOF.
    pub mode: FlowLegacyTableCheckMode,
    /// Exact definition revision, independently of the value record's carrier.
    pub definition_record: FlowRecordId,
    /// Explicit fresh-object assumption, without native execution credit.
    pub fresh_objects: FlowImsFreshObjects,
    /// Complete stored value payload bytes.
    pub stored_bytes: usize,
    /// Complete inflated value bytes.
    pub decoded_bytes: usize,
    /// Actual physical objects derived from definitions.
    pub stored_objects: usize,
    /// Present shared arrays, including present-empty arrays.
    pub shared_arrays: usize,
    /// Present shared arrays with zero values.
    pub empty_shared_arrays: usize,
    /// Total shared raw-wide patterns.
    pub shared_values: usize,
    /// Present BL bodies, including bodies with zero rows.
    pub bl_bodies: usize,
    /// Present BL bodies with zero rows.
    pub empty_bl_bodies: usize,
    /// Total actual BL rows.
    pub bl_rows: usize,
    /// Total BL raw-wide patterns, exactly 18 per row.
    pub bl_values: usize,
    /// Zero for exact serialized checks; no geometric accuracy is asserted.
    pub achieved_error: usize,
}
/// Context, runtime and interpretation exclusions for complete serialized values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowImsValueDiagnostic {
    /// Caller declared fresh objects; the native object state was not inspected.
    FreshObjectsAssumed,
    /// Caller paired these values with retained definitions; no engineering join was inferred.
    DefinitionPairingAssumed,
    /// Native loading and partial-on-failure mutation were not executed.
    NativeRuntimeNotReproduced,
    /// Raw array positions and BL columns have no assigned engineering meanings.
    EngineeringColumnsNotAssigned,
    /// Native application acceptance was not executed.
    NativeApplicationNotChecked,
}
