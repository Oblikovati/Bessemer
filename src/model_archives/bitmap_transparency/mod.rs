//! Revision-owned saved bitmap settings, qualified by C-3311 / #187.
mod projection;
mod types;
pub use types::{
    ModelBitmapTransparencyEdit, ModelBitmapTransparencyState, ModelBitmapTransparencyWords,
};

use super::{ModelFieldValue, PartResolvedFeaturesArchive, PartResolvedFeaturesEdit};
use crate::Result;

impl PartResolvedFeaturesArchive {
    /// Lists physical saved bitmap settings in wire order; e.g. `let settings = source.bitmap_transparency_states()?;`.
    pub fn bitmap_transparency_states(&self) -> Result<Vec<ModelBitmapTransparencyState>> {
        self.fields()
            .iter()
            .enumerate()
            .filter(|(_, field)| field.path.ends_with(".raw_70"))
            .filter(|(index, _)| projection::bitmap_candidate(self.fields(), *index))
            .map(|(index, _)| self.bitmap_transparency_state(&self.field_handle(index)?))
            .collect()
    }

    /// Binds the leading raw70 word to its complete schema1 bitmap owner; e.g. `let setting = source.bitmap_transparency_state(&field)?;`.
    pub fn bitmap_transparency_state(
        &self,
        field: &super::ModelFieldHandle,
    ) -> Result<ModelBitmapTransparencyState> {
        self.resolve_field(field)?;
        projection::project(self, field).map_err(|error| {
            super::resolved_failure::selected_feature_error(error, self.configuration())
        })
    }

    /// Replaces one full signed setting without refreshing runtime caches; e.g. `source.edit_bitmap_transparency(&setting, ModelBitmapTransparencyEdit::LevelParameter(-1))?;`.
    pub fn edit_bitmap_transparency(
        &self,
        state: &ModelBitmapTransparencyState,
        edit: ModelBitmapTransparencyEdit,
    ) -> Result<PartResolvedFeaturesEdit> {
        for field in state.source_fields() {
            self.resolve_field(field)?;
        }
        let (position, value) = edit.replacement();
        let selected = &state.source_fields()[position];
        let mut fields = self.fields().to_vec();
        let field = fields
            .get_mut(selected.index())
            .ok_or_else(|| projection::setting_error("saved bitmap setting", selected.index()))?;
        field.value = ModelFieldValue::I32(value);
        self.edit_fields(fields)
    }
}
