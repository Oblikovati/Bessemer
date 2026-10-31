use super::{
    SimulationShellPropertyCheckMode, SimulationShellPropertyContext,
    SimulationShellPropertyDiagnostic, SimulationShellPropertyLimits,
};
use crate::simulation::SimulationPersistVersion;

/// Complete mandatory-body reads and isolated edits have distinct completion states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationShellPropertyCompletion {
    /// Every field of the selected complete Property body was checked.
    CompletedBody,
    /// The exact requested stored pattern already matched.
    Unchanged,
    /// One complete isolated stored value changed.
    Changed,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct ShellPropertyStatistics {
    pub base_end: usize,
    pub subtype: i32,
    pub layers: usize,
    pub materials: usize,
    pub descriptors: usize,
}

/// Exact complete framing coverage with explicit runtime context and semantic exclusions.
#[derive(Debug, Clone)]
pub struct SimulationShellPropertyCoverage {
    /// Once-normalized outer version.
    pub version: SimulationPersistVersion,
    /// Caller budgets actually applied.
    pub limits: SimulationShellPropertyLimits,
    /// Explicit concrete family and supplied runtime layer lookup results.
    pub context: SimulationShellPropertyContext,
    /// Aggregate declared references, pairs, tables, layers and Material entries.
    pub declared_entries: usize,
    /// Complete mandatory-body or isolated-edit status.
    pub completion: SimulationShellPropertyCompletion,
    /// Exact stored framing and any requested preservation checks.
    pub check_mode: SimulationShellPropertyCheckMode,
    /// Every checked input byte.
    pub checked_bytes: usize,
    /// Every primitive outer field, including scoped inline Material fields.
    pub checked_fields: usize,
    /// One mandatory complete Property body.
    pub body_count: usize,
    /// Exact end of the complete shared base.
    pub base_end: usize,
    /// Raw saved subtype, without inferred physical meaning.
    pub subtype: i32,
    /// Every declared composite layer.
    pub layers: usize,
    /// Every complete inline Material body.
    pub materials: usize,
    /// Every conditional bit-1/bit-2 descriptor body.
    pub descriptors: usize,
    /// Every complete embedded reference root.
    pub embedded_archives: usize,
    /// Every physical field within embedded roots.
    pub embedded_fields: usize,
    /// Runtime, native application and geometry exclusions.
    pub diagnostics: [SimulationShellPropertyDiagnostic; 4],
    /// Achieved requested byte-pattern error, zero.
    pub requested_pattern_error_bits: u32,
}
