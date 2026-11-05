use super::*;

impl ModelParameterKind {
    pub(super) fn from_field(field: &ModelField) -> Option<Self> {
        let ModelFieldValue::ObjectTag(tag) = &field.value else {
            return None;
        };
        let ModelObjectRecord::New { class, .. } = &tag.record else {
            return None;
        };
        Some(match class.name.as_str() {
            "moDensityParameter_c" => Self::Density,
            "moLengthParameter_c" => Self::Length,
            "moAngleParameter_c" => Self::Angle,
            "moIntegerParameter_c" => Self::Integer,
            "moScalerParameter_c" => Self::AttributeScalar,
            "moStringParameter_c" => Self::AttributeString,
            _ => return None,
        })
    }

    fn roles(self) -> (&'static str, &'static str, &'static str, &'static str) {
        match self {
            Self::AttributeScalar | Self::AttributeString => (
                ".name_40",
                ".raw_value_50",
                ".override_presence_60",
                ".raw_override_58",
            ),
            _ => (
                ".parameter_name",
                ".value",
                ".has_alternate_value",
                ".alternate_value",
            ),
        }
    }
}

pub(super) fn enumerate(source: ModelParameterSource) -> Result<Vec<ModelParameterStoredState>> {
    source
        .fields()
        .iter()
        .enumerate()
        .filter(|(_, field)| ModelParameterKind::from_field(field).is_some())
        .map(|(index, _)| project(source.clone(), &source.handle(index)?))
        .collect()
}

pub(super) fn project(
    source: ModelParameterSource,
    owner: &ModelFieldHandle,
) -> Result<ModelParameterStoredState> {
    let field = source
        .fields()
        .get(owner.index())
        .ok_or_else(|| parameter_error(owner))?;
    let kind = ModelParameterKind::from_field(field).ok_or_else(|| parameter_error(field))?;
    let prefix = field.path.as_ref();
    let (name_role, primary_role, presence_role, alternate_role) = kind.roles();
    let names = [
        names::project(&source, prefix, ".node.name")?,
        names::project(&source, prefix, name_role)?,
    ];
    let values = ParameterValues {
        primary: value_at(&source, prefix, primary_role, kind)?,
        presence: presence_at(&source, prefix, presence_role, kind)?,
    };
    let alternate = alternate_at(&source, prefix, alternate_role, kind, values.presence.1)?;
    let copies = copy_relations(&source, prefix, &names)?;
    Ok(parameter_snapshot(
        source, owner, kind, names, values, alternate, copies,
    ))
}

struct ParameterValues {
    primary: (ModelFieldHandle, ModelParameterValue),
    presence: (Option<ModelFieldHandle>, Option<u8>),
}

fn parameter_snapshot(
    source: ModelParameterSource,
    owner: &ModelFieldHandle,
    kind: ModelParameterKind,
    names: [ModelParameterName; 2],
    values: ParameterValues,
    alternate: Option<(ModelFieldHandle, ModelParameterValue)>,
    copies: Vec<ModelParameterFieldCopy>,
) -> ModelParameterStoredState {
    ModelParameterStoredState {
        source,
        kind,
        names,
        primary: values.primary.1,
        primary_field: values.primary.0,
        presence: values.presence.1,
        presence_field: values.presence.0,
        alternate_field: alternate.as_ref().map(|(handle, _)| handle.clone()),
        alternate: alternate.map(|(_, value)| value),
        owner: owner.clone(),
        copies: copies.into(),
    }
}

pub(super) fn field_at(source: &ModelParameterSource, prefix: &str, suffix: &str) -> Result<usize> {
    let mut matches = source
        .fields()
        .iter()
        .enumerate()
        .filter(|(_, field)| field.path.strip_prefix(prefix) == Some(suffix));
    let (index, _) = matches
        .next()
        .ok_or_else(|| parameter_error((prefix, suffix, "missing")))?;
    if matches.next().is_some() {
        return Err(parameter_error((
            prefix,
            suffix,
            "duplicate physical field",
        )));
    }
    Ok(index)
}

fn value_at(
    source: &ModelParameterSource,
    prefix: &str,
    suffix: &str,
    kind: ModelParameterKind,
) -> Result<(ModelFieldHandle, ModelParameterValue)> {
    let index = field_at(source, prefix, suffix)?;
    let stored = &source.fields()[index].value;
    let value = match (kind, stored) {
        (ModelParameterKind::Integer, ModelFieldValue::I32(value)) => {
            ModelParameterValue::Integer(*value)
        }
        (ModelParameterKind::AttributeString, ModelFieldValue::Text(value)) => {
            ModelParameterValue::Text(value.clone())
        }
        (kind, ModelFieldValue::F64(value))
            if kind != ModelParameterKind::Integer
                && kind != ModelParameterKind::AttributeString =>
        {
            ModelParameterValue::Double(*value)
        }
        _ => return Err(parameter_error((kind, prefix, suffix, stored))),
    };
    Ok((source.handle(index)?, value))
}

fn presence_at(
    source: &ModelParameterSource,
    prefix: &str,
    suffix: &str,
    kind: ModelParameterKind,
) -> Result<(Option<ModelFieldHandle>, Option<u8>)> {
    if kind == ModelParameterKind::AttributeString {
        return Ok((None, None));
    }
    let index = field_at(source, prefix, suffix)?;
    let ModelFieldValue::U8(value) = source.fields()[index].value else {
        return Err(parameter_error(&source.fields()[index]));
    };
    Ok((Some(source.handle(index)?), Some(value)))
}

fn alternate_at(
    source: &ModelParameterSource,
    prefix: &str,
    suffix: &str,
    kind: ModelParameterKind,
    presence: Option<u8>,
) -> Result<Option<(ModelFieldHandle, ModelParameterValue)>> {
    if presence.is_none_or(|value| value == 0) {
        return Ok(None);
    }
    value_at(source, prefix, suffix, kind).map(Some)
}

fn copy_relations(
    source: &ModelParameterSource,
    prefix: &str,
    names: &[ModelParameterName; 2],
) -> Result<Vec<ModelParameterFieldCopy>> {
    let mut handles = scoped_handles(source, prefix)?;
    for handle in names.iter().filter_map(|name| name.text_field.as_ref()) {
        if !handles.contains(handle) {
            handles.push(handle.clone());
        }
    }
    Ok(handles
        .into_iter()
        .map(|handle| ModelParameterFieldCopy {
            parents: [handle.clone()],
            result: handle,
        })
        .collect())
}

fn scoped_handles(source: &ModelParameterSource, prefix: &str) -> Result<Vec<ModelFieldHandle>> {
    source
        .fields()
        .iter()
        .enumerate()
        .filter(|(_, field)| {
            field.path.as_ref() == prefix
                || field
                    .path
                    .strip_prefix(prefix)
                    .is_some_and(|suffix| suffix.starts_with('.'))
        })
        .map(|(index, _)| source.handle(index))
        .collect()
}
