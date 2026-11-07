//! Strict writer-derived control payloads do not adopt or reconcile an index.
mod edit;
mod read;
mod transport;
mod types;
mod view;
use super::*;
use std::sync::Arc;
pub use types::*;
const SAVED: u32 = 0xffffff9c;
const INDEX: u32 = 0xffffff9b;
const OPERATION: &str = "Flow strict control writer profiles";
fn invalid(identifier: u32, cause: String) -> BessemerError {
    table_failure(identifier, FailureClass::Invalid, cause)
}
impl FlowCarrier {
    /// Reads a complete strict zero-marker saved-span writer profile; e.g. `let saved = carrier.saved_span_writer(&id, options)?`.
    pub fn saved_span_writer(
        &self,
        id: &FlowRecordId,
        options: FlowControlOptions,
    ) -> Result<FlowSavedSpans> {
        read::saved(self, id, options).map_err(|error| table_boundary(error, SAVED))
    }
    /// Reads a complete strict terminal-index writer profile under explicit length context; e.g. `let index = carrier.terminal_index_writer(&id, FlowControlLengthWidth::EightBytes, options)?`.
    pub fn terminal_index_writer(
        &self,
        id: &FlowRecordId,
        width: FlowControlLengthWidth,
        options: FlowControlOptions,
    ) -> Result<FlowTerminalIndex> {
        read::index(self, id, width, options).map_err(|error| table_boundary(error, INDEX))
    }
}
