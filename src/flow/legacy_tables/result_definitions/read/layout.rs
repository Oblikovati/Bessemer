use super::*;
mod objects;
pub(super) fn read(
    raw: &[u8],
    options: FlowResultDefinitionOptions,
) -> Result<([DefinitionSpan; 5], Vec<ImsSpan>, usize)> {
    let mut cursor = ArchiveCursor::new(raw);
    cursor.u8()?;
    let mut remaining_values = options.maximum_values;
    let objects = primary_objects(
        &mut cursor,
        raw.len(),
        options.maximum_objects,
        &mut remaining_values,
    )?;
    let mut collections = [DefinitionSpan { start: 0, end: 0 }; 5];
    collections[0] = DefinitionSpan {
        start: 1,
        end: raw.len() - cursor.remaining(),
    };
    read_empty_collections(&mut cursor, raw.len(), &mut collections)?;
    cursor.finish()?;
    let values = options.maximum_values - remaining_values;
    Ok((collections, objects, values))
}
fn primary_count(cursor: &mut ArchiveCursor<'_>, maximum: usize) -> Result<usize> {
    let count = cursor.i32()?;
    let available = cursor.remaining().saturating_sub(16) / 24;
    if count < 0 || count as usize > maximum || count as usize > available {
        return Err(invalid(format!(
            "primary object count {count}, remaining {} bytes; expected nonnegative count <= {maximum} and <= {available} complete identifier/GUID/version tuples plus four empty collections",
            cursor.remaining()
        )));
    }
    Ok(count as usize)
}
fn read_empty_collections(
    cursor: &mut ArchiveCursor<'_>,
    total: usize,
    spans: &mut [DefinitionSpan; 5],
) -> Result<()> {
    for (ordinal, span) in spans.iter_mut().enumerate().skip(1) {
        let start = total - cursor.remaining();
        let count = cursor.i32()?;
        if count != 0 {
            return Err(invalid(format!(
                "secondary collection {ordinal} count {count}; expected exactly zero in the IMS-definition profile"
            )));
        }
        *span = DefinitionSpan {
            start,
            end: total - cursor.remaining(),
        };
    }
    Ok(())
}

fn primary_objects(
    cursor: &mut ArchiveCursor<'_>,
    total: usize,
    maximum: usize,
    remaining: &mut usize,
) -> Result<Vec<ImsSpan>> {
    let count = primary_count(cursor, maximum)?;
    let ids_start = total - cursor.remaining();
    cursor.take(count * 4)?;
    let mut result = Vec::with_capacity(count);
    for ordinal in 0..count {
        result.push(objects::read(
            cursor,
            total,
            ids_start + ordinal * 4,
            remaining,
        )?);
    }
    Ok(result)
}
