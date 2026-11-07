//! Complete record432 stored state; native recomputation is reported, never simulated.
mod edit;
mod types;
mod view;
use super::*;
pub use types::*;

impl FlowCarrier {
    /// Reads complete raw state; e.g. `let state = carrier.record_432_stored_state(&id, options)?`.
    pub fn record_432_stored_state(
        &self,
        id: &FlowRecordId,
        options: FlowStoredStateOptions,
    ) -> Result<FlowStoredState> {
        read(self, id, options).map_err(|error| table_boundary(error, 432))
    }
}
fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowStoredStateOptions,
) -> Result<FlowStoredState> {
    let wire = wire::read(
        source,
        id,
        432,
        FlowLegacyTableTransport::Raw,
        options.maximum_payload_bytes,
    )?;
    validate_version(wire.version)?;
    validate_length(&wire)?;
    Ok(FlowStoredState { wire, options })
}
fn validate_version(version: i32) -> Result<()> {
    if version < 0 {
        return Err(table_failure(
            432,
            FailureClass::Unsupported,
            format!(
                "version {}; expected a nonnegative signed-i32 version",
                version
            ),
        ));
    }
    Ok(())
}
fn validate_length(wire: &TableWire) -> Result<()> {
    let expected = if [1, 2].contains(&wire.version) {
        20
    } else {
        16
    };
    if wire.decoded.len() != expected {
        return Err(table_failure(
            432,
            FailureClass::Invalid,
            format!(
                "version {} payload {} bytes; expected exactly {expected} raw bytes",
                wire.version,
                wire.decoded.len()
            ),
        ));
    }
    Ok(())
}
