use super::*;

pub(super) fn versioned<F>(
    source: &FlowCarrier,
    id: &FlowRecordId,
    identifier: u32,
    compression: FlowLegacyTableTransport,
    operation: &'static str,
    expand_body: F,
) -> Result<TableWire>
where
    F: FnOnce(&[u8], i32) -> Result<Vec<u8>>,
{
    let stored = super::super::payload_codec::record_payload(source, id, identifier, operation)?;
    let version = ArchiveCursor::new(stored).i32()?;
    let decoded = expand_body(&stored[4..], version)?;
    Ok(TableWire {
        source: source.clone(),
        record: id.clone(),
        decoded: decoded.into(),
        version,
        stored_bytes: stored.len(),
        compression,
        identifier,
    })
}

pub(super) fn edited_versioned(
    wire: &TableWire,
    body: &[u8],
    operation: &'static str,
) -> Result<FlowCarrierEdit> {
    if body == wire.decoded.as_ref() {
        let stored = super::super::payload_codec::record_payload(
            &wire.source,
            &wire.record,
            wire.identifier,
            operation,
        )?;
        return wire.source.replace_payload(&wire.record, stored);
    }
    let payload = versioned_payload(wire, body, operation)?;
    wire.source.replace_payload(&wire.record, &payload)
}

fn versioned_payload(wire: &TableWire, body: &[u8], operation: &'static str) -> Result<Vec<u8>> {
    let encoded = match wire.compression {
        FlowLegacyTableTransport::Raw => body.to_vec(),
        FlowLegacyTableTransport::Zlib => {
            super::super::payload_codec::encode_zlib(body, operation)?
        }
    };
    let mut payload = wire.version.to_le_bytes().to_vec();
    payload.extend_from_slice(&encoded);
    Ok(payload)
}

pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    identifier: u32,
    compression: FlowLegacyTableTransport,
    maximum: usize,
) -> Result<TableWire> {
    if maximum < 4 {
        return Err(table_failure(
            identifier,
            FailureClass::Invalid,
            format!("decoded byte limit {maximum}; expected at least four"),
        ));
    }
    let stored =
        super::super::payload_codec::record_payload(source, id, identifier, "Flow legacy tables")?;
    let decoded = expand(stored, identifier, compression, maximum)?;
    let version = ArchiveCursor::new(&decoded).i32()?;
    Ok(TableWire {
        source: source.clone(),
        record: id.clone(),
        decoded: decoded.into(),
        version,
        stored_bytes: stored.len(),
        compression,
        identifier,
    })
}

fn expand(
    stored: &[u8],
    identifier: u32,
    compression: FlowLegacyTableTransport,
    maximum: usize,
) -> Result<Vec<u8>> {
    if compression == FlowLegacyTableTransport::Zlib {
        return super::super::payload_codec::inflate_zlib(stored, maximum, "Flow legacy tables");
    }
    if stored.len() <= maximum {
        return Ok(stored.to_vec());
    }
    Err(table_failure(
        identifier,
        FailureClass::Invalid,
        format!(
            "raw payload {} bytes; expected at most {maximum} decoded bytes",
            stored.len()
        ),
    ))
}

pub(super) fn edited(wire: &TableWire, decoded: &[u8]) -> Result<FlowCarrierEdit> {
    if decoded == wire.decoded.as_ref() {
        let stored = super::super::payload_codec::record_payload(
            &wire.source,
            &wire.record,
            wire.identifier,
            "Flow legacy tables",
        )?;
        return wire.source.replace_payload(&wire.record, stored);
    }
    let stored = match wire.compression {
        FlowLegacyTableTransport::Raw => decoded.to_vec(),
        FlowLegacyTableTransport::Zlib => {
            super::super::payload_codec::encode_zlib(decoded, "Flow legacy tables")?
        }
    };
    wire.source.replace_payload(&wire.record, &stored)
}

pub(super) fn result_id(wire: &TableWire, result: &FlowCarrier) -> Result<FlowRecordId> {
    result
        .records()
        .nth(wire.record.ordinal())
        .map(|record| record.id)
        .ok_or_else(|| {
            table_index_error(
                wire.identifier,
                "result occurrence",
                wire.record.ordinal(),
                result.records().len(),
            )
        })
}

