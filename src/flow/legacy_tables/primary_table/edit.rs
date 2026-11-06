use super::*;

impl FlowPrimaryTable {
    /// Patches an existing element on an isolated copy; e.g. `let edit = table.replace_field(&id, 0, FlowPrimaryValue::WordBits(7))?`.
    pub fn replace_field(
        &self,
        id: &FlowPrimaryFieldId,
        index: usize,
        value: FlowPrimaryValue,
    ) -> Result<FlowPrimaryTableEdit> {
        let field = self.field(id)?;
        let offset = editable_offset(field, index)?;
        let bytes = replacement_bytes(field.encoding(), value)?;
        let mut decoded = self.wire.decoded.to_vec();
        decoded[offset..offset + bytes.len()].copy_from_slice(&bytes);
        let carrier_edit = wire::edited(&self.wire, &decoded)?;
        let record = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .record_439_primary_table(&record, self.options)?;
        check_structure(self, &result, id, value)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowPrimaryTableEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}

fn editable_offset(field: FlowPrimaryField<'_>, index: usize) -> Result<usize> {
    if !field.is_editable() {
        return Err(primary_invalid(format!(
            "structural field {:?}; expected an existing nonstructural field",
            field.kind()
        )));
    }
    field
        .element_offset(index)
        .ok_or_else(|| table_index_error(439, "field element", index, field.layout().element_count))
}

fn replacement_bytes(encoding: FlowPrimaryEncoding, value: FlowPrimaryValue) -> Result<Vec<u8>> {
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
        _ => Err(primary_invalid(format!(
            "field value {value:?}; expected exact {encoding:?} encoding"
        ))),
    }
}

fn check_structure(
    before: &FlowPrimaryTable,
    after: &FlowPrimaryTable,
    field: &FlowPrimaryFieldId,
    value: FlowPrimaryValue,
) -> Result<()> {
    if before.scopes == after.scopes && before.fields == after.fields {
        return Ok(());
    }
    Err(primary_invalid(format!(
        "field {field:?} value {value:?} changed structure; expected identical complete scopes and field layouts"
    )))
}
