use super::super::{load_edit, load_identity, thermal};
use super::*;
use crate::{Result, simulation::invalid};

impl SimulationBeamProperty {
    /// Bind a physical occurrence to this revision; e.g. `property.field_id(0)?`.
    pub fn field_id(&self, ordinal: usize) -> Result<SimulationBeamFieldId> {
        load_identity::field_id(
            &self.revision,
            ordinal,
            self.fields.len(),
            "beam child Property",
        )
    }

    /// Resolve only an identity owned by this revision; e.g. `property.field(&id)?`.
    pub fn field(&self, id: &SimulationBeamFieldId) -> Result<&SimulationBeamField> {
        load_identity::field(&self.fields, &self.revision, id, "beam child Property")
    }

    /// Replace one complete typed value with isolated ordered history; e.g. `property.edit_field(&id, value)?`.
    pub fn edit_field(
        &self,
        id: &SimulationBeamFieldId,
        replacement: SimulationBeamValue,
    ) -> Result<SimulationBeamEdit> {
        let original = self.field(id)?;
        require_kind(&original.value, &replacement, &original.path)?;
        let wire_value = fields::wire_value(&replacement);
        let archive = self.replaced_field(id.ordinal(), &wire_value)?;
        let changed = self.revision != archive.revision;
        let (history, nested_history) =
            thermal::compose(self, &archive, id.ordinal(), &wire_value, changed)?;
        let coverage = edit_coverage(&archive, changed);
        let result = archive.field_id(id.ordinal())?;
        Ok(SimulationBeamEdit {
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
        replacement: &thermal::SimulationThermalValue,
    ) -> Result<Self> {
        load_edit::replaced_field(self, ordinal, replacement, |fields| {
            let bytes = thermal::encode_fields(&fields, self.limits.maximum_bytes)?;
            let archive = Self::from_bytes(&bytes, self.version, self.limits)?;
            if archive.wire_fields.as_ref() != fields {
                return Err(thermal::construction_mismatch(
                    &fields,
                    &archive.wire_fields,
                    "beam child Property edit",
                ));
            }
            Ok(archive)
        })
    }
}

fn require_kind(
    original: &SimulationBeamValue,
    replacement: &SimulationBeamValue,
    path: &str,
) -> Result<()> {
    match (original, replacement) {
        (SimulationBeamValue::SignedShort(_), SimulationBeamValue::SignedShort(_)) => Ok(()),
        (SimulationBeamValue::Shared(a), SimulationBeamValue::Shared(b)) => {
            load_edit::require_kind(a, b, path)
        }
        _ => Err(invalid(
            path,
            format!("replacement {replacement:?}; expected stored kind matching {original:?}"),
        )),
    }
}

fn edit_coverage(archive: &SimulationBeamProperty, changed: bool) -> SimulationBeamCoverage {
    let mut coverage = archive.coverage();
    coverage.check_mode = SimulationBeamCheckMode::ExactSelectedProfilesAndPreservation;
    coverage.completion = if changed {
        SimulationBeamCompletion::Changed
    } else {
        SimulationBeamCompletion::Unchanged
    };
    coverage
}
