use super::{ModelField, ModelFieldValue};
use crate::archive_codec::{ArchiveMapIndex, ObjectRecord};
use crate::{BessemerError, FailureClass, Result};

pub(super) fn stored_object_integer(
    fields: &[ModelField],
    record: &ObjectRecord,
    label: &str,
) -> Result<i32> {
    let index = record_index(record)?;
    let prefix = body_prefix(fields, index)?;
    integer_at_path(fields, index, &format!("{prefix}.{label}"))
}

fn record_index(record: &ObjectRecord) -> Result<ArchiveMapIndex> {
    match record {
        ObjectRecord::New { index, .. } | ObjectRecord::Reference { index, .. } => Ok(*index),
        ObjectRecord::Null => Err(super::invalid(
            "object Null; expected a nonnull decoded object",
        )),
    }
}

pub(super) fn stored_object_count(
    fields: &[ModelField],
    record: &ObjectRecord,
    label: &str,
) -> Result<usize> {
    let index = record_index(record)?;
    let prefix = body_prefix(fields, index)?;
    let path = format!("{prefix}.{label}");
    match count_field(fields, &format!("{path}.count_word")) {
        Some(ModelFieldValue::U16(u16::MAX)) => extended_count(fields, &path),
        Some(ModelFieldValue::U16(value)) => Ok(usize::from(*value)),
        actual => Err(missing_fact(
            &path,
            format!("count {actual:?}; expected decoded MFC count"),
        )),
    }
}

fn extended_count(fields: &[ModelField], path: &str) -> Result<usize> {
    match count_field(fields, &format!("{path}.count_dword")) {
        Some(ModelFieldValue::U32(value)) => Ok(*value as usize),
        actual => Err(missing_fact(
            path,
            format!("extended count {actual:?}; expected decoded u32"),
        )),
    }
}

fn count_field<'a>(fields: &'a [ModelField], path: &str) -> Option<&'a ModelFieldValue> {
    fields
        .iter()
        .find(|field| field.path.as_ref() == path)
        .map(|field| &field.value)
}

fn integer_at_path(fields: &[ModelField], index: ArchiveMapIndex, path: &str) -> Result<i32> {
    let value = fields
        .iter()
        .find(|field| field.path.as_ref() == path)
        .map(|field| &field.value);
    match value {
        Some(ModelFieldValue::I32(value)) => Ok(*value),
        actual => Err(missing_fact(
            path,
            format!(
                "object {} field {actual:?}; expected a decoded stored i32",
                index.value()
            ),
        )),
    }
}

fn body_prefix(fields: &[ModelField], index: ArchiveMapIndex) -> Result<&str> {
    fields
        .iter()
        .find_map(|field| new_object_prefix(field, index))
        .ok_or_else(|| {
            missing_fact(
                "object fields",
                format!(
                    "object identity {}; expected its completed decoded body in this operation",
                    index.value()
                ),
            )
        })
}

fn new_object_prefix(field: &ModelField, index: ArchiveMapIndex) -> Option<&str> {
    let ModelFieldValue::ObjectTag(tag) = &field.value else {
        return None;
    };
    match &tag.record {
        ObjectRecord::New {
            index: candidate, ..
        } if *candidate == index => Some(field.path.as_ref()),
        _ => None,
    }
}

fn missing_fact(entity: &str, cause: String) -> BessemerError {
    BessemerError::new("model archive", entity, FailureClass::Missing, cause)
}
