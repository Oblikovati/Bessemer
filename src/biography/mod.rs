//! Biography records, environment snapshots, and explicit string abbreviation (clause 11.9).
mod abbreviations;
mod codec;
mod types;

use crate::archive_codec::PreservedContents;
pub use crate::archive_codec::{ArchiveString, TextEncoding};
use crate::{BessemerError, FailureClass, Result};
pub use types::{BiographyRecord, EnvironmentPath, EnvironmentSnapshot};

/// Immutable biography with raw stored spellings and byte-preserving equivalent edits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BiographyArchive {
    schema: u16,
    records: PreservedContents<Vec<BiographyRecord>>,
}

impl BiographyArchive {
    /// Decodes the complete standalone biography without querying the local machine.
    ///
    /// ```no_run
    /// let archive = bessemer::biography::BiographyArchive::from_bytes(&std::fs::read("biography.bin")?)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let (schema, records) = codec::decode(bytes)?;
        Ok(Self {
            schema,
            records: PreservedContents::decoded(bytes, records),
        })
    }

    /// Constructs an archive, including a completed empty record collection.
    ///
    /// ```
    /// let archive = bessemer::biography::BiographyArchive::new(0, Vec::new())?;
    /// assert!(archive.records().is_empty());
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn new(schema: u16, records: Vec<BiographyRecord>) -> Result<Self> {
        codec::checked_length(&records)?;
        Ok(Self {
            schema,
            records: PreservedContents::constructed(records),
        })
    }

    /// Returns the MFC root schema, separately from each product file version.
    ///
    /// ```
    /// assert_eq!(bessemer::biography::BiographyArchive::new(7, Vec::new())?.root_schema(), 7);
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn root_schema(&self) -> u16 {
        self.schema
    }

    /// Borrows immutable records with their stored `*` spellings intact.
    ///
    /// ```
    /// let archive = bessemer::biography::BiographyArchive::new(0, Vec::new())?;
    /// assert_eq!(archive.records().len(), 0);
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn records(&self) -> &[BiographyRecord] {
        self.records.current()
    }

    /// Returns an isolated archive with validated replacement records.
    ///
    /// ```
    /// let archive = bessemer::biography::BiographyArchive::new(0, Vec::new())?;
    /// let edited = archive.with_records(archive.records().to_vec())?;
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn with_records(&self, records: Vec<BiographyRecord>) -> Result<Self> {
        codec::checked_length(&records)?;
        Ok(Self {
            schema: self.schema,
            records: self.records.edited(records),
        })
    }

    /// Expands repeated-string chains into a separate owned semantic view.
    ///
    /// ```
    /// let archive = bessemer::biography::BiographyArchive::new(0, Vec::new())?;
    /// assert!(archive.expanded_records().is_empty());
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn expanded_records(&self) -> Vec<BiographyRecord> {
        abbreviations::expanded(self.records())
    }

    /// Returns explicit native-style abbreviations without altering the archive.
    ///
    /// ```
    /// let archive = bessemer::biography::BiographyArchive::new(0, Vec::new())?;
    /// let abbreviated = archive.with_records(archive.abbreviated_records()?)?;
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn abbreviated_records(&self) -> Result<Vec<BiographyRecord>> {
        abbreviations::abbreviated(self.records())
    }

    /// Serializes raw records, preserving original bytes for equivalent edits and reverts.
    ///
    /// ```
    /// let archive = bessemer::biography::BiographyArchive::new(0, Vec::new())?;
    /// let copied = bessemer::biography::BiographyArchive::from_bytes(&archive.to_bytes()?)?;
    /// assert!(copied.records().is_empty());
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        if let Some(bytes) = self.records.original_bytes() {
            return Ok(bytes.to_vec());
        }
        codec::encode(self.schema, self.records())
    }
}

fn invalid(cause: impl Into<String>) -> BessemerError {
    BessemerError {
        operation: "biography archive",
        entity: "biography".to_owned(),
        class: FailureClass::Invalid,
        cause: cause.into(),
    }
}
