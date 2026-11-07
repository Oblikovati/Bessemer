use super::{SimulationMaterialCompletion, SimulationMaterialDiagnostic};
use crate::simulation::SimulationPersistVersion;

/// Explicit operation limits; declared entries include counts whose conditional payload is absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationMaterialLimits {
    /// Maximum consumed body bytes, independently of a following carrier suffix.
    pub maximum_bytes: usize,
    /// Every physical scalar/text field.
    pub maximum_fields: usize,
    /// Aggregate declared property, group and point counts.
    pub maximum_entries: usize,
    /// Maximum payload bytes in each persisted text, including its terminator.
    pub maximum_text_bytes: usize,
}

/// The supplied boundary is part of the validation claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationMaterialBoundary {
    /// The complete input must end with this body.
    Standalone,
    /// Only one complete body is consumed; following carrier bytes remain outside the result.
    Prefix,
}

/// Exact grammar checks with explicit EOF scope, independent of native application acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationMaterialCheckMode {
    /// All body fields, budgets and standalone EOF.
    ExactStandalone,
    /// All body fields and budgets through its complete prefix endpoint.
    ExactPrefix,
    /// Complete owned body and exact preservation outside one requested value.
    ExactBodyAndPreservation,
}

/// Completion, complete stored coverage, diagnostics and achieved pattern error together.
#[derive(Debug, Clone, Copy)]
pub struct SimulationMaterialCoverage {
    /// Once-normalized version argument.
    pub version: SimulationPersistVersion,
    /// Limits actually applied to this body.
    pub limits: SimulationMaterialLimits,
    /// Complete mandatory-body status or isolated edit outcome.
    pub completion: SimulationMaterialCompletion,
    /// Explicit carrier boundary checked by this operation.
    pub boundary: SimulationMaterialBoundary,
    /// Exact scope of stored-field validation.
    pub check_mode: SimulationMaterialCheckMode,
    /// Complete consumed body bytes.
    pub checked_bytes: usize,
    /// Every checked physical field.
    pub checked_fields: usize,
    /// Complete declared property records.
    pub property_count: usize,
    /// Complete direct delegated bodies across both groups.
    pub delegated_count: usize,
    /// Actual interleaved point pairs consumed across all tables.
    pub point_pairs: usize,
    /// Aggregate declared entries, including absent positive property-table counts.
    pub declared_entries: usize,
    /// Runtime, physical and geometric exclusions.
    pub diagnostics: [SimulationMaterialDiagnostic; 4],
    /// Achieved requested raw-pattern error, zero.
    pub requested_pattern_error_bits: u32,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct MaterialStatistics {
    pub properties: usize,
    pub delegates: usize,
    pub pairs: usize,
    pub entries: usize,
}
