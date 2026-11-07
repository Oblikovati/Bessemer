use super::*;
use std::sync::Arc;

/// Isolated edit retaining complete inputs, result, ordered history and exact validation.
#[derive(Debug, Clone)]
pub struct SimulationInertialEdit {
    /// Complete original immutable collection.
    pub(super) source: SimulationInertialCollection,
    /// Complete validated result.
    pub(super) archive: SimulationInertialCollection,
    /// Requested original occurrence, including unchanged requests.
    pub(super) parent: SimulationInertialFieldId,
    /// Requested result occurrence.
    pub(super) result: SimulationInertialFieldId,
    /// Complete supplied value, including any text or archive parents.
    pub(super) replacement: SimulationInertialValue,
    /// Exact completion, validation, diagnostics and achieved error.
    pub(super) coverage: SimulationInertialCoverage,
    /// Every outer occurrence in physical order.
    pub(super) history: Arc<[SimulationInertialHistory]>,
    /// Every embedded occurrence with composed copy/replacement provenance.
    pub(super) nested_history: Arc<[SimulationInertialNestedHistory]>,
}
