use super::super::{load_reader::BinaryLoadReader, thermal};
use super::{SimulationBeamField, SimulationBeamLimits, decode, fields};
use crate::{
    Result,
    simulation::{SimulationPersistVersion, invalid},
};
use std::sync::Arc;

/// Complete immutable beam child Property under an explicit persisted version.
#[derive(Debug, Clone)]
pub struct SimulationBeamProperty {
    pub(super) parent: Arc<[u8]>,
    pub(super) revision: crate::archive_codec::ArchiveRevision,
    pub(super) wire_fields: Arc<[thermal::SimulationThermalField]>,
    pub(super) fields: Arc<[SimulationBeamField]>,
    pub(super) offsets: Arc<[(usize, usize)]>,
    pub(super) version: SimulationPersistVersion,
    pub(super) limits: SimulationBeamLimits,
}

impl SimulationBeamProperty {
    /// Read the complete body through exact EOF; e.g. `Self::from_bytes(bytes, version, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        version: SimulationPersistVersion,
        limits: SimulationBeamLimits,
    ) -> Result<Self> {
        let mut reader =
            BinaryLoadReader::new(bytes, version, limits, "beam child Property input")?;
        decode::read(&mut reader)?;
        reader.finish()?;
        let fields = reader
            .fields
            .iter()
            .map(fields::project_field)
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            parent: reader.bytes,
            revision: Default::default(),
            wire_fields: reader.fields.into(),
            fields: fields.into(),
            offsets: reader.offsets.into(),
            version,
            limits,
        })
    }

    /// Construct and validate every typed field; e.g. `Self::from_fields(source.fields().to_vec(), version, limits)?`.
    pub fn from_fields(
        fields: Vec<SimulationBeamField>,
        version: SimulationPersistVersion,
        limits: SimulationBeamLimits,
    ) -> Result<Self> {
        let wire_fields: Vec<thermal::SimulationThermalField> =
            fields.iter().map(fields::wire_field).collect();
        let bytes = thermal::encode_fields(&wire_fields, limits.maximum_bytes)?;
        let archive = Self::from_bytes(&bytes, version, limits)?;
        if archive.fields() != fields {
            return Err(construction_error(&fields, archive.fields()));
        }
        Ok(archive)
    }

    /// Borrow every complete typed physical value; e.g. `property.fields()`.
    pub fn fields(&self) -> &[SimulationBeamField] {
        &self.fields
    }

    /// Borrow every body-relative physical span; e.g. `property.field_offsets()`.
    pub fn field_offsets(&self) -> &[(usize, usize)] {
        &self.offsets
    }

    /// Return normalized stored-version context; e.g. `property.version()`.
    pub fn version(&self) -> SimulationPersistVersion {
        self.version
    }

    /// Reconstruct every raw stored byte; e.g. `property.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.parent.to_vec()
    }
}

fn construction_error(
    supplied: &[SimulationBeamField],
    expected: &[SimulationBeamField],
) -> crate::BessemerError {
    let index = supplied
        .iter()
        .zip(expected)
        .position(|(a, b)| a != b)
        .unwrap_or(supplied.len().min(expected.len()));
    invalid(
        "beam child Property construction",
        format!(
            "field {index}: {:?}; expected {:?} in the complete typed shape",
            supplied.get(index),
            expected.get(index)
        ),
    )
}
