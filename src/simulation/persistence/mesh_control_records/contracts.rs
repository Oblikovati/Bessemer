use super::*;
use std::sync::Arc;

/// Isolated edit retaining complete inputs, result, ordered history and exact validation.
#[derive(Debug, Clone)]
pub struct SimulationMeshControlEdit {
    /// Complete original immutable counted-record body.
    pub(super) source: SimulationMeshControlRecords,
    /// Complete validated result.
    pub(super) archive: SimulationMeshControlRecords,
    /// Requested original occurrence, including unchanged requests.
    pub(super) parent: SimulationMeshControlFieldId,
    /// Requested result occurrence.
    pub(super) result: SimulationMeshControlFieldId,
    /// Complete supplied value, including any text or archive parents.
    pub(super) replacement: SimulationMeshControlValue,
    /// Exact completion, validation, diagnostics and achieved error.
    pub(super) coverage: SimulationMeshControlCoverage,
    /// Every outer occurrence in physical order.
    pub(super) history: Arc<[SimulationMeshControlHistory]>,
    /// Every embedded occurrence with composed copy/replacement provenance.
    pub(super) nested_history: Arc<[SimulationMeshControlNestedHistory]>,
}
