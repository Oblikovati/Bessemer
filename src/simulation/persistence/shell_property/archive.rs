use super::super::{load_reader::BinaryLoadReader, thermal};
use super::{
    SimulationShellPropertyContext, SimulationShellPropertyField, SimulationShellPropertyLimits,
    types::ShellPropertyStatistics,
};
use crate::{Result, simulation::SimulationPersistVersion};
use std::sync::Arc;

/// Immutable complete Shells/MidSurfShells Property body under explicit runtime context.
#[derive(Debug, Clone)]
pub struct SimulationShellProperty {
    pub(super) parent: Arc<[u8]>,
    pub(super) revision: crate::archive_codec::ArchiveRevision,
    pub(super) fields: Arc<[SimulationShellPropertyField]>,
    pub(super) offsets: Arc<[(usize, usize)]>,
    pub(super) version: SimulationPersistVersion,
    pub(super) context: SimulationShellPropertyContext,
    pub(super) entries: usize,
    pub(super) limits: SimulationShellPropertyLimits,
    pub(super) statistics: ShellPropertyStatistics,
}

impl SimulationShellProperty {
    /// Read one complete mandatory Property through EOF; e.g. `Self::from_bytes(bytes, version, context, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        version: SimulationPersistVersion,
        context: SimulationShellPropertyContext,
        limits: SimulationShellPropertyLimits,
    ) -> Result<Self> {
        let mut reader = BinaryLoadReader::new(bytes, version, limits, "shell Property input")
            .map_err(super::decode::frame_error)?;
        let statistics =
            super::decode::read(&mut reader, &context).map_err(super::decode::frame_error)?;
        reader.finish().map_err(super::decode::frame_error)?;
        let entries = reader.declared_entries();
        Ok(Self {
            parent: reader.bytes,
            revision: Default::default(),
            fields: reader.fields.into(),
            offsets: reader.offsets.into(),
            version,
            context,
            entries,
            limits,
            statistics,
        })
    }

    /// Construct and revalidate every supplied physical field; e.g. `Self::from_fields(source.fields().to_vec(), version, context, limits)?`.
    pub fn from_fields(
        fields: Vec<SimulationShellPropertyField>,
        version: SimulationPersistVersion,
        context: SimulationShellPropertyContext,
        limits: SimulationShellPropertyLimits,
    ) -> Result<Self> {
        let bytes = thermal::encode_fields(&fields, limits.maximum_bytes)?;
        let property = Self::from_bytes(&bytes, version, context, limits)?;
        if property.fields() != fields {
            return Err(thermal::construction_mismatch(
                &fields,
                property.fields(),
                "shell Property construction",
            ));
        }
        Ok(property)
    }

    /// Borrow all immutable complete physical values; e.g. `property.fields()`.
    pub fn fields(&self) -> &[SimulationShellPropertyField] {
        &self.fields
    }

    /// Borrow every physical parent-relative span; e.g. `property.field_offsets()`.
    pub fn field_offsets(&self) -> &[(usize, usize)] {
        &self.offsets
    }

    /// Return the normalized outer version; e.g. `property.version()`.
    pub fn version(&self) -> SimulationPersistVersion {
        self.version
    }

    /// Borrow the explicit immutable runtime context; e.g. `property.context().kind`.
    pub fn context(&self) -> &SimulationShellPropertyContext {
        &self.context
    }

    /// Reconstruct every saved bit without runtime normalization; e.g. `property.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.parent.to_vec()
    }
}

impl super::super::load_snapshot::BinaryLoadSnapshot for SimulationShellProperty {
    fn fields(&self) -> &[SimulationShellPropertyField] {
        self.fields()
    }
    fn field_id(&self, ordinal: usize) -> Result<super::SimulationShellPropertyFieldId> {
        self.field_id(ordinal)
    }
    fn field_offsets(&self) -> &[(usize, usize)] {
        self.field_offsets()
    }
    fn wire_bytes(&self) -> &[u8] {
        &self.parent
    }
    fn subject(&self) -> &'static str {
        "shell Property"
    }
}
