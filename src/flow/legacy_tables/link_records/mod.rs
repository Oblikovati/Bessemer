//! Complete legacy pointer links; runtime reconstruction remains separate.
mod decode;
mod edit;
mod fields;
mod types;
mod view;
use super::*;
pub use fields::*;
pub use types::*;
use types::{EntrySpan, FinalSpan, LinkTailSpan, NumericSpan};

impl FlowCarrier {
    /// Reads one complete legacy link payload; e.g. `let links = carrier.record_402_links(&id, options)?`.
    pub fn record_402_links(
        &self,
        id: &FlowRecordId,
        options: FlowLinkRecordOptions,
    ) -> Result<FlowLinkRecords> {
        decode::read(self, id, options).map_err(|error| table_boundary(error, 402))
    }
}

fn link_invalid(cause: String) -> BessemerError {
    table_failure(402, FailureClass::Invalid, cause)
}
