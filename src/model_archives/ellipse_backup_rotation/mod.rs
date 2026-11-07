//! Full-width saved ellipse-backup rotation snapshots (C-3370/C-3372 / #192).
mod projection;

use super::{
    ModelFieldHandle, ModelFieldValue, PartResolvedFeaturesArchive, PartResolvedFeaturesEdit,
};
use crate::Result;

/// Immutable saved rotation snapshot with physical owner and word revision identities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelEllipseBackupRotationState {
    owner: ModelFieldHandle,
    field: ModelFieldHandle,
    snapshot: i32,
}

impl ModelEllipseBackupRotationState {
    /// Copies every stored bit without assigning a convention; e.g. `let raw = state.rotation_direction_snapshot();`.
    pub fn rotation_direction_snapshot(&self) -> i32 {
        self.snapshot
    }

    /// Identifies the physical ellipse definition; e.g. `source.ellipse_backup_rotation_state(state.owner_field())?`.
    pub fn owner_field(&self) -> &ModelFieldHandle {
        &self.owner
    }

    /// Identifies the stored I32 word; e.g. `source.resolve_field(state.source_field())?`.
    pub fn source_field(&self) -> &ModelFieldHandle {
        &self.field
    }
}

impl PartResolvedFeaturesArchive {
    /// Lists physical stored ellipse snapshots in wire order; e.g. `let states = source.ellipse_backup_rotation_states()?;`.
    pub fn ellipse_backup_rotation_states(&self) -> Result<Vec<ModelEllipseBackupRotationState>> {
        self.fields()
            .iter()
            .enumerate()
            .filter(|(_, field)| projection::ellipse_candidate(field))
            .map(|(index, _)| self.ellipse_backup_rotation_state(&self.field_handle(index)?))
            .collect()
    }

    /// Requires a physical canonical ellipse owner; e.g. `let state = source.ellipse_backup_rotation_state(&owner)?;`.
    pub fn ellipse_backup_rotation_state(
        &self,
        owner: &ModelFieldHandle,
    ) -> Result<ModelEllipseBackupRotationState> {
        let definition = self.resolve_field(owner)?;
        projection::project_rotation(self, owner, definition).map_err(|error| {
            super::resolved_failure::selected_feature_error(error, self.configuration())
        })
    }

    /// Replaces only the saved snapshot without restoring geometry; e.g. `source.edit_ellipse_backup_rotation(&state, i32::MIN)?;`.
    pub fn edit_ellipse_backup_rotation(
        &self,
        state: &ModelEllipseBackupRotationState,
        snapshot: i32,
    ) -> Result<PartResolvedFeaturesEdit> {
        self.resolve_field(state.owner_field())?;
        self.resolve_field(state.source_field())?;
        let mut fields = self.fields().to_vec();
        let selected = fields.get_mut(state.field.index()).ok_or_else(|| {
            projection::rotation_error("saved ellipse snapshot", state.field.index())
        })?;
        selected.value = ModelFieldValue::I32(snapshot);
        self.edit_fields(fields)
    }
}
