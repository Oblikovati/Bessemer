use super::*;
pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowWideVectorOptions,
) -> Result<FlowWideVectors> {
    let wire = complete_wire(source, id, options.maximum_payload_bytes)?;
    let count = vector_count(&wire.decoded, options.maximum_vectors)?;
    let start = 8 + count * 4;
    validate_lengths(&wire.decoded, count, start, options.maximum_values)?;
    let spans = vector_spans(&wire.decoded, count, start);
    let values = (wire.decoded.len() - start) / 8;
    Ok(FlowWideVectors {
        wire,
        options,
        spans,
        values,
    })
}
fn complete_wire(source: &FlowCarrier, id: &FlowRecordId, maximum: usize) -> Result<TableWire> {
    if maximum < 8 {
        return Err(invalid(format!(
            "payload limit {maximum}; expected at least eight bytes"
        )));
    }
    let wire = wire::read(source, id, 148, FlowLegacyTableTransport::Raw, maximum)?;
    if wire.version != 0 {
        return Err(table_failure(
            148,
            FailureClass::Unsupported,
            format!("version {}; expected zero", wire.version),
        ));
    }
    if wire.decoded.len() < 8 {
        return Err(invalid(format!(
            "payload {} bytes; expected at least eight header bytes",
            wire.decoded.len()
        )));
    }
    Ok(wire)
}
fn vector_count(raw: &[u8], maximum: usize) -> Result<usize> {
    let count = wire::signed_at(raw, 4);
    if count >= 0 && count as usize <= maximum && count as usize <= (raw.len() - 8) / 4 {
        return Ok(count as usize);
    }
    Err(invalid(format!(
        "vector count {count}, payload {} bytes; expected nonnegative count <= {maximum} fitting a complete four-byte-per-vector length table",
        raw.len()
    )))
}
fn validate_lengths(raw: &[u8], count: usize, start: usize, maximum: usize) -> Result<()> {
    let mut offset = start;
    let mut values = 0;
    for ordinal in 0..count {
        let length = wire::signed_at(raw, 8 + ordinal * 4);
        let admitted = admit_length(length, ordinal, (raw.len() - offset) / 8, maximum - values)?;
        offset += admitted * 8;
        values += admitted;
    }
    if offset == raw.len() {
        return Ok(());
    }
    Err(invalid(format!(
        "vector bodies ended at {offset}, payload {} bytes; expected exact complete EOF",
        raw.len()
    )))
}
fn admit_length(length: i32, ordinal: usize, available: usize, remaining: usize) -> Result<usize> {
    if length >= 0 && length as usize <= available && length as usize <= remaining {
        return Ok(length as usize);
    }
    Err(invalid(format!(
        "vector {ordinal} length {length}; expected nonnegative count <= remaining {remaining} values fitting {available} available raw wide words"
    )))
}
fn vector_spans(raw: &[u8], count: usize, start: usize) -> Arc<[VectorSpan]> {
    let mut offset = start;
    (0..count)
        .map(|ordinal| {
            let start = offset;
            offset += wire::unsigned_at(raw, 8 + ordinal * 4) as usize * 8;
            VectorSpan { start, end: offset }
        })
        .collect()
}
fn invalid(cause: String) -> BessemerError {
    table_failure(148, FailureClass::Invalid, cause)
}
