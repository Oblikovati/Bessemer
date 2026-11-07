//! Standalone `_MO_VERSION_<n>/History` archives (specification clause 11.7).

pub(crate) mod decode;
pub(crate) mod encode;
mod types;
mod validate;

use crate::archive_codec::PreservedContents;

pub use crate::archive_codec::{ArchiveString, TextEncoding};
pub use types::{
    DateCodeHistory, DateCodeId, HistoryContents, HistoryMemory, HistoryRecent, HistoryTail,
    RevisionStamp, StateDateCodes, VersionState,
};

use crate::{BessemerError, FailureClass, Result};

/// Immutable history graph with byte-preserving no-op serialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionHistory {
    root_schema: u16,
    contents: PreservedContents<HistoryContents>,
}

impl VersionHistory {
    /// Decodes and validates a complete standalone history stream.
    ///
    /// ```no_run
    /// let bytes = std::fs::read("history.bin")?;
    /// let history = bessemer::history::VersionHistory::from_bytes(&bytes)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let (root_schema, contents) = decode::history(bytes).map_err(decode_context)?;
        validate::contents(&contents)?;
        Ok(Self {
            root_schema,
            contents: PreservedContents::decoded(bytes, contents),
        })
    }

    /// Constructs a new history after checking versions, references, and tail groups.
    ///
    /// ```no_run
    /// # let source = bessemer::history::VersionHistory::from_bytes(&std::fs::read("history.bin")?)?;
    /// let rebuilt = bessemer::history::VersionHistory::new(source.root_schema(), source.contents().clone())?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(root_schema: u16, contents: HistoryContents) -> Result<Self> {
        validate::contents(&contents)?;
        Ok(Self {
            root_schema,
            contents: PreservedContents::constructed(contents),
        })
    }

    /// Returns the root class schema, separately from each state's file version.
    ///
    /// ```no_run
    /// # let history = bessemer::history::VersionHistory::from_bytes(&std::fs::read("history.bin")?)?;
    /// println!("root class schema {}", history.root_schema());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn root_schema(&self) -> u16 {
        self.root_schema
    }

    /// Borrows the immutable decoded graph.
    ///
    /// ```no_run
    /// # let history = bessemer::history::VersionHistory::from_bytes(&std::fs::read("history.bin")?)?;
    /// println!("{} states", history.contents().states.len());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn contents(&self) -> &HistoryContents {
        self.contents.current()
    }

    /// Validates an owned replacement graph and returns an isolated history.
    ///
    /// ```no_run
    /// # use bessemer::history::{VersionHistory, ArchiveString, TextEncoding};
    /// # let source = VersionHistory::from_bytes(&std::fs::read("history.bin")?)?;
    /// let mut edited = source.contents().clone();
    /// edited.states[0].stamp.comment = ArchiveString::new("Reviewed", TextEncoding::Utf16)?;
    /// let result = source.with_contents(edited)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn with_contents(&self, contents: HistoryContents) -> Result<Self> {
        validate::contents(&contents)?;
        Ok(Self {
            root_schema: self.root_schema,
            contents: self.contents.edited(contents),
        })
    }

    /// Encodes the graph, retaining the exact original bytes for equivalent content.
    ///
    /// ```no_run
    /// # let history = bessemer::history::VersionHistory::from_bytes(&std::fs::read("history.bin")?)?;
    /// std::fs::write("history-copy.bin", history.to_bytes()?)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        if let Some(original) = self.contents.original_bytes() {
            return Ok(original.to_vec());
        }
        encode::history(self.root_schema, self.contents.current())
    }
}

pub(super) fn invalid(cause: impl Into<String>) -> BessemerError {
    BessemerError::new("version history", "History", FailureClass::Invalid, cause)
}

fn decode_context(mut error: BessemerError) -> BessemerError {
    error.operation = "decode version history";
    if !error.entity.starts_with("History") {
        error.entity = format!("History {}", error.entity);
    }
    error
}
