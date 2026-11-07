//! Complete raw record148 lengths precede all bodies; stored empty vectors retain ordinals.
mod edit;
mod read;
mod view;
use super::*;
use std::sync::Arc;

/// Independent complete-payload, actual-vector and total raw-value bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowWideVectorOptions {
    /// Complete raw payload bytes, at least eight.
    pub maximum_payload_bytes: usize,
    /// Actual length-table entries; zero admits no stored vectors.
    pub maximum_vectors: usize,
    /// Total values across every vector; zero admits present-empty vectors.
    pub maximum_values: usize,
}
#[derive(Clone, Copy, Debug)]
struct VectorSpan {
    start: usize,
    end: usize,
}
/// Immutable complete raw wide-vector table with no per-value allocation.
#[derive(Clone, Debug)]
pub struct FlowWideVectors {
    wire: TableWire,
    options: FlowWideVectorOptions,
    spans: Arc<[VectorSpan]>,
    values: usize,
}
/// Complete exact framing and actual vector/value coverage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowWideVectorCoverage {
    /// Complete raw framing check mode.
    pub mode: FlowLegacyTableCheckMode,
    /// Complete validated payload extent.
    pub payload_bytes: usize,
    /// Actual physical length-table entries.
    pub stored_vectors: usize,
    /// Total raw patterns across all vectors.
    pub stored_values: usize,
    /// Present-empty vectors, distinct from an absent vector table.
    pub empty_vectors: usize,
    /// Zero for exact serialized checks; no geometric accuracy is asserted.
    pub achieved_error: usize,
}
impl FlowCarrier {
    /// Reads complete raw wide vectors; e.g. `let vectors = carrier.record_148_wide_vectors(&id, options)?`.
    pub fn record_148_wide_vectors(
        &self,
        id: &FlowRecordId,
        options: FlowWideVectorOptions,
    ) -> Result<FlowWideVectors> {
        read::read(self, id, options).map_err(|error| table_boundary(error, 148))
    }
}
