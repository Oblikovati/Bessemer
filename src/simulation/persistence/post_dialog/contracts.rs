use super::*;
use std::sync::Arc;

/// Isolated edit retaining complete inputs, result, ordered history and exact validation.
#[derive(Debug, Clone)]
pub struct SimulationPostDialogEdit {
    /// Complete original immutable Property body.
    pub(super) source: SimulationPostDialog,
    /// Complete validated result.
    pub(super) archive: SimulationPostDialog,
    /// Requested original occurrence, including unchanged requests.
    pub(super) parent: SimulationPostDialogFieldId,
    /// Requested result occurrence.
    pub(super) result: SimulationPostDialogFieldId,
    /// Complete supplied value, including any text or archive parents.
    pub(super) replacement: SimulationPostDialogValue,
    /// Exact completion, validation, diagnostics and achieved error.
    pub(super) coverage: SimulationPostDialogCoverage,
    /// Every outer occurrence in physical order.
    pub(super) history: Arc<[SimulationPostDialogHistory]>,
    /// Every embedded occurrence with composed copy/replacement provenance.
    pub(super) nested_history: Arc<[SimulationPostDialogNestedHistory]>,
}

impl SimulationPostDialogEdit {
    pub(super) fn from_isolated_dialog(
        source: &SimulationPostDialog,
        archive: SimulationPostDialog,
        id: &SimulationPostDialogFieldId,
        replacement: SimulationPostDialogValue,
        wire: &super::super::thermal::SimulationThermalValue,
    ) -> crate::Result<Self> {
        let changed = archive.revision != source.revision;
        let (history, nested_history) =
            super::super::thermal::compose(source, &archive, id.ordinal(), wire, changed)?;
        let coverage = super::edit::edit_coverage(&archive, changed);
        let result = archive.field_id(id.ordinal())?;
        Ok(Self {
            source: source.clone(),
            archive,
            parent: id.clone(),
            result,
            replacement,
            coverage,
            history,
            nested_history,
        })
    }
}
