/// Byte provenance within a normalization result's retained input snapshots.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxiliaryCopySource {
    /// Complete stored row supplied to normalization.
    StoredRow,
    /// Explicit prior workspace supplied for legacy conversion.
    InitialWorkspace,
    /// Supplied u64 result bytes captured by composed pointer binding.
    SuppliedAllocationResult,
}

/// One exact nonoverlapping copied span, including the real parent of retained slot bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationAuxiliarySpanCopy {
    /// Retained input snapshot supplying the bytes.
    pub source: SimulationAuxiliaryCopySource,
    /// Byte offset within that source.
    pub source_offset: usize,
    /// Byte offset within the normalized result.
    pub destination_offset: usize,
    /// Exact copied extent.
    pub length: usize,
}

/// Nonportable/runtime responsibilities outside these byte projections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxiliaryRuntimeDiagnostic {
    /// Numeric stored/result bits are never dereferenced pointers.
    StoredPointerNeverDereferenced,
    /// Allocation requests/results were projected, not executed or inferred.
    NativeAllocationNotExecuted,
    /// Delegated vector reads were not executed or assigned a scalar domain.
    NativeReadNotExecuted,
    /// No native producer or file write executed.
    NativeFileWriteNotExecuted,
}

/// Exact normalization scope and explicit partial interpretation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationAuxiliaryNormalizationCoverage {
    /// Exact span/history scope.
    pub mode: SimulationAuxiliaryNormalizationCheckMode,
    /// All 288 destination bytes have explicit input parents.
    pub destination_bytes: usize,
    /// Prior workspace bytes retained: eight for legacy, zero for modern.
    pub prior_workspace_bytes: usize,
    /// Stored bytes not copied, retained in original source provenance.
    pub uncopied_stored_bytes: usize,
    /// Always true: this result is not a file publication.
    pub memory_only: bool,
}

/// Exact memory conversion scope, without native runtime acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxiliaryNormalizationCheckMode {
    /// All documented stored spans and supplied prior slot bytes.
    ExactStoredSpansAndSuppliedSlot,
}

pub(super) fn runtime_diagnostics() -> [SimulationAuxiliaryRuntimeDiagnostic; 4] {
    [
        SimulationAuxiliaryRuntimeDiagnostic::StoredPointerNeverDereferenced,
        SimulationAuxiliaryRuntimeDiagnostic::NativeAllocationNotExecuted,
        SimulationAuxiliaryRuntimeDiagnostic::NativeReadNotExecuted,
        SimulationAuxiliaryRuntimeDiagnostic::NativeFileWriteNotExecuted,
    ]
}
