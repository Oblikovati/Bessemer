use super::super::{load_edit, load_identity};
use super::*;
use crate::Result;

impl SimulationInertialCollection {
    /// Bind an original outer field to this immutable revision; e.g. `collection.field_id(0)?`.
    pub fn field_id(&self, ordinal: usize) -> Result<SimulationInertialFieldId> {
        load_identity::field_id(&self.revision, ordinal, self.fields.len(), "inertial")
    }

    /// Resolve a field only on its owning revision; e.g. `collection.field(&id)?`.
    pub fn field(&self, id: &SimulationInertialFieldId) -> Result<&SimulationInertialField> {
        load_identity::field(&self.fields, &self.revision, id, "inertial")
    }

    /// Replace one complete typed value, preserving every other byte; e.g. `collection.edit_field(&id, value)?`.
    pub fn edit_field(
        &self,
        id: &SimulationInertialFieldId,
        replacement: SimulationInertialValue,
    ) -> Result<SimulationInertialEdit> {
        let original = self.field(id)?;
        load_edit::require_kind(&original.value, &replacement, &original.path)?;
        let archive = self.replaced_field(id.ordinal(), &replacement)?;
        let changed = archive.revision != self.revision;
        let (history, nested_history) =
            super::super::thermal::compose(self, &archive, id.ordinal(), &replacement, changed)?;
        let coverage = edit_coverage(&archive, changed);
        let result = archive.field_id(id.ordinal)?;
        Ok(SimulationInertialEdit {
            source: self.clone(),
            archive,
            parent: id.clone(),
            result,
            replacement,
            coverage,
            history,
            nested_history,
        })
    }

    fn replaced_field(
        &self,
        ordinal: usize,
        replacement: &SimulationInertialValue,
    ) -> Result<Self> {
        load_edit::replaced_field(self, ordinal, replacement, |fields| {
            Self::from_fields(fields, self.version, self.kind, self.limits)
        })
    }
}

fn edit_coverage(
    archive: &SimulationInertialCollection,
    changed: bool,
) -> SimulationInertialCoverage {
    let mut coverage = archive.coverage();
    coverage.check_mode = SimulationInertialCheckMode::ExactSelectedProfilesAndPreservation;
    coverage.completion = if changed {
        SimulationInertialCompletion::Changed
    } else {
        SimulationInertialCompletion::Unchanged
    };
    coverage
}
