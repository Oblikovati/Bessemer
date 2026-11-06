//! Complete legacy per-leaf extra fields, with caller-supplied runtime context.
mod decode;
mod edit;
mod schema;
mod types;
mod view;
use super::*;
pub use schema::*;
pub use types::*;
use types::{ExtraEntitySpan, ExtraFieldSpan, ExtraIndexSource};

impl FlowCarrier {
    /// Reads one selected complete legacy extra payload; e.g. `let extras = carrier.record_408_extras(&id, options)?`.
    pub fn record_408_extras(
        &self,
        id: &FlowRecordId,
        options: FlowBlockExtraOptions,
    ) -> Result<FlowBlockExtras> {
        decode::read(self, id, options).map_err(|error| table_boundary(error, 408))
    }
}

fn extra_invalid(cause: String) -> BessemerError {
    table_failure(408, FailureClass::Invalid, cause)
}
