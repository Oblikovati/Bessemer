use super::*;
use std::sync::Arc;

/// Isolated edit retaining complete inputs, result, ordered history and exact validation.
#[derive(Debug, Clone)]
pub struct SimulationSheetEdit {
    /// Complete original immutable Property body.
    pub(super) source: SimulationSheetProperty,
    /// Complete validated result.
    pub(super) archive: SimulationSheetProperty,
    /// Requested original occurrence, including unchanged requests.
    pub(super) parent: SimulationSheetFieldId,
    /// Requested result occurrence.
    pub(super) result: SimulationSheetFieldId,
    /// Complete supplied value, including any text or archive parents.
    pub(super) replacement: SimulationSheetValue,
    /// Exact completion, validation, diagnostics and achieved error.
    pub(super) coverage: SimulationSheetCoverage,
    /// Every outer occurrence in physical order.
    pub(super) history: Arc<[SimulationSheetHistory]>,
    /// Every embedded occurrence with composed copy/replacement provenance.
    pub(super) nested_history: Arc<[SimulationSheetNestedHistory]>,
}
