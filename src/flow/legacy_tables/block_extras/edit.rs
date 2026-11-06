use super::*;

impl FlowBlockExtras {
    /// Replaces an existing nonstructural element on an isolated copy; e.g. `let edit = extras.replace_field(&id, 0, FlowBlockExtraValue::Byte(7))?`.
    pub fn replace_field(
        &self,
        id: &FlowBlockExtraFieldId,
        index: usize,
        value: FlowBlockExtraValue,
    ) -> Result<FlowBlockExtraEdit> {
        let field = self.field(id)?;
        let offset = editable_offset(field, index)?;
        let bytes = field_bytes(field.encoding(), value)?;
        let mut decoded = self.wire.decoded.to_vec();
        decoded[offset..offset + bytes.len()].copy_from_slice(&bytes);
        let carrier_edit = wire::edited(&self.wire, &decoded)?;
        let record = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .record_408_extras(&record, self.options)?;
        check_structure(self, &result, id, value)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowBlockExtraEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}

fn editable_offset(field: FlowBlockExtraField<'_>, index: usize) -> Result<usize> {
    if !field.is_editable() {
        return Err(extra_invalid(format!(
            "structural field {:?}; expected an existing nonstructural field",
            field.kind()
        )));
    }
    field
        .element_offset(index)
        .ok_or_else(|| table_index_error(408, "field element", index, field.layout().element_count))
}

fn field_bytes(encoding: FlowBlockExtraEncoding, value: FlowBlockExtraValue) -> Result<Vec<u8>> {
    match (encoding, value) {
        (FlowBlockExtraEncoding::Byte, FlowBlockExtraValue::Byte(value)) => Ok(vec![value]),
        (FlowBlockExtraEncoding::Signed32, FlowBlockExtraValue::Signed32(value)) => {
            Ok(value.to_le_bytes().to_vec())
        }
        (FlowBlockExtraEncoding::WideBits, FlowBlockExtraValue::WideBits(value)) => {
            Ok(value.to_le_bytes().to_vec())
        }
        _ => Err(extra_invalid(format!(
            "field value {value:?}; expected exact {encoding:?} encoding"
        ))),
    }
}

fn check_structure(
    before: &FlowBlockExtras,
    after: &FlowBlockExtras,
    field: &FlowBlockExtraFieldId,
    value: FlowBlockExtraValue,
) -> Result<()> {
    if before.entities == after.entities && before.fields == after.fields {
        return Ok(());
    }
    Err(extra_invalid(format!(
        "field {field:?} value {value:?} changed parsed structure; expected identical physical scopes and field layouts"
    )))
}
