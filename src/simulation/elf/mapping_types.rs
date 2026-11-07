use super::{
    SimulationElfDiagnostic, SimulationElfIndexRevision, SimulationElfRecordId,
    SimulationElfScalarRecord, SimulationElfSelector,
};
use super::{SimulationElfDirectoryCoverage, SimulationElfProjectionCheckMode};
use std::sync::Arc;

/// One assigned destination with its exact physical source parent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationElfMappedRecord {
    /// Original physical source record identity.
    pub source: SimulationElfRecordId,
    /// Independent complete record value, retaining the original identifier.
    pub record: SimulationElfScalarRecord,
}

/// One exact copy in native visit order, including overwritten destination history.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationElfMappingWrite {
    /// Complete source parent, independent of repeated numeric identifiers.
    pub source: SimulationElfRecordId,
    /// Original signed positive identifier, never rewritten to the destination index.
    pub identifier: i32,
    /// Normalized zero-based destination slot.
    pub destination_index: usize,
    /// Previous copied parent overwritten in this operation, if present.
    pub overwritten_source: Option<SimulationElfRecordId>,
}

/// Successful copy projection distinguished from a completed empty result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationElfMappingCompletion {
    /// No source record was copied; unassigned destination slots remain explicit.
    CompletedEmpty,
    /// All selected positive mappings were applied in visit order.
    CompletedWrites,
}

/// Complete exact selected-copy coverage with the source traversal scope retained.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationElfMappingCoverage {
    /// Explicit exact selected-copy check mode.
    pub mode: SimulationElfProjectionCheckMode,
    /// Source records checked, including stop/skip and unvisited-source exclusions.
    pub selected_source: SimulationElfDirectoryCoverage,
    /// Every full-record copy, including later-overwritten copies.
    pub copied_records: usize,
    /// Exact record bytes copied, including overwritten copies.
    pub copied_bytes: usize,
    /// Complete explicit destination extent.
    pub destination_slots: usize,
    /// Final slots with an established copied source parent.
    pub assigned_slots: usize,
}

/// Complete explicit selected-copy result, without native runtime or geometry acceptance.
#[derive(Debug, Clone)]
pub struct SimulationElfMappingResult {
    /// Original complete source context identity.
    pub parent_revision: SimulationElfIndexRevision,
    /// Selected normalized directory entry.
    pub selector: SimulationElfSelector,
    /// Final independent copied values; none means unassigned, not invented prior memory.
    pub slots: Arc<[Option<SimulationElfMappedRecord>]>,
    /// Every copy and overwrite in deterministic native visit order.
    pub writes: Arc<[SimulationElfMappingWrite]>,
    /// Every still-unassigned destination in ascending slot order.
    pub unassigned_indices: Arc<[usize]>,
    /// Completed empty versus completed copy projection.
    pub completion: SimulationElfMappingCompletion,
    /// Complete selected source records examined, including stop and skip records.
    pub visited_records: usize,
    /// Complete selected traversal and copy checks with explicit scope.
    pub coverage: SimulationElfMappingCoverage,
    /// Achieved copied-pattern error, zero; no physical error was established.
    pub copied_pattern_error_bits: u32,
    /// Required unchecked responsibilities.
    pub diagnostics: [SimulationElfDiagnostic; 4],
}
