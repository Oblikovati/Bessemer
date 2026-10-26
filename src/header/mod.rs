//! Typed header archives and their shared external-reference graphs (clause 11.8).

mod context;
mod decode;
mod encode;
mod fields;
mod kinds;
mod records;
mod types;
mod validate;
mod values;

#[cfg(test)]
pub(crate) use fields::{FieldRule, WireKind};

#[cfg(test)]
pub(crate) fn external_field_rules() -> &'static [FieldRule] {
    ExternalFileReference::RULES
}

use crate::archive_codec::PreservedContents;
use crate::{BessemerError, FailureClass, Result};

pub use crate::archive_codec::{ArchiveString, HeaderBox, StoredDouble, TextEncoding};
pub use records::{ExternalFileReference, HeaderRecord, LogRecord, ObjectList, StringArray};
pub use types::{
    ArchiveVersion, HeaderBounds, HeaderContents, HeaderNode, HeaderObject, HeaderObjectId,
    HeaderReference, SharedFileName,
};

/// Immutable, validated header archive with byte-preserving no-op serialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentHeader {
    version: ArchiveVersion,
    contents: PreservedContents<HeaderContents>,
}

impl DocumentHeader {
    /// Decodes a complete header with an explicit object-allocation budget.
    ///
    /// ```no_run
    /// # use bessemer::header::{ArchiveVersion, DocumentHeader};
    /// let header = DocumentHeader::from_bytes(&std::fs::read("header.bin")?, ArchiveVersion::new(19000), 100_000)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_bytes(
        bytes: &[u8],
        version: ArchiveVersion,
        maximum_objects: usize,
    ) -> Result<Self> {
        let contents = decode::header(bytes, version, maximum_objects)?;
        validate::contents(&contents, version)?;
        Ok(Self {
            version,
            contents: PreservedContents::decoded(bytes, contents),
        })
    }

    /// Constructs an archive after validating field gates and every reachable reference.
    ///
    /// ```no_run
    /// # use bessemer::header::{ArchiveVersion, DocumentHeader};
    /// # let original = DocumentHeader::from_bytes(&std::fs::read("header.bin")?, ArchiveVersion::new(19000), 100_000)?;
    /// let rebuilt = DocumentHeader::new(original.archive_version(), original.contents().clone())?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(version: ArchiveVersion, contents: HeaderContents) -> Result<Self> {
        validate::contents(&contents, version)?;
        Ok(Self {
            version,
            contents: PreservedContents::constructed(contents),
        })
    }

    /// Returns the normalized outer version, separate from any embedded override.
    ///
    /// ```no_run
    /// # use bessemer::header::*;
    /// # let header = DocumentHeader::from_bytes(&std::fs::read("header.bin")?, ArchiveVersion::new(19000), 100_000)?;
    /// assert_eq!(header.archive_version().value(), 19000);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn archive_version(&self) -> ArchiveVersion {
        self.version
    }

    /// Borrows the immutable graph, including all stored unknown fields.
    ///
    /// ```no_run
    /// # use bessemer::header::*;
    /// # let header = DocumentHeader::from_bytes(&std::fs::read("header.bin")?, ArchiveVersion::new(19000), 100_000)?;
    /// println!("{} objects", header.contents().objects.len());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn contents(&self) -> &HeaderContents {
        self.contents.current()
    }

    /// Validates an owned edit and returns a result isolated from the source archive.
    ///
    /// ```no_run
    /// # use bessemer::header::*;
    /// # let header = DocumentHeader::from_bytes(&std::fs::read("header.bin")?, ArchiveVersion::new(19000), 100_000)?;
    /// let edited = header.with_contents(header.contents().clone())?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn with_contents(&self, contents: HeaderContents) -> Result<Self> {
        validate::contents(&contents, self.version)?;
        Ok(Self {
            version: self.version,
            contents: self.contents.edited(contents),
        })
    }

    /// Emits original bytes for equivalent graphs, rebuilding changed archives iteratively.
    ///
    /// ```no_run
    /// # use bessemer::header::*;
    /// # let header = DocumentHeader::from_bytes(&std::fs::read("header.bin")?, ArchiveVersion::new(19000), 100_000)?;
    /// std::fs::write("header-copy.bin", header.to_bytes()?)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        if let Some(bytes) = self.contents.original_bytes() {
            return Ok(bytes.to_vec());
        }
        encode::header(self.contents())
    }
}

fn invalid(cause: impl Into<String>) -> BessemerError {
    BessemerError {
        operation: "header archive",
        entity: "document header".to_owned(),
        class: FailureClass::Invalid,
        cause: cause.into(),
    }
}

fn unsupported(cause: impl Into<String>) -> BessemerError {
    BessemerError {
        operation: "header archive",
        entity: "document header".to_owned(),
        class: FailureClass::Unsupported,
        cause: cause.into(),
    }
}
