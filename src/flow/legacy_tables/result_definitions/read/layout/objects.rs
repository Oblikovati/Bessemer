use super::*;
pub(super) fn read(
    cursor: &mut ArchiveCursor<'_>,
    total: usize,
    identifier_start: usize,
    remaining: &mut usize,
) -> Result<ImsSpan> {
    let guid_start = total - cursor.remaining();
    let guid = cursor.take(16)?;
    if guid != IMS_GUID {
        return Err(table_failure(
            500,
            FailureClass::Unsupported,
            format!("object GUID {guid:02x?} at {guid_start}; expected IMS GUID {IMS_GUID:02x?}"),
        ));
    }
    read_body(cursor, total, identifier_start, guid_start, remaining)
}
fn read_body(
    cursor: &mut ArchiveCursor<'_>,
    total: usize,
    identifier_start: usize,
    guid_start: usize,
    remaining: &mut usize,
) -> Result<ImsSpan> {
    let start = total - cursor.remaining();
    let version = read_version(cursor.i32()?)?;
    let presence = version.presence();
    let first = read_pair(cursor, total, [presence[0], presence[1]], remaining)?;
    let allocation = read_allocation(cursor, total, presence[1])?;
    let extra = read_pair(cursor, total, [presence[2], presence[3]], remaining)?;
    Ok(ImsSpan {
        identifier_start,
        guid_start,
        body: DefinitionSpan {
            start,
            end: total - cursor.remaining(),
        },
        version,
        allocation,
        arrays: [first[0], first[1], extra[0], extra[1]],
    })
}
fn read_version(value: i32) -> Result<FlowImsDefinitionVersion> {
    use FlowImsDefinitionVersion::*;
    match value {
        0 => Ok(V0),
        1 => Ok(V1),
        2 => Ok(V2),
        3 => Ok(V3),
        _ => Err(table_failure(
            500,
            FailureClass::Unsupported,
            format!("IMS definition version {value}; expected zero through three"),
        )),
    }
}
fn read_pair(
    cursor: &mut ArchiveCursor<'_>,
    total: usize,
    present: [bool; 2],
    remaining: &mut usize,
) -> Result<[Option<DefinitionSpan>; 2]> {
    let mut spans = [None; 2];
    for (index, present) in present.into_iter().enumerate() {
        if present {
            spans[index] = Some(read_array(cursor, total, remaining)?);
        }
    }
    Ok(spans)
}
fn read_allocation(
    cursor: &mut ArchiveCursor<'_>,
    total: usize,
    present: bool,
) -> Result<Option<usize>> {
    if !present {
        return Ok(None);
    }
    let start = total - cursor.remaining();
    cursor.u8()?;
    Ok(Some(start))
}
fn read_array(
    cursor: &mut ArchiveCursor<'_>,
    total: usize,
    remaining: &mut usize,
) -> Result<DefinitionSpan> {
    let start = total - cursor.remaining();
    let marker = cursor.i32()?;
    if marker != 1 {
        return Err(invalid(format!(
            "shared-array marker {marker} at {start}; expected signed marker one"
        )));
    }
    let count = checked_count(cursor, *remaining, start)?;
    cursor.take(count * 4)?;
    *remaining -= count;
    Ok(DefinitionSpan {
        start,
        end: total - cursor.remaining(),
    })
}

fn checked_count(cursor: &mut ArchiveCursor<'_>, remaining: usize, start: usize) -> Result<usize> {
    let count = cursor.u64()?;
    let available = cursor.remaining() / 4;
    if count > i32::MAX as u64 || count > remaining as u64 || count > available as u64 {
        return Err(invalid(format!(
            "shared-array count {count} at {start}; expected <= signed native maximum {}, remaining value bound {} and {available} complete raw words",
            i32::MAX,
            remaining
        )));
    }
    Ok(count as usize)
}
