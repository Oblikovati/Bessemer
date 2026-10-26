use std::collections::BTreeMap;

use crate::Result;
use crate::archive_codec::{ArchiveClass, ArchiveMapIndex, ArchiveString};
use crate::history::RevisionStamp;

use super::{
    ExternalFileReference, HeaderBox, HeaderRecord, LogRecord, ObjectList, StringArray, invalid,
};

/// Archive version supplied by the document's `_MO_VERSION_<n>` directory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArchiveVersion(u32);

impl ArchiveVersion {
    /// Retains a normalized unsigned archive version.
    ///
    /// ```
    /// assert_eq!(bessemer::header::ArchiveVersion::new(19000).value(), 19000);
    /// ```
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the unsigned version controlling archive layout.
    ///
    /// ```
    /// # use bessemer::header::ArchiveVersion;
    /// assert_eq!(ArchiveVersion::new(9000).value(), 9000);
    /// ```
    pub const fn value(self) -> u32 {
        self.0
    }

    /// Parses a model stream name, normalizing signed 32-bit directory values.
    ///
    /// ```
    /// # use bessemer::header::ArchiveVersion;
    /// assert_eq!(ArchiveVersion::from_stream_name("_MO_VERSION_-1/History")?.value(), u32::MAX);
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn from_stream_name(name: &str) -> Result<Self> {
        let token = name
            .strip_prefix("_MO_VERSION_")
            .and_then(|tail| tail.split_once('/'))
            .map(|(version, _)| version)
            .ok_or_else(|| {
                invalid(format!(
                    "stream {name:?}; expected _MO_VERSION_<decimal>/..."
                ))
            })?;
        Self::decimal(token)
    }

    pub(super) fn decimal(token: &str) -> Result<Self> {
        let value = token.parse::<i64>().map_err(|error| {
            invalid(format!(
                "version {token:?}: {error}; expected signed i32 or unsigned u32 decimal"
            ))
        })?;
        if value < i64::from(i32::MIN) || value > i64::from(u32::MAX) {
            return Err(invalid(format!(
                "version {token:?}; expected {}..={}",
                i32::MIN,
                u32::MAX
            )));
        }
        Ok(Self(value as u32))
    }
}

/// Object identity within one header graph; wire indices may change on reconstruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeaderObjectId(pub(crate) ArchiveMapIndex);

impl HeaderObjectId {
    /// Creates an identity for constructing or extending a typed graph.
    ///
    /// ```
    /// assert_eq!(bessemer::header::HeaderObjectId::new(2)?.value(), 2);
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn new(value: u32) -> Result<Self> {
        ArchiveMapIndex::nonzero(value).map(Self)
    }

    /// Returns this graph's local identity value.
    ///
    /// ```
    /// # use bessemer::header::HeaderObjectId;
    /// assert_eq!(HeaderObjectId::new(4)?.value(), 4);
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn value(self) -> u32 {
        self.0.value()
    }
}

/// A stored null pointer or reference, distinct from an absent version-gated field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderReference {
    /// Stored null object pointer.
    Null,
    /// Shared object in the archive-wide map.
    Object(HeaderObjectId),
}

/// Raw box-presence word and its conditional body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderBounds {
    /// Original i32 presence word; any nonzero value selects the box.
    pub has_box: i32,
    /// Present exactly when `has_box` is nonzero.
    pub bounds: Option<HeaderBox>,
}

/// Version-dependent shared-file path or name representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SharedFileName {
    /// Direct CString before archive version 0xFA7.
    Text(ArchiveString),
    /// Reference to a `moCStringHandle_c` object from version 0xFA7.
    Handle(HeaderReference),
}

/// Fully typed body of one supported header-family class.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeaderObject {
    /// `moHeader_c`.
    Header(Box<HeaderRecord>),
    /// `su_CStringArray`.
    Strings(StringArray),
    /// `suObList`.
    List(ObjectList),
    /// `moExtObjectList_c`.
    ExternalList(ObjectList),
    /// `moLogs_c`.
    Log(LogRecord),
    /// `moStamp_c`.
    Stamp(RevisionStamp),
    /// `moCStringHandle_c`.
    StringHandle(ArchiveString),
    /// Modern `moExtObject_c`.
    External(Box<ExternalFileReference>),
}

/// Class schema and body stored at an object identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderNode {
    /// MFC schema, independent of archive version.
    pub schema: u16,
    /// Complete known class body.
    pub object: HeaderObject,
}

impl HeaderNode {
    /// Keeps the stored schema with its native class, e.g. `node.archive_class()` for model imports.
    pub(crate) fn archive_class(&self) -> ArchiveClass {
        ArchiveClass {
            name: self.object.kind().name().to_owned(),
            schema: self.schema,
        }
    }
}

/// Owned graph submitted for complete header validation before publication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderContents {
    /// Identity of the `moHeader_c` root.
    pub root: HeaderObjectId,
    /// Reachable objects, preserving sharing without mutable pointer aliases.
    pub objects: BTreeMap<HeaderObjectId, HeaderNode>,
}
