use super::{ModelBitmapTransparencyState, ModelBitmapTransparencyWords};
use crate::archive_codec::ObjectRecord;
use crate::model_archives::{
    ModelField, ModelFieldHandle, ModelFieldValue, PartResolvedFeaturesArchive,
};
use crate::{BessemerError, FailureClass, Result};

pub(super) fn setting_error(path: &str, actual: impl std::fmt::Debug) -> BessemerError {
    BessemerError::new(
        "saved bitmap transparency",
        path,
        FailureClass::Invalid,
        format!(
            "{actual:?} at {path:?}; expected a physical ordinary schema1 moSketchBitmapData_c definition with adjacent I32 raw70/raw60/raw64, U32 raw68 and I32 raw6c metadata"
        ),
    )
}

pub(super) fn bitmap_candidate(fields: &[ModelField], index: usize) -> bool {
    bitmap_definition(fields, index).is_ok_and(|(_, class)| class.name == "moSketchBitmapData_c")
}

pub(super) fn bitmap_owner(fields: &[ModelField], index: usize) -> Result<&str> {
    let (prefix, class) = bitmap_definition(fields, index)?;
    if class.name != "moSketchBitmapData_c" || class.schema != 1 {
        return Err(setting_error(prefix, class));
    }
    Ok(prefix)
}

fn bitmap_definition(
    fields: &[ModelField],
    index: usize,
) -> Result<(&str, &crate::archive_codec::ArchiveClass)> {
    let selected = fields
        .get(index)
        .ok_or_else(|| setting_error("bitmap setting", index))?;
    let prefix = setting_prefix(selected)?;
    let owner = fields[..index]
        .iter()
        .rev()
        .find(|field| field.path.as_ref() == prefix);
    match owner.map(|field| &field.value) {
        Some(ModelFieldValue::ObjectTag(tag)) => match &tag.record {
            ObjectRecord::New { class, .. } => Ok((prefix, class)),
            other => Err(setting_error(prefix, other)),
        },
        other => Err(setting_error(prefix, other)),
    }
}

fn metadata_words(fields: &[ModelField], prefix: &str) -> Result<ModelBitmapTransparencyWords> {
    let names = [".raw_70", ".raw_60", ".raw_64", ".raw_68", ".raw_6c"];
    for (index, (field, suffix)) in fields.iter().zip(names).enumerate() {
        let valid_type = if index == 3 {
            matches!(field.value, ModelFieldValue::U32(_))
        } else {
            matches!(field.value, ModelFieldValue::I32(_))
        };
        if field.path.strip_prefix(prefix) != Some(suffix) || !valid_type {
            return Err(setting_error(prefix, field));
        }
    }
    Ok(ModelBitmapTransparencyWords {
        has_transparency_setting: signed_setting(&fields[0])?,
        level_parameter: signed_setting(&fields[1])?,
        type_selector: signed_setting(&fields[4])?,
    })
}

fn signed_setting(field: &ModelField) -> Result<i32> {
    match field.value {
        ModelFieldValue::I32(value) => Ok(value),
        ref other => Err(setting_error(&field.path, other)),
    }
}

pub(super) fn project(
    source: &PartResolvedFeaturesArchive,
    selected: &ModelFieldHandle,
) -> Result<ModelBitmapTransparencyState> {
    let index = selected.index();
    let prefix = bitmap_owner(source.fields(), index)?;
    let end = index
        .checked_add(5)
        .ok_or_else(|| setting_error(prefix, index))?;
    let metadata = source
        .fields()
        .get(index..end)
        .ok_or_else(|| setting_error(prefix, index))?;
    let words = metadata_words(metadata, prefix)?;
    Ok(ModelBitmapTransparencyState {
        words,
        source_fields: [
            selected.clone(),
            source.field_handle(index + 1)?,
            source.field_handle(index + 4)?,
        ],
    })
}

fn setting_prefix(selected: &ModelField) -> Result<&str> {
    selected
        .path
        .strip_suffix(".raw_70")
        .filter(|prefix| !prefix.is_empty())
        .ok_or_else(|| setting_error(&selected.path, &selected.value))
}
