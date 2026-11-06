use super::*;

pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    limits: FlowIdStateLimits,
) -> Result<FlowIdStates> {
    if limits.maximum_payload_bytes < 8 || limits.maximum_entries == 0 {
        return Err(state_invalid(format!(
            "limits {limits:?}; expected payload bound at least eight and positive row bound"
        )));
    }
    let wire = wire::read(
        source,
        id,
        410,
        FlowLegacyTableTransport::Raw,
        limits.maximum_payload_bytes,
    )?;
    let count = rows(&wire.decoded, limits.maximum_entries)?;
    Ok(FlowIdStates {
        wire,
        limits,
        count,
    })
}

fn rows(raw: &[u8], maximum: usize) -> Result<usize> {
    let mut cursor = ArchiveCursor::new(raw);
    cursor.u32()?;
    let count = cursor.nonnegative_count(5, "ID/state rows")?;
    if count > maximum {
        return Err(state_invalid(format!(
            "row count {count}; expected at most {maximum} ordered rows"
        )));
    }
    cursor.take(count * 5)?;
    cursor.finish()?;
    Ok(count)
}
