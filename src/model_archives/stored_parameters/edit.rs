use super::*;

impl PartConfigurationArchive {
    /// Replace one stored parameter value through complete archive validation; e.g. `source.edit_stored_parameter(&state, edit)?`.
    pub fn edit_stored_parameter(
        &self,
        state: &ModelParameterStoredState,
        edit: ModelParameterStoredEdit,
    ) -> Result<PartConfigurationEdit> {
        self.resolve_field(&state.owner)?;
        self.resolve_field(&state.primary_field)?;
        self.edit_fields(replacement_fields(self.fields(), state, edit)?)
    }
}

impl PartResolvedFeaturesArchive {
    /// Replace one stored value without evaluating native setter effects; e.g. `source.edit_stored_parameter(&state, edit)?`.
    pub fn edit_stored_parameter(
        &self,
        state: &ModelParameterStoredState,
        edit: ModelParameterStoredEdit,
    ) -> Result<PartResolvedFeaturesEdit> {
        self.resolve_field(&state.owner)?;
        self.resolve_field(&state.primary_field)?;
        self.edit_fields(replacement_fields(self.fields(), state, edit)?)
    }
}

fn replacement_fields(
    fields: &[ModelField],
    state: &ModelParameterStoredState,
    edit: ModelParameterStoredEdit,
) -> Result<Vec<ModelField>> {
    let (selected, replacement) = selected_replacement(state, edit)?;
    let mut replacement_fields = fields.to_vec();
    let field = replacement_fields
        .get_mut(selected.index())
        .ok_or_else(|| parameter_error(selected))?;
    field.value = replacement;
    Ok(replacement_fields)
}

fn selected_replacement(
    state: &ModelParameterStoredState,
    edit: ModelParameterStoredEdit,
) -> Result<(&ModelFieldHandle, ModelFieldValue)> {
    match edit {
        ModelParameterStoredEdit::Primary(value) => {
            Ok((&state.primary_field, same_type(&state.primary, value)?))
        }
        ModelParameterStoredEdit::Alternate(value) => {
            let field = state
                .alternate_field
                .as_ref()
                .ok_or_else(|| parameter_error("absent alternate value"))?;
            let before = state
                .alternate
                .as_ref()
                .ok_or_else(|| parameter_error("absent alternate value"))?;
            Ok((field, same_type(before, value)?))
        }
        ModelParameterStoredEdit::AlternatePresence(value) => presence_replacement(state, value),
    }
}

fn presence_replacement(
    state: &ModelParameterStoredState,
    value: u8,
) -> Result<(&ModelFieldHandle, ModelFieldValue)> {
    let field = state
        .presence_field
        .as_ref()
        .ok_or_else(|| parameter_error((state.kind, value, "no presence byte")))?;
    if (value != 0) != state.alternate.is_some() {
        return Err(parameter_error((
            state.kind,
            value,
            "presence edit would change the stored field shape",
        )));
    }
    Ok((field, ModelFieldValue::U8(value)))
}

fn same_type(before: &ModelParameterValue, after: ModelParameterValue) -> Result<ModelFieldValue> {
    match (before, after) {
        (ModelParameterValue::Double(_), ModelParameterValue::Double(value)) => {
            Ok(ModelFieldValue::F64(value))
        }
        (ModelParameterValue::Integer(_), ModelParameterValue::Integer(value)) => {
            Ok(ModelFieldValue::I32(value))
        }
        (ModelParameterValue::Text(_), ModelParameterValue::Text(value)) => {
            Ok(ModelFieldValue::Text(value))
        }
        (before, after) => Err(parameter_error((
            before,
            after,
            "different stored value types",
        ))),
    }
}
