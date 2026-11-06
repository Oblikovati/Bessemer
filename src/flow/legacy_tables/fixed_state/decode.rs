use super::*;
pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowFixedStateOptions,
) -> Result<FlowFixedState> {
    let wire = wire::read(
        source,
        id,
        139,
        FlowLegacyTableTransport::Raw,
        options.maximum_payload_bytes,
    )?;
    let layout = FixedStateLayout::try_from(wire.version)?;
    let count = parse_rows(&wire.decoded, layout, options.maximum_rows)?;
    Ok(FlowFixedState {
        wire,
        options,
        layout,
        count,
    })
}
fn parse_rows(raw: &[u8], layout: FixedStateLayout, maximum: usize) -> Result<Option<u32>> {
    let mut cursor = ArchiveCursor::new(raw);
    cursor.take(4)?;
    if layout == FixedStateLayout::NoRows {
        cursor.finish()?;
        return Ok(None);
    }
    let count = row_count(&mut cursor)?;
    let width = layout.stride();
    check_rows(count as usize, width, cursor.remaining(), maximum)?;
    cursor.take(count as usize * width)?;
    cursor.finish()?;
    Ok(Some(count))
}
fn row_count(cursor: &mut ArchiveCursor<'_>) -> Result<u32> {
    let count = cursor.i32()?;
    if count >= 0 {
        return Ok(count as u32);
    }
    Err(fixed_state_invalid(format!(
        "row count {count}; expected a nonnegative signed-i32 count"
    )))
}
fn check_rows(count: usize, width: usize, remaining: usize, maximum: usize) -> Result<()> {
    if count <= maximum && count <= remaining / width {
        return Ok(());
    }
    Err(fixed_state_invalid(format!(
        "row count {count}, width {width}, remaining {remaining} bytes; expected at most {maximum} rows and at most {} complete rows",
        remaining / width
    )))
}
