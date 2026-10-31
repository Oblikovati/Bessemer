use super::super::{load_edit, load_identity};
use super::*;
use crate::Result;

impl SimulationPostGraphParameters {
    /// Bind an original outer field to this immutable revision; e.g. `property.field_id(0)?`.
    pub fn field_id(&self, ordinal: usize) -> Result<SimulationPostGraphFieldId> {
        load_identity::field_id(
            &self.revision,
            ordinal,
            self.fields.len(),
            "graph parameters",
        )
    }

    /// Resolve a field only on its owning revision; e.g. `property.field(&id)?`.
    pub fn field(&self, id: &SimulationPostGraphFieldId) -> Result<&SimulationPostGraphField> {
        load_identity::field(&self.fields, &self.revision, id, "graph parameters")
    }

    /// Replace one complete typed value, preserving every other byte; e.g. `property.edit_field(&id, value)?`.
    pub fn edit_field(
        &self,
        id: &SimulationPostGraphFieldId,
        replacement: SimulationPostGraphValue,
    ) -> Result<SimulationPostGraphEdit> {
        let original = self.field(id)?;
        super::values::require_kind(&original.value, &replacement, &original.path)?;
        let wire = super::values::wire_value(&replacement);
        let archive = self.replaced_field(id.ordinal(), &wire)?;
        let changed = archive.revision != self.revision;
        let (history, nested_history) =
            super::super::thermal::compose(self, &archive, id.ordinal(), &wire, changed)?;
        let coverage = edit_coverage(&archive, changed);
        let result = archive.field_id(id.ordinal)?;
        Ok(SimulationPostGraphEdit {
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
        replacement: &super::super::thermal::SimulationThermalValue,
    ) -> Result<Self> {
        load_edit::replaced_field(self, ordinal, replacement, |fields| {
            let typed = super::values::from_wire_fields(&fields, self.limits)?;
            Self::from_fields(typed, self.version, self.limits)
        })
    }
}

fn edit_coverage(
    archive: &SimulationPostGraphParameters,
    changed: bool,
) -> SimulationPostGraphCoverage {
    let mut coverage = archive.coverage();
    coverage.check_mode = SimulationPostGraphCheckMode::ExactSelectedProfilesAndPreservation;
    coverage.completion = if changed {
        SimulationPostGraphCompletion::Changed
    } else {
        SimulationPostGraphCompletion::Unchanged
    };
    coverage
}
