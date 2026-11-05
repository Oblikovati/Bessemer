use super::{ModelBitmapAppearanceState, ModelBitmapAppearanceWords};
use crate::model_archives::{ModelFieldHandle, ModelFieldValue, PartResolvedFeaturesArchive};
use crate::{BessemerError, FailureClass, Result};

pub(super) fn appearance_error(actual: impl std::fmt::Debug) -> BessemerError {
    BessemerError::new(
        "saved bitmap appearance",
        "canonical bitmap settings",
        FailureClass::Invalid,
        format!(
            "{actual:?}; expected canonical schema1 bitmap I32 raw64, U32 raw68 and optional version-gated I32 raw80; ink edits require a stored raw80 field"
        ),
    )
}

fn signed_appearance(
    source: &PartResolvedFeaturesArchive,
    field: &ModelFieldHandle,
) -> Result<i32> {
    match &source.resolve_field(field)?.value {
        ModelFieldValue::I32(value) => Ok(*value),
        actual => Err(appearance_error(actual)),
    }
}

fn transparent_color(
    source: &PartResolvedFeaturesArchive,
    field: &ModelFieldHandle,
) -> Result<u32> {
    match &source.resolve_field(field)?.value {
        ModelFieldValue::U32(value) => Ok(*value),
        actual => Err(appearance_error(actual)),
    }
}

fn ink_field(
    source: &PartResolvedFeaturesArchive,
    index: usize,
) -> Result<Option<ModelFieldHandle>> {
    let selected = source
        .fields()
        .get(index)
        .ok_or_else(|| appearance_error(index))?;
    let prefix = selected
        .path
        .strip_suffix(".raw_70")
        .ok_or_else(|| appearance_error(&selected.path))?;
    let scope = format!("{prefix}.");
    let expected = format!("{scope}raw_80");
    source.fields()[index..]
        .iter()
        .enumerate()
        .take_while(|(_, field)| field.path.starts_with(&scope))
        .find(|(_, field)| field.path.as_ref() == expected)
        .map(|(offset, _)| source.field_handle(index + offset))
        .transpose()
}

pub(super) fn project(
    source: &PartResolvedFeaturesArchive,
    leading: &ModelFieldHandle,
) -> Result<ModelBitmapAppearanceState> {
    source.bitmap_transparency_state(leading)?;
    let mut source_fields = vec![
        source.field_handle(leading.index() + 2)?,
        source.field_handle(leading.index() + 3)?,
    ];
    let ink = ink_field(source, leading.index())?;
    let words = ModelBitmapAppearanceWords {
        matching_color_percentage: signed_appearance(source, &source_fields[0])?,
        transparent_color: transparent_color(source, &source_fields[1])?,
        ink_markup_texture: ink
            .as_ref()
            .map(|field| signed_appearance(source, field))
            .transpose()?,
    };
    source_fields.extend(ink);
    Ok(ModelBitmapAppearanceState {
        words,
        source_fields,
    })
}
