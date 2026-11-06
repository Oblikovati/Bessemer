use super::*;

/// Explicit supported native string dialect.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMetadataStringMode {
    /// Signed code-unit count followed by raw UTF-16LE, with a terminal NUL when nonempty.
    NativeUtf16,
}
/// Independent positive physical and metadata bounds for the prefix reader.
#[derive(Clone, Copy, Debug)]
pub struct FlowMetadataPrefixOptions {
    /// Explicit string dialect, never inferred from bytes.
    pub string_mode: FlowMetadataStringMode,
    /// Bound for the complete payload including opaque tail, at least one.
    pub maximum_payload_bytes: usize,
    /// Maximum vector headers.
    pub maximum_vectors: usize,
    /// Maximum object headers.
    pub maximum_objects: usize,
    /// Maximum total vector values.
    pub maximum_values: usize,
    /// Maximum total stored UTF-16 code units, including terminators.
    pub maximum_string_units: usize,
}
/// Stored object-string role, independent of native text fallback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMetadataStringRole {
    /// First stored string, member 30.
    First,
    /// Second stored string, member 00.
    Second,
    /// Third stored string, member 60.
    Third,
    /// Fourth stored string, member 90; absent in version two.
    Fourth,
}
/// Exact prefix coverage; complete record acceptance is always false.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowMetadataPrefixCoverage {
    /// Complete stored payload, including the opaque tail.
    pub payload_bytes: usize,
    /// Exactly decoded bytes through the prefix boundary.
    pub known_prefix_bytes: usize,
    /// Preserved backend or inactive-branch bytes.
    pub unknown_bytes: usize,
    /// Complete outer record schema has not been checked.
    pub full_payload_schema: bool,
    /// Exact prefix byte error, without approximation.
    pub achieved_error: usize,
}
/// Why parsing stopped at the retained boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMetadataPrefixStop {
    /// Signed version is at most one; all following bytes are opaque.
    InactiveVersionBranch,
    /// Active branch reached the backend virtual-call boundary.
    RequiresBackend,
}
/// Native-state exclusions accompanying every prefix result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMetadataPrefixDiagnostic {
    /// The backend and final outer collections remain opaque.
    BackendNotDecoded,
    /// Native object allocation and other runtime state were not reproduced.
    NativeRuntimeNotReproduced,
    /// Native application acceptance was not executed.
    NativeApplicationNotChecked,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct VectorSpan {
    pub start: usize,
    pub end: usize,
    pub count: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct TextSpan {
    pub start: usize,
    pub end: usize,
    pub native_end: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ObjectSpan {
    pub start: usize,
    pub end: usize,
    pub strings: [Option<TextSpan>; 4],
    pub state: usize,
}
/// Immutable partial decode with exact raw UTF-16 and opaque backend ownership.
#[derive(Clone, Debug)]
pub struct FlowMetadataPrefix {
    pub(super) source: FlowCarrier,
    pub(super) record: FlowRecordId,
    pub(super) raw: Arc<[u8]>,
    pub(super) options: FlowMetadataPrefixOptions,
    pub(super) vectors: Arc<[VectorSpan]>,
    pub(super) objects: Arc<[ObjectSpan]>,
    pub(super) end: usize,
}
/// Borrowed vector whose signed values retain wire order.
#[derive(Clone, Copy, Debug)]
pub struct FlowMetadataVector<'a> {
    pub(super) owner: &'a FlowMetadataPrefix,
    pub(super) ordinal: usize,
}
/// Borrowed object with independent stored and native text projections.
#[derive(Clone, Copy, Debug)]
pub struct FlowMetadataObject<'a> {
    pub(super) owner: &'a FlowMetadataPrefix,
    pub(super) ordinal: usize,
}
/// Borrowed raw string, preserving empty encodings, surrogates and embedded NULs.
#[derive(Clone, Copy, Debug)]
pub struct FlowMetadataText<'a> {
    pub(super) owner: &'a FlowMetadataPrefix,
    pub(super) span: &'a TextSpan,
}
/// Isolated prefix edit with typed checkpoints and complete carrier history.
#[derive(Clone, Debug)]
pub struct FlowMetadataPrefixEdit {
    pub(super) source: FlowMetadataPrefix,
    pub(super) result: FlowMetadataPrefix,
    pub(super) carrier_edit: FlowCarrierEdit,
}
