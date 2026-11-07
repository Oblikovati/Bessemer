use super::{SimulationGenArchive, SimulationGenRevision, SimulationNodeId};
use crate::simulation::SimulationDiagnostic;

/// Exact complete stored framing coverage, with semantic validation excluded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationGenCoverage {
    /// All stored input bytes, with exact EOF established.
    pub image_bytes: usize,
    /// Every 52-byte node record.
    pub nodes: u32,
    /// Every 140-byte element record, whether or not accessor views are valid.
    pub elements: u32,
    /// Every auxiliary record selected by the documented trial rule.
    pub auxiliary_records: u32,
    /// Present vectors, including present-empty vectors.
    pub present_vectors: usize,
}

/// Isolated coordinate replacement with exact bit fidelity and identity derivation.
#[derive(Debug, Clone)]
pub struct SimulationGenEdit {
    /// Complete immutable result.
    pub archive: SimulationGenArchive,
    /// Whether any coordinate bit changed.
    pub completion: crate::display_lists::DisplayEditCompletion,
    /// Original complete GEN identity.
    pub parent_revision: SimulationGenRevision,
    /// Original node identity.
    pub parent: SimulationNodeId,
    /// Derived node identity in the resulting complete image.
    pub result: SimulationNodeId,
    /// Complete stored framing coverage.
    pub coverage: SimulationGenCoverage,
    /// Exact coordinate-pattern error, zero; no physical-distance error was established.
    pub coordinate_pattern_error_bits: u32,
    /// Required unchecked semantic responsibilities.
    pub diagnostics: [SimulationDiagnostic; 3],
}

impl SimulationGenArchive {
    /// Quantify complete stored framing, e.g. `image.coverage().nodes`.
    pub fn coverage(&self) -> SimulationGenCoverage {
        SimulationGenCoverage {
            image_bytes: self.bytes.len(),
            nodes: self.layout.counts[3],
            elements: self.layout.counts[4],
            auxiliary_records: self.layout.counts[8],
            present_vectors: self
                .layout
                .vectors
                .iter()
                .filter(|span| span.is_some())
                .count(),
        }
    }
}
