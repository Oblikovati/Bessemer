use super::*;
use crate::archive_codec::ObjectRecord;
use crate::model_archives::{ModelField, ModelFieldValue};
use std::ops::Range;

#[cfg(test)]
#[path = "../../../tests/model_spline_equation_state/unit_scope.rs"]
mod tests;

pub(super) struct EquationScope<'a> {
    pub prefix: &'a str,
    pub range: Range<usize>,
}

pub(super) fn equation_error(path: &str, actual: impl std::fmt::Debug) -> crate::BessemerError {
    crate::BessemerError::new(
        "stored spline equation state",
        path,
        crate::FailureClass::Invalid,
        format!(
            "{actual:?} at {path:?}; expected a physical admitted spline in a new sgSketch definition with equation flag4, six same-scope CStrings and two signed words"
        ),
    )
}

pub(super) fn physical_scope(fields: &[ModelField], selected: usize) -> Result<EquationScope<'_>> {
    let field = fields
        .get(selected)
        .ok_or_else(|| equation_error("flags_32c", selected))?;
    let (sketch, ordinal, prefix) = parse_equation_scope(field)?;
    let owner = sketch_definition(fields, selected, sketch)?;
    let category = format!("{prefix}.stored_category_mask");
    let start = fields[owner..selected]
        .iter()
        .rposition(|field| field.path.as_ref() == category)
        .map(|offset| owner + offset)
        .ok_or_else(|| equation_error(prefix, "missing physical subtype"))?;
    require_equation_count(&fields[owner..start], sketch, ordinal)?;
    require_equation_category(&fields[start])?;
    let end = equation_extent_end(fields, selected, prefix)?;
    Ok(EquationScope {
        prefix,
        range: start..end,
    })
}

fn parse_equation_scope(field: &ModelField) -> Result<(&str, usize, &str)> {
    let (sketch, suffix) = field
        .path
        .rsplit_once(".splines[")
        .ok_or_else(|| equation_error(&field.path, &field.value))?;
    let raw = suffix
        .strip_suffix("].flags_32c")
        .ok_or_else(|| equation_error(&field.path, suffix))?;
    let ordinal = raw
        .parse::<usize>()
        .map_err(|_| equation_error(&field.path, raw))?;
    if sketch.is_empty() || field.path.as_ref() != format!("{sketch}.splines[{ordinal}].flags_32c")
    {
        return Err(equation_error(&field.path, raw));
    }
    let prefix = field
        .path
        .strip_suffix(".flags_32c")
        .ok_or_else(|| equation_error(&field.path, suffix))?;
    Ok((sketch, ordinal, prefix))
}

fn sketch_definition(fields: &[ModelField], selected: usize, sketch: &str) -> Result<usize> {
    let index = fields[..selected]
        .iter()
        .rposition(|field| field.path.as_ref() == sketch)
        .ok_or_else(|| equation_error(sketch, "missing preceding sketch definition"))?;
    match &fields[index].value {
        ModelFieldValue::ObjectTag(tag) => match &tag.record {
            ObjectRecord::New { class, .. } if class.name == "sgSketch" => Ok(index),
            other => Err(equation_error(sketch, other)),
        },
        other => Err(equation_error(sketch, other)),
    }
}

fn require_equation_count(fields: &[ModelField], sketch: &str, ordinal: usize) -> Result<()> {
    let path = format!("{sketch}.spline_count");
    let count = fields
        .iter()
        .rev()
        .find(|field| field.path.as_ref() == path)
        .ok_or_else(|| equation_error(sketch, "missing direct collection count"))?;
    match count.value {
        ModelFieldValue::I32(value) if value > 0 && ordinal < value as usize => Ok(()),
        ref other => Err(equation_error(sketch, other)),
    }
}

fn require_equation_category(field: &ModelField) -> Result<()> {
    let ModelFieldValue::U16(mask) = field.value else {
        return Err(equation_error(&field.path, &field.value));
    };
    super::super::sketch_splines::require_subtype(mask)
        .map(|_| ())
        .map_err(|_| equation_error(&field.path, mask))
}

fn equation_extent_end(fields: &[ModelField], selected: usize, prefix: &str) -> Result<usize> {
    let final_word = format!("{prefix}.unknown_190");
    fields[selected..]
        .iter()
        .position(|field| field.path.as_ref() == final_word)
        .map(|offset| selected + offset + 1)
        .ok_or_else(|| equation_error(prefix, "missing complete spline extent"))
}
