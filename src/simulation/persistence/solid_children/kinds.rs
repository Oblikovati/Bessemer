/// Distinct component and folder Property bodies; sibling payloads never become inline fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationSolidChildKind {
    /// Direct component text/words, independently bound to its storage class.
    Component,
    /// Static folder header with the documented strict negative-count rejection policy.
    Folder,
}

/// Evidence supporting the selected grammar, independent of caller-file provenance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationSolidChildEvidence {
    /// Bound component reader, copied actual parser, native corpus and owned historical profiles.
    BoundComponentReader,
    /// Bound static folder reader and independent literal vectors, with no retained native payload.
    StaticFolderProfile,
}

impl SimulationSolidChildKind {
    pub(super) fn evidence(self) -> SimulationSolidChildEvidence {
        match self {
            Self::Component => SimulationSolidChildEvidence::BoundComponentReader,
            Self::Folder => SimulationSolidChildEvidence::StaticFolderProfile,
        }
    }
}
