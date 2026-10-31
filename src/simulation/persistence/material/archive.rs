use super::super::{load_reader::BinaryLoadReader, thermal};
use super::{
    SimulationMaterialBoundary, SimulationMaterialField, SimulationMaterialLimits, decode,
    types::MaterialStatistics,
};
use crate::{Result, simulation::SimulationPersistVersion};
use std::sync::Arc;

/// Immutable complete study-manager Material body under an explicit version and boundary.
#[derive(Debug, Clone)]
pub struct SimulationStudyMaterial {
    pub(super) parent: Arc<[u8]>,
    pub(super) revision: crate::archive_codec::ArchiveRevision,
    pub(super) fields: Arc<[SimulationMaterialField]>,
    pub(super) offsets: Arc<[(usize, usize)]>,
    pub(super) version: SimulationPersistVersion,
    pub(super) limits: SimulationMaterialLimits,
    pub(super) statistics: MaterialStatistics,
    pub(super) boundary: SimulationMaterialBoundary,
}

impl SimulationStudyMaterial {
    /// Read a complete standalone body through exact EOF; e.g. `Self::from_bytes(bytes, version, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        version: SimulationPersistVersion,
        limits: SimulationMaterialLimits,
    ) -> Result<Self> {
        read(
            bytes,
            version,
            limits,
            SimulationMaterialBoundary::Standalone,
        )
    }

    /// Read one complete bounded body while excluding the following carrier; e.g. `let (body, used) = Self::from_prefix(bytes, version, limits)?;`.
    pub fn from_prefix(
        bytes: &[u8],
        version: SimulationPersistVersion,
        limits: SimulationMaterialLimits,
    ) -> Result<(Self, usize)> {
        let body = read(bytes, version, limits, SimulationMaterialBoundary::Prefix)?;
        let consumed = body.parent.len();
        Ok((body, consumed))
    }

    /// Construct and revalidate the complete supplied shape; e.g. `Self::from_fields(source.fields().to_vec(), version, limits)?`.
    pub fn from_fields(
        fields: Vec<SimulationMaterialField>,
        version: SimulationPersistVersion,
        limits: SimulationMaterialLimits,
    ) -> Result<Self> {
        let bytes = thermal::encode_fields(&fields, limits.maximum_bytes)?;
        let body = Self::from_bytes(&bytes, version, limits)?;
        if body.fields() != fields {
            return Err(thermal::construction_mismatch(
                &fields,
                body.fields(),
                "study-manager Material construction",
            ));
        }
        Ok(body)
    }

    /// Borrow every immutable physical value; e.g. `body.fields()`.
    pub fn fields(&self) -> &[SimulationMaterialField] {
        &self.fields
    }

    /// Borrow every complete body-relative field span; e.g. `body.field_offsets()`.
    pub fn field_offsets(&self) -> &[(usize, usize)] {
        &self.offsets
    }

    /// Return the explicit normalized version; e.g. `body.version()`.
    pub fn version(&self) -> SimulationPersistVersion {
        self.version
    }

    /// Reconstruct only this complete body with all raw bits intact; e.g. `body.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.parent.to_vec()
    }
}

fn read(
    bytes: &[u8],
    version: SimulationPersistVersion,
    limits: SimulationMaterialLimits,
    boundary: SimulationMaterialBoundary,
) -> Result<SimulationStudyMaterial> {
    let input = match boundary {
        SimulationMaterialBoundary::Standalone => bytes,
        SimulationMaterialBoundary::Prefix => &bytes[..bytes.len().min(limits.maximum_bytes)],
    };
    let mut reader = BinaryLoadReader::new(
        input,
        version,
        reader_limits(limits),
        "study-manager Material input",
    )?;
    let statistics = decode::read(&mut reader)?;
    if boundary == SimulationMaterialBoundary::Standalone {
        reader.finish()?;
    }
    publish(reader, statistics, version, limits, boundary)
}

fn publish(
    reader: BinaryLoadReader<'_>,
    statistics: MaterialStatistics,
    version: SimulationPersistVersion,
    limits: SimulationMaterialLimits,
    boundary: SimulationMaterialBoundary,
) -> Result<SimulationStudyMaterial> {
    let parts = reader.into_body_parts()?;
    Ok(SimulationStudyMaterial {
        parent: parts.bytes,
        revision: Default::default(),
        fields: parts.fields.into(),
        offsets: parts.offsets.into(),
        version,
        limits,
        statistics,
        boundary,
    })
}

fn reader_limits(
    limits: SimulationMaterialLimits,
) -> super::super::thermal::SimulationThermalLimits {
    super::super::thermal::SimulationThermalLimits {
        maximum_bytes: limits.maximum_bytes,
        maximum_fields: limits.maximum_fields,
        maximum_entries: limits.maximum_entries,
        maximum_text_bytes: limits.maximum_text_bytes,
        embedded: crate::model_archives::ModelArchiveLimits {
            maximum_bytes: 0,
            maximum_fields: 0,
            maximum_entries: 0,
            maximum_depth: 0,
            maximum_objects: 0,
        },
    }
}
