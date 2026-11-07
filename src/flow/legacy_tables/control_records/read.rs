use super::*;
pub(super) fn saved(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowControlOptions,
) -> Result<FlowSavedSpans> {
    validate_options(SAVED, options, 16)?;
    let stored = transport::payload(source, id, SAVED)?;
    let decoded = transport::inflate(stored, options.maximum_decoded_bytes)?;
    let count = saved_count(&decoded, options.maximum_entries)?;
    let wire = complete_wire(source, id, SAVED, decoded, stored.len());
    Ok(FlowSavedSpans {
        wire,
        options,
        count,
    })
}
pub(super) fn index(
    source: &FlowCarrier,
    id: &FlowRecordId,
    width: FlowControlLengthWidth,
    options: FlowControlOptions,
) -> Result<FlowTerminalIndex> {
    validate_options(INDEX, options, 4)?;
    let stored = transport::payload(source, id, INDEX)?;
    let (decoded, footer) = transport::read_index(stored, width, options.maximum_decoded_bytes)?;
    let count = index_count(&decoded, options.maximum_entries)?;
    let wire = complete_wire(source, id, INDEX, decoded, stored.len());
    Ok(FlowTerminalIndex {
        wire,
        options,
        width,
        footer,
        count,
    })
}
fn complete_wire(
    source: &FlowCarrier,
    id: &FlowRecordId,
    identifier: u32,
    decoded: Vec<u8>,
    stored_bytes: usize,
) -> ControlWire {
    ControlWire {
        source: source.clone(),
        record: id.clone(),
        identifier,
        decoded: decoded.into(),
        stored_bytes,
    }
}
fn validate_options(identifier: u32, options: FlowControlOptions, minimum: usize) -> Result<()> {
    if options.maximum_decoded_bytes >= minimum {
        return Ok(());
    }
    Err(invalid(
        identifier,
        format!(
            "decoded limit {}; expected at least {minimum} bytes for a complete empty writer profile",
            options.maximum_decoded_bytes
        ),
    ))
}
fn saved_count(raw: &[u8], maximum: usize) -> Result<usize> {
    let mut cursor = ArchiveCursor::new(raw);
    let marker = cursor.u64()?;
    transport::require_zero(SAVED, marker)?;
    let count = cursor.u64()?;
    let count = admit_count(SAVED, count, maximum, cursor.remaining() / 16)?;
    cursor.take(count * 16)?;
    cursor.finish()?;
    Ok(count)
}
fn index_count(raw: &[u8], maximum: usize) -> Result<usize> {
    let mut cursor = ArchiveCursor::new(raw);
    let count = cursor.i32()?;
    if count < 0 {
        return Err(invalid(
            INDEX,
            format!("index count {count}; expected a nonnegative signed count"),
        ));
    }
    let count = admit_count(INDEX, count as u64, maximum, cursor.remaining() / 20)?;
    cursor.take(count * 20)?;
    cursor.finish()?;
    Ok(count)
}
fn admit_count(identifier: u32, count: u64, maximum: usize, available: usize) -> Result<usize> {
    if count > maximum as u64 || count > available as u64 {
        return Err(invalid(
            identifier,
            format!(
                "entry count {count}; expected <= configured bound {maximum} and {available} complete remaining entries"
            ),
        ));
    }
    Ok(count as usize)
}
