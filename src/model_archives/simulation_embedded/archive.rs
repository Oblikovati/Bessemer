use super::{SimulationEmbeddedProfile, decoder};
use crate::Result;
use crate::archive_codec::{ArchiveRevision, ObjectMapSnapshot};
use crate::model_archives::{
    ModelArchiveCoverage, ModelArchiveDiagnostic, ModelField, ModelFieldLimits,
};
use std::sync::Arc;

/// Immutable complete embedded archive with an explicitly selected helper profile.
#[derive(Debug, Clone)]
pub struct SimulationEmbeddedArchive {
    pub(super) profile: SimulationEmbeddedProfile,
    pub(super) limits: ModelFieldLimits,
    pub(super) fields: Arc<[ModelField]>,
    pub(super) map: ObjectMapSnapshot,
    pub(super) revision: ArchiveRevision,
    pub(super) wire_bytes: usize,
}

impl SimulationEmbeddedArchive {
    pub(crate) fn from_prefix(
        bytes: &[u8],
        profile: SimulationEmbeddedProfile,
        limits: ModelFieldLimits,
    ) -> Result<(Self, usize)> {
        let (fields, map, consumed) = decoder::parse_prefix(bytes, profile, limits)?;
        let archive = Self {
            profile,
            limits,
            fields: fields.into(),
            map,
            revision: ArchiveRevision::default(),
            wire_bytes: consumed,
        };
        Ok((archive, consumed))
    }

    /// Read one complete embedded root through EOF; e.g. `Self::from_bytes(bytes, profile, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        profile: SimulationEmbeddedProfile,
        limits: ModelFieldLimits,
    ) -> Result<Self> {
        let (fields, map) = decoder::parse(bytes, profile, limits)?;
        Ok(Self {
            profile,
            limits,
            fields: fields.into(),
            map,
            revision: ArchiveRevision::default(),
            wire_bytes: bytes.len(),
        })
    }

    /// Construct and reparse the complete owned shape; e.g. `Self::from_fields(source.fields().to_vec(), profile, limits)?`.
    pub fn from_fields(
        fields: Vec<ModelField>,
        profile: SimulationEmbeddedProfile,
        limits: ModelFieldLimits,
    ) -> Result<Self> {
        let bytes = crate::model_archives::encode_fields(&fields, limits.maximum_bytes)?;
        let archive = Self::from_bytes(&bytes, profile, limits)?;
        if archive.fields() == fields {
            return Ok(archive);
        }
        Err(crate::model_archives::field_mismatch::construction_mismatch(&fields, archive.fields()))
    }

    /// Borrow physical fields in stored order; e.g. `archive.fields()`.
    pub fn fields(&self) -> &[ModelField] {
        &self.fields
    }

    /// Reconstruct every stored bit and framing tag; e.g. `archive.to_bytes()?`.
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        crate::model_archives::encode_fields(&self.fields, self.limits.maximum_bytes)
    }

    /// Report exact stored-archive validation, independently of geometry; e.g. `archive.coverage().wire_bytes`.
    pub fn coverage(&self) -> ModelArchiveCoverage {
        crate::model_archives::validation::field_coverage(&self.fields, self.wire_bytes, 0)
    }

    /// Report excluded geometry and external document checks; e.g. `archive.diagnostics()`.
    pub fn diagnostics(&self) -> &[ModelArchiveDiagnostic] {
        crate::model_archives::validation::DIAGNOSTICS
    }

    /// Return the complete object-map extent, including null; e.g. `archive.object_map_slots()`.
    pub fn object_map_slots(&self) -> usize {
        self.map.slot_count()
    }
}
