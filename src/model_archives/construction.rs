use super::{ModelField, ModelFieldValue, encode_fields, invalid};
use crate::Result;

pub(super) fn section_lengths(fields: &mut [ModelField], maximum_bytes: usize) -> Result<()> {
    embedded_section_lengths(fields, maximum_bytes)?;
    let lightweight = section_index(fields, "root.lightweight_size")?;
    let resolved = section_index(fields, "root.resolved_size")?;
    rewrite_section_pair(fields, lightweight, resolved, fields.len(), maximum_bytes)
}

/// Derive inner section lengths before their containing parts, e.g. `embedded_section_lengths(fields, limit)?`.
pub(super) fn embedded_section_lengths(
    fields: &mut [ModelField],
    maximum_bytes: usize,
) -> Result<()> {
    let starts: Vec<usize> = fields
        .iter()
        .enumerate()
        .filter(|(_, field)| field.path.ends_with(".part_08.lightweight_size"))
        .map(|(index, _)| index)
        .collect();
    for start in starts.into_iter().rev() {
        embedded_section_pair(fields, start, maximum_bytes)?;
    }
    Ok(())
}

fn embedded_section_pair(
    fields: &mut [ModelField],
    start: usize,
    maximum_bytes: usize,
) -> Result<()> {
    let prefix = fields[start]
        .path
        .trim_end_matches("lightweight_size")
        .to_owned();
    let lightweight = section_index(fields, &format!("{prefix}lightweight_size"))?;
    let resolved = section_index(fields, &format!("{prefix}resolved_size"))?;
    let end = fields
        .iter()
        .enumerate()
        .skip(resolved + 1)
        .find(|(_, field)| !field.path.starts_with(&prefix))
        .map_or(fields.len(), |(index, _)| index);
    rewrite_section_pair(fields, lightweight, resolved, end, maximum_bytes)
}

fn rewrite_section_pair(
    fields: &mut [ModelField],
    lightweight: usize,
    resolved: usize,
    end: usize,
    maximum_bytes: usize,
) -> Result<()> {
    if lightweight >= resolved || resolved >= end || end > fields.len() {
        return Err(invalid(format!(
            "section indices {lightweight}/{resolved}/{end}; expected lightweight < resolved < end <= {}",
            fields.len()
        )));
    }
    let lightweight_bytes = encode_fields(&fields[lightweight + 1..resolved], maximum_bytes)?.len();
    let resolved_bytes = encode_fields(&fields[resolved + 1..end], maximum_bytes)?.len();
    fields[lightweight].value = ModelFieldValue::I32(signed_length(lightweight_bytes)?);
    fields[resolved].value = ModelFieldValue::I32(signed_length(resolved_bytes)?);
    Ok(())
}

fn section_index(fields: &[ModelField], label: &str) -> Result<usize> {
    let matches: Vec<_> = fields
        .iter()
        .enumerate()
        .filter(|(_, field)| field.path.as_ref() == label)
        .collect();
    let [(index, field)] = matches.as_slice() else {
        return Err(invalid(format!(
            "section field {label:?} has {} matches; expected exactly one",
            matches.len()
        )));
    };
    if !matches!(field.value, ModelFieldValue::I32(_)) {
        return Err(invalid(format!(
            "section field {label:?} value {:?}; expected i32 derived length",
            field.value
        )));
    }
    Ok(*index)
}

fn signed_length(length: usize) -> Result<i32> {
    i32::try_from(length).map_err(|_| {
        invalid(format!(
            "section byte length {length}; expected <= {}",
            i32::MAX
        ))
    })
}
