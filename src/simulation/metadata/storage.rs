use super::{
    Result, SimulationScalarArchive, SimulationScalarEdit, SimulationScalarId,
    SimulationScalarKind, require,
};
use crate::compound::{CompoundEdit, CompoundFile, CompoundLimits};
use crate::{BessemerError, FailureClass};

/// Explicit complete carrier and known string-stream budgets.
#[derive(Debug, Clone, Copy)]
pub struct SimulationMetadataLimits {
    /// Existing complete compound-file budgets.
    pub carrier: CompoundLimits,
    /// Maximum complete optional GUID-string stream bytes, including an opaque tail.
    pub maximum_guid_stream_bytes: usize,
}

/// Immutable known Simulation metadata snapshots inside a complete preserved carrier.
#[derive(Debug, Clone)]
pub struct SimulationMetadataArchive {
    pub(super) carrier: CompoundFile,
    pub(super) scalars: [Option<SimulationScalarArchive>; 2],
    pub(super) guids: Option<super::SimulationGuidArchive>,
    pub(super) limits: SimulationMetadataLimits,
}

/// Complete existing-scalar edit composed with all carrier derivations and checks.
#[derive(Debug, Clone)]
pub struct SimulationMetadataScalarEdit {
    /// Resulting immutable complete carrier and cached metadata.
    pub archive: SimulationMetadataArchive,
    /// Exact bare-word replacement, source identities, coverage and exclusions.
    pub scalar_edit: SimulationScalarEdit,
    /// All-entry carrier history, exact logical fidelity and structural checks.
    pub carrier_edit: CompoundEdit,
}

impl SimulationMetadataArchive {
    /// Read known optional metadata once while preserving every other stream, e.g. `SimulationMetadataArchive::from_bytes(cwr, limits)?`.
    pub fn from_bytes(bytes: &[u8], limits: SimulationMetadataLimits) -> Result<Self> {
        let carrier = CompoundFile::from_bytes(bytes, limits.carrier)?;
        let scalars = [
            read_scalar(&carrier, SimulationScalarKind::TimeStamp)?,
            read_scalar(&carrier, SimulationScalarKind::RunState)?,
        ];
        let guids = read_guids(&carrier, limits.maximum_guid_stream_bytes)?;
        Ok(Self {
            carrier,
            scalars,
            guids,
            limits,
        })
    }

    /// Complete immutable compound carrier, e.g. `metadata.carrier().to_bytes()`.
    pub fn carrier(&self) -> &CompoundFile {
        &self.carrier
    }

    /// Optional normalized scalar snapshot; absence stays explicit, e.g. `metadata.scalar(kind)`.
    pub fn scalar(&self, kind: SimulationScalarKind) -> Option<&SimulationScalarArchive> {
        self.scalars[kind.index()].as_ref()
    }

    /// Optional preserved four-string snapshot, e.g. `metadata.guids()`; unknown tails remain explicitly incomplete.
    pub fn guids(&self) -> Option<&super::SimulationGuidArchive> {
        self.guids.as_ref()
    }

    /// Replace one revision-bound existing bare word, e.g. `metadata.edit_scalar(scalar.id(), bits)?`.
    pub fn edit_scalar(
        &self,
        id: &SimulationScalarId,
        pattern: u32,
    ) -> Result<SimulationMetadataScalarEdit> {
        let scalar = self.required_scalar(id.kind)?;
        require(
            scalar.id() == id,
            id.kind.stream_name(),
            "foreign scalar revision; expected the cached scalar identity in this complete carrier",
        )?;
        let scalar_edit = scalar.edit_pattern(pattern);
        let entry = self.carrier.entry_at_path(id.kind.stream_name())?;
        let carrier_edit = self
            .carrier
            .edit_stream(entry.id(), &scalar_edit.archive.to_bytes())?;
        let mut scalars = self.scalars.clone();
        scalars[id.kind.index()] = Some(scalar_edit.archive.clone());
        let archive = self.revalidated(carrier_edit.file.clone(), scalars, self.guids.clone())?;
        Ok(SimulationMetadataScalarEdit {
            archive,
            scalar_edit,
            carrier_edit,
        })
    }

