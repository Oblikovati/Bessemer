use super::*;
/// Caller-declared native profile; the outer version does not infer this context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowViewerPrefixProfile {
    /// Viewer inflate-metadata-query routine, distinct from FWCPT01 graph layouts.
    InflateMetadataQuery,
}
/// Explicit profile with independent complete decoded-body and known-wide bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowViewerPrefixOptions {
    /// Required caller declaration; no graph-profile fallback or inference is performed.
    pub profile: FlowViewerPrefixProfile,
    /// Complete decoded body including opaque suffix, at least 72 bytes.
    pub maximum_decoded_bytes: usize,
    /// Known array values plus the three discarded values, at least six.
    pub maximum_prefix_wide_values: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ViewerSpan {
    pub start: usize,
    pub end: usize,
}
impl ViewerSpan {
    pub(super) const EMPTY: Self = Self { start: 0, end: 0 };
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ViewerLayout {
    pub arrays: [ViewerSpan; 3],
    pub triplet_start: usize,
    pub prefix_end: usize,
    pub wide_values: usize,
}
/// Immutable exact known prefix and preserved opaque body under an explicit viewer assumption.
#[derive(Clone, Debug)]
pub struct FlowViewerPrefix {
    pub(super) wire: TableWire,
    pub(super) options: FlowViewerPrefixOptions,
    pub(super) layout: ViewerLayout,
}
/// Documented native query outputs, without assigning their engineering meanings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowViewerMetadata {
    /// Final stored signed word, forwarded unchanged.
    pub first_i32: i32,
    /// Stored second word, except a stored one projects to two.
    pub second_i32: i32,
}
/// Existing signed prefix fields; array lengths and outer version remain structural.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowViewerSignedField {
    /// First decoded word, ignored by the native query outputs.
    IgnoredInitial,
    /// Second decoded word, before the one-to-two query projection.
    StoredSecond,
    /// Final signed word of the known prefix.
    FinalWord,
}
/// Exact verified scope; this does not report a full decoded-body pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowViewerPrefixCheckMode {
    /// Exact outer version, checksum-valid single zlib stream and complete known prefix.
    ExactVersionZlibAndKnownPrefix,
}
/// Required full-body checks remain incomplete under this prefix-only profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowViewerBodyValidation {
    /// Unknown continuation is not validated, including when its stored extent is empty.
    Incomplete,
}
/// Prefix/transport coverage keeps known decoded bytes separate from opaque continuation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowViewerPrefixCoverage {
    /// Exact transport and known-prefix check scope.
    pub mode: FlowViewerPrefixCheckMode,
    /// Explicit caller-declared native profile.
    pub profile: FlowViewerPrefixProfile,
    /// Entire stored payload; no proportional compressed semantic credit is assigned.
    pub stored_payload_bytes: usize,
    /// Raw version prefix bytes, outside the zlib stream.
    pub raw_outer_prefix_bytes: usize,
    /// Complete inflated body extent, including unknown bytes.
    pub decoded_body_bytes: usize,
    /// Exact known decoded prefix extent.
    pub known_decoded_prefix_bytes: usize,
    /// Preserved opaque decoded continuation extent.
    pub opaque_decoded_bytes: usize,
    /// Raw wide patterns in the three arrays and discarded triplet.
    pub known_wide_values: usize,
    /// Full-body validation is incomplete, independently of opaque byte count.
    pub body_validation: FlowViewerBodyValidation,
    /// Zero for exact prefix checks; this is not a full-body or geometric error claim.
    pub achieved_prefix_error: usize,
}
/// Explicit profile, continuation, native runtime and interpretation exclusions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowViewerPrefixDiagnostic {
    /// The viewer-query profile was declared without reconstructing its owner context.
    ViewerQueryProfileAssumed,
    /// Full decoded body validation remains incomplete.
    OpaqueBodyNotValidated,
    /// No engineering units or metadata meanings are assigned.
    EngineeringRolesNotResolved,
    /// Native query state and partial-on-failure mutation are not reproduced.
    NativeRuntimeNotReproduced,
    /// Native application acceptance was not executed.
    NativeApplicationNotChecked,
}
/// Completed isolated prefix edit; full decoded-body validation remains incomplete.
#[derive(Clone, Debug)]
pub struct FlowViewerPrefixEdit {
    pub(super) source: FlowViewerPrefix,
    pub(super) result: FlowViewerPrefix,
    pub(super) carrier_edit: FlowCarrierEdit,
}
