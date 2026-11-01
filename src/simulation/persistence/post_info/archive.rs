use super::super::{load_reader::BinaryLoadReader, thermal};
use super::{SimulationPostInfoField, SimulationPostInfoLimits, decode};
use crate::Result;
use std::sync::Arc;

/// Immutable complete single-body PostInfo payload.
#[derive(Debug, Clone)]
pub struct SimulationPostInfo {
    pub(super) parent: Arc<[u8]>,
    pub(super) revision: crate::archive_codec::ArchiveRevision,
    pub(super) fields: Arc<[SimulationPostInfoField]>,
    pub(super) wire_fields: Arc<[super::super::thermal::SimulationThermalField]>,
    pub(super) offsets: Arc<[(usize, usize)]>,
    pub(super) version: crate::simulation::SimulationPersistVersion,
    pub(super) entries: usize,
    pub(super) limits: SimulationPostInfoLimits,
}

impl SimulationPostInfo {
    /// Read one mandatory complete body through EOF; e.g. `Self::from_bytes(bytes, version, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        version: crate::simulation::SimulationPersistVersion,
        limits: SimulationPostInfoLimits,
    ) -> Result<Self> {
        let reader = read_body(bytes, version, limits)?;
        let entries = reader.declared_entries();
        let fields = super::values::from_wire_fields(&reader.fields, limits)?;
        Ok(Self {
            parent: reader.bytes,
            revision: Default::default(),
            fields: fields.into(),
            wire_fields: reader.fields.into(),
            offsets: reader.offsets.into(),
            version,
            limits,
            entries,
        })
    }

    /// Construct the complete matching shape and revalidate it; e.g. `Self::from_fields(source.fields().to_vec(), version, limits)?`.
    pub fn from_fields(
        fields: Vec<SimulationPostInfoField>,
        version: crate::simulation::SimulationPersistVersion,
        limits: SimulationPostInfoLimits,
    ) -> Result<Self> {
        let wire = super::values::to_wire_fields(&fields);
        let bytes = thermal::encode_fields(&wire, limits.maximum_bytes)?;
        let property = Self::from_bytes(&bytes, version, limits)?;
        if property.fields() != fields {
            return Err(thermal::construction_mismatch(
                &fields,
                property.fields(),
                "PostInfo construction",
            ));
        }
        Ok(property)
    }

    /// Borrow immutable complete field values; e.g. `property.fields()`.
    pub fn fields(&self) -> &[SimulationPostInfoField] {
        &self.fields
    }

    /// Return every physical field span; e.g. `property.field_offsets()`.
    pub fn field_offsets(&self) -> &[(usize, usize)] {
        &self.offsets
    }

    /// Return explicit normalized outer context; e.g. `property.version()`.
    pub fn version(&self) -> crate::simulation::SimulationPersistVersion {
        self.version
    }

    /// Reconstruct every stored byte without runtime conversions; e.g. `property.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.parent.to_vec()
    }
}

impl super::super::load_snapshot::BinaryLoadSnapshot for SimulationPostInfo {
    fn fields(&self) -> &[super::super::thermal::SimulationThermalField] {
        &self.wire_fields
    }
    fn field_id(&self, ordinal: usize) -> Result<super::SimulationPostInfoFieldId> {
        self.field_id(ordinal)
    }
    fn field_offsets(&self) -> &[(usize, usize)] {
        self.field_offsets()
    }
    fn wire_bytes(&self) -> &[u8] {
        &self.parent
    }
    fn subject(&self) -> &'static str {
        "PostInfo"
    }
}

fn read_body(
    bytes: &[u8],
    version: crate::simulation::SimulationPersistVersion,
    limits: SimulationPostInfoLimits,
) -> Result<BinaryLoadReader<'_>> {
    let mut reader = BinaryLoadReader::new(bytes, version, limits.wire_limits(), "PostInfo input")?;
    decode::read(&mut reader, limits.maximum_characters)?;
    reader.finish()?;
    Ok(reader)
}
