//! Explicit-context legacy record-400 wire forests; flat storage avoids recursive parsing/drop.
mod decode;
mod edit;
mod forest;
mod types;
mod view;
use super::*;
use crate::archive_codec::ArchiveCursor;
pub use types::*;

impl FlowCarrier {
    /// Reads one revision-bound record under explicit owner context; e.g. `let trees = carrier.record_400_trees(&id, options)?`.
    pub fn record_400_trees(
        &self,
        id: &FlowRecordId,
        options: FlowTreeOptions,
    ) -> Result<FlowBlockTrees> {
        decode::read(self, id, options).map_err(boundary_error)
    }
}

fn tree_failure(class: FailureClass, cause: String) -> BessemerError {
    BessemerError::new("Flow block trees", "record 400", class, cause)
}

fn boundary_error(mut error: BessemerError) -> BessemerError {
    if error.operation == "Flow block trees" {
        return error;
    }
    error.operation = "Flow block trees";
    error.entity = format!("record 400: {}", error.entity);
    error
}
