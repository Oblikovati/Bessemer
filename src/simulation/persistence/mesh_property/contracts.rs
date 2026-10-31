use super::*;
use std::sync::Arc;

/// Isolated edit retaining complete inputs, result, ordered history and exact validation.
#[derive(Debug, Clone)]
pub struct SimulationMeshPropertyEdit {
    /// Complete original immutable Mesh Property body.
    pub(super) source: SimulationMeshProperty,
    /// Complete validated result.
    pub(super) archive: SimulationMeshProperty,
    /// Requested original occurrence, including unchanged requests.
    pub(super) parent: SimulationMeshPropertyFieldId,
    /// Requested result occurrence.
    pub(super) result: SimulationMeshPropertyFieldId,
    /// Complete supplied value, including any text or archive parents.
    pub(super) replacement: SimulationMeshPropertyValue,
    /// Exact completion, validation, diagnostics and achieved error.
    pub(super) coverage: SimulationMeshPropertyCoverage,
    /// Every outer occurrence in physical order.
    pub(super) history: Arc<[SimulationMeshPropertyHistory]>,
    /// Every embedded occurrence with composed copy/replacement provenance.
    pub(super) nested_history: Arc<[SimulationMeshPropertyNestedHistory]>,
}
