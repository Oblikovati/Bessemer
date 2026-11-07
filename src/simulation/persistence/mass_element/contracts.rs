use super::*;
use std::sync::Arc;

/// Isolated edit retaining complete inputs, result, ordered history and exact validation.
#[derive(Debug, Clone)]
pub struct SimulationMassElementEdit {
    /// Complete original immutable Property body.
    pub(super) source: SimulationMassElementProperty,
    /// Complete validated result.
    pub(super) archive: SimulationMassElementProperty,
    /// Requested original occurrence, including unchanged requests.
    pub(super) parent: SimulationMassElementFieldId,
    /// Requested result occurrence.
    pub(super) result: SimulationMassElementFieldId,
    /// Complete supplied value, including any text or archive parents.
    pub(super) replacement: SimulationMassElementValue,
    /// Exact completion, validation, diagnostics and achieved error.
    pub(super) coverage: SimulationMassElementCoverage,
    /// Every outer occurrence in physical order.
    pub(super) history: Arc<[SimulationMassElementHistory]>,
    /// Every embedded occurrence with composed copy/replacement provenance.
    pub(super) nested_history: Arc<[SimulationMassElementNestedHistory]>,
}
