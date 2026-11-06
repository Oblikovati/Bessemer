use super::*;
/// Exact backend framing stop, independently of complete outer-record acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMediumBackendStop {
    /// Both tables are empty; following outer-record bytes remain opaque.
    EmptyTablesEnd,
    /// First-table tag and conditional key are known; its body remains opaque.
    FirstTableObjectBody,
    /// Second-table tag is known; its body remains opaque.
    SecondTableObjectBody,
}
/// Exact known-backend scope and explicit full-record exclusion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowMediumBackendCoverage {
    /// Backend input bytes, excluding the preceding metadata prefix.
    pub backend_bytes: usize,
    /// Exact decoded backend bytes.
    pub known_backend_bytes: usize,
    /// Remaining body or outer-record bytes.
    pub unknown_bytes: usize,
    /// Both table headers are complete and empty.
    pub backend_framing_complete: bool,
    /// Full outer record has not been checked.
    pub full_record_schema: bool,
    /// Exact framing error, without approximation.
    pub achieved_error: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct BackendTableSpan {
    pub start: usize,
    pub end: usize,
    pub count: i32,
    pub kind: Option<i32>,
    pub key: bool,
}
/// Immutable partial backend view retaining the complete metadata/carrier checkpoint.
#[derive(Clone, Debug)]
pub struct FlowMediumBackendPrefix {
    pub(super) prefix: FlowMetadataPrefix,
    pub(super) tables: Arc<[BackendTableSpan]>,
    pub(super) end: usize,
}
/// Borrowed table header and its first populated kind/key, when present.
#[derive(Clone, Copy, Debug)]
pub struct FlowMediumBackendTable<'a> {
    pub(super) owner: &'a FlowMediumBackendPrefix,
    pub(super) ordinal: usize,
}
/// Isolated existing-backend-field edit with typed checkpoints and full physical history.
#[derive(Clone, Debug)]
pub struct FlowMediumBackendEdit {
    pub(super) source: FlowMediumBackendPrefix,
    pub(super) result: FlowMediumBackendPrefix,
    pub(super) prefix_edit: FlowMetadataPrefixEdit,
}
