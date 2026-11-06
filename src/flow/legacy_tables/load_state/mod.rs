//! Complete raw record11 load state with explicit mode and lossless UTF16 units.
mod decode;
mod edit;
mod fields;
mod schema;
mod types;
mod view;
use super::*;
pub use schema::*;
pub use types::*;
use types::{LoadFieldSpan, LoadScopeSpan, LoadTextSpan};

impl FlowCarrier {
    /// Reads complete raw version18 load state; e.g. `let state = carrier.record_11_load_state(&id, options)?`.
    pub fn record_11_load_state(
        &self,
        id: &FlowRecordId,
        options: FlowLoadStateOptions,
    ) -> Result<FlowLoadState> {
        decode::read(self, id, options).map_err(|error| table_boundary(error, 11))
    }
}

fn load_invalid(cause: String) -> BessemerError {
    table_failure(11, FailureClass::Invalid, cause)
}
