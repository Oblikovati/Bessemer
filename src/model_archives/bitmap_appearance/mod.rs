//! Full-width stored bitmap color and ink settings (C-3341 / #189).
mod projection;
mod types;
pub use types::{
    ModelBitmapAppearanceEdit, ModelBitmapAppearanceState, ModelBitmapAppearanceWords,
};

use super::{ModelFieldHandle, PartResolvedFeaturesArchive, PartResolvedFeaturesEdit};
use crate::Result;

impl PartResolvedFeaturesArchive {
    /// Lists canonical saved color/ink settings; e.g. `let appearances = source.bitmap_appearance_states()?;`.
    pub fn bitmap_appearance_states(&self) -> Result<Vec<ModelBitmapAppearanceState>> {
        self.bitmap_transparency_states()?
            .iter()
            .map(|state| self.bitmap_appearance_state(&state.source_fields()[0]))
            .collect()
    }

    /// Binds the owner's leading raw70 field to its appearance settings; e.g. `source.bitmap_appearance_state(&leading)?;`.
    pub fn bitmap_appearance_state(
        &self,
        leading: &ModelFieldHandle,
    ) -> Result<ModelBitmapAppearanceState> {
        projection::project(self, leading).map_err(|error| {
            super::resolved_failure::selected_feature_error(error, self.configuration())
        })
    }

    /// Replaces one saved color/ink word without runtime refresh; e.g. `source.edit_bitmap_appearance(&state, ModelBitmapAppearanceEdit::TransparentColor(0xffff_ffff))?;`.
    pub fn edit_bitmap_appearance(
        &self,
        state: &ModelBitmapAppearanceState,
        edit: ModelBitmapAppearanceEdit,
    ) -> Result<PartResolvedFeaturesEdit> {
        for field in state.source_fields() {
            self.resolve_field(field)?;
        }
        let (role, value) = edit.replacement();
        let selected = state
            .source_fields()
            .get(role)
            .ok_or_else(|| projection::appearance_error(edit))?;
        let mut fields = self.fields().to_vec();
        let field = fields
            .get_mut(selected.index())
            .ok_or_else(|| projection::appearance_error(selected.index()))?;
        field.value = value;
        self.edit_fields(fields)
    }
}
