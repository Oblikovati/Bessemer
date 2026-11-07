use super::*;

pub(super) fn project(
    source: &ModelParameterSource,
    prefix: &str,
    suffix: &str,
) -> Result<ModelParameterName> {
    let index = projection::field_at(source, prefix, suffix)?;
    let field = &source.fields()[index];
    let ModelFieldValue::ObjectTag(tag) = &field.value else {
        return Err(parameter_error(field));
    };
    let definition = name_target(source, &tag.record)?;
    Ok(ModelParameterName {
        pointer: source.handle(index)?,
        tag: tag.as_ref().clone(),
        text: definition.as_ref().map(|(text, _)| text.clone()),
        text_field: definition.map(|(_, handle)| handle),
    })
}

fn name_target(
    source: &ModelParameterSource,
    record: &ModelObjectRecord,
) -> Result<Option<(ArchiveString, ModelFieldHandle)>> {
    let identity = match record {
        ModelObjectRecord::Null => return Ok(None),
        ModelObjectRecord::New { index, class } | ModelObjectRecord::Reference { index, class } => {
            if class.name != "moNodeName_c" {
                return Err(parameter_error(class));
            }
            *index
        }
    };
    definition_text(source, identity).map(Some)
}

fn definition_text(
    source: &ModelParameterSource,
    identity: ModelObjectId,
) -> Result<(ArchiveString, ModelFieldHandle)> {
    if let Some(index) = definition_index(source.fields(), identity)? {
        let text = text_after(source.fields(), index)?;
        return Ok((text, source.handle(index + 1)?));
    }
    let inherited = source
        .inherited()
        .ok_or_else(|| parameter_error(identity))?;
    let index =
        definition_index(inherited.fields(), identity)?.ok_or_else(|| parameter_error(identity))?;
    Ok((
        text_after(inherited.fields(), index)?,
        inherited.field_handle(index + 1)?,
    ))
}

fn definition_index(fields: &[ModelField], identity: ModelObjectId) -> Result<Option<usize>> {
    let mut definitions = fields.iter().enumerate().filter(|(_, field)| {
        matches!(&field.value, ModelFieldValue::ObjectTag(tag)
            if matches!(&tag.record, ModelObjectRecord::New { index, class, .. }
                if *index == identity && class.name == "moNodeName_c"))
    });
    let first = definitions.next().map(|(index, _)| index);
    if definitions.next().is_some() {
        return Err(parameter_error((identity, "duplicate name definition")));
    }
    Ok(first)
}

fn text_after(fields: &[ModelField], definition: usize) -> Result<ArchiveString> {
    let [owner, text] = fields
        .get(definition..)
        .and_then(|tail| tail.get(..2))
        .ok_or_else(|| parameter_error(definition))?
    else {
        return Err(parameter_error(definition));
    };
    if text.path.strip_prefix(owner.path.as_ref()) != Some(".name") {
        return Err(parameter_error(&text.path));
    }
    match &text.value {
        ModelFieldValue::Text(value) => Ok(value.clone()),
        other => Err(parameter_error(other)),
    }
}
