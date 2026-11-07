use super::*;
use crate::archive_codec::ArchiveString;
use crate::model_archives::{ModelField, ModelFieldHandle, ModelFieldValue};
use scope::{EquationScope, equation_error};

#[cfg(test)]
#[path = "../../../tests/model_spline_equation_state/unit_projection.rs"]
mod tests;

pub(super) fn is_equation_candidate(fields: &[ModelField], index: usize) -> bool {
    fields
        .get(index)
        .is_some_and(|field| matches!(field.value, ModelFieldValue::U32(flags) if flags & 4 != 0))
        && scope::physical_scope(fields, index).is_ok()
}

pub(super) fn project_state(
    source: &PartResolvedFeaturesArchive,
    selected: &ModelFieldHandle,
) -> Result<ModelSplineEquationStoredState> {
    let scope = scope::physical_scope(source.fields(), selected.index())?;
    let flags = equation_flags(source, selected, scope.prefix)?;
    let (first, group) = equation_group(source.fields(), &scope)?;
    let strings = equation_strings(group, scope.prefix)?;
    let compatibility = equation_i32(&group[6], scope.prefix, ".equation_word")?;
    let option = equation_i32(&group[7], scope.prefix, ".unknown_1e0")?;
    let fields = equation_handles(source, selected, first)?;
    let copies = std::array::from_fn(|index| ModelSplineEquationFieldCopy {
        parents: [fields[index].clone()],
        result: fields[index].clone(),
    });
    Ok(ModelSplineEquationStoredState {
        source: source.clone(),
        flags,
        strings,
        compatibility,
        option,
        fields,
        copies,
    })
}

fn equation_flags(
    source: &PartResolvedFeaturesArchive,
    selected: &ModelFieldHandle,
    prefix: &str,
) -> Result<u32> {
    match source.resolve_field(selected)?.value {
        ModelFieldValue::U32(flags) if flags & 4 != 0 => Ok(flags),
        ref other => Err(equation_error(prefix, other)),
    }
}

fn equation_group<'a>(
    fields: &'a [ModelField],
    scope: &EquationScope<'_>,
) -> Result<(usize, &'a [ModelField; 8])> {
    let scoped = fields
        .get(scope.range.clone())
        .ok_or_else(|| equation_error(scope.prefix, &scope.range))?;
    let first = scoped
        .iter()
        .position(|field| field.path.strip_prefix(scope.prefix) == Some(".equation_strings[0]"))
        .ok_or_else(|| equation_error(scope.prefix, "missing first equation CString"))?;
    let group = scoped
        .get(first..)
        .and_then(|tail| tail.get(..8))
        .ok_or_else(|| equation_error(scope.prefix, "incomplete equation group"))?;
    let group = group
        .try_into()
        .map_err(|_| equation_error(scope.prefix, group.len()))?;
    Ok((scope.range.start + first, group))
}

fn equation_strings(group: &[ModelField; 8], prefix: &str) -> Result<[ArchiveString; 6]> {
    let mut strings = Vec::with_capacity(6);
    for (index, field) in group[..6].iter().enumerate() {
        if field.path.strip_prefix(prefix) != Some(format!(".equation_strings[{index}]").as_str()) {
            return Err(equation_error(prefix, &field.path));
        }
        match &field.value {
            ModelFieldValue::Text(text) => strings.push(text.clone()),
            other => return Err(equation_error(prefix, other)),
        }
    }
    strings
        .try_into()
        .map_err(|values: Vec<ArchiveString>| equation_error(prefix, values.len()))
}

fn equation_i32(field: &ModelField, prefix: &str, suffix: &str) -> Result<i32> {
    if field.path.strip_prefix(prefix) != Some(suffix) {
        return Err(equation_error(prefix, &field.path));
    }
    match field.value {
        ModelFieldValue::I32(value) => Ok(value),
        ref other => Err(equation_error(prefix, other)),
    }
}

fn equation_handles(
    source: &PartResolvedFeaturesArchive,
    flags: &ModelFieldHandle,
    first: usize,
) -> Result<[ModelFieldHandle; 9]> {
    let mut handles = vec![flags.clone()];
    for offset in 0..8 {
        handles.push(source.field_handle(first + offset)?);
    }
    handles.try_into().map_err(|values: Vec<ModelFieldHandle>| {
        equation_error("equation field handles", values.len())
    })
}
