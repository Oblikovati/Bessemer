use super::*;
use crate::simulation::SimulationPersistVersion;

/// Exact physical record boundaries, independent of the native in-memory subobject adjustment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationMeshControlRecordSpan {
    /// Physical record ordinal within the counted stream.
    pub index: usize,
    /// First stored byte of the shared base.
    pub start: usize,
    /// First stored byte of the derived MeshControl continuation.
    pub base_end: usize,
    /// First byte after the complete record.
    pub end: usize,
}

/// Exact complete wire checks, completion, exclusions and achieved pattern error together.
#[derive(Debug, Clone, Copy)]
pub struct SimulationMeshControlCoverage {
    /// Once-normalized declared outer version.
    pub version: SimulationPersistVersion,
    /// Caller budgets actually applied.
    pub limits: SimulationMeshControlLimits,
    /// Completed empty/record collection or isolated edit status.
    pub completion: SimulationMeshControlCompletion,
    /// Complete stored-profile and optional preservation checks.
    pub check_mode: SimulationMeshControlCheckMode,
    /// All checked outer bytes, through mandatory EOF.
    pub checked_bytes: usize,
    /// Every complete outer field occurrence.
    pub checked_fields: usize,
    /// Every complete declared direct record.
    pub record_count: usize,
    /// Aggregate outer record, reference, optional-pair and curve declarations.
    pub declared_entries: usize,
    /// Every independently framed embedded root.
    pub embedded_archives: usize,
    /// Every physical embedded field occurrence.
    pub embedded_fields: usize,
    /// Native, runtime, physical and geometry exclusions.
    pub diagnostics: [SimulationMeshControlDiagnostic; 4],
    /// Achieved exact requested wire-pattern error, zero.
    pub requested_pattern_error_bits: u32,
}
