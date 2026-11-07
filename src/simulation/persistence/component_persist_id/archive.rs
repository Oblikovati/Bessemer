use super::super::{load_reader::BinaryLoadReader, thermal};
use super::{SimulationComponentPersistIdField, SimulationComponentPersistIdLimits};
use crate::{Result, simulation::SimulationPersistVersion};
use std::sync::Arc;

/// Immutable complete single-body standalone Component PersistID payload.
#[derive(Debug, Clone)]
pub struct SimulationComponentPersistId {
    pub(super) parent: Arc<[u8]>,
    pub(super) revision: crate::archive_codec::ArchiveRevision,
    pub(super) fields: Arc<[SimulationComponentPersistIdField]>,
    pub(super) offsets: Arc<[(usize, usize)]>,
    pub(super) version: SimulationPersistVersion,
    pub(super) limits: SimulationComponentPersistIdLimits,
}

impl SimulationComponentPersistId {
    /// Read one mandatory complete body through EOF; e.g. `Self::from_bytes(bytes, version, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        version: SimulationPersistVersion,
        limits: SimulationComponentPersistIdLimits,
    ) -> Result<Self> {
        let reader = read_body(bytes, version, limits)?;
        Ok(Self {
            parent: reader.bytes,
            revision: Default::default(),
            fields: reader.fields.into(),
            offsets: reader.offsets.into(),
            version,
            limits,
        })
    }

    /// Construct the complete matching shape and revalidate it; e.g. `Self::from_fields(source.fields().to_vec(), version, limits)?`.
    pub fn from_fields(
        fields: Vec<SimulationComponentPersistIdField>,
        version: SimulationPersistVersion,
        limits: SimulationComponentPersistIdLimits,
    ) -> Result<Self> {
        let bytes = thermal::encode_fields(&fields, limits.maximum_bytes)?;
        let property = Self::from_bytes(&bytes, version, limits)?;
        if property.fields() != fields {
            return Err(thermal::construction_mismatch(
                &fields,
                property.fields(),
                "standalone Component PersistID construction",
            ));
        }
        Ok(property)
    }

    /// Borrow immutable complete field values; e.g. `property.fields()`.
    pub fn fields(&self) -> &[SimulationComponentPersistIdField] {
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
}

impl super::super::load_snapshot::BinaryLoadSnapshot for SimulationComponentPersistId {
    fn fields(&self) -> &[SimulationComponentPersistIdField] {
        self.fields()
    }
    fn field_id(&self, ordinal: usize) -> Result<super::SimulationComponentPersistIdFieldId> {
        self.field_id(ordinal)
    }
    fn field_offsets(&self) -> &[(usize, usize)] {
        self.field_offsets()
    }
    fn wire_bytes(&self) -> &[u8] {
        &self.parent
    }
    fn subject(&self) -> &'static str {
        "standalone Component PersistID"
    }
}

fn read_body(
    bytes: &[u8],
    version: SimulationPersistVersion,
    limits: SimulationComponentPersistIdLimits,
) -> Result<BinaryLoadReader<'_>> {
    let mut reader = BinaryLoadReader::new(
        bytes,
        version,
        limits,
        "standalone Component PersistID input",
    )?;
    super::super::load_persist_id::read(&mut reader)?;
    reader.finish()?;
    Ok(reader)
}
