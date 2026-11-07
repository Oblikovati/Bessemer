use super::*;

pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    format: FlowConditionalFormat,
    options: FlowConditionalItemOptions,
) -> Result<TableWire> {
    let stored = super::super::super::payload_codec::record_payload(
        source,
        id,
        format.identifier(),
        "Flow conditional items",
    )?;
    check_limits(stored.len(), format, options)?;
    if format == FlowConditionalFormat::Record128 {
        return versioned(source, id, options);
    }
    body_wire(source, id, format, options)
}

fn check_limits(
    stored_bytes: usize,
    format: FlowConditionalFormat,
    options: FlowConditionalItemOptions,
) -> Result<()> {
    if stored_bytes > options.maximum_stored_bytes || options.maximum_decoded_bytes < 4 {
        return Err(conditional_invalid(
            format,
            format!(
                "stored bytes {}, limits {options:?}; expected stored bytes within bound and decoded byte limit at least four",
                stored_bytes
            ),
        ));
    }
    Ok(())
}

fn body_wire(
    source: &FlowCarrier,
    id: &FlowRecordId,
    format: FlowConditionalFormat,
    options: FlowConditionalItemOptions,
) -> Result<TableWire> {
    let compression = if format == FlowConditionalFormat::Record126 {
        FlowLegacyTableTransport::Raw
    } else {
        FlowLegacyTableTransport::Zlib
    };
    let wire = wire::read(
        source,
        id,
        format.identifier(),
        compression,
        options.maximum_decoded_bytes,
    )?;
    let supported = match format {
        FlowConditionalFormat::Record126 => matches!(wire.version, 1 | 2),
        _ => wire.version == 4,
    };
    if !supported {
        return Err(version_error(format, wire.version));
    }
    Ok(wire)
}

fn versioned(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowConditionalItemOptions,
) -> Result<TableWire> {
    wire::versioned(
        source,
        id,
        128,
        FlowLegacyTableTransport::Zlib,
        "Flow conditional items",
        |body, version| {
            if !matches!(version, 6 | 7) {
                return Err(version_error(FlowConditionalFormat::Record128, version));
            }
            super::super::super::payload_codec::inflate_zlib(
                body,
                options.maximum_decoded_bytes,
                "Flow conditional items",
            )
        },
    )
}

fn version_error(format: FlowConditionalFormat, version: i32) -> BessemerError {
    let expected = match format {
        FlowConditionalFormat::Record126 => "one/two",
        FlowConditionalFormat::Record315 => "four",
        FlowConditionalFormat::Record128 => "six/seven",
    };
    table_failure(
        format.identifier(),
        FailureClass::Unsupported,
        format!("version {version}; expected {expected}"),
    )
}

pub(super) fn edited(source: &FlowConditionalItems, decoded: &[u8]) -> Result<FlowCarrierEdit> {
    if source.format == FlowConditionalFormat::Record128 {
        return wire::edited_versioned(&source.wire, decoded, "Flow conditional items");
    }
    wire::edited(&source.wire, decoded)
}
