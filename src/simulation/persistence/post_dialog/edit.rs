use super::super::{load_edit, load_identity};
use super::*;
use crate::Result;

impl SimulationPostDialog {
    /// Bind an original outer field to this immutable revision; e.g. `property.field_id(0)?`.
    pub fn field_id(&self, ordinal: usize) -> Result<SimulationPostDialogFieldId> {
        load_identity::field_id(&self.revision, ordinal, self.fields.len(), "post dialog")
    }

    /// Resolve a field only on its owning revision; e.g. `property.field(&id)?`.
    pub fn field(&self, id: &SimulationPostDialogFieldId) -> Result<&SimulationPostDialogField> {
        load_identity::field(&self.fields, &self.revision, id, "post dialog")
    }

    /// Replace one complete typed value, preserving every other byte; e.g. `property.edit_field(&id, value)?`.
    pub fn edit_field(
        &self,
        id: &SimulationPostDialogFieldId,
        replacement: SimulationPostDialogValue,
    ) -> Result<SimulationPostDialogEdit> {
        let original = self.field(id)?;
        super::values::require_kind(&original.value, &replacement, &original.path)?;
        let wire = super::values::wire_value(&replacement);
        let archive = self.replaced_field(id.ordinal(), &wire)?;
        SimulationPostDialogEdit::from_isolated_dialog(self, archive, id, replacement, &wire)
    }

    fn replaced_field(
        &self,
        ordinal: usize,
        replacement: &super::super::thermal::SimulationThermalValue,
    ) -> Result<Self> {
        load_edit::replaced_field(self, ordinal, replacement, |fields| {
            let typed = super::values::from_wire_fields(&fields, self.version, self.limits)?;
            Self::from_fields(typed, self.version, self.limits)
        })
    }
}

pub(super) fn edit_coverage(
    archive: &SimulationPostDialog,
    changed: bool,
) -> SimulationPostDialogCoverage {
    let mut coverage = archive.coverage();
    coverage.check_mode = SimulationPostDialogCheckMode::ExactSelectedProfilesAndPreservation;
    coverage.completion = if changed {
        SimulationPostDialogCompletion::Changed
    } else {
        SimulationPostDialogCompletion::Unchanged
    };
    coverage
}
