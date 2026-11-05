use super::{ModelSketchReferenceStoredLookupKeys, ModelSketchReferenceStoredLookupState};
use crate::archive_codec::{ArchiveClass, ObjectRecord};
use crate::model_archives::{
    ModelField, ModelFieldHandle, ModelFieldValue, PartResolvedFeaturesArchive,
};
use crate::{BessemerError, FailureClass, Result};

pub(super) fn lookup_error(path: &str, actual: impl std::fmt::Debug) -> BessemerError {
    BessemerError::new(
        "sketch reference saved lookup keys",
        path,
        FailureClass::Invalid,
        format!(
            "{actual:?} at {path:?}; expected a physical ordinary schema1 moSketchExtRef_w definition with adjacent same-scope I32 unknown_0c/unknown_20/unknown_6c/unknown_70"
        ),
    )
}

fn lookup_prefix(field: &ModelField) -> Result<&str> {
    field
        .path
        .strip_suffix(".unknown_6c")
        .filter(|prefix| !prefix.is_empty())
        .ok_or_else(|| lookup_error(&field.path, &field.value))
}

fn lookup_definition(fields: &[ModelField], index: usize) -> Result<(&str, &ArchiveClass)> {
    let selected = fields
        .get(index)
        .ok_or_else(|| lookup_error("saved sketch key", index))?;
    let prefix = lookup_prefix(selected)?;
    let owner = fields[..index]
        .iter()
        .rev()
        .find(|field| field.path.as_ref() == prefix);
    match owner.map(|field| &field.value) {
        Some(ModelFieldValue::ObjectTag(tag)) => match &tag.record {
            ObjectRecord::New { class, .. } => Ok((prefix, class)),
            other => Err(lookup_error(prefix, other)),
        },
        other => Err(lookup_error(prefix, other)),
    }
}

pub(super) fn lookup_candidate(fields: &[ModelField], index: usize) -> bool {
    lookup_definition(fields, index).is_ok_and(|(_, class)| class.name == "moSketchExtRef_w")
}

fn admitted_lookup_owner(fields: &[ModelField], index: usize) -> Result<&str> {
    let (prefix, class) = lookup_definition(fields, index)?;
    if class.name != "moSketchExtRef_w" || class.schema != 1 {
        return Err(lookup_error(prefix, class));
    }
    Ok(prefix)
}

fn lookup_metadata(
    fields: &[ModelField],
    prefix: &str,
) -> Result<ModelSketchReferenceStoredLookupKeys> {
    for (field, suffix) in
        fields
            .iter()
            .zip([".unknown_0c", ".unknown_20", ".unknown_6c", ".unknown_70"])
    {
        if field.path.strip_prefix(prefix) != Some(suffix)
            || !matches!(field.value, ModelFieldValue::I32(_))
        {
            return Err(lookup_error(prefix, field));
        }
    }
    Ok(ModelSketchReferenceStoredLookupKeys {
        configuration_object_key: saved_lookup_word(&fields[2])?,
        configuration_record_key: saved_lookup_word(&fields[3])?,
    })
}

fn saved_lookup_word(field: &ModelField) -> Result<i32> {
    match field.value {
        ModelFieldValue::I32(value) => Ok(value),
        ref other => Err(lookup_error(&field.path, other)),
    }
}

pub(super) fn project_lookup(
    source: &PartResolvedFeaturesArchive,
    selected: &ModelFieldHandle,
) -> Result<ModelSketchReferenceStoredLookupState> {
    let index = selected.index();
    let prefix = admitted_lookup_owner(source.fields(), index)?;
    let start = index
        .checked_sub(2)
        .ok_or_else(|| lookup_error(prefix, index))?;
    let end = index
        .checked_add(2)
        .ok_or_else(|| lookup_error(prefix, index))?;
    let metadata = source
        .fields()
        .get(start..end)
        .ok_or_else(|| lookup_error(prefix, index))?;
    Ok(ModelSketchReferenceStoredLookupState {
        keys: lookup_metadata(metadata, prefix)?,
        source_fields: [selected.clone(), source.field_handle(index + 1)?],
    })
}
