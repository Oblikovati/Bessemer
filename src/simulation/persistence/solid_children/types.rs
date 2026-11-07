use super::{
    SimulationSolidChildCompletion, SimulationSolidChildDiagnostic, SimulationSolidChildEvidence,
    SimulationSolidChildKind,
};
use crate::simulation::SimulationPersistVersion;

/// Exact child-header budgets without unrelated embedded-archive options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationSolidChildLimits {
    /// Entire input and reconstructed output byte cap.
    pub maximum_bytes: usize,
    /// Every signed count/word and complete counted text frame.
    pub maximum_fields: usize,
    /// Aggregate entries in the optional folder text list.
    pub maximum_entries: usize,
    /// Maximum payload bytes per text, including its retained terminator.
    pub maximum_text_bytes: usize,
}

/// Exact stored-header scope, independent of native/runtime/geometric acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationSolidChildCheckMode {
    /// Every selected field, resource limit and standalone EOF.
    ExactStandalone,
    /// Complete result plus exact preservation outside one requested value.
    ExactBodyAndPreservation,
}

/// Complete exact validation, diagnostics and achieved pattern error together.
#[derive(Debug, Clone, Copy)]
pub struct SimulationSolidChildCoverage {
    /// Explicit component or folder body profile.
    pub kind: SimulationSolidChildKind,
    /// Supporting grammar evidence; this does not identify caller payload provenance.
    pub evidence: SimulationSolidChildEvidence,
    /// Recorded strict independent behavior profile, initially one.
    pub behavior_profile: u32,
    /// Once-normalized explicit historical context.
    pub version: SimulationPersistVersion,
    /// Budgets actually applied to the complete body.
    pub limits: SimulationSolidChildLimits,
    /// Complete mandatory header or isolated edit outcome.
    pub completion: SimulationSolidChildCompletion,
    /// Exact stored-profile and preservation scope.
    pub check_mode: SimulationSolidChildCheckMode,
    /// All input bytes checked through EOF.
    pub checked_bytes: usize,
    /// Every physical field checked.
    pub checked_fields: usize,
    /// One mandatory complete header, including empty text lists.
    pub body_count: usize,
    /// All declared entries across the folder list, preserving duplicates.
    pub list_entries: usize,
    /// Native, runtime, physical and geometry exclusions.
    pub diagnostics: [SimulationSolidChildDiagnostic; 4],
    /// Achieved exact requested wire-pattern error, zero.
    pub requested_pattern_error_bits: u32,
}
