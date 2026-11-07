use super::*;
pub(super) fn optional_shared(
    cursor: &mut ArchiveCursor<'_>,
    total: usize,
    options: FlowImsValueOptions,
    totals: &mut ValueTotals,
) -> Result<Option<ValueInterval>> {
    if cursor.u8()? == 0 {
        return Ok(None);
    }
    shared(cursor, total, options, totals).map(Some)
}
pub(super) fn shared(
    cursor: &mut ArchiveCursor<'_>,
    total: usize,
    options: FlowImsValueOptions,
    totals: &mut ValueTotals,
) -> Result<ValueInterval> {
    let start = total - cursor.remaining();
    let marker = cursor.i32()?;
    if marker != 1 {
        return Err(invalid(format!(
            "shared-array marker {marker} at {start}; expected signed marker one"
        )));
    }
    let count = checked_count(
        cursor,
        8,
        options.maximum_shared_values - totals.shared,
        "shared raw-wide values",
    )?;
    cursor.take(count * 8)?;
    totals.shared += count;
    Ok(ValueInterval {
        start,
        end: total - cursor.remaining(),
    })
}
pub(super) fn optional_bl(
    cursor: &mut ArchiveCursor<'_>,
    total: usize,
    options: FlowImsValueOptions,
    totals: &mut ValueTotals,
) -> Result<Option<ValueInterval>> {
    if cursor.u8()? == 0 {
        return Ok(None);
    }
    let start = total - cursor.remaining();
    let version = cursor.i32()?;
    if version != 0 {
        return Err(table_failure(
            501,
            FailureClass::Unsupported,
            format!("BL body at {start} version {version}; expected version zero"),
        ));
    }
    read_bl_rows(cursor, total, start, options, totals).map(Some)
}
fn read_bl_rows(
    cursor: &mut ArchiveCursor<'_>,
    total: usize,
    start: usize,
    options: FlowImsValueOptions,
    totals: &mut ValueTotals,
) -> Result<ValueInterval> {
    let remaining = (options.maximum_bl_rows - totals.bl_rows)
        .min((options.maximum_bl_values - totals.bl_values) / 18);
    let count = checked_count(
        cursor,
        144,
        remaining,
        "BL rows within independent row and raw-value budgets",
    )?;
    cursor.take(count * 144)?;
    totals.bl_rows += count;
    totals.bl_values += count * 18;
    Ok(ValueInterval {
        start,
        end: total - cursor.remaining(),
    })
}
fn checked_count(
    cursor: &mut ArchiveCursor<'_>,
    width: usize,
    maximum: usize,
    label: &str,
) -> Result<usize> {
    let count = cursor.u64()?;
    let available = cursor.remaining() / width;
    if count > i32::MAX as u64 || count > maximum as u64 || count > available as u64 {
        return Err(invalid(format!(
            "{label} count {count}; expected <= native signed maximum {}, remaining configured bound {maximum} and {available} complete {width}-byte elements",
            i32::MAX
        )));
    }
    Ok(count as usize)
}
