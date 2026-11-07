//! Record 108 keeps its raw version outside the explicitly selected body codec.
mod context;
mod decode;
mod edit;
mod projection;
mod types;
mod view;
use super::*;
pub use context::*;
pub use projection::*;
use types::FieldValueSpan;
pub use types::*;

impl FlowCarrier {
    /// Reads a complete version-prefixed field body; e.g. `let fields = carrier.record_108_values(&id, options)?`.
    pub fn record_108_values(
        &self,
        id: &FlowRecordId,
        options: FlowFieldValueOptions,
    ) -> Result<FlowFieldValues> {
        decode::read(self, id, options).map_err(|error| table_boundary(error, 108))
    }
}

fn field_invalid(cause: String) -> BessemerError {
    table_failure(108, FailureClass::Invalid, cause)
}
