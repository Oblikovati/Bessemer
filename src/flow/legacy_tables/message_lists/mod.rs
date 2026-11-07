//! Complete record301 messages; legacy byte transfers never acquire a text interpretation.
mod decode;
mod edit;
mod scan;
mod schema;
mod types;
mod view;
use super::*;
pub use schema::*;
pub use types::*;
use types::{MessageItemSpan, MessageListSpan, MessageTextSpan};

impl FlowCarrier {
    /// Reads complete message lists; e.g. `let messages = carrier.record_301_message_lists(&id, options)?`.
    pub fn record_301_message_lists(
        &self,
        id: &FlowRecordId,
        options: FlowMessageListOptions,
    ) -> Result<FlowMessageLists> {
        decode::read(self, id, options).map_err(|error| table_boundary(error, 301))
    }
}
fn message_invalid(cause: String) -> BessemerError {
    table_failure(301, FailureClass::Invalid, cause)
}
