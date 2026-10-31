use super::*;
use std::sync::Arc;

/// Isolated edit retaining complete inputs, result, ordered history and exact validation.
#[derive(Debug, Clone)]
pub struct SimulationResultsEdit {
    /// Complete original immutable Property body.
    pub(super) source: SimulationResultsProperty,
    /// Complete validated result.
    pub(super) archive: SimulationResultsProperty,
    /// Requested original occurrence, including unchanged requests.
    pub(super) parent: SimulationResultsFieldId,
    /// Requested result occurrence.
    pub(super) result: SimulationResultsFieldId,
    /// Complete supplied value, including any text or archive parents.
    pub(super) replacement: SimulationResultsValue,
    /// Exact completion, validation, diagnostics and achieved error.
    pub(super) coverage: SimulationResultsCoverage,
    /// Every outer occurrence in physical order.
    pub(super) history: Arc<[SimulationResultsHistory]>,
    /// Every embedded occurrence with composed copy/replacement provenance.
    pub(super) nested_history: Arc<[SimulationResultsNestedHistory]>,
}
