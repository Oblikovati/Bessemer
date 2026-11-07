use super::ComposerArchive;
use crate::archive_codec::ArchiveRevision;
use std::sync::Arc;

/// Completed changed/unchanged state shared with the other archive operations.
pub use crate::display_lists::DisplayEditCompletion as ComposerEditCompletion;

/// Validation invariant for a published Composer carrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerCheckMode {
    /// Whole-file partition and every member's complete integrity checks.
    ExactContainer,
}

/// Achieved replacement fidelity, independent of member/application semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerEditFidelity {
    /// Exact requested decoded contents and unchanged encoded representations of other members.
    ExactMemberContents,
}

/// Explicit wire, entry and decoded-byte budgets for one container operation.
#[derive(Debug, Clone, Copy)]
pub struct ComposerLimits {
    /// Maximum encoded archive bytes.
    pub maximum_bytes: usize,
    /// Maximum member count, including duplicate names.
    pub maximum_members: usize,
    /// Maximum decoded bytes for one member.
    pub maximum_member_bytes: usize,
    /// Maximum aggregate decoded bytes.
    pub maximum_expanded_bytes: usize,
}

/// The two documented ZIP compression methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerCompression {
    /// Uncompressed method 0.
    Stored,
    /// Raw-deflate method 8.
    Deflated,
}

/// Member occurrence bound to an immutable archive revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposerMemberId {
    pub(super) revision: ArchiveRevision,
    pub(super) index: usize,
}

/// Immutable directory-order member description; an identity distinguishes duplicate names.
#[derive(Debug, Clone)]
pub struct ComposerMember {
    /// Revision-bound occurrence.
    pub id: ComposerMemberId,
    /// Decoded UTF-8 or CP437 name, without path normalization.
    pub name: Arc<str>,
    /// Stored compression method.
    pub compression: ComposerCompression,
    /// Decoded byte count.
    pub expanded_bytes: u32,
    /// Encoded payload byte count.
    pub stored_bytes: u32,
    /// Declared and checked CRC-32.
    pub crc32: u32,
}

/// New member with explicit contents and compression; new names use ZIP UTF-8 framing.
#[derive(Debug, Clone)]
pub struct ComposerMemberInput {
    /// Exact member name.
    pub name: String,
    /// Owned decoded bytes.
    pub contents: Vec<u8>,
    /// Requested compression method.
    pub compression: ComposerCompression,
}

/// Exact whole-container framing and payload-integrity coverage, never geometry validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComposerCoverage {
    /// Explicit full-container check mode.
    pub mode: ComposerCheckMode,
    /// Entire wire partition checked through EOF.
    pub wire_bytes: usize,
    /// Every member whose termination, decoded size and CRC were checked.
    pub checked_members: usize,
    /// Aggregate verified decoded bytes.
    pub expanded_bytes: usize,
}

/// Checks outside the carrier operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerDiagnostic {
    /// Member XML, geometry, images and scene schemas were not validated.
    MemberSchemasOutsideScope,
    /// Native Composer application acceptance was not checked.
    NativeApplicationNotChecked,
}

/// Completed isolated replacement with exact container coverage and member correspondence.
#[derive(Debug, Clone)]
pub struct ComposerEdit {
    archive: ComposerArchive,
    relation: Option<(ComposerMemberId, ComposerMemberId)>,
}

impl ComposerEdit {
    pub(super) fn unchanged(archive: ComposerArchive) -> Self {
        Self {
            archive,
            relation: None,
        }
    }
    pub(super) fn changed(
        archive: ComposerArchive,
        before: ComposerMemberId,
        after: ComposerMemberId,
    ) -> Self {
        Self {
            archive,
            relation: Some((before, after)),
        }
    }

    /// Borrow the checked result, e.g. `edit.archive().to_bytes()`.
    pub fn archive(&self) -> &ComposerArchive {
        &self.archive
    }
    /// Distinguish completed no-op replacements, e.g. `edit.is_unchanged()`.
    pub fn is_unchanged(&self) -> bool {
        self.relation.is_none()
    }
    /// Report typed completion, e.g. `edit.completion()`.
    pub fn completion(&self) -> ComposerEditCompletion {
        if self.is_unchanged() {
            ComposerEditCompletion::Unchanged
        } else {
            ComposerEditCompletion::Changed
        }
    }
    /// Report exact decoded-member fidelity, e.g. `edit.fidelity()`.
    pub fn fidelity(&self) -> ComposerEditFidelity {
        ComposerEditFidelity::ExactMemberContents
    }
    /// Return the derived member's complete parent/result relation, e.g. `edit.derived_member()`.
    pub fn derived_member(&self) -> Option<(&ComposerMemberId, &ComposerMemberId)> {
        self.relation.as_ref().map(|(a, b)| (a, b))
    }
    /// Return exact complete-container coverage, e.g. `edit.coverage()`.
    pub fn coverage(&self) -> ComposerCoverage {
        self.archive.coverage()
    }
    /// Borrow payload/native-application exclusions, e.g. `edit.diagnostics()`.
    pub fn diagnostics(&self) -> &[ComposerDiagnostic] {
        self.archive.diagnostics()
    }
}
