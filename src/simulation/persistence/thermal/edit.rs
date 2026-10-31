use super::*;
use crate::{Result, simulation::invalid};

impl SimulationThermalCollection {
    /// Bind an original outer field to this immutable revision; e.g. `collection.field_id(0)?`.
    pub fn field_id(&self, ordinal: usize) -> Result<SimulationThermalFieldId> {
        if ordinal >= self.fields.len() {
            return Err(invalid(
                "thermal field identity",
                format!("ordinal {ordinal}; expected < {}", self.fields.len()),
            ));
        }
        Ok(SimulationThermalFieldId {
            revision: self.revision.clone(),
            ordinal,
        })
    }

    /// Resolve a field only on its owning revision; e.g. `collection.field(&id)?`.
    pub fn field(&self, id: &SimulationThermalFieldId) -> Result<&SimulationThermalField> {
        if id.revision != self.revision {
            return Err(invalid(
                "thermal field revision",
                format!(
                    "ordinal {} from another revision; expected this collection revision",
                    id.ordinal
                ),
            ));
        }
        self.fields.get(id.ordinal).ok_or_else(|| {
            invalid(
                "thermal field identity",
                format!("ordinal {}; expected < {}", id.ordinal, self.fields.len()),
            )
        })
    }

    /// Replace one complete typed value, preserving every other byte; e.g. `collection.edit_field(&id, value)?`.
    pub fn edit_field(
        &self,
        id: &SimulationThermalFieldId,
        replacement: SimulationThermalValue,
    ) -> Result<SimulationThermalEdit> {
        let original = self.field(id)?;
        require_kind(&original.value, &replacement, &original.path)?;
        let archive = self.replaced_field(id.ordinal, &replacement)?;
        let changed = archive.revision != self.revision;
        let (history, nested_history) =
            history::compose(self, &archive, id.ordinal, &replacement, changed)?;
        let coverage = edit_coverage(&archive, changed);
        let result = archive.field_id(id.ordinal)?;
        Ok(SimulationThermalEdit {
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

    fn replaced_field(&self, ordinal: usize, replacement: &SimulationThermalValue) -> Result<Self> {
        let original = self.fields.get(ordinal).ok_or_else(|| {
            invalid(
                "thermal replacement",
                format!("ordinal {ordinal}; expected < {}", self.fields.len()),
            )
        })?;
        if original.value == *replacement {
            return Ok(self.clone());
        }
        let mut fields = self.fields.to_vec();
        let target = fields.get_mut(ordinal).ok_or_else(|| {
            invalid(
                "thermal replacement",
                format!("ordinal {ordinal}; expected an existing copied field"),
            )
        })?;
        target.value = replacement.clone();
        let result = Self::from_fields(fields, self.version, self.kind, self.limits)?;
        verify_preservation(self, &result, ordinal)?;
        Ok(result)
    }
}

fn require_kind(
    original: &SimulationThermalValue,
    replacement: &SimulationThermalValue,
    path: &str,
) -> Result<()> {
    if std::mem::discriminant(original) == std::mem::discriminant(replacement) {
        return Ok(());
    }
    Err(invalid(
        path,
        format!(
            "replacement kind {}; expected {}",
            kind_name(replacement),
            kind_name(original)
        ),
    ))
}

fn kind_name(value: &SimulationThermalValue) -> &'static str {
    match value {
        SimulationThermalValue::Signed(_) => "Signed",
        SimulationThermalValue::Unsigned(_) => "Unsigned",
        SimulationThermalValue::DoubleBits(_) => "DoubleBits",
        SimulationThermalValue::Text(_) => "Text",
        SimulationThermalValue::Bytes(_) => "Bytes",
        SimulationThermalValue::Embedded(_) => "Embedded",
    }
}

fn verify_preservation(
    source: &SimulationThermalCollection,
    result: &SimulationThermalCollection,
    ordinal: usize,
) -> Result<()> {
    let (start, end) = field_span(source, ordinal)?;
    let (next_start, next_end) = field_span(result, ordinal)?;
    if start == next_start
        && source.parent[..start] == result.parent[..next_start]
        && source.parent[end..] == result.parent[next_end..]
    {
        return Ok(());
    }
    Err(invalid(
        "thermal edit preservation",
        format!(
            "source span {start}..{end}, result {next_start}..{next_end}; expected identical prefix and suffix"
        ),
    ))
}

fn field_span(collection: &SimulationThermalCollection, ordinal: usize) -> Result<(usize, usize)> {
    collection
        .field_offsets()
        .get(ordinal)
        .copied()
        .ok_or_else(|| {
            invalid(
                "thermal field span",
                format!(
                    "ordinal {ordinal}; expected < {} complete field spans",
                    collection.fields().len()
                ),
            )
        })
}

fn edit_coverage(
    archive: &SimulationThermalCollection,
    changed: bool,
) -> SimulationThermalCoverage {
    let mut coverage = archive.coverage();
    coverage.check_mode = SimulationThermalCheckMode::ExactSelectedProfilesAndPreservation;
    coverage.completion = if changed {
        SimulationThermalCompletion::Changed
    } else {
        SimulationThermalCompletion::Unchanged
    };
    coverage
}
