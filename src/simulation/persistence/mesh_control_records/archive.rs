use super::super::thermal;
use super::{SimulationMeshControlField, SimulationMeshControlLimits};
use crate::{Result, simulation::SimulationPersistVersion};
use std::sync::Arc;

/// Immutable complete counted MeshControl records under an explicit outer version.
#[derive(Debug, Clone)]
pub struct SimulationMeshControlRecords {
    pub(super) parent: Arc<[u8]>,
    pub(super) revision: crate::archive_codec::ArchiveRevision,
    pub(super) fields: Arc<[SimulationMeshControlField]>,
    pub(super) offsets: Arc<[(usize, usize)]>,
    pub(super) version: SimulationPersistVersion,
    pub(super) entries: usize,
    pub(super) records: Arc<[super::SimulationMeshControlRecordSpan]>,
    pub(super) limits: SimulationMeshControlLimits,
}

impl SimulationMeshControlRecords {
    /// Read one mandatory complete body through EOF; e.g. `Self::from_bytes(bytes, version, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        version: SimulationPersistVersion,
        limits: SimulationMeshControlLimits,
    ) -> Result<Self> {
        let (reader, records) =
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
        })
    }

    /// Construct the complete matching shape and revalidate it; e.g. `Self::from_fields(source.fields().to_vec(), version, limits)?`.
    pub fn from_fields(
        fields: Vec<SimulationMeshControlField>,
        version: SimulationPersistVersion,
        limits: SimulationMeshControlLimits,
    ) -> Result<Self> {
        let bytes = thermal::encode_fields(&fields, limits.maximum_bytes)?;
        let property = Self::from_bytes(&bytes, version, limits)?;
        if property.fields() != fields {
            return Err(thermal::construction_mismatch(
                &fields,
                property.fields(),
                "MeshControl records construction",
            ));
        }
        Ok(property)
    }

    /// Borrow immutable complete field values; e.g. `property.fields()`.
    pub fn fields(&self) -> &[SimulationMeshControlField] {
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
    /// Borrow every physical record/base boundary in wire order; e.g. `body.record_spans()`.
    pub fn record_spans(&self) -> &[super::SimulationMeshControlRecordSpan] {
        &self.records
    }
}

impl super::super::load_snapshot::BinaryLoadSnapshot for SimulationMeshControlRecords {
    fn fields(&self) -> &[SimulationMeshControlField] {
        self.fields()
    }
    fn field_id(&self, ordinal: usize) -> Result<super::SimulationMeshControlFieldId> {
        self.field_id(ordinal)
    }
    fn field_offsets(&self) -> &[(usize, usize)] {
        self.field_offsets()
    }
    fn wire_bytes(&self) -> &[u8] {
        &self.parent
    }
    fn subject(&self) -> &'static str {
        "MeshControl records"
    }
}
