use super::{
    SimulationPostGraphCheckMode, SimulationPostGraphDiagnostic, SimulationPostGraphLimits,
};

/// Complete mandatory body reads remain distinct from isolated edit outcomes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPostGraphCompletion {
    /// Every field of one mandatory complete body was checked.
    CompletedBody,
    /// The requested exact wire pattern already matched.
    Unchanged,
    /// One isolated complete value changed.
    Changed,
}

/// Exact stored-profile read coverage with semantic exclusions and achieved pattern error.
#[derive(Debug, Clone, Copy)]
pub struct SimulationPostGraphCoverage {
    /// Explicit normalized graph reader version.
    pub version: crate::simulation::SimulationPersistVersion,
    /// Caller budgets actually applied.
    pub limits: SimulationPostGraphLimits,
    /// Complete mandatory-body or isolated-edit status, independent of errors.
    pub completion: SimulationPostGraphCompletion,
    /// Exact full stored-profile check mode.
    pub check_mode: SimulationPostGraphCheckMode,
    /// Every checked input byte.
    pub checked_bytes: usize,
    /// Every complete outer field.
    pub checked_fields: usize,
    /// One mandatory complete body.
    pub body_count: usize,
    /// Total declared UTF-16 characters across all text fields.
    pub declared_entries: usize,
    /// Every checked embedded root.
    pub embedded_archives: usize,
    /// Every physical field inside embedded roots.
    pub embedded_fields: usize,
    /// Native, runtime and geometry exclusions.
    pub diagnostics: [SimulationPostGraphDiagnostic; 4],
    /// Achieved requested byte-pattern error, zero.
    pub requested_pattern_error_bits: u32,
}
