//! Bounded record-143 raw identities; runtime matching and alternate string modes remain external.
mod decode;
mod edit;
mod text_edit;
mod types;
mod view;
use super::*;
use crate::archive_codec::ArchiveCursor;
pub use types::*;

impl FlowCarrier {
    /// Reads the last identity list with explicit bounds/context; e.g. `let list = carrier.feature_identities(options)?`.
    pub fn feature_identities(
        &self,
        options: FlowFeatureOptions,
    ) -> Result<Option<FlowFeatureIdentities>> {
        decode::validate_options(options)?;
        let Some(record) = self.latest(143)? else {
            return Ok(None);
        };
        decode::read(self, record, options)
            .map(Some)
            .map_err(feature_boundary_error)
    }
}

fn feature_failure(class: FailureClass, cause: String) -> BessemerError {
    BessemerError::new("Flow feature identities", "record 143", class, cause)
}

fn feature_boundary_error(mut error: BessemerError) -> BessemerError {
    if error.operation == "Flow feature identities" {
        return error;
    }
    error.operation = "Flow feature identities";
    error.entity = format!("record 143: {}", error.entity);
    error
}
