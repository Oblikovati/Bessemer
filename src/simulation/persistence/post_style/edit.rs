use super::super::{load_edit, load_identity};
use super::*;
use crate::Result;

impl SimulationPostStyle {
    /// Bind an original outer field to this immutable revision; e.g. `property.field_id(0)?`.
    pub fn field_id(&self, ordinal: usize) -> Result<SimulationPostStyleFieldId> {
        load_identity::field_id(&self.revision, ordinal, self.fields.len(), "PostStyle")
    }

    /// Resolve a field only on its owning revision; e.g. `property.field(&id)?`.
    pub fn field(&self, id: &SimulationPostStyleFieldId) -> Result<&SimulationPostStyleField> {
        load_identity::field(&self.fields, &self.revision, id, "PostStyle")
    }

    /// Replace one complete typed value, preserving every other byte; e.g. `property.edit_field(&id, value)?`.
    pub fn edit_field(
        &self,
        id: &SimulationPostStyleFieldId,
        replacement: SimulationPostStyleValue,
    ) -> Result<SimulationPostStyleEdit> {
        let original = self.field(id)?;
        super::values::require_kind(&original.value, &replacement, &original.path)?;
        let wire = super::values::wire_value(&replacement);
        let archive = self.replaced_field(id.ordinal(), &wire)?;
        SimulationPostStyleEdit::from_isolated_style(self, archive, id, replacement, &wire)
    }

    fn replaced_field(
        &self,
        ordinal: usize,
        replacement: &super::super::thermal::SimulationThermalValue,
    ) -> Result<Self> {
        load_edit::replaced_field(self, ordinal, replacement, |fields| {
            let typed = super::values::from_wire_fields(&fields, self.limits)?;
            Self::from_fields(typed, self.version, self.limits)
        })
    }
}

pub(super) fn edit_coverage(
    archive: &SimulationPostStyle,
    changed: bool,
) -> SimulationPostStyleCoverage {
    let mut coverage = archive.coverage();
    coverage.check_mode = SimulationPostStyleCheckMode::ExactSelectedProfilesAndPreservation;
    coverage.completion = if changed {
        SimulationPostStyleCompletion::Changed
    } else {
        SimulationPostStyleCompletion::Unchanged
    };
    coverage
}
