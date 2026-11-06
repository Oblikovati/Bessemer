//! Complete raw ID/state rows; the initial word is not a version dispatch.
mod decode;
mod edit;
mod projection;
mod types;
mod view;
use super::*;
pub use projection::*;
pub use types::*;

impl FlowCarrier {
    /// Reads complete ordered state rows without applying them; e.g. `let states = carrier.record_410_states(&id, limits)?`.
    pub fn record_410_states(
        &self,
        id: &FlowRecordId,
        limits: FlowIdStateLimits,
    ) -> Result<FlowIdStates> {
        decode::read(self, id, limits).map_err(|error| table_boundary(error, 410))
    }
}

fn state_invalid(cause: String) -> BessemerError {
    table_failure(410, FailureClass::Invalid, cause)
}
