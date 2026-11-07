//! Complete record311 UTF-16 transfers and pure fifty-slot existing-state projections.
mod edit;
mod projection;
mod read;
mod state;
mod types;
mod view;
use super::*;
use std::sync::Arc;
pub use types::*;
const SLOT_COUNT: usize = 50;
impl FlowCarrier {
    /// Reads exactly fifty strict native UTF-16 slots; e.g. `let slots = carrier.record_311_text_slots(&id, options)?`.
    pub fn record_311_text_slots(
        &self,
        id: &FlowRecordId,
        options: FlowTextSlotOptions,
    ) -> Result<FlowTextSlots> {
        read::read(self, id, options).map_err(|error| table_boundary(error, 311))
    }
}
fn invalid(cause: String) -> BessemerError {
    table_failure(311, FailureClass::Invalid, cause)
}
fn admit_total(total: usize, additional: usize, maximum: usize, role: &str) -> Result<usize> {
    total.checked_add(additional).filter(|sum|*sum<=maximum).ok_or_else(|| {
        invalid(format!("{role} accumulated {total} units plus {additional}; expected a representable total <= {maximum}"))
    })
}
