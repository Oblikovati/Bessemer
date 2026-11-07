use super::super::{load_reader::BinaryLoadReader, thermal};
use super::{SimulationShellManagerField, SimulationShellManagerLimits, decode};
use crate::{Result, simulation::SimulationPersistVersion};
use std::sync::Arc;

/// Immutable complete single-body direct ShellManager Property payload.
#[derive(Debug, Clone)]
pub struct SimulationShellManagerProperty {
    pub(super) parent: Arc<[u8]>,
    pub(super) revision: crate::archive_codec::ArchiveRevision,
    pub(super) fields: Arc<[SimulationShellManagerField]>,
    pub(super) offsets: Arc<[(usize, usize)]>,
    pub(super) version: SimulationPersistVersion,
    pub(super) limits: SimulationShellManagerLimits,
    pub(super) profile: super::super::SimulationShellWireProfile,
}

impl SimulationShellManagerProperty {
    /// Read one mandatory complete body through EOF; e.g. `Self::from_bytes(bytes, version, profile, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        version: SimulationPersistVersion,
        profile: super::super::SimulationShellWireProfile,
        limits: SimulationShellManagerLimits,
    ) -> Result<Self> {
        let reader = read_body(bytes, version, profile, limits)?;
        Ok(Self {
            parent: reader.bytes,
            revision: Default::default(),
            fields: reader.fields.into(),
            offsets: reader.offsets.into(),
            version,
            limits,
            profile,
        })
    }

    /// Construct the complete matching shape and revalidate it; e.g. `Self::from_fields(source.fields().to_vec(), version, profile, limits)?`.
    pub fn from_fields(
        fields: Vec<SimulationShellManagerField>,
        version: SimulationPersistVersion,
        profile: super::super::SimulationShellWireProfile,
        limits: SimulationShellManagerLimits,
    ) -> Result<Self> {
        let bytes = thermal::encode_fields(&fields, limits.maximum_bytes)?;
        let property = Self::from_bytes(&bytes, version, profile, limits)?;
        if property.fields() != fields {
            return Err(thermal::construction_mismatch(
                &fields,
                property.fields(),
                "ShellManager Property construction",
            ));
        }
        Ok(property)
    }

    /// Borrow immutable complete field values; e.g. `property.fields()`.
    pub fn fields(&self) -> &[SimulationShellManagerField] {
        &self.fields
    }

    /// Return every physical field span; e.g. `property.field_offsets()`.
    pub fn field_offsets(&self) -> &[(usize, usize)] {
        &self.offsets
    }

    /// Return explicit normalized outer context; e.g. `property.version()`.
    pub fn version(&self) -> SimulationPersistVersion {
        self.version
    }

    /// Return the selected historical-reader/current-writer framing; e.g. `property.wire_profile()`.
    pub fn wire_profile(&self) -> super::super::SimulationShellWireProfile {
        self.profile
    }

    /// Reconstruct every stored byte without runtime conversions; e.g. `property.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.parent.to_vec()
    }
}

impl super::super::load_snapshot::BinaryLoadSnapshot for SimulationShellManagerProperty {
    fn fields(&self) -> &[SimulationShellManagerField] {
        self.fields()
    }
    fn field_id(&self, ordinal: usize) -> Result<super::SimulationShellManagerFieldId> {
        self.field_id(ordinal)
    }
    fn field_offsets(&self) -> &[(usize, usize)] {
        self.field_offsets()
    }
    fn wire_bytes(&self) -> &[u8] {
        &self.parent
    }
    fn subject(&self) -> &'static str {
        "ShellManager Property"
    }
}

fn read_body(
    bytes: &[u8],
    version: SimulationPersistVersion,
    profile: super::super::SimulationShellWireProfile,
    limits: SimulationShellManagerLimits,
) -> Result<BinaryLoadReader<'_>> {
    let reader_limits = super::super::load_reader::outer_limits(
        limits.maximum_bytes,
        limits.maximum_fields,
        limits.maximum_entries,
        limits.maximum_text_bytes,
    );
    let mut reader = BinaryLoadReader::new(
        bytes,
        version,
        reader_limits,
        "direct ShellManager Property input",
    )?;
    decode::read(&mut reader, profile)?;
    reader.finish()?;
    Ok(reader)
}
