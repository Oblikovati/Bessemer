use super::{
    DisplayArchive, DisplayArchiveContents, DisplayArchiveEdit, DisplayArchiveLimits, DisplayField,
    DisplayFieldHandle, DisplayFieldValue,
};
use crate::Result;
use crate::entity_visual::{
    EntityVisualStoredEdit, EntityVisualStoredWords, VisualScalar, visual_field_words,
    visual_role_error,
};
use std::sync::Arc;

/// Physical component visual words owned by one immutable display archive revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayEntityVisualStoredState {
    words: EntityVisualStoredWords,
    first: DisplayFieldHandle,
    path: Arc<str>,
}

impl DisplayEntityVisualStoredState {
    /// Copies the complete stored words; e.g. `let raw = visual.words();`.
    pub fn words(&self) -> EntityVisualStoredWords {
        self.words
    }

    /// Borrows the original leading field identity; e.g. `source.resolve_field(visual.source_field())?`.
    pub fn source_field(&self) -> &DisplayFieldHandle {
        &self.first
    }
}

fn display_visual_scalar(field: &DisplayField) -> Result<VisualScalar> {
    match field.value {
        DisplayFieldValue::U16(value) => Ok(VisualScalar::U16(value)),
        DisplayFieldValue::U8(value) => Ok(VisualScalar::U8(value)),
        DisplayFieldValue::U32(value) => Ok(VisualScalar::U32(value)),
        DisplayFieldValue::I16(value) => Ok(VisualScalar::I16(value)),
        DisplayFieldValue::F32(value) => Ok(VisualScalar::F32(value)),
        ref other => Err(visual_role_error(&field.path, other)),
    }
}

fn display_visual_value(scalar: VisualScalar) -> DisplayFieldValue {
    match scalar {
        VisualScalar::U16(value) => DisplayFieldValue::U16(value),
        VisualScalar::U8(value) => DisplayFieldValue::U8(value),
        VisualScalar::U32(value) => DisplayFieldValue::U32(value),
        VisualScalar::I16(value) => DisplayFieldValue::I16(value),
        VisualScalar::F32(value) => DisplayFieldValue::F32(value),
    }
}

fn display_visual_prefix(path: &str) -> Result<&str> {
    let Some(prefix) = path.strip_suffix(".unknown_0") else {
        return Err(visual_role_error(path, "not a leading visual field"));
    };
    if prefix.ends_with(".ent_vis_prop") {
        return Ok(prefix);
    }
    Err(visual_role_error(
        path,
        "not a declared component visual scope",
    ))
}

fn changed_display_visual_contents(
    source: &DisplayArchive,
    visual: &DisplayEntityVisualStoredState,
    edit: EntityVisualStoredEdit,
) -> Result<DisplayArchiveContents> {
    let replacements = visual.words.edited(edit, &visual.path)?.scalars();
    let mut contents = source.contents().clone();
    for (relative, value) in replacements.into_iter().enumerate() {
        let field = visual
            .first
            .position
            .checked_add(relative)
            .and_then(|index| contents.fields.get_mut(index));
        let Some(field) = field else {
            return Err(visual_role_error(&visual.path, relative));
        };
        field.value = display_visual_value(value);
    }
    Ok(contents)
}

impl DisplayArchive {
    /// Enumerates physical visual definitions in wire order; e.g. `let visuals = source.entity_visual_states()?;`.
    /// A complete archive with no definitions returns an empty vector.
    pub fn entity_visual_states(&self) -> Result<Vec<DisplayEntityVisualStoredState>> {
        self.contents()
            .fields
            .iter()
            .enumerate()
            .filter(|(_, field)| field.path.ends_with(".ent_vis_prop.unknown_0"))
            .map(|(index, _)| self.entity_visual_state(&self.field_identity(index)))
            .collect()
    }

    /// Projects one revision-bound physical record; e.g. `source.entity_visual_state(visual.source_field())?`.
    pub fn entity_visual_state(
        &self,
        field: &DisplayFieldHandle,
    ) -> Result<DisplayEntityVisualStoredState> {
        let leading = self.resolve_field(field)?;
        let prefix = display_visual_prefix(&leading.path)?;
        let tail = self
            .contents()
            .fields
            .get(field.position..)
            .ok_or_else(|| visual_role_error(prefix, field.position))?;
        let words = visual_field_words(
            tail,
            prefix,
            |field| field.path.as_ref(),
            display_visual_scalar,
        )?;
        Ok(DisplayEntityVisualStoredState {
            words,
            first: field.clone(),
            path: Arc::from(prefix),
        })
    }

    /// Applies an isolated stored-role request with complete validation/history; e.g. `source.edit_entity_visual_state(&visual, edit, limits)?`.
    pub fn edit_entity_visual_state(
        &self,
        visual: &DisplayEntityVisualStoredState,
        edit: EntityVisualStoredEdit,
        limits: DisplayArchiveLimits,
    ) -> Result<DisplayArchiveEdit> {
        self.resolve_field(&visual.first)?;
        let contents = changed_display_visual_contents(self, visual, edit)?;
        self.edit_with_history(contents, limits)
    }
}
