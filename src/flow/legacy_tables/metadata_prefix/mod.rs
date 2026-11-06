//! Bounded record-424 metadata prefix, retaining every backend byte as opaque.
mod backend;
mod decode;
mod edit;
mod items;
mod types;
mod view;
use super::*;
pub use backend::*;
use std::sync::Arc;
pub use types::*;

impl FlowCarrier {
    /// Reads the documented raw prefix in explicit UTF-16 mode; e.g. `let prefix = carrier.record_424_metadata_prefix(&id, options)?`.
    pub fn record_424_metadata_prefix(
        &self,
        id: &FlowRecordId,
        options: FlowMetadataPrefixOptions,
    ) -> Result<FlowMetadataPrefix> {
        decode::read(self, id, options).map_err(|error| table_boundary(error, 424))
    }
}
fn metadata_invalid(cause: String) -> BessemerError {
    table_failure(424, FailureClass::Invalid, cause)
}
