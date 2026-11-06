//! Complete record-12 count-plus-bytes framing, without runtime index assignments.
mod edit;
mod view;
use super::*;

/// Independent complete-payload and actual transferred-value limits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowCountedByteOptions {
    /// Complete raw payload limit, at least four bytes.
    pub maximum_payload_bytes: usize,
    /// Actual raw byte count; zero admits a stored-empty payload.
    pub maximum_values: usize,
}
/// Immutable complete counted-byte payload with no version prefix or text interpretation.
#[derive(Clone, Debug)]
pub struct FlowCountedBytes {
    wire: TableWire,
    options: FlowCountedByteOptions,
}
impl FlowCarrier {
    /// Reads complete raw counted bytes; e.g. `let values = carrier.record_12_counted_bytes(&id, options)?`.
    pub fn record_12_counted_bytes(
        &self,
        id: &FlowRecordId,
        options: FlowCountedByteOptions,
    ) -> Result<FlowCountedBytes> {
        read(self, id, options).map_err(|error| table_boundary(error, 12))
    }
}
fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowCountedByteOptions,
) -> Result<FlowCountedBytes> {
    let wire = wire::read(
        source,
        id,
        12,
        FlowLegacyTableTransport::Raw,
        options.maximum_payload_bytes,
    )?;
    let count = wire.version;
    validate_count(count, wire.decoded.len() - 4, options.maximum_values)?;
    Ok(FlowCountedBytes { wire, options })
}
fn validate_count(count: i32, available: usize, maximum: usize) -> Result<()> {
    if count >= 0 && count as usize == available && available <= maximum {
        return Ok(());
    }
    Err(table_failure(
        12,
        FailureClass::Invalid,
        format!(
            "stored count {count}, remaining bytes {available}, value limit {maximum}; expected a nonnegative signed-i32 count equal to the remaining raw bytes and at most the value limit"
        ),
    ))
}
