//! Complete raw object records; physical trailer presence is independent of runtime appending.
mod edit;
mod fields;
mod row;
mod types;
mod view;
use super::*;
pub use types::*;

impl FlowCarrier {
    /// Reads complete raw objects; e.g. `let objects = carrier.record_304_objects(&id, options)?`.
    pub fn record_304_objects(
        &self,
        id: &FlowRecordId,
        options: FlowObjectRecordOptions,
    ) -> Result<FlowObjectRecords> {
        read(self, id, options).map_err(|error| table_boundary(error, 304))
    }
}
fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowObjectRecordOptions,
) -> Result<FlowObjectRecords> {
    validate_options(options)?;
    let wire = wire::read(
        source,
        id,
        304,
        FlowLegacyTableTransport::Raw,
        options.maximum_payload_bytes,
    )?;
    let stride = row_stride(wire.version)?;
    let mut cursor = ArchiveCursor::new(&wire.decoded);
    cursor.i32()?;
    let count = cursor.i32()?;
    validate_extent(&wire, count, stride, options.maximum_rows)?;
    Ok(FlowObjectRecords {
        wire,
        options,
        count: count as usize,
        stride,
    })
}
fn validate_options(options: FlowObjectRecordOptions) -> Result<()> {
    if options.maximum_payload_bytes >= 8 {
        return Ok(());
    }
    Err(table_failure(
        304,
        FailureClass::Invalid,
        format!(
            "payload byte limit {}; expected at least eight",
            options.maximum_payload_bytes
        ),
    ))
}
fn row_stride(version: i32) -> Result<usize> {
    match version {
        0..=3 => Ok(36),
        4 => Ok(40),
        5 => Ok(24),
        6 => Ok(25),
        7 => Ok(81),
        _ => Err(table_failure(
            304,
            FailureClass::Unsupported,
            format!("version {version}; expected zero through seven"),
        )),
    }
}
fn validate_extent(wire: &TableWire, count: i32, stride: usize, maximum: usize) -> Result<()> {
    let available = wire.decoded.len().saturating_sub(8);
    validate_count(count, available, stride, maximum)?;
    let trailer = if count > 0 && wire.version < 5 { 20 } else { 0 };
    let row_bytes = count as usize * stride;
    if available.checked_sub(row_bytes) == Some(trailer) {
        return Ok(());
    }
    Err(table_failure(
        304,
        FailureClass::Invalid,
        format!(
            "payload {} bytes, rows {count}; expected eight header bytes, {row_bytes} row bytes and {trailer} trailer bytes",
            wire.decoded.len()
        ),
    ))
}

fn validate_count(count: i32, available: usize, stride: usize, maximum: usize) -> Result<()> {
    if count < 0 || count as usize > maximum || count as usize > available / stride {
        return Err(table_failure(
            304,
            FailureClass::Invalid,
            format!(
                "row count {count}, available {available} bytes; expected nonnegative count <= {maximum} fitting {stride}-byte rows"
            ),
        ));
    }
    Ok(())
}
