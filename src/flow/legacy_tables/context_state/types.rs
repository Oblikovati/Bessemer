use super::*;

/// Explicit native member +0x380 count assumption and independent physical resource bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowContextStateOptions {
    /// Required for positive stored object counts in versions two through four; ignored otherwise.
    pub index_count: Option<FlowSimpleLegacyCount>,
    /// Maximum complete raw payload bytes, at least four.
    pub maximum_payload_bytes: usize,
    /// Maximum actual serialized index words; zero admits present-empty and inactive arrays.
    pub maximum_index_words: usize,
}
/// Immutable complete record138, without opaque extensions or inferred native context.
#[derive(Clone, Debug)]
pub struct FlowContextState {
    pub(super) wire: TableWire,
    pub(super) options: FlowContextStateOptions,
    pub(super) object_count: u32,
    pub(super) index_end: usize,
}
/// Exact serialized checks and the applied count assumption, separate from native acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowContextStateCoverage {
    /// Always exact raw framing through EOF.
    pub mode: FlowLegacyTableCheckMode,
    /// Complete stored raw bytes.
    pub payload_bytes: usize,
    /// Stored object count, not the number of index words.
    pub object_count: u32,
    /// Applied external count, including present zero; absent for inactive and version-five records.
    pub applied_index_count: Option<FlowSimpleLegacyCount>,
    /// Actual serialized index words.
    pub index_words: usize,
    /// Signed tail presence in versions two and three.
    pub tail_present: bool,
    /// Zero serialized approximation error.
    pub achieved_error: usize,
}
/// Qualifications accompanying exact raw context-state reads and edits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowContextStateDiagnostic {
    /// Index width was supplied explicitly, without verifying native member +0x380.
    NativeIndexCountAssumed,
    /// Native runtime population was not reconstructed.
    NativeRuntimeNotReproduced,
    /// Engineering interpretation of raw words remains unresolved.
    EngineeringRolesNotResolved,
    /// No acceptance check was performed in SolidWorks.
    NativeApplicationNotChecked,
}
/// Completed isolated edit, with typed source/result checkpoints and full carrier history.
#[derive(Clone, Debug)]
pub struct FlowContextStateEdit {
    pub(super) source: FlowContextState,
    pub(super) result: FlowContextState,
    pub(super) carrier_edit: FlowCarrierEdit,
}
