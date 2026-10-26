use super::{ModelEditCheckMode, ModelFieldRelation, PartConfigurationArchive};
use crate::archive_codec::ArchiveRevision;
use std::sync::Arc;

/// Exact validation coverage of a complete stored model archive, independent of geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelArchiveCoverage {
    /// Whole-stream stored-schema checks, never a sampled geometry pass.
    pub mode: ModelEditCheckMode,
    /// Reconstructed bytes checked through EOF.
    pub wire_bytes: usize,
    /// Typed physical fields whose schema, order and stored values were checked.
    pub typed_fields: usize,
    /// New model object definitions, excluding imported header slots.
    pub new_objects: usize,
    /// Alias tags resolved in the shared map.
    pub object_aliases: usize,
    /// Header slots imported in native order, including null slots.
    pub imported_slots: usize,
}

/// Validation outside the declared stored-archive operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelArchiveDiagnostic {
    /// Topology, geometry, self-intersections and geometric tolerances were not evaluated.
    GeometryOutsideScope,
    /// Referenced external documents and their geometry were not opened or checked.
    ExternalDocumentsNotChecked,
}

/// Structural field occurrence bound to one immutable model archive revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelFieldHandle {
    pub(super) revision: ArchiveRevision,
    pub(super) index: usize,
}

/// Changed physical field with all source parents and its optional result occurrence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelFieldChange {
    pub(super) path: Arc<str>,
    pub(super) relation: ModelFieldRelation,
    pub(super) parents: Vec<ModelFieldHandle>,
    pub(super) result: Option<ModelFieldHandle>,
}

/// Completed isolated edit with exact coverage, achieved fidelity, diagnostics and field history.
#[derive(Debug, Clone)]
pub struct PartConfigurationEdit {
    pub(super) archive: PartConfigurationArchive,
    pub(super) completion: super::ModelEditCompletion,
    pub(super) coverage: ModelArchiveCoverage,
    pub(super) changes: Vec<ModelFieldChange>,
    pub(super) unchanged_fields: usize,
}
