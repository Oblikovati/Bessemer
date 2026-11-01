use super::super::{load_reader::BinaryLoadReader, thermal};
use super::{SimulationShellCompositeField, SimulationShellCompositeLimits};
use crate::{Result, simulation::SimulationPersistVersion};
use std::sync::Arc;

/// Immutable complete single-body shell composite payload.
#[derive(Debug, Clone)]
pub struct SimulationShellComposite {
    pub(super) parent: Arc<[u8]>,
    pub(super) revision: crate::archive_codec::ArchiveRevision,
    pub(super) fields: Arc<[SimulationShellCompositeField]>,
    pub(super) offsets: Arc<[(usize, usize)]>,
    pub(super) version: SimulationPersistVersion,
    pub(super) profile: super::SimulationShellCompositeProfile,
    pub(super) entries: usize,
    pub(super) limits: SimulationShellCompositeLimits,
}

impl SimulationShellComposite {
    /// Read one mandatory complete body through EOF; e.g. `Self::from_bytes(bytes, version, profile, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        version: SimulationPersistVersion,
        profile: super::SimulationShellCompositeProfile,
        limits: SimulationShellCompositeLimits,
    ) -> Result<Self> {
        let reader = read_body(bytes, version, profile, limits)?;
        let entries = reader.declared_entries();
        Ok(Self {
            parent: reader.bytes,
            revision: Default::default(),
            fields: reader.fields.into(),
            offsets: reader.offsets.into(),
            version,
            limits,
            profile,
            entries,
        })
    }

    /// Construct the complete matching shape and revalidate it; e.g. `Self::from_fields(source.fields().to_vec(), version, profile, limits)?`.
    pub fn from_fields(
        fields: Vec<SimulationShellCompositeField>,
        version: SimulationPersistVersion,
        profile: super::SimulationShellCompositeProfile,
        limits: SimulationShellCompositeLimits,
    ) -> Result<Self> {
        let bytes = thermal::encode_fields(&fields, limits.maximum_bytes)?;
        let property = Self::from_bytes(&bytes, version, profile, limits)?;
        if property.fields() != fields {
            return Err(thermal::construction_mismatch(
                &fields,
                property.fields(),
                "shell composite construction",
            ));
        }
        Ok(property)
    }

    /// Borrow immutable complete field values; e.g. `property.fields()`.
    pub fn fields(&self) -> &[SimulationShellCompositeField] {
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

    /// Reconstruct every stored byte without runtime conversions; e.g. `property.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.parent.to_vec()
    }
    /// Return the normalized stream/direction context; e.g. `body.profile()`.
    pub fn profile(&self) -> super::SimulationShellCompositeProfile {
        self.profile
    }
}

impl super::super::load_snapshot::BinaryLoadSnapshot for SimulationShellComposite {
    fn fields(&self) -> &[SimulationShellCompositeField] {
        self.fields()
    }
    fn field_id(&self, ordinal: usize) -> Result<super::SimulationShellCompositeFieldId> {
        self.field_id(ordinal)
    }
    fn field_offsets(&self) -> &[(usize, usize)] {
        self.field_offsets()
    }
    fn wire_bytes(&self) -> &[u8] {
        &self.parent
    }
    fn subject(&self) -> &'static str {
        "shell composite"
    }
}

fn read_body(
    bytes: &[u8],
    version: SimulationPersistVersion,
    profile: super::SimulationShellCompositeProfile,
    limits: SimulationShellCompositeLimits,
) -> Result<BinaryLoadReader<'_>> {
    let mut reader = BinaryLoadReader::new(bytes, version, limits, "shell composite input")?;
    super::decode::read(&mut reader, profile)?;
    reader.finish()?;
    Ok(reader)
}
