use super::{
    SimulationThermalCollection, SimulationThermalKind, SimulationThermalLimits,
    SimulationThermalValue,
};
use crate::archive_codec::ArchiveRevision;
use crate::model_archives::ModelFieldHandle;
use crate::simulation::SimulationPersistVersion;
use std::sync::Arc;

/// One physical outer occurrence tied to an immutable complete collection revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationThermalFieldId {
    pub(in crate::simulation::persistence) revision: ArchiveRevision,
    pub(in crate::simulation::persistence) ordinal: usize,
}

impl SimulationThermalFieldId {
    /// Return the physical ordinal without weakening revision checks; e.g. `id.ordinal()`.
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }
}

/// Completed reads and isolated edits remain distinct from typed errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationThermalCompletion {
    /// Complete framing with zero records.
    CompletedEmptyRecords,
    /// Every declared complete record was checked.
    CompletedRecords,
    /// Requested pattern already matched, preserving original identities.
    Unchanged,
    /// One isolated outer value changed on a complete validated collection.
    Changed,
}

/// Exact stored-profile checks, independent of native application acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationThermalCheckMode {
    /// Entire outer grammar, embedded profiles, budgets and EOF.
    ExactSelectedProfiles,
    /// Complete profiles and literal preservation outside the requested value.
    ExactSelectedProfilesAndPreservation,
}

/// Explicit exclusions from byte-pattern validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationThermalDiagnostic {
    /// No native loader, writer or COM operation executed.
    NativeApplicationNotExecuted,
    /// Caller version does not establish native runtime dispatch.
    RuntimeVersionNotInferred,
    /// Raw values do not assign physical units or domain meanings.
    PhysicalSemanticsUnassigned,
    /// Topology, geometry, intersections and geometric tolerances were not checked.
    GeometryNotChecked,
}

/// Complete status, exact coverage, exclusions and achieved pattern error together.
#[derive(Debug, Clone, Copy)]
pub struct SimulationThermalCoverage {
    /// Explicit outer family.
    pub kind: SimulationThermalKind,
    /// Once-normalized outer version.
    pub version: SimulationPersistVersion,
    /// Exact operation budgets.
    pub limits: SimulationThermalLimits,
    /// Complete read or edit status.
    pub completion: SimulationThermalCompletion,
    /// Declared exact validation mode.
    pub check_mode: SimulationThermalCheckMode,
    /// Entire checked result image.
    pub checked_bytes: usize,
    /// Every outer field, including complete archive values.
    pub checked_fields: usize,
    /// Every complete declared record.
    pub record_count: usize,
    /// Complete embedded roots checked under their own versions.
    pub embedded_archives: usize,
    /// Physical fields checked inside all embedded roots.
    pub embedded_fields: usize,
    /// Native, runtime and geometric exclusions.
    pub diagnostics: [SimulationThermalDiagnostic; 4],
    /// Achieved requested raw-pattern error, zero.
    pub requested_pattern_error_bits: u32,
}

/// Structural copy, derivation, creation or deletion without geometric interpretation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationThermalRelation {
    /// Complete source pattern survived unchanged.
    Copied,
    /// Result derives from retained source/replacement parents.
    Derived,
    /// Result occurrence has no corresponding source occurrence.
    Created,
    /// Source occurrence has no corresponding result occurrence.
    Deleted,
}

/// Explicit provenance of one outer field relation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationThermalHistoryParent {
    /// Original complete collection field.
    Source(SimulationThermalFieldId),
    /// Complete supplied value retained by the edit result.
    Replacement,
}

/// One relation per outer field, in physical order.
#[derive(Debug, Clone)]
pub struct SimulationThermalHistory {
    /// All source and replacement parents.
    pub parents: Arc<[SimulationThermalHistoryParent]>,
    /// Complete validated result occurrence.
    pub result: SimulationThermalFieldId,
    /// Copy or derivation for the isolated outer operation.
    pub relation: SimulationThermalRelation,
}

/// Explicit provenance within a complete embedded archive value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationThermalNestedParent {
    /// Original embedded field identity.
    Source(ModelFieldHandle),
    /// Supplied embedded replacement field identity.
    Replacement(ModelFieldHandle),
}

/// Composed nested history in source order followed by created result occurrences.
#[derive(Debug, Clone)]
pub struct SimulationThermalNestedHistory {
    /// Containing original archive value.
    pub outer_parent: SimulationThermalFieldId,
    /// Containing result archive value.
    pub outer_result: SimulationThermalFieldId,
    /// Structural path with occurrence identity retained by handles.
    pub path: Arc<str>,
    /// Every original/replacement parent.
    pub parents: Arc<[SimulationThermalNestedParent]>,
    /// Result occurrence, absent for deletions.
    pub result: Option<ModelFieldHandle>,
    /// Full structural relation.
    pub relation: SimulationThermalRelation,
}

/// Isolated edit retaining complete inputs, result, ordered history and exact validation.
#[derive(Debug, Clone)]
pub struct SimulationThermalEdit {
    /// Complete original immutable collection.
    pub(super) source: SimulationThermalCollection,
    /// Complete validated result.
    pub(super) archive: SimulationThermalCollection,
    /// Requested original occurrence, including unchanged requests.
    pub(super) parent: SimulationThermalFieldId,
    /// Requested result occurrence.
    pub(super) result: SimulationThermalFieldId,
    /// Complete supplied value, including any text or archive parents.
    pub(super) replacement: SimulationThermalValue,
    /// Exact completion, validation, diagnostics and achieved error.
    pub(super) coverage: SimulationThermalCoverage,
    /// Every outer occurrence in physical order.
    pub(super) history: Arc<[SimulationThermalHistory]>,
    /// Every embedded occurrence with composed copy/replacement provenance.
    pub(super) nested_history: Arc<[SimulationThermalNestedHistory]>,
}
