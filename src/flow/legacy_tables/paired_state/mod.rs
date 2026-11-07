//! Complete raw record417; integer quotients do not establish native array semantics.
mod decode;
mod edit;
mod types;
mod view;
use super::*;
use types::PairedGroupSpan;
pub use types::*;

impl FlowCarrier {
    /// Reads both complete raw groups; e.g. `let state = carrier.record_417_paired_state(&id, options)?`.
    pub fn record_417_paired_state(
        &self,
        id: &FlowRecordId,
        options: FlowPairedStateOptions,
    ) -> Result<FlowPairedState> {
        decode::read(self, id, options).map_err(|error| table_boundary(error, 417))
    }
}
fn paired_invalid(cause: String) -> BessemerError {
    table_failure(417, FailureClass::Invalid, cause)
}
