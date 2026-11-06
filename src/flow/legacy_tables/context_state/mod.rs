//! Complete record138 with explicit caller context; object count never determines index width.
mod decode;
mod edit;
mod types;
mod view;
use super::*;
pub use types::*;

impl FlowCarrier {
    /// Reads complete explicit-count state; e.g. `let state = carrier.record_138_context_state(&id, options)?`.
    pub fn record_138_context_state(
        &self,
        id: &FlowRecordId,
        options: FlowContextStateOptions,
    ) -> Result<FlowContextState> {
        decode::read(self, id, options).map_err(|error| table_boundary(error, 138))
    }
}
fn context_invalid(cause: String) -> BessemerError {
    table_failure(138, FailureClass::Invalid, cause)
}
