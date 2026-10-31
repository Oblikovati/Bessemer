use super::*;

impl SimulationManagerProperty {
    /// Replace an original raw word on an isolated copy, e.g. `header.edit_field(&id, i32::MIN)?`.
    pub fn edit_field(
        &self,
        id: &SimulationManagerPropertyFieldId,
        replacement: i32,
    ) -> Result<SimulationManagerPropertyEdit> {
        let original = self.field(id)?;
        let archive = self.replaced_field(original, replacement)?;
        let changed = self.revision != archive.revision;
        let coverage = edit_coverage(&archive, changed);
        let history = edit_history(self, &archive, id, changed);
        Ok(SimulationManagerPropertyEdit {
            source: self.clone(),
            result: archive.field_id(id.ordinal)?,
            archive,
            parent: id.clone(),
            replacement,
            coverage,
            history,
        })
    }

    fn replaced_field(
        &self,
        original: &SimulationManagerPropertyField,
        replacement: i32,
    ) -> Result<Self> {
        if original.value == replacement {
            return Ok(self.clone());
        }
        let offset = original.stream_offset;
        let mut bytes = self.bytes.to_vec();
        bytes[offset..offset + 4].copy_from_slice(&replacement.to_le_bytes());
        let archive = Self::from_bytes(&bytes, self.profile)?;
        require(
            archive.bytes[..offset] == self.bytes[..offset]
                && archive.bytes[offset..offset + 4] == replacement.to_le_bytes()
                && archive.bytes[offset + 4..] == self.bytes[offset + 4..],
            "manager Property preservation",
            format!(
                "{} bytes, field {offset}, replacement {replacement}; expected unchanged prefix/suffix and exact replacement",
                archive.bytes.len()
            ),
        )?;
        Ok(archive)
    }
}

fn edit_coverage(
    archive: &SimulationManagerProperty,
    changed: bool,
) -> SimulationManagerPropertyCoverage {
    let mut coverage = archive.coverage();
    coverage.check_mode = SimulationManagerPropertyCheckMode::ExactProfileAndPreservation;
    coverage.completion = if changed {
        SimulationManagerPropertyCompletion::Changed
    } else {
        SimulationManagerPropertyCompletion::Unchanged
    };
    coverage
}

fn edit_history(
    source: &SimulationManagerProperty,
    result: &SimulationManagerProperty,
    requested: &SimulationManagerPropertyFieldId,
    changed: bool,
) -> Arc<[SimulationManagerPropertyHistory]> {
    source
        .fields
        .iter()
        .zip(result.fields.iter())
        .map(|(parent, result)| SimulationManagerPropertyHistory {
            parent: parent.id.clone(),
            result: result.id.clone(),
            relation: if changed && parent.id == *requested {
                SimulationManagerPropertyRelation::Derived
            } else {
                SimulationManagerPropertyRelation::Copied
            },
        })
        .collect()
}
