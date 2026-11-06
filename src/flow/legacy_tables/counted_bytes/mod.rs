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
    wire::validate_counted_bytes(12, count, wire.decoded.len() - 4, options.maximum_values)?;
    Ok(FlowCountedBytes { wire, options })
}
