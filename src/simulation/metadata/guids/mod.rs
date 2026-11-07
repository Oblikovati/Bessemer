mod codec;
mod edit;
mod types;
mod writer;
use crate::archive_codec::ArchiveRevision;
use crate::simulation::{Result, invalid, require};
pub use edit::*;
use std::sync::Arc;
pub use types::*;
pub use writer::*;

/// Preserved optional four-string stream; no GUID syntax or field role is assigned.
#[derive(Debug, Clone)]
pub struct SimulationGuidArchive {
    pub(super) bytes: Arc<[u8]>,
    pub(super) fields: [SimulationGuidField; 4],
    pub(super) end: usize,
    pub(super) revision: SimulationGuidRevision,
    pub(super) maximum: usize,
}

impl SimulationGuidArchive {
    /// Decode four frames and explicitly retain any unknown tail, e.g. `SimulationGuidArchive::from_bytes(stream, maximum)?`.
    pub fn from_bytes(bytes: &[u8], maximum: usize) -> Result<Self> {
        let revision = SimulationGuidRevision(ArchiveRevision::default());
        let (fields, end) = codec::decode(bytes, maximum, &revision)?;
        let archive = Self {
            bytes: bytes.into(),
            fields,
            end,
            revision,
            maximum,
        };
        require(
            codec::encode(&archive.parts(), maximum)? == bytes,
            "TimeStampGUIDs reconstruction",
            format!(
                "{} bytes differ from complete field/tail reconstruction; expected exact preserved source",
                bytes.len()
            ),
        )?;
        Ok(archive)
    }

    /// Construct all stored fields and tail without applying a caller creation gate, e.g. `SimulationGuidArchive::from_parts(parts, maximum)?`.
    pub fn from_parts(parts: SimulationGuidParts, maximum: usize) -> Result<Self> {
        Self::from_bytes(&codec::encode(&parts, maximum)?, maximum)
    }

    /// Exact stored parts, with null and allocated-empty distinctions, e.g. `archive.parts()`.
    pub fn parts(&self) -> SimulationGuidParts {
        SimulationGuidParts {
            fields: self.fields.each_ref().map(|field| field.units.clone()),
            tail: self.bytes[self.end..].into(),
        }
    }

    /// Entire exact preserved stream, including unknown tail, e.g. `archive.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    /// All four distinct frames in stored order, e.g. `archive.fields()`.
    pub fn fields(&self) -> &[SimulationGuidField; 4] {
        &self.fields
    }

    /// Unknown trailing bytes, e.g. `archive.tail()`; these never imply complete stream acceptance.
    pub fn tail(&self) -> &[u8] {
        &self.bytes[self.end..]
    }

    /// Immutable string-stream identity, e.g. `archive.revision()`.
    pub fn revision(&self) -> &SimulationGuidRevision {
        &self.revision
    }

    /// Resolve a revision-bound stored field, e.g. `archive.field(&id)?`.
    pub fn field(&self, id: &SimulationGuidFieldId) -> Result<&SimulationGuidField> {
        require(
            id.revision == self.revision,
            "TimeStampGUIDs field identity",
            format!(
                "field {} from foreign revision; expected this preserved string stream",
                id.index
            ),
        )?;
        self.fields.get(id.index).ok_or_else(|| {
            invalid(
                "TimeStampGUIDs field",
                format!("index {}; expected 0..4", id.index),
            )
        })
    }

    /// Exact count/content/terminator frame, e.g. `archive.field_bytes(&id)?`.
    pub fn field_bytes(&self, id: &SimulationGuidFieldId) -> Result<&[u8]> {
        let field = self.field(id)?;
        Ok(&self.bytes[field.stream_offset..field.stream_offset + 4 + field.byte_length as usize])
    }

    /// Exact known-frame coverage and explicit incomplete-tail state, e.g. `archive.coverage().completion`.
    pub fn coverage(&self) -> SimulationGuidCoverage {
        SimulationGuidCoverage {
            mode: SimulationGuidCheckMode::ExactFourFramesAndTailPreservation,
            source_bytes: self.bytes.len(),
            decoded_bytes: self.end,
            unassigned_tail_bytes: self.bytes.len() - self.end,
            transfer_prefix: false,
            completion: if self.end == self.bytes.len() {
                SimulationGuidStreamCompletion::CompleteFourFields
            } else {
                SimulationGuidStreamCompletion::IncompleteUnassignedTail
            },
        }
    }
}
