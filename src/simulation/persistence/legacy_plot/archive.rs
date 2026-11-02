use super::super::{load_reader::BinaryLoadReader, load_snapshot::BinaryLoadSnapshot, thermal};
use super::{
    SimulationLegacyPlotField, SimulationLegacyPlotFieldId, SimulationLegacyPlotLimits, decode,
    values,
};
use crate::{Result, archive_codec::ArchiveRevision, simulation::SimulationPersistVersion};
use std::sync::Arc;

/// Immutable complete historical helper body; parsing does not assert native caller acceptance.
#[derive(Debug, Clone)]
pub struct SimulationLegacyPlot {
    pub(super) parent: Arc<[u8]>,
    pub(super) revision: ArchiveRevision,
    pub(super) fields: Arc<[SimulationLegacyPlotField]>,
    pub(super) wire_fields: Arc<[thermal::SimulationThermalField]>,
    pub(super) offsets: Arc<[(usize, usize)]>,
    pub(super) version: SimulationPersistVersion,
    pub(super) entries: usize,
    pub(super) limits: SimulationLegacyPlotLimits,
}

impl SimulationLegacyPlot {
    /// Read one complete version-selected helper through EOF; e.g. `Self::from_bytes(bytes, version, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        version: SimulationPersistVersion,
        limits: SimulationLegacyPlotLimits,
    ) -> Result<Self> {
        let mut reader = BinaryLoadReader::new(
            bytes,
            version,
            limits.wire_limits(),
            "historical plot input",
        )?;
        decode::read(&mut reader, limits.maximum_characters)?;
        reader.finish()?;
        let entries = reader.declared_entries();
        let fields = values::typed_fields(&reader.fields, limits)?;
        Ok(Self {
            parent: reader.bytes,
            revision: Default::default(),
            fields: fields.into(),
            wire_fields: reader.fields.into(),
            offsets: reader.offsets.into(),
            version,
            entries,
            limits,
        })
    }

    /// Construct and revalidate the complete exact field shape; e.g. `Self::from_fields(source.fields().to_vec(), version, limits)?`.
    pub fn from_fields(
        fields: Vec<SimulationLegacyPlotField>,
        version: SimulationPersistVersion,
        limits: SimulationLegacyPlotLimits,
    ) -> Result<Self> {
        let bytes = thermal::encode_fields(&values::wire_fields(&fields), limits.maximum_bytes)?;
        let archive = Self::from_bytes(&bytes, version, limits)?;
        if archive.fields() != fields {
            return Err(thermal::construction_mismatch(
                &fields,
                archive.fields(),
                "historical plot construction",
            ));
        }
        Ok(archive)
    }

    /// Borrow complete immutable physical fields; e.g. `plot.fields()`.
    pub fn fields(&self) -> &[SimulationLegacyPlotField] {
        &self.fields
    }

    /// Borrow every exact stored span in physical order; e.g. `plot.field_offsets()`.
    pub fn field_offsets(&self) -> &[(usize, usize)] {
        &self.offsets
    }

    /// Return the once-normalized supplied version; e.g. `plot.version()`.
    pub fn version(&self) -> SimulationPersistVersion {
        self.version
    }

    /// Return only the recovered caller version gate, not runtime open success; e.g. `plot.named_stream_dispatch_enabled()`.
    pub fn named_stream_dispatch_enabled(&self) -> bool {
        self.version.value() < 700335
    }

    /// Preserve every original byte without native conversions; e.g. `plot.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.parent.to_vec()
    }
}

impl BinaryLoadSnapshot for SimulationLegacyPlot {
    fn fields(&self) -> &[thermal::SimulationThermalField] {
        &self.wire_fields
    }
    fn field_id(&self, ordinal: usize) -> Result<SimulationLegacyPlotFieldId> {
        self.field_id(ordinal)
    }
    fn field_offsets(&self) -> &[(usize, usize)] {
        self.field_offsets()
    }
    fn wire_bytes(&self) -> &[u8] {
        &self.parent
    }
    fn subject(&self) -> &'static str {
        "historical plot"
    }
}
