use super::{
    SimulationBeamGroupCheckMode, SimulationBeamGroupCompletion, SimulationBeamGroupDiagnostic,
    SimulationBeamGroupKind, SimulationBeamGroupLimits,
};
use crate::simulation::SimulationPersistVersion;

/// Exact stored-profile read coverage with semantic exclusions and achieved pattern error.
#[derive(Debug, Clone, Copy)]
pub struct SimulationBeamGroupCoverage {
    /// Explicit separate body kind and normalized expected child-count context.
    pub kind: SimulationBeamGroupKind,
    /// Once-normalized outer version.
    pub version: SimulationPersistVersion,
    /// Caller budgets actually applied.
    pub limits: SimulationBeamGroupLimits,
    /// Complete mandatory-body or isolated-edit status, independent of errors.
    pub completion: SimulationBeamGroupCompletion,
    /// Exact full stored-profile check mode.
    pub check_mode: SimulationBeamGroupCheckMode,
    /// Every checked input byte.
    pub checked_bytes: usize,
    /// Every complete outer field.
    pub checked_fields: usize,
    /// One mandatory complete body.
    pub body_count: usize,
    /// Every checked embedded root.
    pub embedded_archives: usize,
    /// Every physical field inside embedded roots.
    pub embedded_fields: usize,
    /// Native, runtime and geometry exclusions.
    pub diagnostics: [SimulationBeamGroupDiagnostic; 4],
    /// Achieved requested byte-pattern error, zero.
    pub requested_pattern_error_bits: u32,
}
