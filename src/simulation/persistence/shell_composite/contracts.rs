use super::*;
use std::sync::Arc;

/// Isolated edit retaining complete inputs, result, ordered history and exact validation.
#[derive(Debug, Clone)]
pub struct SimulationShellCompositeEdit {
    /// Complete original immutable composite body.
    pub(super) source: SimulationShellComposite,
    /// Complete validated result.
    pub(super) archive: SimulationShellComposite,
    /// Requested original occurrence, including unchanged requests.
    pub(super) parent: SimulationShellCompositeFieldId,
    /// Requested result occurrence.
    pub(super) result: SimulationShellCompositeFieldId,
    /// Complete supplied value, including any text or archive parents.
    pub(super) replacement: SimulationShellCompositeValue,
    /// Exact completion, validation, diagnostics and achieved error.
    pub(super) coverage: SimulationShellCompositeCoverage,
    /// Every outer occurrence in physical order.
    pub(super) history: Arc<[SimulationShellCompositeHistory]>,
    /// Every embedded occurrence with composed copy/replacement provenance.
    pub(super) nested_history: Arc<[SimulationShellCompositeNestedHistory]>,
}
