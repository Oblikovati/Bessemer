use super::*;
pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowContextStateOptions,
) -> Result<FlowContextState> {
    let wire = read_context_wire(source, id, options.maximum_payload_bytes)?;
    let mut cursor = ArchiveCursor::new(&wire.decoded);
    cursor.take(4)?;
    let object_count = object_count(&mut cursor)?;
    let count = applied_count(wire.version, object_count, options.index_count)?;
    let index_end = indices(&mut cursor, count, options.maximum_index_words)?;
    if wire.version < 4 {
        cursor.i32()?;
    }
    cursor.finish()?;
    Ok(FlowContextState {
        wire,
        options,
        object_count,
        index_end,
    })
}
fn read_context_wire(source: &FlowCarrier, id: &FlowRecordId, maximum: usize) -> Result<TableWire> {
    let wire = wire::read(source, id, 138, FlowLegacyTableTransport::Raw, maximum)?;
    supported_version(wire.version)?;
    Ok(wire)
}
fn supported_version(version: i32) -> Result<()> {
    if (2..=5).contains(&version) {
        return Ok(());
    }
    Err(table_failure(
        138,
        FailureClass::Unsupported,
        format!("version {version}; expected two through five"),
    ))
}
fn object_count(cursor: &mut ArchiveCursor<'_>) -> Result<u32> {
    let count = cursor.i32()?;
    if count >= 0 {
        return Ok(count as u32);
    }
    Err(context_invalid(format!(
        "object count {count}; expected a nonnegative signed-i32 count"
    )))
}
fn applied_count(
    version: i32,
    objects: u32,
    count: Option<FlowSimpleLegacyCount>,
) -> Result<usize> {
    if version == 5 || objects == 0 {
        return Ok(0);
    }
    count.map(|count| count.value() as usize).ok_or_else(|| {
        table_failure(138, FailureClass::Missing, format!("version {version} object count {objects}, index context None; expected explicit native +0x380 count zero through {}", i32::MAX))
    })
}
fn indices(cursor: &mut ArchiveCursor<'_>, count: usize, maximum: usize) -> Result<usize> {
    if count > maximum || count > cursor.remaining() / 4 {
        return Err(context_invalid(format!(
            "index word count {count}; expected at most {maximum} and at most {} complete words in remaining bytes",
            cursor.remaining() / 4
        )));
    }
    cursor.take(count * 4)?;
    Ok(8 + count * 4)
}
