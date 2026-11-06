//! Complete record139 fixed rows, retaining inactive and version-dependent field absence.
mod decode;
mod edit;
mod layout;
mod types;
mod view;
use super::*;
use layout::FixedStateLayout;
pub use types::*;

impl FlowCarrier {
    /// Reads complete fixed state rows; e.g. `let state = carrier.record_139_fixed_state(&id, options)?`.
    pub fn record_139_fixed_state(
        &self,
        id: &FlowRecordId,
        options: FlowFixedStateOptions,
    ) -> Result<FlowFixedState> {
        decode::read(self, id, options).map_err(|error| table_boundary(error, 139))
    }
}
fn fixed_state_invalid(cause: String) -> BessemerError {
    table_failure(139, FailureClass::Invalid, cause)
}
