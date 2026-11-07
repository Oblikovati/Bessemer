//! Complete paired IMS values under retained fresh-definition provenance.
mod context;
mod edit;
mod read;
mod types;
mod view;
use super::*;
use std::sync::Arc;
pub use types::*;
const OPERATION: &str = "Flow paired IMS values";
fn invalid(cause: String) -> BessemerError {
    table_failure(501, FailureClass::Invalid, cause)
}
impl FlowCarrier {
    /// Reads complete paired values under retained fresh definitions; e.g. `let values = carrier.record_501_ims_values(&id, &context, options)?`.
    pub fn record_501_ims_values(
        &self,
        id: &FlowRecordId,
        context: &FlowImsValueContext,
        options: FlowImsValueOptions,
    ) -> Result<FlowImsValues> {
        read::read(self, id, context, options).map_err(|error| table_boundary(error, 501))
    }
}
