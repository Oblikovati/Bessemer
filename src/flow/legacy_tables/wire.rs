use super::*;

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

pub(super) fn unsigned_at(raw: &[u8], offset: usize) -> u32 {
    let span = &raw[offset..offset + 4];
    u32::from_le_bytes([span[0], span[1], span[2], span[3]])
}

pub(super) fn wide_at(raw: &[u8], offset: usize) -> u64 {
    u64::from(unsigned_at(raw, offset)) | u64::from(unsigned_at(raw, offset + 4)) << 32
}
