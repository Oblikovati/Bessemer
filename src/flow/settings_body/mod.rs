//! Strict record-511 bodies, with explicit legacy runtime context and opaque native members.
mod edit;
mod types;
mod view;
mod wire;
use super::*;
pub use types::*;

impl FlowCarrier {
    /// Reads the optional body without inferring runtime context; e.g. `let body = carrier.settings_body(None)?`.
    pub fn settings_body(&self, runtime_integer_slot_88: Option<i32>) -> Result<FlowSettingsBody> {
        let record = self.latest(511)?;
        let parsed = record
            .as_ref()
            .map(|record| wire::decode(record.payload))
            .transpose()?;
        Ok(FlowSettingsBody {
            source: self.clone(),
            record: record.map(|record| record.id),
            parsed,
            runtime_integer_slot_88,
        })
    }
}

fn body_failure(class: FailureClass, cause: String) -> BessemerError {
    BessemerError::new("Flow settings body", "record 511", class, cause)
}
