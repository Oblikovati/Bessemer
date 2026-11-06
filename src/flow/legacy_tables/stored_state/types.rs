use super::*;

/// Complete raw payload limit; the record itself occupies sixteen or twenty bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowStoredStateOptions {
    /// Maximum complete raw bytes; values below four are invalid.
    pub maximum_payload_bytes: usize,
}
/// Immutable complete record432, with its original carrier checkpoint.
#[derive(Clone, Debug)]
pub struct FlowStoredState {
    pub(super) wire: TableWire,
    pub(super) options: FlowStoredStateOptions,
}
/// Stored members retain their native offset names without assigning engineering meaning.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowStoredStateMember {
    /// First signed member, at native offset 0x5e0.
    Member5E0,
    /// Second signed member, at native offset 0x5d8.
    Member5D8,
    /// Third signed member, at native offset 0x5dc.
    Member5DC,
    /// Additional physically stored signed member, present only in versions one and two.
    Discarded,
}
/// Exact serialized validation coverage, separate from native runtime acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowStoredStateCoverage {
    /// Always exact raw framing through EOF.
    pub mode: FlowLegacyTableCheckMode,
    /// Complete version-dependent raw payload length.
    pub payload_bytes: usize,
    /// Three members, plus the discarded member when present; excludes version.
    pub stored_members: usize,
    /// True for versions zero and one; recomputation itself was not performed.
    pub native_recomputation_required: bool,
    /// Zero serialized approximation error.
    pub achieved_error: usize,
}
/// Exclusions accompanying every successfully framed state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowStoredStateDiagnostic {
    /// Stored members have not been replaced by native recomputation.
    NativeRecomputationNotPerformed,
    /// Native runtime population was not reconstructed.
    NativeRuntimeNotReproduced,
    /// Engineering interpretation of signed members remains unresolved.
    EngineeringRolesNotResolved,
    /// No acceptance check was performed in SolidWorks.
    NativeApplicationNotChecked,
}
/// Isolated completed edit with immutable typed checkpoints and physical carrier history.
#[derive(Clone, Debug)]
pub struct FlowStoredStateEdit {
    pub(super) source: FlowStoredState,
    pub(super) result: FlowStoredState,
    pub(super) carrier_edit: FlowCarrierEdit,
}
