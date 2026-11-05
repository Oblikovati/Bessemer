//! Saved canonical sketch-reference lookup keys, without runtime resolution (#188 / C-3334).
mod projection;
mod types;
pub use types::{
    ModelSketchReferenceStoredLookupEdit, ModelSketchReferenceStoredLookupKeys,
    ModelSketchReferenceStoredLookupState,
};

use super::{
    ModelFieldHandle, ModelFieldValue, PartResolvedFeaturesArchive, PartResolvedFeaturesEdit,
};
use crate::Result;

impl PartResolvedFeaturesArchive {
    /// Lists physical canonical saved key pairs in wire order; e.g. `let keys = source.sketch_reference_lookup_states()?;`.
    pub fn sketch_reference_lookup_states(
        &self,
    ) -> Result<Vec<ModelSketchReferenceStoredLookupState>> {
        self.fields()
            .iter()
            .enumerate()
            .filter(|(_, field)| field.path.ends_with(".unknown_6c"))
            .filter(|(index, _)| projection::lookup_candidate(self.fields(), *index))
            .map(|(index, _)| self.sketch_reference_lookup_state(&self.field_handle(index)?))
            .collect()
    }

    /// Binds the leading saved6C field to its physical schema1 owner; e.g. `let state = source.sketch_reference_lookup_state(&field)?;`.
    pub fn sketch_reference_lookup_state(
        &self,
        field: &ModelFieldHandle,
    ) -> Result<ModelSketchReferenceStoredLookupState> {
        self.resolve_field(field)?;
        projection::project_lookup(self, field).map_err(|error| {
            super::resolved_failure::selected_feature_error(error, self.configuration())
        })
    }

    /// Replaces saved keys without lookup, generation or STORE projection; e.g. `source.edit_sketch_reference_lookup_keys(&state, ModelSketchReferenceStoredLookupEdit::ConfigurationObjectKey(-12345))?;`.
    pub fn edit_sketch_reference_lookup_keys(
        &self,
        state: &ModelSketchReferenceStoredLookupState,
        edit: ModelSketchReferenceStoredLookupEdit,
    ) -> Result<PartResolvedFeaturesEdit> {
        for field in state.source_fields() {
            self.resolve_field(field)?;
        }
        let replacements = edit.lookup_replacement_keys(state.keys());
        let mut fields = self.fields().to_vec();
        for (source, value) in state.source_fields().iter().zip(replacements) {
            let field = fields.get_mut(source.index()).ok_or_else(|| {
                projection::lookup_error("saved sketch-reference key", source.index())
            })?;
            field.value = ModelFieldValue::I32(value);
        }
        self.edit_fields(fields)
    }
}
