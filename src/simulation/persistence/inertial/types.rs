use super::{
    SimulationInertialCheckMode, SimulationInertialCompletion, SimulationInertialDiagnostic,
    SimulationInertialLimits,
};
use crate::simulation::SimulationPersistVersion;

/// Explicit binary load family; storage ancestry and runtime version selection remain caller context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationInertialKind {
    /// The persisted Gravity collection.
    Gravity,
    /// The persisted CentriFugal collection.
    Centrifugal,
    /// The persisted BaseExcitation collection.
    BaseExcitation,
}

/// Exact stored-profile read coverage with semantic exclusions and achieved pattern error.
#[derive(Debug, Clone, Copy)]
pub struct SimulationInertialCoverage {
    /// Explicit selected outer family.
    pub kind: SimulationInertialKind,
    /// Once-normalized outer version.
    pub version: SimulationPersistVersion,
    /// Caller budgets actually applied.
    pub limits: SimulationInertialLimits,
    /// Completed empty/populated status, independent of errors.
    pub completion: SimulationInertialCompletion,
    /// Exact full stored-profile check mode.
    pub check_mode: SimulationInertialCheckMode,
    /// Every checked input byte.
    pub checked_bytes: usize,
    /// Every complete outer field.
    pub checked_fields: usize,
    /// Every complete declared record.
    pub record_count: usize,
    /// Every checked embedded root.
    pub embedded_archives: usize,
    /// Every physical field inside embedded roots.
    pub embedded_fields: usize,
    /// Native, runtime and geometry exclusions.
    pub diagnostics: [SimulationInertialDiagnostic; 4],
    /// Achieved requested byte-pattern error, zero.
    pub requested_pattern_error_bits: u32,
}
