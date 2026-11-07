use super::{
    Result, SimulationMetadataArchive, SimulationScalarEdit, SimulationScalarHelperProjection,
    SimulationScalarKind, require,
};
use crate::compound::{CompoundEntryHistory, CompoundHistory};
use crate::display_lists::DisplayEditCompletion;
use std::sync::Arc;

/// Complete two-existing-stream helper application with composed all-entry history.
#[derive(Debug, Clone)]
pub struct SimulationMetadataHelperEdit {
    /// Complete immutable resulting carrier and metadata.
    pub archive: SimulationMetadataArchive,
    /// Byte-change completion; missing streams fail before any result is published.
    pub completion: DisplayEditCompletion,
    /// Both exact scalar replacements in inspected writer order: RunState then TimeStamp.
    pub scalar_edits: [SimulationScalarEdit; 2],
    /// All original carrier entries derived directly to their final identities.
    pub history: CompoundHistory,
    /// Always false: no COM/storage operation, command-line conversion or solver was executed.
    pub native_io_executed: bool,
}

impl SimulationMetadataArchive {
    /// Apply supplied helper words only when both roots exist, e.g. `metadata.apply_helper_stamp(-1)?`.
    pub fn apply_helper_stamp(&self, stamp: i32) -> Result<SimulationMetadataHelperEdit> {
        let state_id = self
            .required_scalar(SimulationScalarKind::RunState)?
            .id()
            .clone();
        self.required_scalar(SimulationScalarKind::TimeStamp)?;
        let projection = SimulationScalarHelperProjection::from_update_stamp(stamp);
        let state = self.edit_scalar(&state_id, projection.run_state.pattern())?;
        let stamp_id = state
            .archive
            .required_scalar(SimulationScalarKind::TimeStamp)?
            .id()
            .clone();
        let timestamp = state
            .archive
            .edit_scalar(&stamp_id, projection.time_stamp.pattern())?;
        helper_result(state, timestamp)
    }
}

fn compose_history(first: &CompoundHistory, second: &CompoundHistory) -> Result<CompoundHistory> {
    validate_history_join(first, second)?;
    let entries: Arc<[CompoundEntryHistory]> = first
        .entries
        .iter()
        .zip(second.entries.iter())
        .map(compose_entry)
        .collect::<Result<Vec<_>>>()?
        .into();
    Ok(CompoundHistory {
        parent: first.parent.clone(),
        result: second.result.clone(),
        entries,
    })
}

fn compose_entry(
    (first, second): (&CompoundEntryHistory, &CompoundEntryHistory),
) -> Result<CompoundEntryHistory> {
    require(
        first.result == second.parent,
        "scalar helper entry history",
        format!(
            "intermediate slots {} and {}; expected the same revision-bound entry",
            first.result.slot(),
            second.parent.slot()
        ),
    )?;
    Ok(CompoundEntryHistory {
        parent: first.parent.clone(),
        result: second.result.clone(),
        replaced_stream: first.replaced_stream || second.replaced_stream,
    })
}

fn helper_result(
    state: super::SimulationMetadataScalarEdit,
    timestamp: super::SimulationMetadataScalarEdit,
) -> Result<SimulationMetadataHelperEdit> {
    let history = compose_history(&state.carrier_edit.history, &timestamp.carrier_edit.history)?;
    let changed = state.scalar_edit.completion == DisplayEditCompletion::Changed
        || timestamp.scalar_edit.completion == DisplayEditCompletion::Changed;
    Ok(SimulationMetadataHelperEdit {
        archive: timestamp.archive,
        completion: if changed {
            DisplayEditCompletion::Changed
        } else {
            DisplayEditCompletion::Unchanged
        },
        scalar_edits: [state.scalar_edit, timestamp.scalar_edit],
        history,
        native_io_executed: false,
    })
}

fn validate_history_join(first: &CompoundHistory, second: &CompoundHistory) -> Result<()> {
    require(
        first.result == second.parent && first.entries.len() == second.entries.len(),
        "scalar helper carrier history",
        format!(
            "{} and {} entry derivations with linked revision {}; expected one complete shared intermediate carrier",
            first.entries.len(),
            second.entries.len(),
            first.result == second.parent
        ),
    )
}
