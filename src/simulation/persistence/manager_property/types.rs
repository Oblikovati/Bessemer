use super::*;
use crate::archive_codec::ArchiveRevision;

/// Immutable identity of a complete direct manager header and explicit profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationManagerPropertyRevision(pub(super) ArchiveRevision);

/// Once-normalized physical field position tied to its original header revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationManagerPropertyFieldId {
    pub(super) revision: SimulationManagerPropertyRevision,
    pub(super) ordinal: usize,
}

/// Complete signed word with distinct physical and runtime positions.
#[derive(Debug, Clone)]
pub struct SimulationManagerPropertyField {
    /// Original normalized physical identity.
    pub id: SimulationManagerPropertyFieldId,
    /// Physical byte position in the bare stream.
    pub stream_offset: usize,
    /// Documented runtime destination without assigned physical meaning.
    pub member: SimulationManagerPropertyMember,
    /// Exact signed view of all four stored bytes.
    pub value: i32,
}

/// Completed byte framing or isolated exact edit status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationManagerPropertyCompletion {
    /// Every selected word and exact EOF passed.
    CompletedFraming,
    /// Entire original header already matched; identities survived.
    Unchanged,
    /// One isolated word changed on a complete validated header.
    Changed,
}

/// Declared exact byte checks, independent of application or geometry validity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationManagerPropertyCheckMode {
    /// Explicit selected profile and every byte through EOF.
    ExactProfile,
    /// Complete profile and unchanged prefix/suffix outside the requested word.
    ExactProfileAndPreservation,
}

/// Responsibilities excluded from direct manager header acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationManagerPropertyDiagnostic {
    /// No installed reader/writer, storage enumeration or COM operation executed.
    NativeApplicationNotExecuted,
    /// Supplied version context does not establish the application's dispatch path.
    RuntimeDispatchNotInferred,
    /// Raw words and destinations do not establish counter/physical meanings.
    FieldMeaningsUnassigned,
    /// Nested folders, objects and materials remain separate schemas.
    NestedObjectsNotDecoded,
}

/// Completion, exact scope, diagnostics and achieved pattern error returned together.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationManagerPropertyCoverage {
    /// Complete supplied kind/version profile.
    pub profile: SimulationManagerPropertyProfile,
    /// Completed framing or edit status.
    pub completion: SimulationManagerPropertyCompletion,
    /// Exact declared validation scope.
    pub check_mode: SimulationManagerPropertyCheckMode,
    /// Every byte checked through EOF.
    pub checked_bytes: usize,
    /// Every selected signed word checked.
    pub checked_fields: usize,
    /// Explicit native/semantic exclusions.
    pub diagnostics: [SimulationManagerPropertyDiagnostic; 4],
    /// Achieved requested full-width pattern error, zero.
    pub requested_pattern_error_bits: u32,
}

/// Relation of each physical result word to its original parent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationManagerPropertyRelation {
    /// Entire original word survived byte for byte.
    Copied,
    /// Word derives from the original position and retained replacement pattern.
    Derived,
}

/// One ordered relation per original/result field.
#[derive(Debug, Clone)]
pub struct SimulationManagerPropertyHistory {
    /// Original physical field.
    pub parent: SimulationManagerPropertyFieldId,
    /// Validated result field.
    pub result: SimulationManagerPropertyFieldId,
    /// Exact copy or derivation relation.
    pub relation: SimulationManagerPropertyRelation,
}

/// Complete isolated word edit with both input parents and every field relation.
#[derive(Debug, Clone)]
pub struct SimulationManagerPropertyEdit {
    /// Original complete immutable header.
    pub source: SimulationManagerProperty,
    /// Complete validated result header.
    pub archive: SimulationManagerProperty,
    /// Requested original physical field.
    pub parent: SimulationManagerPropertyFieldId,
    /// Requested result physical field.
    pub result: SimulationManagerPropertyFieldId,
    /// Entire supplied four-byte signed pattern.
    pub replacement: i32,
    /// Exact completion, coverage, exclusions and achieved pattern error.
    pub coverage: SimulationManagerPropertyCoverage,
    /// All fields in original wire order.
    pub history: Arc<[SimulationManagerPropertyHistory]>,
}
