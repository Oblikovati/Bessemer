use super::*;
use std::sync::Arc;

/// Isolated edit retaining complete inputs, result, ordered history and exact validation.
#[derive(Debug, Clone)]
pub struct SimulationSolidBodyEdit {
    /// Complete original immutable Property body.
    pub(super) source: SimulationSolidBodyProperty,
    /// Complete validated result.
    pub(super) archive: SimulationSolidBodyProperty,
    /// Requested original occurrence, including unchanged requests.
    pub(super) parent: SimulationSolidBodyFieldId,
    /// Requested result occurrence.
    pub(super) result: SimulationSolidBodyFieldId,
    /// Complete supplied value, including any text or archive parents.
    pub(super) replacement: SimulationSolidBodyValue,
    /// Exact completion, validation, diagnostics and achieved error.
    pub(super) coverage: SimulationSolidBodyCoverage,
    /// Every outer occurrence in physical order.
    pub(super) history: Arc<[SimulationSolidBodyHistory]>,
    /// Every embedded occurrence with composed copy/replacement provenance.
    pub(super) nested_history: Arc<[SimulationSolidBodyNestedHistory]>,
}
