//! Complete parallel, per-leaf and raw table profiles, with explicit external counts.
mod decode;
mod edit;
mod options;
mod types;
mod view;
use super::*;
pub use options::*;
pub use types::*;
use types::{ParallelSpan, SimpleDeclaredCount};

impl FlowCarrier {
    /// Reads a complete parallel table; e.g. `let table = carrier.record_407_arrays(&id, options)?`.
    pub fn record_407_arrays(
        &self,
        id: &FlowRecordId,
        options: FlowParallelTableOptions,
    ) -> Result<FlowParallelTable> {
        decode::parallel(self, id, options).map_err(|error| table_boundary(error, 407))
    }
    /// Reads explicit per-leaf pairs; e.g. `let pairs = carrier.record_437_leaf_pairs(&id, options)?`.
    pub fn record_437_leaf_pairs(
        &self,
        id: &FlowRecordId,
        options: FlowLeafPairOptions,
    ) -> Result<FlowLeafPairs> {
        decode::leaf_pairs(self, id, options).map_err(|error| table_boundary(error, 437))
    }
    /// Reads raw five-word rows without inventing inactive rows; e.g. `let table = carrier.record_411_rows(&id, options)?`.
    pub fn record_411_rows(
        &self,
        id: &FlowRecordId,
        options: FlowFiveWordTableOptions,
    ) -> Result<FlowFiveWordTable> {
        decode::five_words(self, id, options).map_err(|error| table_boundary(error, 411))
    }
    /// Reads a complete raw byte table; e.g. `let table = carrier.record_413_bytes(&id, options)?`.
    pub fn record_413_bytes(
        &self,
        id: &FlowRecordId,
        options: FlowRawByteTableOptions,
    ) -> Result<FlowRawByteTable> {
        decode::bytes(self, id, options).map_err(|error| table_boundary(error, 413))
    }
}

fn simple_invalid(identifier: u32, cause: String) -> BessemerError {
    table_failure(identifier, FailureClass::Invalid, cause)
}
