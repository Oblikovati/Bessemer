use super::{ModelField, ModelFieldValue, encode_fields, invalid};
use crate::Result;

pub(super) fn section_lengths(fields: &mut [ModelField], maximum_bytes: usize) -> Result<()> {
    let lightweight = section_index(fields, "root.lightweight_size")?;
    let resolved = section_index(fields, "root.resolved_size")?;
    if lightweight >= resolved {
        return Err(invalid(format!(
            "section indices {lightweight}/{resolved}; expected lightweight before resolved"
        )));
    }
    let lightweight_bytes = encode_fields(&fields[lightweight + 1..resolved], maximum_bytes)?.len();
    let resolved_bytes = encode_fields(&fields[resolved + 1..], maximum_bytes)?.len();
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
