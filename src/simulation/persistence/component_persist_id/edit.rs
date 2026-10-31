use super::super::{load_edit, load_identity};
use super::*;
use crate::Result;

impl SimulationComponentPersistId {
    /// Bind an original outer field to this immutable revision; e.g. `property.field_id(0)?`.
    pub fn field_id(&self, ordinal: usize) -> Result<SimulationComponentPersistIdFieldId> {
        load_identity::field_id(
            &self.revision,
            ordinal,
            self.fields.len(),
            "standalone Component PersistID",
        )
    }

    /// Resolve a field only on its owning revision; e.g. `property.field(&id)?`.
    pub fn field(
        &self,
        id: &SimulationComponentPersistIdFieldId,
    ) -> Result<&SimulationComponentPersistIdField> {
        load_identity::field(
            &self.fields,
            &self.revision,
            id,
            "standalone Component PersistID",
        )
    }

    /// Replace one complete typed value, preserving every other byte; e.g. `property.edit_field(&id, value)?`.
    pub fn edit_field(
        &self,
        id: &SimulationComponentPersistIdFieldId,
        replacement: SimulationComponentPersistIdValue,
    ) -> Result<SimulationComponentPersistIdEdit> {
        let original = self.field(id)?;
        load_edit::require_kind(&original.value, &replacement, &original.path)?;
        let archive = self.replaced_field(id.ordinal(), &replacement)?;
        let changed = archive.revision != self.revision;
        let (history, nested_history) =
            super::super::thermal::compose(self, &archive, id.ordinal(), &replacement, changed)?;
        let coverage = edit_coverage(&archive, changed);
        let result = archive.field_id(id.ordinal)?;
        Ok(SimulationComponentPersistIdEdit {
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
        replacement: &SimulationComponentPersistIdValue,
    ) -> Result<Self> {
        load_edit::replaced_field(self, ordinal, replacement, |fields| {
            Self::from_fields(fields, self.version, self.limits)
        })
    }
}

fn edit_coverage(
    archive: &SimulationComponentPersistId,
    changed: bool,
) -> SimulationComponentPersistIdCoverage {
    let mut coverage = archive.coverage();
    coverage.check_mode =
        SimulationComponentPersistIdCheckMode::ExactSelectedProfilesAndPreservation;
    coverage.completion = if changed {
        SimulationComponentPersistIdCompletion::Changed
    } else {
        SimulationComponentPersistIdCompletion::Unchanged
    };
    coverage
}
