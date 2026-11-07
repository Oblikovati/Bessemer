use super::{
    SimulationMeshControlHeaderCheckMode, SimulationMeshControlHeaderCompletion,
    SimulationMeshControlHeaderDiagnostic, SimulationMeshControlHeaderLimits,
};
use crate::simulation::SimulationPersistVersion;

/// Complete exact validation, diagnostics and achieved pattern error together.
#[derive(Debug, Clone, Copy)]
pub struct SimulationMeshControlHeaderCoverage {
    /// Once-normalized explicit historical context.
    pub version: SimulationPersistVersion,
    /// Budgets actually applied to the complete body.
    pub limits: SimulationMeshControlHeaderLimits,
    /// Complete mandatory header or isolated edit outcome.
    pub completion: SimulationMeshControlHeaderCompletion,
    /// Exact stored-profile and preservation scope.
    pub check_mode: SimulationMeshControlHeaderCheckMode,
    /// All input bytes checked through EOF.
    pub checked_bytes: usize,
    /// Every physical field checked.
    pub checked_fields: usize,
    /// One mandatory complete header.
    pub body_count: usize,
    /// Explicit historical-reader or current-writer wire grammar.
    pub wire_profile: super::super::SimulationMeshControlHeaderProfile,
    /// Native, runtime, physical and geometry exclusions.
    pub diagnostics: [SimulationMeshControlHeaderDiagnostic; 4],
    /// Achieved exact requested wire-pattern error, zero.
    pub requested_pattern_error_bits: u32,
}
