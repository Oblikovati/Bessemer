use super::*;

/// Exact stored-field byte change status, independent of native application acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPersistScalarEditCompletion {
    /// Every byte already matched and original identities survived.
    Unchanged,
    /// One field changed on an isolated complete validated image.
    Changed,
}

/// Complete per-field source relationship, including unchanged header metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPersistScalarRelation {
    /// Original complete stored field survived verbatim.
    Copied,
    /// Field derives from its original position and retained supplied replacement value.
    Derived,
}

/// Ordered history for every count, capacity and payload field.
#[derive(Debug, Clone)]
pub struct SimulationPersistScalarHistory {
    /// Original normalized field identity.
    pub parent: SimulationPersistScalarId,
    /// Result normalized field identity.
    pub result: SimulationPersistScalarId,
    /// Exact copy or derivation relation.
    pub relation: SimulationPersistScalarRelation,
}

/// Complete isolated same-width field edit retaining every source and exact validation scope.
#[derive(Debug, Clone)]
pub struct SimulationPersistScalarEdit {
    /// Complete immutable original image and profile.
    pub source: SimulationPersistScalarArchive,
    /// Complete validated resulting image and profile.
    pub archive: SimulationPersistScalarArchive,
    /// Original requested field identity.
    pub parent: SimulationPersistScalarId,
    /// Resulting requested field identity.
    pub result: SimulationPersistScalarId,
    /// Entire supplied typed replacement pattern.
    pub replacement: SimulationPersistScalarValue,
    /// Authentic no-op versus changed byte image.
    pub completion: SimulationPersistScalarEditCompletion,
    /// Complete frame/preservation checks with diagnostics and achieved pattern error.
    pub coverage: SimulationPersistScalarCoverage,
    /// Every stored field in total physical order, including unchanged headers.
    pub history: Arc<[SimulationPersistScalarHistory]>,
}

impl SimulationPersistScalarArchive {
    /// Replace an entry/capacity or preserve an identical count, e.g. `archive.edit_field(&id, SimulationPersistScalarValue::I64(-1))?`; count changes require complete construction.
    pub fn edit_field(
        &self,
        id: &SimulationPersistScalarId,
        replacement: SimulationPersistScalarValue,
    ) -> Result<SimulationPersistScalarEdit> {
        let original = self.field(id)?;
        validate_replacement(self, original, replacement)?;
        let archive = self.replaced_field(original, replacement)?;
        let changed = self.revision != archive.revision;
        let history = scalar_history(self, &archive, id.ordinal, changed);
        let coverage = edit_coverage(&archive);
        Ok(SimulationPersistScalarEdit {
            source: self.clone(),
            result: archive.fields[id.ordinal].id.clone(),
            archive,
            parent: id.clone(),
            replacement,
            completion: if changed {
                SimulationPersistScalarEditCompletion::Changed
            } else {
                SimulationPersistScalarEditCompletion::Unchanged
            },
            coverage,
            history,
        })
    }

    fn replaced_field(
        &self,
        original: &SimulationPersistScalarField,
        replacement: SimulationPersistScalarValue,
    ) -> Result<Self> {
        if original.value == replacement {
            return Ok(self.clone());
        }
        let pattern = pattern_bytes(replacement);
        let start = original.stream_offset;
        let end = start + pattern.len();
        let mut bytes = self.bytes.to_vec();
        bytes[start..end].copy_from_slice(&pattern);
        let archive = Self::from_bytes(&bytes, self.profile, self.maximum_entries)?;
        verify_preservation(self, &archive, start, &pattern)?;
        Ok(archive)
    }
}

fn validate_replacement(
    archive: &SimulationPersistScalarArchive,
    field: &SimulationPersistScalarField,
    replacement: SimulationPersistScalarValue,
) -> Result<()> {
    validate_width(field.value, replacement)?;
    match field.role {
        SimulationPersistScalarRole::UsedCount => require(
            replacement == field.value,
            "scalar count edit",
            format!(
                "{replacement:?} for {} entries; expected the original count or construction of a complete matching payload",
                archive.layout.used
            ),
        ),
        SimulationPersistScalarRole::AllocationCapacity => {
            validate_capacity(archive.layout.used, replacement)
        }
        SimulationPersistScalarRole::Entry(_) => Ok(()),
    }
}

fn validate_width(
    original: SimulationPersistScalarValue,
    replacement: SimulationPersistScalarValue,
) -> Result<()> {
    use SimulationPersistScalarValue::{I32, I64};
    let compatible = matches!((original, replacement), (I32(_), I32(_)) | (I64(_), I64(_)));
    require(
        compatible,
        "scalar replacement width",
        format!("{replacement:?} for {original:?}; expected the original stored scalar width"),
    )
}

fn validate_capacity(used: usize, replacement: SimulationPersistScalarValue) -> Result<()> {
    match replacement {
        SimulationPersistScalarValue::I32(capacity)
            if capacity >= 0 && capacity as usize >= used =>
        {
            Ok(())
        }
        other => Err(invalid(
            "scalar capacity edit",
            format!(
                "{other:?} for {used} used entries; expected a nonnegative signed-i32 capacity at least {used}"
            ),
        )),
    }
}

fn pattern_bytes(value: SimulationPersistScalarValue) -> Vec<u8> {
    match value {
        SimulationPersistScalarValue::I32(value) => value.to_le_bytes().to_vec(),
        SimulationPersistScalarValue::I64(value) => value.to_le_bytes().to_vec(),
    }
}

fn verify_preservation(
    source: &SimulationPersistScalarArchive,
    result: &SimulationPersistScalarArchive,
    start: usize,
    pattern: &[u8],
) -> Result<()> {
    let end = start + pattern.len();
    verify_profile(source, result)?;
    require(
        result.bytes[..start] == source.bytes[..start]
            && result.bytes[start..end] == *pattern
            && result.bytes[end..] == source.bytes[end..],
        "scalar edit preservation",
        format!(
            "{} result bytes at {start}..{end}; expected unchanged prefix/suffix and exact replacement {pattern:?}",
            result.bytes.len()
        ),
    )
}

fn scalar_history(
    source: &SimulationPersistScalarArchive,
    result: &SimulationPersistScalarArchive,
    requested: usize,
    changed: bool,
) -> Arc<[SimulationPersistScalarHistory]> {
    source
        .fields
        .iter()
        .zip(result.fields.iter())
        .enumerate()
        .map(
            |(ordinal, (parent, child))| SimulationPersistScalarHistory {
                parent: parent.id.clone(),
                result: child.id.clone(),
                relation: if changed && ordinal == requested {
                    SimulationPersistScalarRelation::Derived
                } else {
                    SimulationPersistScalarRelation::Copied
                },
            },
        )
        .collect()
}

fn edit_coverage(archive: &SimulationPersistScalarArchive) -> SimulationPersistScalarCoverage {
    let mut coverage = archive.coverage();
    coverage.check_mode = SimulationPersistScalarCheckMode::ExactCompleteFrameAndPreservation;
    coverage
}

fn verify_profile(
    source: &SimulationPersistScalarArchive,
    result: &SimulationPersistScalarArchive,
) -> Result<()> {
    require(
        source.profile == result.profile && source.fields.len() == result.fields.len(),
        "scalar edit profile",
        format!(
            "{:?} with {} fields; expected {:?} with {} fields",
            result.profile,
            result.fields.len(),
            source.profile,
            source.fields.len()
        ),
    )
}
