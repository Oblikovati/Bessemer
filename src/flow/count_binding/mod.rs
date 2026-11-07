//! Known record-406 prefixes and explicitly conditional absent-map count binding.
mod bind;
mod decode;
mod types;
mod view;
use super::global_counts::GlobalCountPair;
use super::*;
pub use types::*;

impl FlowCarrier {
    /// Reads only the exact known count prefix and retains the opaque body; e.g. `let counts = carrier.record_406_context_counts(&id, options)?`.
    pub fn record_406_context_counts(
        &self,
        id: &FlowRecordId,
        options: FlowCountPrefixOptions,
    ) -> Result<FlowContextCounts> {
        decode::read(self, id, options).map_err(|mut error| {
            error.operation = "Flow context counts";
            error.entity = format!("record 406: {}", error.entity);
            error
        })
    }
}

fn count_failure(class: FailureClass, cause: String) -> BessemerError {
    BessemerError::new("Flow context counts", "record 406", class, cause)
}