pub(super) fn check_decoded(wire: &TableWire, expected: &[u8]) -> Result<()> {
    if wire.decoded.as_ref() == expected {
        return Ok(());
    }
    Err(table_failure(
        wire.identifier,
        FailureClass::Integrity,
        format!(
            "reopened {} decoded bytes; expected complete patched {} bytes",
            wire.decoded.len(),
            expected.len()
        ),
    ))
}

pub(super) fn mode(wire: &TableWire) -> FlowLegacyTableCheckMode {
    match wire.compression {
        FlowLegacyTableTransport::Raw => FlowLegacyTableCheckMode::ExactRawTable,
        FlowLegacyTableTransport::Zlib => FlowLegacyTableCheckMode::ExactZlibTable,
    }
}

pub(super) fn signed_at(raw: &[u8], offset: usize) -> i32 {
    unsigned_at(raw, offset) as i32
}
pub(super) fn element_offset(
    identifier: u32,
    start: usize,
    count: usize,
    index: usize,
    width: usize,
) -> Result<usize> {
    if index < count {
        return Ok(start + index * width);
    }
    Err(table_index_error(identifier, "array element", index, count))
}

pub(super) fn unsigned_at(raw: &[u8], offset: usize) -> u32 {
    let span = &raw[offset..offset + 4];
    u32::from_le_bytes([span[0], span[1], span[2], span[3]])
}

pub(super) fn wide_at(raw: &[u8], offset: usize) -> u64 {
    u64::from(unsigned_at(raw, offset)) | u64::from(unsigned_at(raw, offset + 4)) << 32
}
pub(super) fn unit_at(raw: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([raw[offset], raw[offset + 1]])
}
pub(super) fn utf16_prefix_units(raw: &[u8]) -> usize {
    let units = raw.as_chunks::<2>().0;
    units
        .iter()
        .position(|unit| *unit == [0, 0])
        .unwrap_or(units.len())
}
pub(super) fn utf16_text_prefix(identifier: u32, start: usize, raw: &[u8]) -> Result<usize> {
    if !raw.is_empty() && raw[raw.len() - 2..] != [0, 0] {
        let count = raw.len() / 2;
        return Err(table_failure(
            identifier,
            FailureClass::Invalid,
            format!("UTF-16 string at {start} with {count} units; expected terminal NUL"),
        ));
    }
    Ok(utf16_prefix_units(raw))
}

pub(super) fn scalar_value(encoding: FlowPrimaryEncoding, raw: &[u8]) -> FlowPrimaryValue {
    match encoding {
        FlowPrimaryEncoding::Byte => FlowPrimaryValue::Byte(raw[0]),
        FlowPrimaryEncoding::Signed32 => FlowPrimaryValue::Signed32(signed_at(raw, 0)),
        FlowPrimaryEncoding::WordBits => FlowPrimaryValue::WordBits(unsigned_at(raw, 0)),
        FlowPrimaryEncoding::WideBits => FlowPrimaryValue::WideBits(wide_at(raw, 0)),
    }
}

pub(super) fn scalar_bytes(
    identifier: u32,
    encoding: FlowPrimaryEncoding,
    value: FlowPrimaryValue,
) -> Result<Vec<u8>> {
    match (encoding, value) {
        (FlowPrimaryEncoding::Byte, FlowPrimaryValue::Byte(value)) => Ok(vec![value]),
        (FlowPrimaryEncoding::Signed32, FlowPrimaryValue::Signed32(value)) => {
            Ok(value.to_le_bytes().to_vec())
        }
        (FlowPrimaryEncoding::WordBits, FlowPrimaryValue::WordBits(value)) => {
            Ok(value.to_le_bytes().to_vec())
        }
        (FlowPrimaryEncoding::WideBits, FlowPrimaryValue::WideBits(value)) => {
            Ok(value.to_le_bytes().to_vec())
        }
        _ => Err(table_failure(
            identifier,
            FailureClass::Invalid,
            format!("field value {value:?}; expected exact {encoding:?} encoding"),
        )),
    }
}
