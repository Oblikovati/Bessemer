use super::*;

impl FlowLoadState {
    /// Replaces one existing nonstructural element on a copy; e.g. `let edit = state.replace_field(&id, 0, FlowLoadValue::Utf16Unit(0))?`.
    pub fn replace_field(
        &self,
        id: &FlowLoadFieldId,
        index: usize,
        value: FlowLoadValue,
    ) -> Result<FlowLoadStateEdit> {
        let field = self.field(id)?;
        let offset = editable_offset(field, index)?;
        let bytes = encoded_value(field.encoding(), value)?;
        let mut decoded = self.wire.decoded.to_vec();
        decoded[offset..offset + bytes.len()].copy_from_slice(&bytes);
        let carrier_edit = wire::edited(&self.wire, &decoded)?;
        let record = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .record_11_load_state(&record, self.options)?;
        check_structure(self, &result, id, value)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowLoadStateEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}

fn editable_offset(field: FlowLoadField<'_>, index: usize) -> Result<usize> {
    if !field.is_editable() {
        return Err(load_invalid(format!(
            "structural field {:?}; expected existing nonstructural field",
            field.kind()
        )));
    }
    field
        .element_offset(index)
        .ok_or_else(|| table_index_error(11, "field element", index, field.layout().element_count))
}

fn encoded_value(encoding: FlowLoadEncoding, value: FlowLoadValue) -> Result<Vec<u8>> {
    match (encoding, value) {
        (FlowLoadEncoding::Scalar(encoding), FlowLoadValue::Scalar(value)) => {
            wire::scalar_bytes(11, encoding, value)
        }
        (FlowLoadEncoding::Utf16Unit, FlowLoadValue::Utf16Unit(value)) => {
            Ok(value.to_le_bytes().to_vec())
        }
        _ => Err(load_invalid(format!(
            "field value {value:?}; expected {encoding:?} encoding"
        ))),
    }
}

fn check_structure(
    before: &FlowLoadState,
    after: &FlowLoadState,
    id: &FlowLoadFieldId,
    value: FlowLoadValue,
) -> Result<()> {
    if before.fields == after.fields && before.scopes == after.scopes {
        return Ok(());
    }
    Err(load_invalid(format!(
        "field {id:?} value {value:?} changed structure; expected identical complete fields and scopes"
    )))
}
