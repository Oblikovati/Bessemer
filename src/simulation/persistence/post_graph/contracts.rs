use super::*;
use std::sync::Arc;

/// Isolated edit retaining complete inputs, result, ordered history and exact validation.
#[derive(Debug, Clone)]
pub struct SimulationPostGraphEdit {
    /// Complete original immutable Property body.
    pub(super) source: SimulationPostGraphParameters,
    /// Complete validated result.
    pub(super) archive: SimulationPostGraphParameters,
    /// Requested original occurrence, including unchanged requests.
    pub(super) parent: SimulationPostGraphFieldId,
    /// Requested result occurrence.
    pub(super) result: SimulationPostGraphFieldId,
    /// Complete supplied value, including any text or archive parents.
    pub(super) replacement: SimulationPostGraphValue,
    /// Exact completion, validation, diagnostics and achieved error.
    pub(super) coverage: SimulationPostGraphCoverage,
    /// Every outer occurrence in physical order.
    pub(super) history: Arc<[SimulationPostGraphHistory]>,
    /// Every embedded occurrence with composed copy/replacement provenance.
    pub(super) nested_history: Arc<[SimulationPostGraphNestedHistory]>,
}
