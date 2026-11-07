use super::*;

pub(super) fn read(bytes: &[u8], limits: FlowLimits) -> Result<FlowCarrier> {
    validate_limits(bytes.len(), limits)?;
    let version = version_word(bytes)?;
    let spans = scan_spans(bytes, limits)?;
    Ok(FlowCarrier {
        revision: Arc::new(FlowRevision {
            wire: Arc::from(bytes),
            spans,
            limits,
            version,
        }),
    })
}

fn scan_spans(bytes: &[u8], limits: FlowLimits) -> Result<Vec<FlowSpan>> {
    let mut spans = Vec::new();
    let mut position = 2;
    while position < bytes.len() {
        if spans.len() == limits.maximum_records {
            return Err(failure(
                "records",
                FailureClass::Invalid,
                format!(
                    "record count exceeds {}; expected at most this limit",
                    limits.maximum_records
                ),
            ));
        }
        let span = next_span(bytes, position, limits.maximum_payload_bytes)?;
        position = span.end;
        spans.push(span);
    }
    Ok(spans)
}

fn validate_limits(length: usize, limits: FlowLimits) -> Result<()> {
    require_positive_limits(limits)?;
    if length > limits.maximum_wire_bytes {
        return Err(failure(
            "wire",
            FailureClass::Invalid,
            format!(
                "{length} bytes; expected at most {}",
                limits.maximum_wire_bytes
            ),
        ));
    }
    Ok(())
}

fn require_positive_limits(limits: FlowLimits) -> Result<()> {
    let positive = [
        limits.maximum_wire_bytes,
        limits.maximum_records,
        limits.maximum_payload_bytes,
    ];
    if positive.contains(&0) {
        return Err(failure(
            "limits",
            FailureClass::Invalid,
            format!("{limits:?}; expected all positive bounds"),
        ));
    }
    Ok(())
}

fn version_word(bytes: &[u8]) -> Result<u16> {
    let word = bytes.get(..2).ok_or_else(|| {
        failure(
            "version",
            FailureClass::Invalid,
            format!("{} bytes; expected at least two version bytes", bytes.len()),
        )
    })?;
    let version = u16::from_le_bytes([word[0], word[1]]);
    if !matches!(version, 7 | 8) {
        return Err(failure(
            "version",
            FailureClass::Unsupported,
            format!("word {version:#06x}; expected observed little-endian 7 or 8"),
        ));
    }
    Ok(version)
}

fn next_span(bytes: &[u8], start: usize, maximum_payload: usize) -> Result<FlowSpan> {
    let remaining = &bytes[start..];
    let header = record_header(remaining, start)?;
    let identifier = u32::from_le_bytes([header[0], header[1], header[2], header[3]]);
    let size = u64::from_le_bytes([
        header[4], header[5], header[6], header[7], header[8], header[9], header[10], header[11],
    ]);
    check_payload_size(
        identifier,
        size,
        start,
        remaining.len() - 12,
        maximum_payload,
    )?;
    Ok(FlowSpan {
        identifier,
        start,
        end: start + 12 + size as usize,
    })
}

fn record_header(remaining: &[u8], start: usize) -> Result<&[u8]> {
    remaining.get(..12).ok_or_else(|| {
        failure(
            "record header",
            FailureClass::Invalid,
            format!(
                "{} trailing bytes at {start}; expected twelve-byte header",
                remaining.len()
            ),
        )
    })
}

fn check_payload_size(
    identifier: u32,
    size: u64,
    start: usize,
    available: usize,
    maximum_payload: usize,
) -> Result<()> {
    if size > available as u64 || size > maximum_payload as u64 {
        return Err(failure(
            "record payload",
            FailureClass::Invalid,
            format!(
                "ID {identifier:#x}, size {size} at {start}; expected at most {} remaining bytes and bound {maximum_payload}",
                available
            ),
        ));
    }
    Ok(())
}
