use super::{
    SimulationThermalField, SimulationThermalKind, SimulationThermalLimits, SimulationThermalValue,
    base, tails,
};
use crate::Result;
use crate::simulation::{SimulationPersistVersion, invalid};
use std::sync::Arc;

/// Immutable complete binary thermal collection with independent embedded object contexts.
#[derive(Debug, Clone)]
pub struct SimulationThermalCollection {
    pub(super) parent: Arc<[u8]>,
    pub(super) version: SimulationPersistVersion,
    pub(super) kind: SimulationThermalKind,
    pub(super) fields: Arc<[SimulationThermalField]>,
    offsets: Arc<[(usize, usize)]>,
    pub(super) record_count: usize,
    pub(super) revision: crate::archive_codec::ArchiveRevision,
    pub(super) limits: SimulationThermalLimits,
}

impl SimulationThermalCollection {
    /// Read all records and require exact EOF; e.g. `Self::from_bytes(bytes, version, kind, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        version: SimulationPersistVersion,
        kind: SimulationThermalKind,
        limits: SimulationThermalLimits,
    ) -> Result<Self> {
        let (reader, count) =
            base::collection(bytes, version, limits, "thermal input", |reader, label| {
                tails::read(reader, label, kind)
            })?;
        Ok(Self {
            parent: reader.bytes,
            version,
            kind,
            fields: reader.fields.into(),
            offsets: reader.offsets.into(),
            record_count: count,
            revision: Default::default(),
            limits,
        })
    }

    /// Construct and verify complete owned field parts; e.g. `Self::from_fields(source.fields().to_vec(), version, kind, limits)?`.
    pub fn from_fields(
        fields: Vec<SimulationThermalField>,
        version: SimulationPersistVersion,
        kind: SimulationThermalKind,
        limits: SimulationThermalLimits,
    ) -> Result<Self> {
        let bytes = super::encode_fields(&fields, limits.maximum_bytes)?;
        let collection = Self::from_bytes(&bytes, version, kind, limits)?;
        if collection.fields() != fields {
            return Err(super::construction_mismatch(
                &fields,
                collection.fields(),
                "thermal construction",
            ));
        }
        Ok(collection)
    }

    /// Borrow immutable typed outer fields and nested archives; e.g. `collection.fields()`.
    pub fn fields(&self) -> &[SimulationThermalField] {
        &self.fields
    }

    /// Return exact physical spans corresponding to fields; e.g. `collection.field_offsets()`.
    pub fn field_offsets(&self) -> &[(usize, usize)] {
        &self.offsets
    }

    /// Return the explicit outer Simulation version; e.g. `collection.version()`.
    pub fn version(&self) -> SimulationPersistVersion {
        self.version
    }

    /// Return the selected collection family; e.g. `collection.kind()`.
    pub fn kind(&self) -> SimulationThermalKind {
        self.kind
    }

    /// Distinguish a completed empty collection from failures; e.g. `collection.record_count()`.
    pub fn record_count(&self) -> usize {
        self.record_count
    }

    /// Reconstruct every field without native insertion or deduplication; e.g. `collection.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.parent.to_vec()
    }
}

pub(in crate::simulation::persistence) fn construction_mismatch(
    supplied: &[SimulationThermalField],
    parsed: &[SimulationThermalField],
    entity: &str,
) -> crate::BessemerError {
    let index = supplied
        .iter()
        .zip(parsed)
        .position(|(left, right)| left != right)
        .unwrap_or(supplied.len().min(parsed.len()));
    invalid(
        entity,
        format!(
            "field {index} {:?}; expected selected grammar field {:?}",
            supplied.get(index),
            parsed.get(index)
        ),
    )
}

fn encoded_size(value: &SimulationThermalValue) -> usize {
    match value {
        SimulationThermalValue::Signed(_) | SimulationThermalValue::Unsigned(_) => 4,
        SimulationThermalValue::DoubleBits(_) => 8,
        SimulationThermalValue::Text(text) => 4 + text.payload().len(),
        SimulationThermalValue::Bytes(bytes) => bytes.len(),
        SimulationThermalValue::Embedded(archive) => archive.coverage().wire_bytes,
    }
}

pub(in crate::simulation::persistence) fn encode_fields(
    fields: &[SimulationThermalField],
    maximum: usize,
) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    for field in fields {
        let size = encoded_size(&field.value);
        if size > maximum.saturating_sub(bytes.len()) {
            return Err(invalid(
                &field.path,
                format!("{size} bytes; expected within {maximum} output bytes"),
            ));
        }
        bytes.try_reserve(size).map_err(|error| {
            invalid(
                &field.path,
                format!("{size} bytes: {error}; expected available output storage"),
            )
        })?;
        append_value(&mut bytes, &field.value)?;
    }
    Ok(bytes)
}

fn append_value(bytes: &mut Vec<u8>, value: &SimulationThermalValue) -> Result<()> {
    match value {
        SimulationThermalValue::Signed(value) => bytes.extend_from_slice(&value.to_le_bytes()),
        SimulationThermalValue::Unsigned(value) => bytes.extend_from_slice(&value.to_le_bytes()),
        SimulationThermalValue::DoubleBits(value) => bytes.extend_from_slice(&value.to_le_bytes()),
        SimulationThermalValue::Text(text) => bytes.extend_from_slice(text.encoded()),
        SimulationThermalValue::Bytes(raw) => bytes.extend_from_slice(raw),
        SimulationThermalValue::Embedded(archive) => bytes.extend_from_slice(&archive.to_bytes()?),
    }
    Ok(())
}

impl super::super::load_snapshot::BinaryLoadSnapshot for SimulationThermalCollection {
    fn fields(&self) -> &[SimulationThermalField] {
        self.fields()
    }
    fn field_id(&self, ordinal: usize) -> Result<super::SimulationThermalFieldId> {
        self.field_id(ordinal)
    }
    fn field_offsets(&self) -> &[(usize, usize)] {
        self.field_offsets()
    }
    fn wire_bytes(&self) -> &[u8] {
        &self.parent
    }
    fn subject(&self) -> &'static str {
        "thermal"
    }
}
