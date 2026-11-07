use super::super::{load_reader::BinaryLoadReader, thermal};
use super::{SimulationSolidBodyField, SimulationSolidBodyLimits, decode};
use crate::{Result, simulation::SimulationPersistVersion};
use std::sync::Arc;

/// Immutable complete single-body SolidManager solid-body Property payload.
#[derive(Debug, Clone)]
pub struct SimulationSolidBodyProperty {
    pub(super) parent: Arc<[u8]>,
    pub(super) revision: crate::archive_codec::ArchiveRevision,
    pub(super) fields: Arc<[SimulationSolidBodyField]>,
    pub(super) offsets: Arc<[(usize, usize)]>,
    pub(super) version: SimulationPersistVersion,
    pub(super) limits: SimulationSolidBodyLimits,
}

impl SimulationSolidBodyProperty {
    /// Read one mandatory complete body through EOF; e.g. `Self::from_bytes(bytes, version, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        version: SimulationPersistVersion,
        limits: SimulationSolidBodyLimits,
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
        fields: Vec<SimulationSolidBodyField>,
        version: SimulationPersistVersion,
        limits: SimulationSolidBodyLimits,
    ) -> Result<Self> {
        let bytes = thermal::encode_fields(&fields, limits.maximum_bytes)?;
        let property = Self::from_bytes(&bytes, version, limits)?;
        if property.fields() != fields {
            return Err(thermal::construction_mismatch(
                &fields,
                property.fields(),
                "solid-body Property construction",
            ));
        }
        Ok(property)
    }

    /// Borrow immutable complete field values; e.g. `property.fields()`.
    pub fn fields(&self) -> &[SimulationSolidBodyField] {
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

impl super::super::load_snapshot::BinaryLoadSnapshot for SimulationSolidBodyProperty {
    fn fields(&self) -> &[SimulationSolidBodyField] {
        self.fields()
    }
    fn field_id(&self, ordinal: usize) -> Result<super::SimulationSolidBodyFieldId> {
        self.field_id(ordinal)
    }
    fn field_offsets(&self) -> &[(usize, usize)] {
        self.field_offsets()
    }
    fn wire_bytes(&self) -> &[u8] {
        &self.parent
    }
    fn subject(&self) -> &'static str {
        "solid-body Property"
    }
}

fn read_body(
    bytes: &[u8],
    version: SimulationPersistVersion,
    limits: SimulationSolidBodyLimits,
) -> Result<BinaryLoadReader<'_>> {
    let mut reader = BinaryLoadReader::new(bytes, version, limits, "solid-body Property input")?;
    decode::read(&mut reader)?;
    reader.finish()?;
    Ok(reader)
}
