use super::super::super::payload_codec::record_payload;
use super::*;
pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    context: &FlowSelectionContext,
    options: FlowSelectedValueOptions,
) -> Result<FlowSelectedValues> {
    let count = context.selected_count();
    if count > options.maximum_values || count > options.maximum_payload_bytes / 8 {
        return Err(selected_invalid(format!(
            "selected count {count}, value limit {}, payload byte limit {}; expected count within both actual-value and eight-byte transfer bounds",
            options.maximum_values, options.maximum_payload_bytes
        )));
    }
    let wire = raw_wire(source, id, count * 8)?;
    Ok(FlowSelectedValues {
        wire,
        context: context.clone(),
        options,
    })
}
fn raw_wire(source: &FlowCarrier, id: &FlowRecordId, expected: usize) -> Result<TableWire> {
    let raw = record_payload(source, id, 112, "Flow selected values")?;
    if raw.len() != expected {
        return Err(selected_invalid(format!(
            "raw payload length {}; expected exactly {expected} headerless selected-value bytes",
            raw.len()
        )));
    }
    Ok(TableWire {
        source: source.clone(),
        record: id.clone(),
        decoded: raw.to_vec().into(),
        version: 0,
        stored_bytes: raw.len(),
        compression: FlowLegacyTableTransport::Raw,
        identifier: 112,
    })
}
