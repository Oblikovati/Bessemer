use super::{FieldDefinition, FieldKind, FieldValues, invalid};
use crate::Result;

pub(super) fn validate(
    field: &FieldDefinition,
    values: &FieldValues,
    length: Option<u32>,
    delta: bool,
) -> Result<()> {
    let count = super::field_decode::element_count(field, length)?;
    let actual = matching_count(field.kind, values, delta).ok_or_else(|| {
        invalid(format!(
            "field {:?} value variant {:?}; expected {:?} with delta={delta}",
            field.name,
            value_shape(values),
            field.kind
        ))
    })?;
    if actual != count {
        return Err(invalid(format!(
            "field {:?} element count {actual}; expected {count}",
            field.name
        )));
    }
    require_pointer_flags(values, delta)
}

fn require_pointer_flags(values: &FieldValues, delta: bool) -> Result<()> {
    if let FieldValues::Pointers(pointers) = values
        && let Some(pointer) = pointers
            .iter()
            .find(|pointer| pointer.delta_flag.is_some() != delta)
    {
        return Err(invalid(format!(
            "pointer flag {:?}; expected flag presence {delta}",
            pointer.delta_flag
        )));
    }
    Ok(())
}

fn matching_count(kind: FieldKind, values: &FieldValues, delta: bool) -> Option<usize> {
    match (kind, values) {
        (FieldKind::Integer, FieldValues::Integers(values)) => Some(values.len()),
        (FieldKind::Pointer, FieldValues::Pointers(values)) => Some(values.len()),
        (
            FieldKind::UnsignedByte | FieldKind::Character | FieldKind::Logical,
            FieldValues::Bytes(values),
        ) => Some(values.len()),
        (FieldKind::Short, FieldValues::Shorts(values)) => Some(values.len()),
        (FieldKind::WideUnit, FieldValues::WideUnits(values)) => Some(values.len()),
        (FieldKind::Real, FieldValues::Reals(values)) => Some(values.len()),
        (FieldKind::Interval, FieldValues::Intervals(values)) => Some(values.len()),
        (FieldKind::Vector, FieldValues::Vectors(values)) => Some(values.len()),
        (FieldKind::HVector, FieldValues::PartitionHVectors(values)) if !delta => {
            Some(values.len())
        }
        (FieldKind::HVector, FieldValues::DeltaHVectors(values)) if delta => Some(values.len()),
        (FieldKind::Box, FieldValues::Boxes(values)) => Some(values.len()),
        _ => None,
    }
}

fn value_shape(values: &FieldValues) -> &'static str {
    match values {
        FieldValues::Integers(_) => "Integers",
        FieldValues::Pointers(_) => "Pointers",
        FieldValues::Bytes(_) => "Bytes",
        FieldValues::Shorts(_) => "Shorts",
        FieldValues::WideUnits(_) => "WideUnits",
        FieldValues::Reals(_) => "Reals",
        FieldValues::Intervals(_) => "Intervals",
        FieldValues::Vectors(_) => "Vectors",
        FieldValues::PartitionHVectors(_) => "PartitionHVectors",
        FieldValues::DeltaHVectors(_) => "DeltaHVectors",
        FieldValues::Boxes(_) => "Boxes",
    }
}
