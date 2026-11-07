use super::{ElectricalArchive, ElectricalIniEdit};
use crate::composer::ComposerMemberId;
use std::sync::Arc;

pub use crate::display_lists::DisplayEditCompletion as ElectricalEditCompletion;

/// Explicit whole-carrier and lexical allocation/traversal limits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ElectricalLimits {
    /// Maximum encoded plain ZIP bytes, excluding internal adapter storage.
    pub maximum_bytes: usize,
    /// Maximum member occurrences, including duplicate names.
    pub maximum_members: usize,
    /// Maximum decoded bytes in one member.
    pub maximum_member_bytes: usize,
    /// Maximum aggregate decoded member bytes.
    pub maximum_expanded_bytes: usize,
    /// Maximum lexical records in any requested Archive.ini view.
    pub maximum_ini_records: usize,
}

/// A particular member occurrence bound to an immutable carrier revision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElectricalMemberId(pub(super) ComposerMemberId);

/// Documented ZIP compression methods under the selected carrier scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElectricalCompression {
    /// Stored method 0.
    Stored,
    /// Raw-deflate method 8.
    Deflated,
}

/// Immutable directory-order metadata; names are never filesystem destinations.
#[derive(Clone, Debug)]
pub struct ElectricalMember {
    /// Revision-bound occurrence, distinct even for duplicate names.
    pub id: ElectricalMemberId,
    /// Exact decoded UTF-8/CP437 spelling, without path normalization.
    pub name: Arc<str>,
    /// Checked compression method.
    pub compression: ElectricalCompression,
    /// Complete decoded payload length.
    pub expanded_bytes: u32,
    /// Complete encoded payload length.
    pub stored_bytes: u32,
    /// Declared and verified CRC-32.
    pub crc32: u32,
}

/// Borrowed raw ZIP metadata retained independently of member payload schemas.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ElectricalMemberMetadata<'a> {
    /// Exact stored general-purpose flag word.
    pub flags: u16,
    /// Physical local-header position in the plain ZIP file.
    pub header_offset: usize,
    /// Complete central-directory extra bytes, including unknown fields.
    pub extra: &'a [u8],
    /// Complete member comment bytes.
    pub comment: &'a [u8],
}

/// Complete carrier checks, independent of member/application semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ElectricalCoverage {
    /// Exact selected container check mode.
    pub mode: ElectricalCheckMode,
    /// Complete original ZIP extent.
    pub wire_bytes: usize,
    /// All framed and integrity-checked member occurrences.
    pub checked_members: usize,
    /// Aggregate verified decoded bytes.
    pub expanded_bytes: usize,
    /// Reparse disagreement; successful operations require zero.
    pub achieved_byte_error: usize,
}

/// Explicit container validation scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElectricalCheckMode {
    /// Complete ZIP32 partition, termination, decoded lengths and CRCs.
    ExactZipCarrier,
}

/// Exclusions returned with every published carrier and edit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElectricalDiagnostic {
    /// SQLite schemas/rows and EWG drawing semantics were not validated.
    MemberSchemasOutsideScope,
    /// Native Electrical loader, migration and application acceptance were not checked.
    NativeApplicationNotChecked,
}

/// Relation of one complete source/result member occurrence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElectricalMemberHistory {
    /// Complete source occurrence.
    pub parent: ElectricalMemberId,
    /// Complete result occurrence.
    pub result: ElectricalMemberId,
    /// Whether the requested decoded member changed.
    pub relation: ElectricalMemberRelation,
}

/// Member-level provenance; byte-level lexical history is reported separately.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElectricalMemberRelation {
    /// Encoded representation and decoded contents were retained.
    Copied,
    /// Complete result member derived from the selected source occurrence.
    Derived,
}

/// Immutable completed carrier edit with a retained source compression checkpoint.
#[derive(Clone, Debug)]
pub struct ElectricalArchiveEdit {
    pub(super) source: ElectricalArchive,
    pub(super) result: ElectricalArchive,
    pub(super) completion: ElectricalEditCompletion,
    pub(super) history: Vec<ElectricalMemberHistory>,
    pub(super) ini: Option<ElectricalIniEdit>,
}
