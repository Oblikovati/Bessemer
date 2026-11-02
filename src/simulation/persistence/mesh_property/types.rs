use super::*;
use crate::simulation::SimulationPersistVersion;

/// Exact physical record boundaries, for direct records, including both nested extensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationMeshPropertyRecordSpan {
    /// Physical record ordinal within the counted stream.
    pub index: usize,
    /// First stored byte of the complete direct record.
    pub start: usize,
    /// First byte after the complete record.
    pub end: usize,
}

/// Exact complete wire checks, completion, exclusions and achieved pattern error together.
#[derive(Debug, Clone, Copy)]
pub struct SimulationMeshPropertyCoverage {
    /// Once-normalized declared outer version.
    pub version: SimulationPersistVersion,
    /// Caller budgets actually applied.
    pub limits: SimulationMeshPropertyLimits,
    /// Complete mandatory body or isolated edit status.
    pub completion: SimulationMeshPropertyCompletion,
    /// Complete stored-profile and optional preservation checks.
    pub check_mode: SimulationMeshPropertyCheckMode,
    /// All checked outer bytes, through mandatory EOF.
    pub checked_bytes: usize,
    /// Every complete outer field occurrence.
    pub checked_fields: usize,
    /// Every complete declared direct record.
    pub record_count: usize,
    /// Every complete body in either direct-record nested extension.
    pub nested_record_count: usize,
    /// One complete mandatory Mesh Property body.
    pub body_count: usize,
    /// Aggregate explicitly declared text, reference, record and array entries.
    pub declared_entries: usize,
    /// Every independently framed embedded root.
    pub embedded_archives: usize,
    /// Every physical embedded field occurrence.
    pub embedded_fields: usize,
    /// Native, runtime, physical and geometry exclusions.
    pub diagnostics: [SimulationMeshPropertyDiagnostic; 4],
    /// Achieved exact requested wire-pattern error, zero.
    pub requested_pattern_error_bits: u32,
}
