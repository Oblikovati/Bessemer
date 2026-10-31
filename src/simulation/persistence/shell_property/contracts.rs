use super::*;
use std::sync::Arc;

/// Isolated edit retaining complete inputs, result, ordered history and exact validation.
#[derive(Debug, Clone)]
pub struct SimulationShellPropertyEdit {
    /// Complete original immutable composite body.
    pub(super) source: SimulationShellProperty,
    /// Complete validated result.
    pub(super) archive: SimulationShellProperty,
    /// Requested original occurrence, including unchanged requests.
    pub(super) parent: SimulationShellPropertyFieldId,
    /// Requested result occurrence.
    pub(super) result: SimulationShellPropertyFieldId,
    /// Complete supplied value, including any text or archive parents.
    pub(super) replacement: SimulationShellPropertyValue,
    /// Exact completion, validation, diagnostics and achieved error.
    pub(super) coverage: SimulationShellPropertyCoverage,
    /// Every outer occurrence in physical order.
    pub(super) history: Arc<[SimulationShellPropertyHistory]>,
    /// Every embedded occurrence with composed copy/replacement provenance.
    pub(super) nested_history: Arc<[SimulationShellPropertyNestedHistory]>,
}
