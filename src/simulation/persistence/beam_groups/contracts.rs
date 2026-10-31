use super::*;
use std::sync::Arc;

/// Isolated edit retaining complete inputs, result, ordered history and exact validation.
#[derive(Debug, Clone)]
pub struct SimulationBeamGroupEdit {
    /// Complete original immutable Property body.
    pub(super) source: SimulationBeamGroupBody,
    /// Complete validated result.
    pub(super) archive: SimulationBeamGroupBody,
    /// Requested original occurrence, including unchanged requests.
    pub(super) parent: SimulationBeamGroupFieldId,
    /// Requested result occurrence.
    pub(super) result: SimulationBeamGroupFieldId,
    /// Complete supplied value, including any text or archive parents.
    pub(super) replacement: SimulationBeamGroupValue,
    /// Exact completion, validation, diagnostics and achieved error.
    pub(super) coverage: SimulationBeamGroupCoverage,
    /// Every outer occurrence in physical order.
    pub(super) history: Arc<[SimulationBeamGroupHistory]>,
    /// Every embedded occurrence with composed copy/replacement provenance.
    pub(super) nested_history: Arc<[SimulationBeamGroupNestedHistory]>,
}
