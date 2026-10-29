use super::ComposerXmlLinks;
use crate::archive_codec::ArchiveRevision;
use crate::composer::geometry::ComposerSceneRevision;
use crate::metadata::{ExpandedName, XmlAttribute, XmlLimits};
use std::sync::Arc;

/// Explicit XML and index-allocation budgets.
#[derive(Debug, Clone, Copy)]
pub struct ComposerLinkLimits {
    /// Physical XML, lexical event, attribute and depth bounds.
    pub xml: XmlLimits,
    /// Maximum binary geodes considered, including unkeyed records.
    pub maximum_geodes: usize,
    /// Maximum `IdentGeom` occurrences retained.
    pub maximum_references: usize,
    /// Maximum cumulative child-ordinal path components.
    pub maximum_path_units: usize,
}

/// One candidate in the exact geometry snapshot used by an index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposerGeodeId {
    pub(super) revision: ComposerSceneRevision,
    pub(super) entry: usize,
}

impl ComposerGeodeId {
    /// Original top-level scene entry index, e.g. `candidate.entry_index()`.
    pub fn entry_index(&self) -> usize {
        self.entry
    }

    /// Geometry revision required by this candidate, e.g. `candidate.scene_revision()`.
    pub fn scene_revision(&self) -> &ComposerSceneRevision {
        &self.revision
    }
}

/// One exact, nonempty binary GUID key with all its serialized candidates.
#[derive(Debug, Clone)]
pub struct ComposerGuidGroup {
    /// Raw UTF-16 content; it is not parsed as a UUID or normalized.
    pub identifier: Arc<[u16]>,
    /// Every candidate in original scene-entry order.
    pub candidates: Arc<[ComposerGeodeId]>,
}

/// XML occurrence identity scoped to one immutable index revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposerXmlReferenceId {
    pub(super) revision: ArchiveRevision,
    pub(super) index: usize,
}

/// Exact local equality classification, without inferred runtime resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerLinkStatus {
    /// Stored empty XML reference.
    Empty,
    /// Nonempty text with no exact local candidate.
    Unmatched,
    /// Exactly one local equality candidate.
    Unique,
    /// More than one local candidate; no first/last-wins selection.
    Ambiguous,
}

/// One `IdentGeom` occurrence; repeated references retain separate identities.
#[derive(Debug, Clone)]
pub struct ComposerXmlReference {
    /// Revision-bound occurrence identity.
    pub id: ComposerXmlReferenceId,
    /// Child-element ordinals, with an empty path for the root.
    pub element_path: Arc<[usize]>,
    /// Namespace-expanded element name.
    pub name: ExpandedName,
    /// All ordinary attributes in source order, including unknown values.
    pub attributes: Arc<[XmlAttribute]>,
    /// Exact XML-decoded reference text.
    pub identifier: Arc<str>,
    /// Exact local equality result.
    pub status: ComposerLinkStatus,
    /// All local candidates, scoped to the retained geometry revision.
    pub candidates: Arc<[ComposerGeodeId]>,
    pub(super) element_ordinal: usize,
}

/// Scope of successful identifier indexing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerLinkCheckMode {
    /// Complete UTF-8 XML parsing and all exact local identifier comparisons.
    ExactLocalEquality,
}

/// Entire accepted local-index scope; unmatched references are completed outcomes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComposerLinkCoverage {
    /// Explicit validation scope.
    pub mode: ComposerLinkCheckMode,
    /// Entire original XML byte count, including any BOM.
    pub xml_bytes: usize,
    /// Complete parsed element count.
    pub elements: usize,
    /// Serialized binary geodes considered.
    pub geodes: usize,
    /// All XML `IdentGeom` occurrences.
    pub references: usize,
}

/// Limits of a complete local equality index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerLinkDiagnostic {
    /// Filesystem/path resolution and external loading were not run.
    RuntimeResolutionNotChecked,
    /// Clone/merge/filter/redline runtime branches were not executed.
    GraphSideEffectsNotChecked,
    /// Actor properties, animation/view values and full project semantics remain separate.
    ProjectSemanticsNotChecked,
    /// Native application acceptance was not checked.
    NativeApplicationNotChecked,
}

/// Achieved XML attribute edit fidelity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerLinkEditFidelity {
    /// Exact requested parsed reference text, with unrelated XML markup preserved.
    ExactAttributeText,
}

/// XML occurrence lineage on the same immutable geometry snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposerLinkHistory {
    /// Original occurrence.
    pub parent: ComposerXmlReferenceId,
    /// Derived occurrence (same identity for a completed no-op).
    pub result: ComposerXmlReferenceId,
}

/// Completed isolated XML reference edit and its reindexed local candidates.
#[derive(Debug, Clone)]
pub struct ComposerLinkEdit {
    /// Immutable accepted index/result XML.
    pub index: ComposerXmlLinks,
    /// Completed changed/unchanged state.
    pub completion: crate::display_lists::DisplayEditCompletion,
    /// Exact achieved attribute fidelity.
    pub fidelity: ComposerLinkEditFidelity,
    /// Original/derived reference history.
    pub history: ComposerLinkHistory,
    /// Complete XML/local-index checks.
    pub coverage: ComposerLinkCoverage,
    /// Runtime and complete-project exclusions.
    pub diagnostics: [ComposerLinkDiagnostic; 4],
}
