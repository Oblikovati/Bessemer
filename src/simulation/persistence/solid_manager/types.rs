use super::{SimulationSolidManagerCompletion, SimulationSolidManagerDiagnostic};
use crate::simulation::SimulationPersistVersion;

/// Exact direct-header budgets without unrelated embedded-archive options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationSolidManagerLimits {
    /// Entire input and reconstructed output byte cap.
    pub maximum_bytes: usize,
    /// Every signed count/word and complete counted text frame.
    pub maximum_fields: usize,
    /// Aggregate entries in both historical text lists.
    pub maximum_entries: usize,
    /// Maximum payload bytes per text, including its retained terminator.
    pub maximum_text_bytes: usize,
}

/// Exact stored-header scope, independent of native/runtime/geometric acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationSolidManagerCheckMode {
    /// Every selected field, resource limit and standalone EOF.
    ExactStandalone,
    /// Complete result plus exact preservation outside one requested value.
    ExactBodyAndPreservation,
}

/// Complete exact validation, diagnostics and achieved pattern error together.
#[derive(Debug, Clone, Copy)]
pub struct SimulationSolidManagerCoverage {
    /// Once-normalized explicit historical context.
    pub version: SimulationPersistVersion,
    /// Budgets actually applied to the complete body.
    pub limits: SimulationSolidManagerLimits,
    /// Complete mandatory header or isolated edit outcome.
    pub completion: SimulationSolidManagerCompletion,
    /// Exact stored-profile and preservation scope.
    pub check_mode: SimulationSolidManagerCheckMode,
    /// All input bytes checked through EOF.
    pub checked_bytes: usize,
    /// Every physical field checked.
    pub checked_fields: usize,
    /// One mandatory complete header, including empty text lists.
    pub body_count: usize,
    /// All declared entries across both lists, preserving duplicates.
    pub list_entries: usize,
    /// Native, runtime, physical and geometry exclusions.
    pub diagnostics: [SimulationSolidManagerDiagnostic; 4],
    /// Achieved exact requested wire-pattern error, zero.
    pub requested_pattern_error_bits: u32,
}
