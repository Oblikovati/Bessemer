//! Record-144 wire grammar with explicit external matched counts and bounded products.
mod blocks;
mod body;
mod decode;
mod edit;
mod types;
mod view;
use super::*;
use crate::archive_codec::ArchiveCursor;
pub use types::*;

impl FlowCarrier {
    /// Reads the optional mapped list with explicit framing context; e.g. `let list = carrier.mapped_objects(limits, context)?`.
    pub fn mapped_objects(
        &self,
        limits: FlowMappedLimits,
        context: FlowMappedContext,
    ) -> Result<Option<FlowMappedObjects>> {
        decode::validate_limits(limits)?;
        let Some(record) = self.latest(144)? else {
            return Ok(None);
        };
        decode::read(self, record, limits, context)
            .map(Some)
            .map_err(boundary_error)
    }
}

fn mapped_failure(class: FailureClass, cause: String) -> BessemerError {
    BessemerError::new("Flow mapped objects", "record 144", class, cause)
}

fn boundary_error(mut error: BessemerError) -> BessemerError {
    if error.operation == "Flow mapped objects" {
        return error;
    }
    error.operation = "Flow mapped objects";
    error.entity = format!("record 144: {}", error.entity);
    error
}
