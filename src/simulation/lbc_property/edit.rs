use super::{
    Result, SimulationLbcProperty, SimulationLbcPropertyCheckMode, SimulationLbcPropertyCoverage,
    SimulationLbcPropertyFieldId, require,
};
use std::sync::Arc;

/// Exact byte change status independent of native application acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLbcEditCompletion {
    /// Every byte already matched; all original identities survived.
    Unchanged,
    /// One field changed on an isolated complete validated result.
    Changed,
}

/// Exact relation between one result field and its source parents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLbcFieldRelation {
    /// All four original bytes survived verbatim.
    Copied,
    /// Field derives from its original position and retained supplied replacement value.
    Derived,
}

/// One ordered history entry covering every stored field, including unchanged fields.
#[derive(Debug, Clone)]
pub struct SimulationLbcFieldHistory {
    /// Original normalized field identity.
    pub parent: SimulationLbcPropertyFieldId,
    /// Validated resulting field identity.
    pub result: SimulationLbcPropertyFieldId,
    /// Exact copy versus scalar derivation relation.
    pub relation: SimulationLbcFieldRelation,
}

/// Completed isolated edit retaining complete source, supplied value, exact checks and all history.
#[derive(Debug, Clone)]
pub struct SimulationLbcFieldEdit {
    /// Complete immutable original image/profile.
    pub source: SimulationLbcProperty,
    /// Complete validated result image/profile.
    pub archive: SimulationLbcProperty,
    /// Original requested field identity.
    pub parent: SimulationLbcPropertyFieldId,
    /// Resulting requested field identity.
    pub result: SimulationLbcPropertyFieldId,
    /// Complete supplied signed word pattern, retained as a derivation parent.
    pub replacement_value: i32,
    /// Changed versus authentic byte no-op.
    pub completion: SimulationLbcEditCompletion,
    /// Complete profile/preservation checks and native/semantic exclusions.
    pub coverage: SimulationLbcPropertyCoverage,
    /// Total wire-order history, independent of thread scheduling or hash iteration.
    pub history: Arc<[SimulationLbcFieldHistory]>,
}

impl SimulationLbcProperty {
    /// Replace one normalized four-byte word without changing context, e.g. `property.edit_field(&id, i32::MIN)?`.
    pub fn edit_field(
        &self,
        id: &SimulationLbcPropertyFieldId,
        replacement_value: i32,
    ) -> Result<SimulationLbcFieldEdit> {
        let original = self.field(id)?;
        let archive = self.replaced_word(original.stream_offset, replacement_value)?;
        let changed = self.revision != archive.revision;
        let history = field_history(self, &archive, id.ordinal, changed);
        let coverage = edit_coverage(&archive);
        Ok(SimulationLbcFieldEdit {
            source: self.clone(),
            result: archive.field_id(id.ordinal)?,
            archive,
            parent: id.clone(),
            replacement_value,
            completion: if changed {
                SimulationLbcEditCompletion::Changed
            } else {
                SimulationLbcEditCompletion::Unchanged
            },
            coverage,
            history,
        })
    }

    fn replaced_word(&self, offset: usize, replacement: i32) -> Result<Self> {
        let pattern = replacement.to_le_bytes();
        if self.bytes[offset..offset + 4] == pattern {
            return Ok(self.clone());
        }
        let mut bytes = self.bytes.to_vec();
        bytes[offset..offset + 4].copy_from_slice(&pattern);
        let archive = Self::from_bytes(&bytes, self.profile)?;
        verify_preservation(self, &archive, offset, &pattern)?;
        Ok(archive)
    }
}

fn verify_preservation(
    source: &SimulationLbcProperty,
    result: &SimulationLbcProperty,
    offset: usize,
    pattern: &[u8; 4],
) -> Result<()> {
    let end = offset + 4;
    require(
        source.profile == result.profile && source.fields.len() == result.fields.len(),
        "LBC edit profile",
        format!(
            "{:?} with {} fields; expected {:?} with {} fields",
            result.profile,
            result.fields.len(),
            source.profile,
            source.fields.len()
        ),
    )?;
    require(
        result.bytes[..offset] == source.bytes[..offset]
            && result.bytes[offset..end] == *pattern
            && result.bytes[end..] == source.bytes[end..],
        "LBC edit preservation",
        format!(
            "{} result bytes at {offset}..{end}; expected unchanged source prefix/suffix and exact replacement {pattern:?}",
            result.bytes.len()
        ),
    )
}

fn field_history(
    source: &SimulationLbcProperty,
    result: &SimulationLbcProperty,
    changed_ordinal: usize,
    changed: bool,
) -> Arc<[SimulationLbcFieldHistory]> {
    source
        .fields
        .iter()
        .zip(result.fields.iter())
        .enumerate()
        .map(|(ordinal, (parent, child))| SimulationLbcFieldHistory {
            parent: parent.id.clone(),
            result: child.id.clone(),
            relation: if changed && ordinal == changed_ordinal {
                SimulationLbcFieldRelation::Derived
            } else {
                SimulationLbcFieldRelation::Copied
            },
        })
        .collect()
}

fn edit_coverage(archive: &SimulationLbcProperty) -> SimulationLbcPropertyCoverage {
    let mut coverage = archive.coverage();
    coverage.check_mode = SimulationLbcPropertyCheckMode::ExactSelectedProfileAndPreservation;
    coverage
}
