use super::super::{load_reader::BinaryLoadReader, thermal};
use super::{SimulationResultsField, SimulationResultsLimits, decode};
use crate::Result;
use std::sync::Arc;

/// Immutable complete single-body Results hierarchy Property payload.
#[derive(Debug, Clone)]
pub struct SimulationResultsProperty {
    pub(super) parent: Arc<[u8]>,
    pub(super) revision: crate::archive_codec::ArchiveRevision,
    pub(super) fields: Arc<[SimulationResultsField]>,
    pub(super) offsets: Arc<[(usize, usize)]>,
    pub(super) context: super::SimulationResultsContext,
    pub(super) entries: usize,
    pub(super) limits: SimulationResultsLimits,
}

impl SimulationResultsProperty {
    /// Read one mandatory complete body through EOF; e.g. `Self::from_bytes(bytes, context, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        context: super::SimulationResultsContext,
        limits: SimulationResultsLimits,
    ) -> Result<Self> {
        let reader = read_body(bytes, context, limits)?;
        let entries = reader.declared_entries();
        Ok(Self {
            parent: reader.bytes,
            revision: Default::default(),
            fields: reader.fields.into(),
            offsets: reader.offsets.into(),
            context,
            limits,
            entries,
        })
    }

    /// Construct the complete matching shape and revalidate it; e.g. `Self::from_fields(source.fields().to_vec(), context, limits)?`.
    pub fn from_fields(
        fields: Vec<SimulationResultsField>,
        context: super::SimulationResultsContext,
        limits: SimulationResultsLimits,
    ) -> Result<Self> {
        let bytes = thermal::encode_fields(&fields, limits.maximum_bytes)?;
        let property = Self::from_bytes(&bytes, context, limits)?;
        if property.fields() != fields {
            return Err(thermal::construction_mismatch(
                &fields,
                property.fields(),
                "Results Property construction",
            ));
        }
        Ok(property)
    }

    /// Borrow immutable complete field values; e.g. `property.fields()`.
    pub fn fields(&self) -> &[SimulationResultsField] {
        &self.fields
    }

    /// Return every physical field span; e.g. `property.field_offsets()`.
    pub fn field_offsets(&self) -> &[(usize, usize)] {
        &self.offsets
    }

    /// Return explicit normalized outer context; e.g. `property.context()`.
    pub fn context(&self) -> super::SimulationResultsContext {
        self.context
    }

    /// Reconstruct every stored byte without runtime conversions; e.g. `property.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.parent.to_vec()
    }
}

impl super::super::load_snapshot::BinaryLoadSnapshot for SimulationResultsProperty {
    fn fields(&self) -> &[SimulationResultsField] {
        self.fields()
    }
    fn field_id(&self, ordinal: usize) -> Result<super::SimulationResultsFieldId> {
        self.field_id(ordinal)
    }
    fn field_offsets(&self) -> &[(usize, usize)] {
        self.field_offsets()
    }
    fn wire_bytes(&self) -> &[u8] {
        &self.parent
    }
    fn subject(&self) -> &'static str {
        "Results Property"
    }
}

fn read_body(
    bytes: &[u8],
    context: super::SimulationResultsContext,
    limits: SimulationResultsLimits,
) -> Result<BinaryLoadReader<'_>> {
    let mut reader = BinaryLoadReader::new(
        bytes,
        context.reader_version()?,
        limits,
        "Results Property input",
    )?;
    decode::read(&mut reader, context)?;
    reader.finish()?;
    Ok(reader)
}
