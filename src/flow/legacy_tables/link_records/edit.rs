mod select;
use super::*;

impl FlowLinkRecords {
    /// Edits one existing field on an isolated copy; e.g. `let edit = links.replace_field(FlowLinkRecordField::EntryCode { entry: 0 }, FlowLinkRecordValue::Byte(31))?`.
    pub fn replace_field(
        &self,
        field: FlowLinkRecordField,
        value: FlowLinkRecordValue,
    ) -> Result<FlowLinkRecordEdit> {
        let (offset, width) = select::offset(self, field)?;
        let bytes = field_bytes(field, value, width)?;
        let mut decoded = self.wire.decoded.to_vec();
        decoded[offset..offset + bytes.len()].copy_from_slice(&bytes);
        let carrier_edit = wire::edited(&self.wire, &decoded)?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit.result().record_402_links(&id, self.options)?;
        check_structure(self, &result, field, value)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowLinkRecordEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}

fn field_bytes(
    field: FlowLinkRecordField,
    value: FlowLinkRecordValue,
    width: usize,
) -> Result<Vec<u8>> {
    match (width, value) {
        (1, FlowLinkRecordValue::Byte(v)) => Ok(vec![v]),
        (4, FlowLinkRecordValue::Signed32(v)) => Ok(v.to_le_bytes().to_vec()),
        (8, FlowLinkRecordValue::WideBits(v)) => Ok(v.to_le_bytes().to_vec()),
        _ => Err(link_invalid(format!(
            "field {field:?} value {value:?}; expected exact {width}-byte Byte/Signed32/WideBits variant"
        ))),
    }
}

fn check_structure(
    before: &FlowLinkRecords,
    after: &FlowLinkRecords,
    field: FlowLinkRecordField,
    value: FlowLinkRecordValue,
) -> Result<()> {
    if before.entries == after.entries && before.tail == after.tail {
        return Ok(());
    }
    Err(link_invalid(format!(
        "field {field:?} value {value:?} changed parsed intervals; expected unchanged entry and tail structure"
    )))
}
