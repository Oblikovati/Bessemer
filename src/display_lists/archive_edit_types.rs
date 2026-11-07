use super::{DisplayArchive, archive_identity::DisplayRevision};
use std::sync::Arc;

/// Completed stored-archive edit; failure never publishes a partial archive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayEditCompletion {
    /// Every supplied typed field matched the source revision.
    Unchanged,
    /// At least one stored field changed after complete schema validation.
    Changed,
}

/// Achieved error for the serialization operation, independent of geometric error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayEditFidelity {
    /// Reconstructed fields equal the requested types, paths, values and stored bits exactly.
    ExactStoredFields,
}

/// Checks excluded from the stored-archive operation, rather than silently certified.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayEditDiagnostic {
    /// Topology, geometry, intersections and tolerances are outside this serialization operation.
    GeometryOutsideScope,
    /// External facet positions require the separate directory/block link check.
    ExternalFaceLinksNotChecked,
}

/// Exact whole-stream checks; counts describe the published result, not sampled geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayEditCheckMode {
    /// Every stored field and shared-map record was checked through complete stream EOF.
    ExactStoredArchive,
}

/// Coverage of the declared check mode, independent of geometric validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayEditCoverage {
    /// Exact stored-archive checks; never a sampled geometry pass.
    pub mode: DisplayEditCheckMode,
    /// Complete encoded stream length validated through EOF.
    pub wire_bytes: usize,
    /// Fields whose types, paths, order and stored values matched a complete reparse.
    pub typed_fields: usize,
    /// Newly loaded objects validated in the shared MFC map.
    pub loaded_objects: usize,
    /// Alias tags checked against already mapped object identities and declared roles.
    pub object_aliases: usize,
    /// External facet references whose target blocks this operation did not check.
    pub unchecked_external_faces: usize,
}

/// One field position bound to an immutable archive revision; clones retain its identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayFieldHandle {
    pub(super) revision: DisplayRevision,
    pub(super) position: usize,
}

/// Structural wire-field relation; this does not infer native CAD entity naming.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayFieldRelation {
    /// The result has a path occurrence absent from the source.
    Created,
    /// The source path occurrence is absent from the result.
    Deleted,
    /// A stored value at the same structural path occurrence changed.
    Derived,
}

/// Changed structural field with every source parent and its optional result field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayFieldChange {
    pub(super) path: Arc<str>,
    pub(super) relation: DisplayFieldRelation,
    pub(super) parents: Vec<DisplayFieldHandle>,
    pub(super) result: Option<DisplayFieldHandle>,
}

/// Immutable edit outcome with completion, diagnostics, exact coverage, fidelity and history.
#[derive(Debug, Clone)]
pub struct DisplayArchiveEdit {
    pub(super) archive: DisplayArchive,
    pub(super) completion: DisplayEditCompletion,
    pub(super) coverage: DisplayEditCoverage,
    pub(super) diagnostics: Vec<DisplayEditDiagnostic>,
    pub(super) changes: Vec<DisplayFieldChange>,
    pub(super) unchanged_fields: usize,
}
