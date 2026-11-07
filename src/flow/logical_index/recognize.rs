use super::*;
pub(super) fn terminal(source: &FlowCarrier) -> Result<Option<FlowRecord<'_>>> {
    let Some(start) = terminal_start(source.as_bytes()) else {
        return Ok(None);
    };
    let Some(record) = at_start(source, start) else {
        return Ok(None);
    };
    if record.identifier != INDEX
        || record.header_offset + 12 + record.payload.len() != source.as_bytes().len()
    {
        return Ok(None);
    }
    Ok(marker_matches(&record)?.then_some(record))
}
fn marker_matches(record: &FlowRecord<'_>) -> Result<bool> {
    if record.payload.len() < 8 {
        return Err(invalid(
            "terminal marker",
            format!(
                "payload length {}; expected at least 8 marker bytes",
                record.payload.len()
            ),
        ));
    }
    Ok(record.payload[..8] == [0; 8])
}
fn terminal_start(raw: &[u8]) -> Option<u64> {
    let footer = raw.get(raw.len().checked_sub(12)?..)?;
    if footer[8..] != INDEX.to_le_bytes() {
        return None;
    }
    let distance = i64::from_le_bytes(footer[..8].try_into().ok()?);
    if distance < 0 || distance as u64 >= (raw.len() - 12) as u64 {
        return None;
    }
    Some((raw.len() - 12) as u64 - distance as u64)
}
pub(super) fn at_start(source: &FlowCarrier, start: u64) -> Option<FlowRecord<'_>> {
    let ordinal = source
        .revision
        .spans
        .binary_search_by_key(&start, |span| span.start as u64)
        .ok()?;
    Some(source.record(ordinal, &source.revision.spans[ordinal]))
}
