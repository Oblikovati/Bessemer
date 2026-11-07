use super::*;
mod layout;
pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    context: &FlowImsValueContext,
    options: FlowImsValueOptions,
) -> Result<FlowImsValues> {
    validate_options(context, options)?;
    let wire = complete_wire(source, id, options.maximum_decoded_bytes)?;
    let (objects, totals) = layout::read(&wire.decoded, context, options)?;
    Ok(FlowImsValues {
        wire,
        context: context.clone(),
        options,
        objects: objects.into(),
        totals,
    })
}
fn validate_options(context: &FlowImsValueContext, options: FlowImsValueOptions) -> Result<()> {
    if options.maximum_decoded_bytes == 0 || context.stored_count() > options.maximum_objects {
        return Err(invalid(format!(
            "decoded limit {}, context objects {}, object limit {}; expected at least one decoded byte and context count within the object bound",
            options.maximum_decoded_bytes,
            context.stored_count(),
            options.maximum_objects
        )));
    }
    Ok(())
}
fn complete_wire(source: &FlowCarrier, id: &FlowRecordId, maximum: usize) -> Result<TableWire> {
    let stored = super::super::super::payload_codec::record_payload(source, id, 501, OPERATION)?;
    let decoded = expand(stored, maximum)?;
    Ok(TableWire {
        source: source.clone(),
        record: id.clone(),
        decoded: decoded.into(),
        version: 2,
        stored_bytes: stored.len(),
        compression: FlowLegacyTableTransport::Zlib,
        identifier: 501,
    })
}
fn expand(stored: &[u8], maximum: usize) -> Result<Vec<u8>> {
    let decoded = super::super::super::payload_codec::inflate_zlib(stored, maximum, OPERATION)?;
    let version = ArchiveCursor::new(&decoded).u8()?;
    if version != 2 {
        return Err(table_failure(
            501,
            FailureClass::Unsupported,
            format!("outer byte version {version}; expected paired IMS-value profile version two"),
        ));
    }
    Ok(decoded)
}
