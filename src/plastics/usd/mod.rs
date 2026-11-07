mod construction;
mod edit;
mod framing;
mod layout;
mod outcome;
mod records;
use crate::archive_codec::ArchiveRevision;
use crate::{BessemerError, FailureClass, Result};
pub use layout::*;
pub use outcome::*;
pub use records::*;
use std::sync::Arc;

/// Caller bounds applied before any record payload is sliced or owned.
#[derive(Debug, Clone, Copy)]
pub struct UsdLimits {
    /// Maximum complete records, including empty records and text after #EOF.
    pub maximum_records: usize,
    /// Maximum byte length for each counted payload.
    pub maximum_payload_bytes: usize,
}

/// Exact character width; narrow bytes have no assumed native code page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsdCharacterWidth {
    /// Opaque single-byte character payload.
    NarrowOpaque,
    /// Raw little-endian UTF-16 units, including unpaired surrogates.
    Utf16CodeUnits,
}

impl UsdCharacterWidth {
    /// Exact bytes per counted unit, e.g. `UsdCharacterWidth::Utf16CodeUnits.byte_width()`.
    pub fn byte_width(self) -> usize {
        match self {
            Self::NarrowOpaque => 1,
            Self::Utf16CodeUnits => 2,
        }
    }
}

/// Identity of one immutable complete record stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsdRevision(ArchiveRevision);

/// Normalized record position bound to one complete USD revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsdRecordId {
    revision: UsdRevision,
    index: usize,
}

/// Complete lossless record framing, without implied writer-layout or loader acceptance.
#[derive(Debug, Clone)]
pub struct UsdArchive {
    bytes: Arc<[u8]>,
    spans: Arc<[framing::UsdRecordSpan]>,
    revision: UsdRevision,
    limits: UsdLimits,
}

impl UsdArchive {
    /// Own every complete record with explicit limits, e.g. `UsdArchive::from_bytes(bytes, limits)?`.
    pub fn from_bytes(bytes: &[u8], limits: UsdLimits) -> Result<Self> {
        let spans = framing::decode_all(bytes, limits)?;
        Ok(Self {
            bytes: bytes.into(),
            spans: spans.into(),
            revision: UsdRevision(ArchiveRevision::default()),
            limits,
        })
    }
    /// Preserve every original prefix and payload byte, e.g. `archive.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
    /// Identity retained by every normalized record position, e.g. `archive.revision()`.
    pub fn revision(&self) -> &UsdRevision {
        &self.revision
    }
    /// Complete record count, including empty records and records after #EOF, e.g. `archive.record_count()`.
    pub fn record_count(&self) -> usize {
        self.spans.len()
    }
    /// Normalize a zero-based record position once, e.g. `archive.record_id(0)?`.
    pub fn record_id(&self, index: usize) -> Result<UsdRecordId> {
        if index >= self.spans.len() {
            return Err(invalid(
                "record position",
                format!("{index}; expected less than {}", self.spans.len()),
            ));
        }
        Ok(UsdRecordId {
            revision: self.revision.clone(),
            index,
        })
    }
    /// Capture a record with its exact original stream offsets, e.g. `archive.record(&id)?`.
    pub fn record(&self, id: &UsdRecordId) -> Result<UsdStringRecord> {
        let index = self.require_record(id)?;
        Ok(UsdStringRecord {
            bytes: self.bytes.clone(),
            span: self.spans[index],
        })
    }
    fn require_record(&self, id: &UsdRecordId) -> Result<usize> {
        if id.revision != self.revision || id.index >= self.spans.len() {
            return Err(invalid(
                "record identity",
                format!(
                    "foreign revision or index {}; expected this revision and position less than {}",
                    id.index,
                    self.spans.len()
                ),
            ));
        }
        Ok(id.index)
    }
}

fn invalid(entity: &str, cause: impl Into<String>) -> BessemerError {
    BessemerError::new("Plastics USD", entity, FailureClass::Invalid, cause)
}
