use super::*;
mod layout;
pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowResultDefinitionOptions,
) -> Result<FlowResultDefinitions> {
    let wire = complete_wire(source, id, options.maximum_decoded_bytes)?;
    let (collections, objects, values) = layout::read(&wire.decoded, options)?;
    Ok(FlowResultDefinitions {
        wire,
        options,
        collections,
        objects: objects.into(),
        values,
    })
}
fn complete_wire(source: &FlowCarrier, id: &FlowRecordId, maximum: usize) -> Result<TableWire> {
    validate_limit(maximum)?;
    let stored = super::super::super::payload_codec::record_payload(source, id, 500, OPERATION)?;
    let decoded = expand(stored, maximum)?;
    Ok(TableWire {
        source: source.clone(),
        record: id.clone(),
        decoded: decoded.into(),
        version: 1,
        stored_bytes: stored.len(),
        compression: FlowLegacyTableTransport::Zlib,
        identifier: 500,
    })
}

fn validate_limit(maximum: usize) -> Result<()> {
    if maximum < 21 {
        return Err(invalid(format!(
            "decoded limit {maximum}; expected at least 21 bytes for five empty collections"
        )));
    }
    Ok(())
}

fn expand(stored: &[u8], maximum: usize) -> Result<Vec<u8>> {
    let decoded = super::super::super::payload_codec::inflate_zlib(stored, maximum, OPERATION)?;
    let version = ArchiveCursor::new(&decoded).u8()?;
    if version != 1 {
        return Err(table_failure(
            500,
            FailureClass::Unsupported,
            format!("outer byte version {version}; expected IMS-definition profile version one"),
        ));
    }
    Ok(decoded)
}
