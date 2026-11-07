use super::super::{load_base, thermal};
use super::{SimulationInertialField, SimulationInertialKind, SimulationInertialLimits, tails};
use crate::{Result, simulation::SimulationPersistVersion};
use std::sync::Arc;

/// Immutable complete Gravity, CentriFugal or BaseExcitation collection.
#[derive(Debug, Clone)]
pub struct SimulationInertialCollection {
    pub(super) parent: Arc<[u8]>,
    pub(super) revision: crate::archive_codec::ArchiveRevision,
    pub(super) fields: Arc<[SimulationInertialField]>,
    pub(super) offsets: Arc<[(usize, usize)]>,
    pub(super) version: SimulationPersistVersion,
    pub(super) kind: SimulationInertialKind,
    pub(super) record_count: usize,
    pub(super) limits: SimulationInertialLimits,
}

impl SimulationInertialCollection {
    /// Read the entire selected grammar through EOF; e.g. `Self::from_bytes(bytes, version, kind, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        version: SimulationPersistVersion,
        kind: SimulationInertialKind,
        limits: SimulationInertialLimits,
    ) -> Result<Self> {
        let (reader, count) =
            load_base::collection(bytes, version, limits, "inertial input", |reader, label| {
                tails::read(reader, label, kind)
            })?;
        Ok(Self {
            parent: reader.bytes,
            revision: Default::default(),
            fields: reader.fields.into(),
            offsets: reader.offsets.into(),
            version,
            kind,
            record_count: count,
            limits,
        })
    }

    /// Construct the complete matching shape and revalidate it; e.g. `Self::from_fields(source.fields().to_vec(), version, kind, limits)?`.
    pub fn from_fields(
        fields: Vec<SimulationInertialField>,
        version: SimulationPersistVersion,
        kind: SimulationInertialKind,
        limits: SimulationInertialLimits,
    ) -> Result<Self> {
        let bytes = thermal::encode_fields(&fields, limits.maximum_bytes)?;
        let collection = Self::from_bytes(&bytes, version, kind, limits)?;
        if collection.fields() != fields {
            return Err(thermal::construction_mismatch(
                &fields,
                collection.fields(),
                "inertial construction",
            ));
        }
        Ok(collection)
    }

    /// Borrow immutable complete field values; e.g. `collection.fields()`.
    pub fn fields(&self) -> &[SimulationInertialField] {
        &self.fields
    }

    /// Return every physical field span; e.g. `collection.field_offsets()`.
    pub fn field_offsets(&self) -> &[(usize, usize)] {
        &self.offsets
    }

    /// Return explicit normalized outer context; e.g. `collection.version()`.
    pub fn version(&self) -> SimulationPersistVersion {
        self.version
    }

    /// Return the selected family; e.g. `collection.kind()`.
    pub fn kind(&self) -> SimulationInertialKind {
        self.kind
    }

    /// Return the complete declared record count; e.g. `collection.record_count()`.
    pub fn record_count(&self) -> usize {
        self.record_count
    }

    /// Reconstruct every stored byte without runtime conversions; e.g. `collection.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.parent.to_vec()
    }
}

impl super::super::load_snapshot::BinaryLoadSnapshot for SimulationInertialCollection {
    fn fields(&self) -> &[SimulationInertialField] {
        self.fields()
    }
    fn field_id(&self, ordinal: usize) -> Result<super::SimulationInertialFieldId> {
        self.field_id(ordinal)
    }
    fn field_offsets(&self) -> &[(usize, usize)] {
        self.field_offsets()
    }
    fn wire_bytes(&self) -> &[u8] {
        &self.parent
    }
    fn subject(&self) -> &'static str {
        "inertial"
    }
}
