use super::super::{load_reader::BinaryLoadReader, thermal};
use super::{
    SimulationSolidChildField, SimulationSolidChildKind, SimulationSolidChildLimits, decode,
};
use crate::{Result, simulation::SimulationPersistVersion};
use std::sync::Arc;

/// Immutable complete single-body SolidManager child Property payload.
#[derive(Debug, Clone)]
pub struct SimulationSolidChildProperty {
    pub(super) parent: Arc<[u8]>,
    pub(super) revision: crate::archive_codec::ArchiveRevision,
    pub(super) fields: Arc<[SimulationSolidChildField]>,
    pub(super) offsets: Arc<[(usize, usize)]>,
    pub(super) version: SimulationPersistVersion,
    pub(super) limits: SimulationSolidChildLimits,
    pub(super) entries: usize,
    pub(super) kind: SimulationSolidChildKind,
}

impl SimulationSolidChildProperty {
    /// Read one mandatory complete body through EOF; e.g. `Self::from_bytes(bytes, version, kind, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        version: SimulationPersistVersion,
        kind: SimulationSolidChildKind,
        limits: SimulationSolidChildLimits,
    ) -> Result<Self> {
        let reader = read_body(bytes, version, kind, limits)?;
        let entries = reader.declared_entries();
        Ok(Self {
            parent: reader.bytes,
            revision: Default::default(),
            fields: reader.fields.into(),
            offsets: reader.offsets.into(),
            version,
            limits,
            entries,
            kind,
        })
    }

    /// Construct the complete matching shape and revalidate it; e.g. `Self::from_fields(source.fields().to_vec(), version, kind, limits)?`.
    pub fn from_fields(
        fields: Vec<SimulationSolidChildField>,
        version: SimulationPersistVersion,
        kind: SimulationSolidChildKind,
        limits: SimulationSolidChildLimits,
    ) -> Result<Self> {
        let bytes = thermal::encode_fields(&fields, limits.maximum_bytes)?;
        let property = Self::from_bytes(&bytes, version, kind, limits)?;
        if property.fields() != fields {
            return Err(thermal::construction_mismatch(
                &fields,
                property.fields(),
                "SolidManager child Property construction",
            ));
        }
        Ok(property)
    }

    /// Borrow immutable complete field values; e.g. `property.fields()`.
    pub fn fields(&self) -> &[SimulationSolidChildField] {
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

    /// Return the selected separate child grammar; e.g. `property.kind()`.
    pub fn kind(&self) -> SimulationSolidChildKind {
        self.kind
    }

    /// Reconstruct every stored byte without runtime conversions; e.g. `property.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.parent.to_vec()
    }
}

impl super::super::load_snapshot::BinaryLoadSnapshot for SimulationSolidChildProperty {
    fn fields(&self) -> &[SimulationSolidChildField] {
        self.fields()
    }
    fn field_id(&self, ordinal: usize) -> Result<super::SimulationSolidChildFieldId> {
        self.field_id(ordinal)
    }
    fn field_offsets(&self) -> &[(usize, usize)] {
        self.field_offsets()
    }
    fn wire_bytes(&self) -> &[u8] {
        &self.parent
    }
    fn subject(&self) -> &'static str {
        "SolidManager child Property"
    }
}

fn read_body(
    bytes: &[u8],
    version: SimulationPersistVersion,
    kind: SimulationSolidChildKind,
    limits: SimulationSolidChildLimits,
) -> Result<BinaryLoadReader<'_>> {
    let mapped = super::super::load_reader::outer_limits(
        limits.maximum_bytes,
        limits.maximum_fields,
        limits.maximum_entries,
        limits.maximum_text_bytes,
    );
    let mut reader =
        BinaryLoadReader::new(bytes, version, mapped, "SolidManager child Property input")?;
    decode::read(&mut reader, kind)?;
    reader.finish()?;
    Ok(reader)
}
