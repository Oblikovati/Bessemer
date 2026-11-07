//! Complete record129 tag block and row bodies, without native context or engineering claims.
mod decode;
mod edit;
mod types;
mod view;
use super::*;
use types::TaggedRowSpan;
pub use types::*;

impl FlowCarrier {
    /// Reads the complete tagged-row layout; e.g. `let state = carrier.record_129_tagged_state(&id, options)?`.
    pub fn record_129_tagged_state(
        &self,
        id: &FlowRecordId,
        options: FlowTaggedStateOptions,
    ) -> Result<FlowTaggedState> {
        decode::read(self, id, options).map_err(|error| table_boundary(error, 129))
    }
}
fn tagged_invalid(cause: String) -> BessemerError {
    table_failure(129, FailureClass::Invalid, cause)
}
