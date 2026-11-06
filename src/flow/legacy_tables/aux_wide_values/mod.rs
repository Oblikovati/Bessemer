//! Complete compressed record444 values, preserved independently of native discard behavior.
mod edit;
mod view;
use super::*;

/// Independent complete decoded-byte and actual raw-wide transfer limits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowAuxWideOptions {
    /// Complete decoded bytes; at least eight for the version-one empty body.
    pub maximum_decoded_bytes: usize,
    /// Actual stored values; zero admits stored-empty arrays in either version.
    pub maximum_values: usize,
}
/// Immutable complete version-zero/one raw wide values and optional discarded legacy word.
#[derive(Clone, Debug)]
pub struct FlowAuxWideValues {
    wire: TableWire,
    options: FlowAuxWideOptions,
    start: usize,
}
impl FlowCarrier {
    /// Reads complete compressed auxiliary values; e.g. `let values = carrier.record_444_aux_wides(&id, options)?`.
    pub fn record_444_aux_wides(
        &self,
        id: &FlowRecordId,
        options: FlowAuxWideOptions,
    ) -> Result<FlowAuxWideValues> {
        read(self, id, options).map_err(|error| table_boundary(error, 444))
    }
}
fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowAuxWideOptions,
) -> Result<FlowAuxWideValues> {
    let (wire, start) = complete_wire(source, id, options.maximum_decoded_bytes)?;
    validate_values(&wire.decoded, start, options.maximum_values)?;
    Ok(FlowAuxWideValues {
        wire,
        options,
        start,
    })
}
fn complete_wire(
    source: &FlowCarrier,
    id: &FlowRecordId,
    maximum: usize,
) -> Result<(TableWire, usize)> {
    if maximum < 8 {
        return Err(extent_failure("decoded byte limit", maximum));
    }
    let wire = wire::read(source, id, 444, FlowLegacyTableTransport::Zlib, maximum)?;
    let start = header_bytes(wire.version)?;
    if wire.decoded.len() < start {
        return Err(extent_failure("decoded body length", wire.decoded.len()));
    }
    Ok((wire, start))
}

fn header_bytes(version: i32) -> Result<usize> {
    match version {
        0 => Ok(12),
        1 => Ok(8),
        _ => Err(table_failure(
            444,
            FailureClass::Unsupported,
            format!("version {version}; expected zero or one"),
        )),
    }
}
fn validate_values(raw: &[u8], start: usize, maximum: usize) -> Result<()> {
    let count = wire::signed_at(raw, start - 4);
    let available = raw.len() - start;
    if count >= 0
        && count as usize <= maximum
        && available.is_multiple_of(8)
        && count as usize == available / 8
    {
        return Ok(());
    }
    Err(table_failure(
        444,
        FailureClass::Invalid,
        format!(
            "stored count {count}, value bytes {available}; expected nonnegative count <= {maximum} and exactly count * eight bytes"
        ),
    ))
}
fn extent_failure(role: &str, value: usize) -> BessemerError {
    table_failure(
        444,
        FailureClass::Invalid,
        format!(
            "{role} {value}; expected at least eight decoded bytes, or twelve for version zero"
        ),
    )
}
