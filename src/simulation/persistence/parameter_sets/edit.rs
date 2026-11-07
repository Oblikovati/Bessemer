use super::*;

impl SimulationParameterSets {
    /// Replace one typed original pattern, e.g. `archive.edit_field(&id, SimulationParameterValue::DoubleBits(1))?`.
    pub fn edit_field(
        &self,
        id: &SimulationParameterFieldId,
        replacement: SimulationParameterValue,
    ) -> Result<SimulationParameterEdit> {
        let original = self.field(id)?;
        require_replacement(self, original, &replacement)?;
        let archive = self.replaced_field(original, &replacement)?;
        let changed = self.revision != archive.revision;
        let history = parameter_history(self, &archive, id, changed);
        let coverage = edit_coverage(&archive, changed);
        Ok(SimulationParameterEdit {
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
        original: &SimulationParameterField,
        replacement: &SimulationParameterValue,
    ) -> Result<Self> {
        let before = original.value.to_bytes();
        let after = replacement.to_bytes();
        if before == after {
            return Ok(self.clone());
        }
        let start = original.stream_offset;
        let end = start + before.len();
        let bytes = splice_field(&self.bytes, start, end, &after)?;
        let archive = Self::from_bytes(&bytes, self.version, self.limits)?;
        verify_edit_layout(self, &archive)?;
        verify_edit_bytes(&self.bytes, &archive.bytes, start, end, &after)?;
        Ok(archive)
    }
}

fn require_replacement(
    source: &SimulationParameterSets,
    original: &SimulationParameterField,
    replacement: &SimulationParameterValue,
) -> Result<()> {
    require_value_kind(original, replacement)?;
    if let SimulationParameterValue::Text(text) = replacement {
        text.require_limit(source.limits.maximum_text_bytes)?;
    }
    require_structural_value(source, original, replacement)
}

fn require_value_kind(
    original: &SimulationParameterField,
    replacement: &SimulationParameterValue,
) -> Result<()> {
    require(
        original.value.kind() == replacement.kind(),
        "ParameterSets replacement type",
        format!(
            "kind {} at {:?}; expected complete kind {}",
            replacement.kind(),
            original.role,
            original.value.kind()
        ),
    )
}

fn require_structural_value(
    source: &SimulationParameterSets,
    original: &SimulationParameterField,
    replacement: &SimulationParameterValue,
) -> Result<()> {
    let structural = original.role == SimulationParameterRole::RecordCount
        || (original.role == SimulationParameterRole::HeaderWord(0)
            && source.version.value() >= 810026);
    if !structural {
        return Ok(());
    }
    require(
        original.value.to_bytes() == replacement.to_bytes(),
        "ParameterSets structural field",
        format!(
            "replacement {:?} at {:?}; expected an authentic unchanged count or complete matching reconstruction",
            replacement, original.role
        ),
    )
}

fn splice_field(source: &[u8], start: usize, end: usize, replacement: &[u8]) -> Result<Vec<u8>> {
    let size = source.len().checked_sub(end - start).and_then(|remaining| remaining.checked_add(replacement.len())).ok_or_else(|| invalid("ParameterSets replacement extent", format!("{} original bytes, range {start}..{end}, {} replacement bytes; expected a representable complete image", source.len(), replacement.len())))?;
    let mut bytes = super::super::reserve_bytes(size)?;
    bytes.extend_from_slice(&source[..start]);
    bytes.extend_from_slice(replacement);
    bytes.extend_from_slice(&source[end..]);
    Ok(bytes)
}

fn verify_edit_layout(
    source: &SimulationParameterSets,
    result: &SimulationParameterSets,
) -> Result<()> {
    let matches = source.fields.len() == result.fields.len()
        && source
            .fields
            .iter()
            .zip(result.fields.iter())
            .all(|(parent, child)| parent.role == child.role);
    require(
        matches,
        "ParameterSets edit field layout",
        format!(
            "{} result fields, {} source fields; expected every original role in unchanged wire order",
            result.fields.len(),
            source.fields.len()
        ),
    )
}

fn verify_edit_bytes(
    source: &[u8],
    result: &[u8],
    start: usize,
    end: usize,
    replacement: &[u8],
) -> Result<()> {
    let replacement_end = start + replacement.len();
    require(
        result[..start] == source[..start]
            && result[start..replacement_end] == *replacement
            && result[replacement_end..] == source[end..],
        "ParameterSets edit preservation",
        format!(
            "{} result bytes at {start}, {} replacement bytes; expected exact unchanged prefix/suffix and complete replacement",
            result.len(),
            replacement.len()
        ),
    )
}

fn parameter_history(
    source: &SimulationParameterSets,
    result: &SimulationParameterSets,
    requested: &SimulationParameterFieldId,
    changed: bool,
) -> Arc<[SimulationParameterHistory]> {
    source
        .fields
        .iter()
        .zip(result.fields.iter())
        .map(|(parent, result)| SimulationParameterHistory {
            parent: parent.id.clone(),
            result: result.id.clone(),
            relation: if changed && parent.id == *requested {
                SimulationParameterRelation::Derived
            } else {
                SimulationParameterRelation::Copied
            },
        })
        .collect()
}

fn edit_coverage(archive: &SimulationParameterSets, changed: bool) -> SimulationParameterCoverage {
    let mut coverage = archive.coverage();
    coverage.check_mode = SimulationParameterCheckMode::ExactSelectedProfileAndPreservation;
    coverage.completion = if changed {
        SimulationParameterCompletion::Changed
    } else {
        SimulationParameterCompletion::Unchanged
    };
    coverage
}
