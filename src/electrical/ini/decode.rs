use super::*;
use std::ops::Range;

pub(super) fn document(
    bytes: &[u8],
    limits: ElectricalIniLimits,
) -> Result<Option<ElectricalIniDocument>> {
    check_limits(bytes, limits)?;
    if !bytes.is_ascii() {
        return Ok(None);
    }
    let records = lexical_records(bytes, limits.maximum_records)?;
    Ok(Some(ElectricalIniDocument {
        bytes: bytes.into(),
        limits,
        records,
    }))
}

fn check_limits(bytes: &[u8], limits: ElectricalIniLimits) -> Result<()> {
    if limits.maximum_bytes == 0
        || limits.maximum_records == 0
        || bytes.len() > limits.maximum_bytes
    {
        return Err(ini_error(
            "limits",
            format!(
                "bytes {}, limits {limits:?}; expected positive limits and bytes <= maximum_bytes",
                bytes.len()
            ),
        ));
    }
    Ok(())
}

fn lexical_records(bytes: &[u8], maximum: usize) -> Result<Vec<ElectricalIniRecord>> {
    let mut records = Vec::new();
    let mut section = None;
    let mut cursor = 0;
    while cursor < bytes.len() {
        if records.len() >= maximum {
            return Err(ini_error(
                cursor,
                format!("record count {}; expected <= {maximum}", records.len() + 1),
            ));
        }
        let (end, body_end) = line_end(bytes, cursor);
        let record = classify(bytes, cursor..end, body_end, section);
        section = record.section.clone();
        records.push(record);
        cursor = end;
    }
    Ok(records)
}

fn line_end(bytes: &[u8], start: usize) -> (usize, usize) {
    let delimiter = bytes[start..]
        .iter()
        .position(|byte| matches!(byte, 10..=13 | 28..=30));
    let Some(position) = delimiter.map(|offset| start + offset) else {
        return (bytes.len(), bytes.len());
    };
    let paired = bytes[position] == 13 && bytes.get(position + 1) == Some(&10);
    let end = position + 1 + usize::from(paired);
    // The reference trims CR/LF only; its other splitline controls remain in values.
    let body_end = if matches!(bytes[position], 10 | 13) {
        position
    } else {
        end
    };
    (end, body_end)
}

fn classify(
    bytes: &[u8],
    raw: Range<usize>,
    body_end: usize,
    section: Option<Range<usize>>,
) -> ElectricalIniRecord {
    let body = &bytes[raw.start..body_end];
    let mut record = ElectricalIniRecord {
        raw: raw.clone(),
        section,
        kind: ElectricalIniKind::Other,
        key: None,
        value: None,
    };
    if body.first() == Some(&b'[') && body.last() == Some(&b']') {
        record.section = Some(raw.start + 1..body_end - 1);
        record.kind = ElectricalIniKind::Section;
        return record;
    }
    assignment_record(record, body, body_end)
}

fn assignment_record(
    mut record: ElectricalIniRecord,
    body: &[u8],
    body_end: usize,
) -> ElectricalIniRecord {
    if matches!(body.first(), Some(b';' | b'#')) {
        return record;
    }
    if let Some(equals) = body.iter().position(|byte| *byte == b'=') {
        record.kind = ElectricalIniKind::Field;
        record.key = Some(record.raw.start..record.raw.start + equals);
        record.value = Some(record.raw.start + equals + 1..body_end);
    }
    record
}
