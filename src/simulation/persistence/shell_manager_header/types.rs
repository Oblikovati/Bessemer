use super::{SimulationShellManagerCompletion, SimulationShellManagerDiagnostic};
use crate::simulation::SimulationPersistVersion;

/// Exact direct-header budgets without unrelated embedded-archive options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationShellManagerLimits {
    /// Entire input and reconstructed output byte cap.
    pub maximum_bytes: usize,
    /// Every complete signed header word.
    pub maximum_fields: usize,
    /// Declared entries; the direct header requires zero.
    pub maximum_entries: usize,
    /// Text payload bytes; the direct header requires zero.
    pub maximum_text_bytes: usize,
}

/// Exact stored-header scope, independent of native/runtime/geometric acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationShellManagerCheckMode {
    /// Every selected field, resource limit and standalone EOF.
    ExactStandalone,
    /// Complete result plus exact preservation outside one requested value.
    ExactBodyAndPreservation,
}

/// Complete exact validation, diagnostics and achieved pattern error together.
#[derive(Debug, Clone, Copy)]
pub struct SimulationShellManagerCoverage {
    /// Once-normalized explicit historical context.
    pub version: SimulationPersistVersion,
    /// Budgets actually applied to the complete body.
    pub limits: SimulationShellManagerLimits,
    /// Complete mandatory header or isolated edit outcome.
    pub completion: SimulationShellManagerCompletion,
    /// Exact stored-profile and preservation scope.
    pub check_mode: SimulationShellManagerCheckMode,
    /// All input bytes checked through EOF.
    pub checked_bytes: usize,
    /// Every physical field checked.
    pub checked_fields: usize,
    /// One mandatory complete header.
    pub body_count: usize,
    /// Explicit historical-reader or current-writer wire grammar.
    pub wire_profile: super::super::SimulationShellWireProfile,
    /// Native, runtime, physical and geometry exclusions.
    pub diagnostics: [SimulationShellManagerDiagnostic; 4],
    /// Achieved exact requested wire-pattern error, zero.
    pub requested_pattern_error_bits: u32,
}
