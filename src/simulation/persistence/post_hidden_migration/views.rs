use super::super::{
    SimulationPersistVersion, SimulationPostDialog, SimulationPostStyle,
    load_snapshot::BinaryLoadSnapshot, post_render::views::optional_render_signed_words,
};
use super::SimulationHiddenPartsMigrationInputs;
use crate::Result;

impl SimulationPostDialog {
    /// Copy the explicit version and actual optional1F0 without migrating bytes; e.g. `dialog.hidden_parts_migration_inputs()?`.
    pub fn hidden_parts_migration_inputs(&self) -> Result<SimulationHiddenPartsMigrationInputs> {
        hidden_inputs_from_snapshot(self, self.version())
    }
}

impl SimulationPostStyle {
    /// Preserve the original stored word independently of derived state; e.g. `style.hidden_parts_migration_inputs()?`.
    pub fn hidden_parts_migration_inputs(&self) -> Result<SimulationHiddenPartsMigrationInputs> {
        hidden_inputs_from_snapshot(self, self.version())
    }
}

fn hidden_inputs_from_snapshot<T: BinaryLoadSnapshot>(
    snapshot: &T,
    version: SimulationPersistVersion,
) -> Result<SimulationHiddenPartsMigrationInputs> {
    let [stored_word_1f0_raw] = optional_render_signed_words(snapshot, ["style.word_1f0"])?;
    Ok(SimulationHiddenPartsMigrationInputs {
        version,
        stored_word_1f0_raw,
    })
}
