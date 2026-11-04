use crate::archive_codec::ArchiveRevision;
use crate::inspection::records::{
    InspectionRecordCoverage, InspectionRecordHistory, InspectionRecordId, InspectionRecords,
};
use std::sync::Arc;

/// Caller-selected DOM retention; neither choice claims native caller configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectionRelationshipPolicy {
    /// Include text, nonempty CDATA, comments and processing instructions.
    RetainedNodes,
    /// Traverse only direct element children.
    ElementsOnly,
}

/// Physical direct-content occurrence anchored to an immutable project revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InspectionRelationshipNodeId {
    pub(super) revision: ArchiveRevision,
    pub(super) container: usize,
    pub(super) content_index: usize,
}

/// Kind retained by the explicit DOM projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectionRelationshipNodeKind {
    /// Element with its exact qualified spelling.
    Element,
    /// Decoded adjacent text and character references.
    Text,
    /// Nonempty CDATA section.
    CData,
    /// Comment, whose original bytes remain in the source envelope.
    Comment,
    /// Processing instruction, whose original bytes remain in the source envelope.
    ProcessingInstruction,
}

/// One complete load attempt before duplicate insertion discards any entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InspectionRelationshipAttempt {
    /// Physical occurrence independent of textual key equality.
    pub id: InspectionRelationshipNodeId,
    /// Explicit DOM kind, including non-element insertion attempts.
    pub kind: InspectionRelationshipNodeKind,
    /// Exact qualified element spelling; absent for non-elements.
    pub element_name: Option<Arc<str>>,
    /// Last key field after exact Qt whitespace trimming.
    pub key: Arc<str>,
    /// Appended value fields in traversal order, including empty values.
    pub values: Arc<[Arc<str>]>,
}

/// Completed projection, including an absent or empty dictionary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectionRelationshipCompletion {
    /// Full bounded XML and the selected explicit-policy traversal completed.
    Completed,
}

/// Exclusions from a completed relationship projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectionRelationshipDiagnostic {
    /// Selected policy does not establish the native caller's DOM configuration.
    NativeDomPolicyUnknown,
    /// Inspection order does not establish native map iteration order.
    NativeMapOrderUnknown,
    /// No graph meaning, referential integrity or application reload was checked.
    GraphAndApplicationSemanticsNotChecked,
}

/// Immutable load attempts and selected map views, retaining the complete source.
#[derive(Debug, Clone)]
pub struct InspectionRelationshipViews {
    /// Original project, retaining all lexical and encrypted-envelope bytes.
    pub source: InspectionRecords,
    /// Validated sheet occurrence owning the selected package.
    pub sheet: InspectionRecordId,
    /// Required explicit DOM policy.
    pub policy: InspectionRelationshipPolicy,
    /// All selected direct-content attempts in physical order.
    pub attempts: Arc<[InspectionRelationshipAttempt]>,
    /// First insertion for each exact UTF-16 key, in first-occurrence inspection order.
    pub entries: Arc<[InspectionRelationshipAttempt]>,
    /// Selected entries after the writer's empty-key filter, without serializing them.
    pub writer_entries: Arc<[InspectionRelationshipAttempt]>,
    /// Completed even when no dictionary or no attempts exist.
    pub completion: InspectionRelationshipCompletion,
    /// Complete XML lexical validation, without graph or geometry validity claims.
    pub coverage: InspectionRecordCoverage,
    /// Unchanged source sheet relation.
    pub history: [InspectionRecordHistory; 1],
    /// Explicit exclusions accompanying every successful projection.
    pub diagnostics: [InspectionRelationshipDiagnostic; 3],
}

impl InspectionRelationshipViews {
    /// Resolve a retained physical attempt; e.g. `views.attempt(&views.attempts[0].id)`.
    pub fn attempt(
        &self,
        id: &InspectionRelationshipNodeId,
    ) -> Option<&InspectionRelationshipAttempt> {
        self.attempts.iter().find(|attempt| &attempt.id == id)
    }
}
