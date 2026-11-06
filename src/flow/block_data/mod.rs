//! Complete legacy record-406 framing with explicit external leaf-count context.
mod context;
mod decode;
mod edit;
mod entry;
mod fields;
mod selection;
mod types;
mod view;
use super::*;
use crate::archive_codec::ArchiveCursor;
pub use context::*;
pub use fields::*;
pub use types::*;
use types::{BlockSpan, EntrySpan};

impl FlowCarrier {
    /// Reads one selected legacy record with explicit context; e.g. `let blocks = carrier.record_406_blocks(&id, options)?`.
    pub fn record_406_blocks(
        &self,
        id: &FlowRecordId,
        options: FlowBlockDataOptions,
    ) -> Result<FlowLegacyBlockData> {
        decode::read(self, id, options).map_err(|mut error| {
            error.operation = "Flow legacy block data";
            error.entity = format!("record 406: {}", error.entity);
            error
        })
    }
}

fn block_failure(class: FailureClass, cause: String) -> BessemerError {
    BessemerError::new("Flow legacy block data", "record 406", class, cause)
}

fn invalid_block(cause: String) -> BessemerError {
    block_failure(FailureClass::Invalid, cause)
}
