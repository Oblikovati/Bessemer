use super::*;
mod blocks;
pub(super) fn read(
    raw: &[u8],
    context: &FlowImsValueContext,
    options: FlowImsValueOptions,
) -> Result<(Vec<ValueSpan>, ValueTotals)> {
    let mut cursor = ArchiveCursor::new(raw);
    cursor.u8()?;
    let mut totals = ValueTotals::default();
    let objects = read_objects(&mut cursor, raw.len(), context, options, &mut totals)?;
    cursor.finish()?;
    Ok((objects, totals))
}
fn read_object(
    cursor: &mut ArchiveCursor<'_>,
    total: usize,
    identifier: i32,
    options: FlowImsValueOptions,
    totals: &mut ValueTotals,
) -> Result<ValueSpan> {
    let start = total - cursor.remaining();
    let version = cursor.i32()?;
    if version != 3 {
        return Err(table_failure(
            501,
            FailureClass::Unsupported,
            format!(
                "value object at {start} version {version}; expected signed IMS-value version three"
            ),
        ));
    }
    read_fields(cursor, total, identifier, start, options, totals)
}
fn read_fields(
    cursor: &mut ArchiveCursor<'_>,
    total: usize,
    identifier: i32,
    start: usize,
    options: FlowImsValueOptions,
    totals: &mut ValueTotals,
) -> Result<ValueSpan> {
    let first_flag = total - cursor.remaining();
    let first = blocks::optional_shared(cursor, total, options, totals)?;
    let main = blocks::shared(cursor, total, options, totals)?;
    let bl_flag = total - cursor.remaining();
    let bl = blocks::optional_bl(cursor, total, options, totals)?;
    let last_flag = total - cursor.remaining();
    let last = blocks::optional_shared(cursor, total, options, totals)?;
    Ok(ValueSpan {
        body: ValueInterval {
            start,
            end: total - cursor.remaining(),
        },
        identifier,
        flags: [first_flag, bl_flag, last_flag],
        arrays: [first, Some(main), last],
        bl,
    })
}

fn read_objects(
    cursor: &mut ArchiveCursor<'_>,
    total: usize,
    context: &FlowImsValueContext,
    options: FlowImsValueOptions,
    totals: &mut ValueTotals,
) -> Result<Vec<ValueSpan>> {
    let count = context.stored_count();
    if count > cursor.remaining() / 19 {
        return Err(invalid(format!(
            "context count {count}, remaining {} bytes; expected at least 19 bytes per complete version-three value object",
            cursor.remaining()
        )));
    }
    let mut objects = Vec::with_capacity(count);
    for ordinal in 0..count {
        let identifier = context.definitions().definition(ordinal)?.identifier();
        objects.push(read_object(cursor, total, identifier, options, totals)?);
    }
    Ok(objects)
}
