use super::super::{load_reader::BinaryLoadReader, thermal};
use super::{SimulationBeamGroupField, SimulationBeamGroupKind, SimulationBeamGroupLimits, decode};
use crate::{Result, simulation::SimulationPersistVersion};
use std::sync::Arc;

/// Immutable complete single-body beam joint/group/list/folder payload.
#[derive(Debug, Clone)]
pub struct SimulationBeamGroupBody {
    pub(super) parent: Arc<[u8]>,
    pub(super) revision: crate::archive_codec::ArchiveRevision,
    pub(super) fields: Arc<[SimulationBeamGroupField]>,
    pub(super) offsets: Arc<[(usize, usize)]>,
    pub(super) version: SimulationPersistVersion,
    pub(super) limits: SimulationBeamGroupLimits,
    pub(super) kind: SimulationBeamGroupKind,
}

impl SimulationBeamGroupBody {
    /// Read one mandatory complete body through EOF; e.g. `Self::from_bytes(bytes, version, kind, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        version: SimulationPersistVersion,
        kind: SimulationBeamGroupKind,
        limits: SimulationBeamGroupLimits,
    ) -> Result<Self> {
        let kind = kind.normalized()?;
        let reader = read_body(bytes, version, kind, limits)?;
        Ok(Self {
            parent: reader.bytes,
            revision: Default::default(),
            fields: reader.fields.into(),
            offsets: reader.offsets.into(),
            version,
            limits,
            kind,
        })
    }

    /// Construct the complete matching shape and revalidate it; e.g. `Self::from_fields(source.fields().to_vec(), version, kind, limits)?`.
    pub fn from_fields(
        fields: Vec<SimulationBeamGroupField>,
        version: SimulationPersistVersion,
        kind: SimulationBeamGroupKind,
        limits: SimulationBeamGroupLimits,
    ) -> Result<Self> {
        let bytes = thermal::encode_fields(&fields, limits.maximum_bytes)?;
        let property = Self::from_bytes(&bytes, version, kind, limits)?;
        if property.fields() != fields {
            return Err(thermal::construction_mismatch(
                &fields,
                property.fields(),
                "beam joint/group/list/folder body construction",
            ));
        }
        Ok(property)
    }

    /// Borrow immutable complete field values; e.g. `property.fields()`.
    pub fn fields(&self) -> &[SimulationBeamGroupField] {
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

    /// Return the explicit body kind and independent count context; e.g. `body.kind()`.
    pub fn kind(&self) -> SimulationBeamGroupKind {
        self.kind
    }

    /// Reconstruct every stored byte without runtime conversions; e.g. `property.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.parent.to_vec()
    }
}

impl super::super::load_snapshot::BinaryLoadSnapshot for SimulationBeamGroupBody {
    fn fields(&self) -> &[SimulationBeamGroupField] {
        self.fields()
    }
    fn field_id(&self, ordinal: usize) -> Result<super::SimulationBeamGroupFieldId> {
        self.field_id(ordinal)
    }
    fn field_offsets(&self) -> &[(usize, usize)] {
        self.field_offsets()
    }
    fn wire_bytes(&self) -> &[u8] {
        &self.parent
    }
    fn subject(&self) -> &'static str {
        "beam joint/group/list/folder body"
    }
}

fn read_body(
    bytes: &[u8],
    version: SimulationPersistVersion,
    kind: SimulationBeamGroupKind,
    limits: SimulationBeamGroupLimits,
) -> Result<BinaryLoadReader<'_>> {
    let mut reader =
        BinaryLoadReader::new(bytes, version, limits, "beam joint/group/list/folder input")?;
    decode::read(&mut reader, kind)?;
    reader.finish()?;
    Ok(reader)
}
