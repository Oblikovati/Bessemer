use super::*;
pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowMessageListOptions,
) -> Result<FlowMessageLists> {
    let wire = message_wire(source, id, options.maximum_payload_bytes)?;
    let encoding = normalized_encoding(wire.version, options.string_mode)?;
    let mut reader = scan::MessageReader::new(&wire.decoded, options, encoding);
    let items = reader.record(wire.version)?;
    Ok(FlowMessageLists {
        items: items.into(),
        texts: reader.texts.into(),
        totals: reader.totals,
        wire,
        options,
        encoding,
    })
}
fn message_wire(source: &FlowCarrier, id: &FlowRecordId, maximum: usize) -> Result<TableWire> {
    if maximum < 8 {
        return Err(message_invalid(format!(
            "payload byte limit {maximum}; expected at least eight"
        )));
    }
    wire::read(source, id, 301, FlowLegacyTableTransport::Raw, maximum)
}
fn normalized_encoding(
    version: i32,
    mode: Option<FlowMetadataStringMode>,
) -> Result<FlowMessageTextEncoding> {
    match version {
        1 => Ok(FlowMessageTextEncoding::LegacyBytes),
        0 | 2 | 3 => modern_encoding(mode),
        _ => Err(table_failure(
            301,
            FailureClass::Unsupported,
            format!("version {version}; expected zero through three"),
        )),
    }
}
fn modern_encoding(mode: Option<FlowMetadataStringMode>) -> Result<FlowMessageTextEncoding> {
    if mode == Some(FlowMetadataStringMode::NativeUtf16) {
        return Ok(FlowMessageTextEncoding::Utf16Units);
    }
    Err(table_failure(
        301,
        FailureClass::Missing,
        format!(
            "string mode {mode:?}; expected explicit NativeUtf16 mode-zero assumption for a modern message record"
        ),
    ))
}
