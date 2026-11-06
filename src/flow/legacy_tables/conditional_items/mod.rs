//! Complete conditional item grammar shared by records 126, 315 and 128.
mod decode;
mod edit;
mod fields;
mod items;
mod schema;
mod transport;
mod types;
mod view;
use super::*;
pub use schema::*;
pub use types::*;
use types::{ConditionalFieldSpan, ConditionalScopeSpan};

impl FlowCarrier {
    /// Reads the complete raw record-126 grammar; e.g. `let items = carrier.record_126_items(&id, options)?`.
    pub fn record_126_items(
        &self,
        id: &FlowRecordId,
        options: FlowConditionalItemOptions,
    ) -> Result<FlowConditionalItems> {
        decode::read(self, id, FlowConditionalFormat::Record126, options)
            .map_err(|error| table_boundary(error, 126))
    }
    /// Reads the complete compressed record-315 grammar; e.g. `let items = carrier.record_315_items(&id, options)?`.
    pub fn record_315_items(
        &self,
        id: &FlowRecordId,
        options: FlowConditionalItemOptions,
    ) -> Result<FlowConditionalItems> {
        decode::read(self, id, FlowConditionalFormat::Record315, options)
            .map_err(|error| table_boundary(error, 315))
    }
    /// Reads a raw version and complete compressed record-128 body; e.g. `let items = carrier.record_128_items(&id, options)?`.
    pub fn record_128_items(
        &self,
        id: &FlowRecordId,
        options: FlowConditionalItemOptions,
    ) -> Result<FlowConditionalItems> {
        decode::read(self, id, FlowConditionalFormat::Record128, options)
            .map_err(|error| table_boundary(error, 128))
    }
}

fn conditional_invalid(format: FlowConditionalFormat, cause: String) -> BessemerError {
    table_failure(format.identifier(), FailureClass::Invalid, cause)
}
