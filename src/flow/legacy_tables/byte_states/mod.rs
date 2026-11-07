//! Complete whole-zlib record316 raw byte states, without header dispatch or index binding.
mod edit;
mod view;
use super::*;

/// Independent bounded inflation and actual byte-transfer limits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowByteStateOptions {
    /// Complete decoded body bound, at least eight bytes.
    pub maximum_decoded_bytes: usize,
    /// Actual stored byte count; zero admits a complete empty array.
    pub maximum_values: usize,
}
/// Immutable complete compressed byte states with an uninterpreted signed header.
#[derive(Clone, Debug)]
pub struct FlowByteStates {
    wire: TableWire,
    options: FlowByteStateOptions,
}
impl FlowCarrier {
    /// Reads complete compressed byte states; e.g. `let states = carrier.record_316_byte_states(&id, options)?`.
    pub fn record_316_byte_states(
        &self,
        id: &FlowRecordId,
        options: FlowByteStateOptions,
    ) -> Result<FlowByteStates> {
        read(self, id, options).map_err(|error| table_boundary(error, 316))
    }
}
fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowByteStateOptions,
) -> Result<FlowByteStates> {
    let wire = complete_wire(source, id, options.maximum_decoded_bytes)?;
    wire::validate_counted_bytes(
        316,
        wire::signed_at(&wire.decoded, 4),
        wire.decoded.len() - 8,
        options.maximum_values,
    )?;
    Ok(FlowByteStates { wire, options })
}
fn complete_wire(source: &FlowCarrier, id: &FlowRecordId, maximum: usize) -> Result<TableWire> {
    if maximum < 8 {
        return Err(invalid_extent("decoded byte limit", maximum));
    }
    let wire = wire::read(source, id, 316, FlowLegacyTableTransport::Zlib, maximum)?;
    if wire.decoded.len() < 8 {
        return Err(invalid_extent("decoded body length", wire.decoded.len()));
    }
    Ok(wire)
}
fn invalid_extent(role: &str, value: usize) -> BessemerError {
    table_failure(
        316,
        FailureClass::Invalid,
        format!("{role} {value}; expected at least eight bytes for the signed header and count"),
    )
}
