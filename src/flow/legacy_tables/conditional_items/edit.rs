use super::*;

impl FlowConditionalItems {
    /// Replaces an existing element on an isolated copy; e.g. `let edit = items.replace_field(&id, 0, FlowConditionalValue::WideBits(7))?`.
    pub fn replace_field(
        &self,
        id: &FlowConditionalFieldId,
        index: usize,
        value: FlowConditionalValue,
    ) -> Result<FlowConditionalItemEdit> {
        let field = self.field(id)?;
        let offset = editable_offset(field, index)?;
        let bytes = wire::scalar_bytes(self.format.identifier(), field.encoding(), value)?;
        let mut decoded = self.wire.decoded.to_vec();
        decoded[offset..offset + bytes.len()].copy_from_slice(&bytes);
        let carrier_edit = transport::edited(self, &decoded)?;
        let record = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = decode::read(carrier_edit.result(), &record, self.format, self.options)?;
        check_structure(self, &result, id, value)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowConditionalItemEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}

fn editable_offset(field: FlowConditionalField<'_>, index: usize) -> Result<usize> {
    let identifier = field.owner.identifier;
    if !field.is_editable() {
        return Err(table_failure(
            identifier,
            FailureClass::Invalid,
            format!(
                "structural field {:?}; expected existing nonstructural field",
                field.kind()
            ),
        ));
    }
    field.element_offset(index).ok_or_else(|| {
        table_index_error(
            identifier,
            "field element",
            index,
            field.layout().element_count,
        )
    })
}

fn check_structure(
    before: &FlowConditionalItems,
    after: &FlowConditionalItems,
    id: &FlowConditionalFieldId,
    value: FlowConditionalValue,
) -> Result<()> {
    if before.fields == after.fields && before.scopes == after.scopes {
        return Ok(());
    }
    Err(conditional_invalid(
        before.format,
        format!(
            "field {id:?} value {value:?} changed structure; expected identical complete scopes and field layouts"
        ),
    ))
}
