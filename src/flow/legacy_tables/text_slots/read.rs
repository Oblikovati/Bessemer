use super::*;
pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowTextSlotOptions,
) -> Result<FlowTextSlots> {
    let wire = complete_wire(source, id, options.maximum_payload_bytes)?;
    let (spans, units) = match options.string_mode {
        FlowMetadataStringMode::NativeUtf16 => read_slots(&wire.decoded, options.maximum_units)?,
    };
    Ok(FlowTextSlots {
        wire,
        options,
        spans: Arc::new(spans),
        units,
    })
}
fn complete_wire(source: &FlowCarrier, id: &FlowRecordId, maximum: usize) -> Result<TableWire> {
    if maximum < 204 {
        return Err(invalid(format!(
            "payload limit {}; expected at least 204 bytes",
            maximum
        )));
    }
    wire::read(source, id, 311, FlowLegacyTableTransport::Raw, maximum)
}

fn read_slots(raw: &[u8], maximum: usize) -> Result<([TextSlotSpan; SLOT_COUNT], usize)> {
    let mut cursor = ArchiveCursor::new(raw);
    cursor.i32()?;
    let mut spans = [TextSlotSpan::EMPTY; SLOT_COUNT];
    let mut total = 0;
    for span in &mut spans {
        *span = read_slot(&mut cursor, raw.len(), maximum - total)?;
        total = admit_total(total, span.units, maximum, "transferred text")?;
    }
    cursor.finish()?;
    Ok((spans, total))
}
fn read_slot(
    cursor: &mut ArchiveCursor<'_>,
    length: usize,
    remaining: usize,
) -> Result<TextSlotSpan> {
    let start = length - cursor.remaining();
    let count = cursor.i32()?;
    if count < 0 || count as usize > remaining || count as usize > cursor.remaining() / 2 {
        return Err(invalid(format!(
            "slot at {start} count {count}, remaining {} bytes; expected nonnegative count <= {remaining} fitting complete UTF-16 units",
            cursor.remaining()
        )));
    }
    let units = count as usize;
    let raw = cursor.take(units * 2)?;
    let prefix = wire::utf16_text_prefix(311, start, raw)?;
    Ok(TextSlotSpan {
        start,
        end: start + 4 + raw.len(),
        units,
        prefix,
    })
}