    pub(super) fn required_scalar(
        &self,
        kind: SimulationScalarKind,
    ) -> Result<&SimulationScalarArchive> {
        self.scalar(kind).ok_or_else(|| BessemerError::new("simulation metadata", kind.stream_name(), FailureClass::Missing,
            format!("missing root stream {}; expected an existing bare four-byte scalar; structural creation is not implied", kind.stream_name())))
    }

    pub(super) fn revalidated(
        &self,
        carrier: CompoundFile,
        scalars: [Option<SimulationScalarArchive>; 2],
        guids: Option<super::SimulationGuidArchive>,
    ) -> Result<Self> {
        validate_scalar_cache(&carrier, &scalars)?;
        validate_guid_cache(&carrier, guids.as_ref())?;
        Ok(Self {
            carrier,
            scalars,
            guids,
            limits: self.limits,
        })
    }
}

fn read_guids(
    carrier: &CompoundFile,
    maximum: usize,
) -> Result<Option<super::SimulationGuidArchive>> {
    let Some(entry) = optional_entry(carrier, "TimeStampGUIDs")? else {
        return Ok(None);
    };
    super::SimulationGuidArchive::from_bytes(carrier.stream(entry.id())?, maximum).map(Some)
}

fn validate_guid_cache(
    carrier: &CompoundFile,
    expected: Option<&super::SimulationGuidArchive>,
) -> Result<()> {
    let entry = optional_entry(carrier, "TimeStampGUIDs")?;
    validate_guid_presence(entry.is_some(), expected.is_some())?;
    if let (Some(entry), Some(expected)) = (entry, expected) {
        require(
            carrier.stream(entry.id())? == expected.bytes.as_ref(),
            "TimeStampGUIDs cache",
            format!(
                "reopened root differs; expected {} exact cached bytes including any tail",
                expected.bytes.len()
            ),
        )?;
    }
    Ok(())
}

fn read_scalar(
    carrier: &CompoundFile,
    kind: SimulationScalarKind,
) -> Result<Option<SimulationScalarArchive>> {
    let Some(entry) = optional_entry(carrier, kind.stream_name())? else {
        return Ok(None);
    };
    SimulationScalarArchive::from_bytes(kind, carrier.stream(entry.id())?).map(Some)
}

fn validate_scalar_cache(
    carrier: &CompoundFile,
    scalars: &[Option<SimulationScalarArchive>; 2],
) -> Result<()> {
    for kind in [
        SimulationScalarKind::TimeStamp,
        SimulationScalarKind::RunState,
    ] {
        let actual = read_scalar(carrier, kind)?;
        let expected = scalars[kind.index()]
            .as_ref()
            .map(SimulationScalarArchive::pattern);
        require(
            actual.as_ref().map(SimulationScalarArchive::pattern) == expected,
            kind.stream_name(),
            format!(
                "reopened pattern {:?}; expected cached {expected:?}",
                actual.as_ref().map(SimulationScalarArchive::pattern)
            ),
        )?;
    }
    Ok(())
}

fn optional_entry<'a>(
    carrier: &'a CompoundFile,
    name: &str,
) -> Result<Option<&'a crate::compound::CompoundEntry>> {
    match carrier.entry_at_path(name) {
        Ok(entry) => Ok(Some(entry)),
        Err(error) if error.class == FailureClass::Missing => Ok(None),
        Err(error) => Err(error),
    }
}

fn validate_guid_presence(actual: bool, expected: bool) -> Result<()> {
    require(
        actual == expected,
        "TimeStampGUIDs cache",
        format!(
            "root presence {}; expected {} from the preserved snapshot",
            actual, expected
        ),
    )
}
