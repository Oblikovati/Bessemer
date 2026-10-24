use std::collections::BTreeMap;

use crate::Result;
use crate::archive_codec::ArchiveMapIndex;

use super::ArchiveString;

/// Identity of a shared date-code object within one typed history graph.
///
/// Serialization may assign a different wire map index while retaining sharing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateCodeId(pub(crate) ArchiveMapIndex);

impl DateCodeId {
    /// Creates an identity for a newly constructed history graph.
    ///
    /// ```
    /// let identity = bessemer::history::DateCodeId::new(4)?;
    /// assert_eq!(identity.value(), 4);
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn new(value: u32) -> Result<Self> {
        ArchiveMapIndex::nonzero(value).map(Self)
    }

    /// Returns the graph identity; it is not a date-code value.
    ///
    /// ```
    /// # use bessemer::history::DateCodeId;
    /// assert_eq!(DateCodeId::new(7)?.value(), 7);
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn value(self) -> u32 {
        self.0.value()
    }
}

/// Raw stored stamp fields; unverified enum values remain integers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevisionStamp {
    /// Stored `moRevisionStatus_e`; its complete domain is not established.
    pub revision_status: u16,
    /// Uninterpreted member labelled by its native in-memory offset.
    pub unknown_0c: u16,
    /// Uninterpreted member labelled by its native in-memory offset.
    pub unknown_10: u16,
    /// Four-byte `su_CTime` archive value.
    pub time: u32,
    /// Comment with its original CString representation.
    pub comment: ArchiveString,
}

/// Date-code branch selected by a state's stored file version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateDateCodes {
    /// No date-code bytes for versions below 0x175.
    Absent,
    /// One raw date-code word for versions 0x175 through 0x177.
    Legacy(u32),
    /// A shared date-code object for versions at least 0x178.
    History(DateCodeId),
}

/// One direct version-state body, without its own object tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionState {
    /// Revision stamp.
    pub stamp: RevisionStamp,
    /// Version controlling this state's date-code layout.
    pub file_version: u32,
    /// Version-dependent date-code representation.
    pub date_codes: StateDateCodes,
}

/// A `moDateCodeHistory_c` object shared by one or more states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateCodeHistory {
    /// MFC class schema retained independently of the file version.
    pub schema: u16,
    /// Stored words, including all date-code flags.
    pub codes: Vec<u32>,
}

/// Tail fields present when the last state has version at least 0x319.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryMemory {
    /// Raw failure severity; enum values are unverified.
    pub max_failure_severity: i32,
    /// Stored low 32 bits of the minimum free-memory observation; units are unknown.
    pub min_free_memory: u32,
}

/// Uninterpreted tail fields present from last-state version 0x46C.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryRecent {
    /// Raw native member +0x38.
    pub unknown_38: u16,
    /// Raw native member +0x3C.
    pub unknown_3c: i32,
    /// Raw native member +0x40.
    pub unknown_40: i32,
}

/// Optional tail groups, selected by the last state's stored version.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HistoryTail {
    /// Present from version 0x319.
    pub memory: Option<HistoryMemory>,
    /// Uninterpreted native member +0x30, present from version 0x45C.
    pub unknown_30: Option<i32>,
    /// Present from version 0x46C.
    pub recent: Option<HistoryRecent>,
}

/// An owned history graph used to construct an immutable published history.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryContents {
    /// Nonempty state list in stored order.
    pub states: Vec<VersionState>,
    /// Date-code objects keyed by stable identities within this graph.
    pub date_histories: BTreeMap<DateCodeId, DateCodeHistory>,
    /// Groups selected by the last state's version.
    pub tail: HistoryTail,
}
