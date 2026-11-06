//! Complete raw primary-class record 439; native allocation and engineering are excluded.
mod decode;
mod edit;
mod fields;
mod scan;
mod schema;
mod types;
mod view;
use super::*;
pub use schema::*;
pub use types::*;
use types::{PrimaryFieldSpan, PrimaryScopeSpan};

impl FlowCarrier {
    /// Reads the complete primary-class grammar; e.g. `let table = carrier.record_439_primary_table(&id, options)?`.
    pub fn record_439_primary_table(
        &self,
        id: &FlowRecordId,
        options: FlowPrimaryTableOptions,
    ) -> Result<FlowPrimaryTable> {
        decode::read(self, id, options).map_err(|error| table_boundary(error, 439))
    }
}

fn primary_invalid(cause: String) -> BessemerError {
    table_failure(439, FailureClass::Invalid, cause)
}
