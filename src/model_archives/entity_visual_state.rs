use super::{
    ModelField, ModelFieldHandle, ModelFieldValue, PartResolvedFeaturesArchive,
    PartResolvedFeaturesEdit,
};
use crate::Result;
use crate::entity_visual::{
    EntityVisualStoredEdit, EntityVisualStoredWords, VisualScalar, visual_role_error,
};
use std::sync::Arc;

const VISUAL_SUFFIXES: [&str; 7] = [
    ".unknown_0",
    ".unknown_2",
    ".unknown_4",
    ".unknown_8",
    ".layer",
    ".line_width.standard_width",
    ".line_width.custom_width",
];

/// Stored visual words bound to the physical record in one immutable model revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelEntityVisualStoredState {
    words: EntityVisualStoredWords,
    first: ModelFieldHandle,
    path: Arc<str>,
}

impl ModelEntityVisualStoredState {
    /// Copies raw words without resolving effective layer state; e.g. `let raw = visual.words();`.
    pub fn words(&self) -> EntityVisualStoredWords {
        self.words
    }

    /// Borrows the revision-bound leading layer field; e.g. `source.resolve_field(visual.source_field())?`.
    pub fn source_field(&self) -> &ModelFieldHandle {
        &self.first
    }
}

fn model_visual_scalar(field: &ModelField) -> Result<VisualScalar> {
    match field.value {
        ModelFieldValue::U16(value) => Ok(VisualScalar::U16(value)),
        ModelFieldValue::U8(value) => Ok(VisualScalar::U8(value)),
        ModelFieldValue::U32(value) => Ok(VisualScalar::U32(value)),
        ModelFieldValue::I16(value) => Ok(VisualScalar::I16(value)),
        ModelFieldValue::F32(value) => Ok(VisualScalar::F32(value)),
        ref other => Err(visual_role_error(&field.path, other)),
    }
}

fn model_visual_value(scalar: VisualScalar) -> ModelFieldValue {
    match scalar {
        VisualScalar::U16(value) => ModelFieldValue::U16(value),
        VisualScalar::U8(value) => ModelFieldValue::U8(value),
        VisualScalar::U32(value) => ModelFieldValue::U32(value),
        VisualScalar::I16(value) => ModelFieldValue::I16(value),
        VisualScalar::F32(value) => ModelFieldValue::F32(value),
    }
}

fn model_visual_prefix(path: &str) -> Result<&str> {
    let Some(prefix) = path.strip_suffix(".unknown_0") else {
        return Err(visual_role_error(path, "not a leading visual field"));
    };
    if prefix.ends_with(".entity_visual_properties") || prefix.ends_with(".visual_properties") {
        return Ok(prefix);
    }
    Err(visual_role_error(
        path,
        "not a declared entity visual scope",
    ))
}

fn model_visual_words(fields: &[ModelField], prefix: &str) -> Result<EntityVisualStoredWords> {
    let mut values = Vec::new();
    for (index, suffix) in VISUAL_SUFFIXES.iter().enumerate() {
        let expected = format!("{prefix}{suffix}");
        let Some(field) = fields
            .get(index)
            .filter(|field| field.path.as_ref() == expected)
        else {
            if index == 6 {
                break;
            }
            return Err(visual_role_error(prefix, format!("missing {expected}")));
        };
        values.push(model_visual_scalar(field)?);
    }
    EntityVisualStoredWords::from_scalars(&values, prefix)
}

fn changed_model_visual_fields(
    source: &[ModelField],
    visual: &ModelEntityVisualStoredState,
    edit: EntityVisualStoredEdit,
) -> Result<Vec<ModelField>> {
    let replacements = visual.words.edited(edit, &visual.path)?.scalars();
    let mut fields = source.to_vec();
    for (relative, value) in replacements.into_iter().enumerate() {
        let field = visual
            .first
            .index()
            .checked_add(relative)
            .and_then(|index| fields.get_mut(index));
        let Some(field) = field else {
            return Err(visual_role_error(&visual.path, relative));
        };
        field.value = model_visual_value(value);
    }
    Ok(fields)
}

impl PartResolvedFeaturesArchive {
    /// Projects a complete physical visual record; e.g. `let visual = source.entity_visual_state(&field)?;`.
    pub fn entity_visual_state(
        &self,
        field: &ModelFieldHandle,
    ) -> Result<ModelEntityVisualStoredState> {
        let leading = self.resolve_field(field)?;
        let prefix = model_visual_prefix(&leading.path)?;
        let tail = self
            .fields()
            .get(field.index()..)
            .ok_or_else(|| visual_role_error(prefix, field.index()))?;
        Ok(ModelEntityVisualStoredState {
            words: model_visual_words(tail, prefix)?,
            first: field.clone(),
            path: Arc::from(prefix),
        })
    }

    /// Edits stored roles with complete validation and original field history; e.g. `source.edit_entity_visual_state(&visual, EntityVisualStoredEdit::ColorRgb([1, 2, 3]))?`.
    pub fn edit_entity_visual_state(
        &self,
        visual: &ModelEntityVisualStoredState,
        edit: EntityVisualStoredEdit,
    ) -> Result<PartResolvedFeaturesEdit> {
        self.resolve_field(&visual.first)?;
        let fields = changed_model_visual_fields(self.fields(), visual, edit)?;
        self.edit_fields(fields)
    }
}
