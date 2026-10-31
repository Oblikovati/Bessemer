use super::super::thermal;
use super::{SimulationMeshPropertyField, SimulationMeshPropertyLimits};
use crate::{Result, simulation::SimulationPersistVersion};
use std::sync::Arc;

/// Immutable complete Mesh Property under an explicit outer version.
#[derive(Debug, Clone)]
pub struct SimulationMeshProperty {
    pub(super) parent: Arc<[u8]>,
    pub(super) revision: crate::archive_codec::ArchiveRevision,
    pub(super) fields: Arc<[SimulationMeshPropertyField]>,
    pub(super) offsets: Arc<[(usize, usize)]>,
    pub(super) version: SimulationPersistVersion,
    pub(super) entries: usize,
    pub(super) nested_records: usize,
    pub(super) records: Arc<[super::SimulationMeshPropertyRecordSpan]>,
    pub(super) limits: SimulationMeshPropertyLimits,
}

impl SimulationMeshProperty {
    /// Read one mandatory complete body through EOF; e.g. `Self::from_bytes(bytes, version, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        version: SimulationPersistVersion,
        limits: SimulationMeshPropertyLimits,
    ) -> Result<Self> {
        let (reader, records, nested_records) =
            super::decode::read(bytes, version, limits).map_err(super::decode::frame_error)?;
        let entries = reader.declared_entries();
        Ok(Self {
            parent: reader.bytes,
            revision: Default::default(),
            fields: reader.fields.into(),
            offsets: reader.offsets.into(),
            version,
            limits,
            records: records.into(),
            entries,
            nested_records,
        })
    }

    /// Construct the complete matching shape and revalidate it; e.g. `Self::from_fields(source.fields().to_vec(), version, limits)?`.
    pub fn from_fields(
        fields: Vec<SimulationMeshPropertyField>,
        version: SimulationPersistVersion,
        limits: SimulationMeshPropertyLimits,
    ) -> Result<Self> {
        let bytes = thermal::encode_fields(&fields, limits.maximum_bytes)?;
        let property = Self::from_bytes(&bytes, version, limits)?;
        if property.fields() != fields {
            return Err(thermal::construction_mismatch(
                &fields,
                property.fields(),
                "Mesh Property construction",
            ));
        }
        Ok(property)
    }

    /// Borrow immutable complete field values; e.g. `property.fields()`.
    pub fn fields(&self) -> &[SimulationMeshPropertyField] {
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
    /// Borrow every physical direct-record boundary in wire order; e.g. `body.record_spans()`.
    pub fn record_spans(&self) -> &[super::SimulationMeshPropertyRecordSpan] {
        &self.records
    }
}

impl super::super::load_snapshot::BinaryLoadSnapshot for SimulationMeshProperty {
    fn fields(&self) -> &[SimulationMeshPropertyField] {
        self.fields()
    }
    fn field_id(&self, ordinal: usize) -> Result<super::SimulationMeshPropertyFieldId> {
        self.field_id(ordinal)
    }
    fn field_offsets(&self) -> &[(usize, usize)] {
        self.field_offsets()
    }
    fn wire_bytes(&self) -> &[u8] {
        &self.parent
    }
    fn subject(&self) -> &'static str {
        "Mesh Property"
    }
}
