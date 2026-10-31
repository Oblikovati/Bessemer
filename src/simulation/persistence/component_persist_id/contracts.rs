use super::*;
use std::sync::Arc;

/// Isolated edit retaining complete inputs, result, ordered history and exact validation.
#[derive(Debug, Clone)]
pub struct SimulationComponentPersistIdEdit {
    /// Complete original immutable Property body.
    pub(super) source: SimulationComponentPersistId,
    /// Complete validated result.
    pub(super) archive: SimulationComponentPersistId,
    /// Requested original occurrence, including unchanged requests.
    pub(super) parent: SimulationComponentPersistIdFieldId,
    /// Requested result occurrence.
    pub(super) result: SimulationComponentPersistIdFieldId,
    /// Complete supplied value, including any text or archive parents.
    pub(super) replacement: SimulationComponentPersistIdValue,
    /// Exact completion, validation, diagnostics and achieved error.
    pub(super) coverage: SimulationComponentPersistIdCoverage,
    /// Every outer occurrence in physical order.
    pub(super) history: Arc<[SimulationComponentPersistIdHistory]>,
    /// Every embedded occurrence with composed copy/replacement provenance.
    pub(super) nested_history: Arc<[SimulationComponentPersistIdNestedHistory]>,
}
