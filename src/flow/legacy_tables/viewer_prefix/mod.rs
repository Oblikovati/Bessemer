//! Explicit record121 viewer-query prefix; exact transport never promotes unknown body validation.
mod edit;
mod read;
mod types;
mod view;
use super::*;
pub use types::*;
const OPERATION: &str = "Flow record121 viewer prefix";
impl FlowCarrier {
    /// Reads the declared viewer-query prefix and retains opaque bytes; e.g. `let prefix = carrier.record_121_viewer_prefix(&id, options)?`.
    pub fn record_121_viewer_prefix(
        &self,
        id: &FlowRecordId,
        options: FlowViewerPrefixOptions,
    ) -> Result<FlowViewerPrefix> {
        read::read(self, id, options).map_err(|error| table_boundary(error, 121))
    }
}
fn invalid(cause: String) -> BessemerError {
    table_failure(121, FailureClass::Invalid, cause)
}
