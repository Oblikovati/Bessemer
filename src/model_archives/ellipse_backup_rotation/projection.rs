use super::ModelEllipseBackupRotationState;
use crate::archive_codec::ObjectRecord;
use crate::model_archives::{
    ModelField, ModelFieldHandle, ModelFieldValue, PartResolvedFeaturesArchive,
};
use crate::{BessemerError, FailureClass, Result};

const ELLIPSE_CLASS: &str = "moEllipseBackedUpData_c";

pub(super) fn rotation_error(path: &str, actual: impl std::fmt::Debug) -> BessemerError {
    BessemerError::new(
        "saved ellipse backup rotation",
        path,
        FailureClass::Invalid,
        format!(
            "{actual:?} at {path:?}; expected a physical schema1 moEllipseBackedUpData_c definition and its direct I32 word_20 snapshot"
        ),
    )
}

pub(super) fn ellipse_candidate(field: &ModelField) -> bool {
    matches!(&field.value, ModelFieldValue::ObjectTag(tag)
        if matches!(&tag.record, ObjectRecord::New { class, .. } if class.name == ELLIPSE_CLASS))
}

fn require_ellipse_definition(field: &ModelField) -> Result<()> {
    let valid = matches!(&field.value, ModelFieldValue::ObjectTag(tag)
        if matches!(&tag.record, ObjectRecord::New { class, .. }
            if class.name == ELLIPSE_CLASS && class.schema == 1));
    if valid {
        return Ok(());
    }
    Err(rotation_error(&field.path, &field.value))
}

fn rotation_word_index(fields: &[ModelField], owner: usize, prefix: &str) -> Result<usize> {
    let scope = format!("{prefix}.");
    let expected = format!("{scope}word_20");
    fields
        .iter()
        .enumerate()
        .skip(owner + 1)
        .take_while(|(_, field)| field.path.starts_with(&scope))
        .find(|(_, field)| field.path.as_ref() == expected)
        .map(|(index, _)| index)
        .ok_or_else(|| rotation_error(prefix, "missing direct word_20"))
}

pub(super) fn project_rotation(
    source: &PartResolvedFeaturesArchive,
    owner: &ModelFieldHandle,
    definition: &ModelField,
) -> Result<ModelEllipseBackupRotationState> {
    require_ellipse_definition(definition)?;
    let index = rotation_word_index(source.fields(), owner.index(), &definition.path)?;
    let field = source.field_handle(index)?;
    let stored = source.resolve_field(&field)?;
    let snapshot = match &stored.value {
        ModelFieldValue::I32(value) => *value,
        actual => return Err(rotation_error(&stored.path, actual)),
    };
    Ok(ModelEllipseBackupRotationState {
        owner: owner.clone(),
        field,
        snapshot,
    })
}
