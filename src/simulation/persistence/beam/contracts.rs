use super::*;
use std::sync::Arc;

/// Isolated edit retaining complete inputs, result, ordered history and exact validation.
#[derive(Debug, Clone)]
pub struct SimulationBeamEdit {
    /// Complete original immutable Property body.
    pub(super) source: SimulationBeamProperty,
    /// Complete validated result.
    pub(super) archive: SimulationBeamProperty,
    /// Requested original occurrence, including unchanged requests.
    pub(super) parent: SimulationBeamFieldId,
    /// Requested result occurrence.
    pub(super) result: SimulationBeamFieldId,
    /// Complete supplied value, including any text or archive parents.
    pub(super) replacement: SimulationBeamValue,
    /// Exact completion, validation, diagnostics and achieved error.
    pub(super) coverage: SimulationBeamCoverage,
    /// Every outer occurrence in physical order.
    pub(super) history: Arc<[SimulationBeamHistory]>,
    /// Every embedded occurrence with composed copy/replacement provenance.
    pub(super) nested_history: Arc<[SimulationBeamNestedHistory]>,
}
