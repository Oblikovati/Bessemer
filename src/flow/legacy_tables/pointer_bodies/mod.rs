//! Complete legacy pools and context-selected bodies; opaque packed bytes stay opaque.
mod decode;
mod edit;
mod options;
mod types;
mod view;
use super::*;
pub use options::*;
pub use types::*;
use types::{BodyEntrySpan, BodyGroupSpan, BodyPoolSpan, BodySpan};

impl FlowCarrier {
    /// Reads complete pools/bodies with caller runtime codes; e.g. `let bodies = carrier.record_421_bodies(&id, options)?`.
    pub fn record_421_bodies(
        &self,
        id: &FlowRecordId,
        options: FlowPointerBodyOptions,
    ) -> Result<FlowPointerBodies> {
        decode::read(self, id, options).map_err(|error| table_boundary(error, 421))
    }
}

fn body_invalid(cause: String) -> BessemerError {
    table_failure(421, FailureClass::Invalid, cause)
}
