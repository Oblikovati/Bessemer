//! Exact record-101 base-grid framing without inferred geometry or refinement.
mod decode;
mod edit;
mod types;
mod view;
use super::*;
pub use types::*;

impl FlowCarrier {
    /// Reads one revision-bound grid with explicit transport; e.g. `let grid = carrier.record_101_grid(&id, options)?`.
    pub fn record_101_grid(
        &self,
        id: &FlowRecordId,
        options: FlowGridOptions,
    ) -> Result<FlowBaseGrid> {
        decode::read(self, id, options).map_err(|mut error| {
            error.operation = "Flow base grid";
            error.entity = format!("record 101: {}", error.entity);
            error
        })
    }
}

fn grid_failure(cause: String) -> BessemerError {
    BessemerError::new("Flow base grid", "record 101", FailureClass::Invalid, cause)
}
