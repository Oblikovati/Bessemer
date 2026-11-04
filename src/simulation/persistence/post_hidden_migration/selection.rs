use super::{
    SimulationHiddenPartsMigrationInputs, SimulationHiddenPartsMigrationPlan,
    SimulationHiddenPartsMigrationValues, SimulationHiddenPartsStore,
};
use crate::{BessemerError, FailureClass, Result};

impl SimulationHiddenPartsMigrationInputs {
    /// Project only the bound legacy load migration; e.g. `inputs.project_legacy_hidden_parts()?.values()`.
    pub fn project_legacy_hidden_parts(self) -> Result<SimulationHiddenPartsMigrationPlan> {
        require_hidden_version(self.version.value())?;
        let original = self.stored_word_1f0_raw.ok_or_else(|| BessemerError::new(
            "legacy hidden-parts migration", "stored style1F0", FailureClass::Missing,
            "None; expected Some(original signed32 style1F0) in the explicit-field migration interval",
        ))?;
        Ok(SimulationHiddenPartsMigrationPlan::complete(
            self,
            hidden_pair_values(original),
        ))
    }
}

fn require_hidden_version(version: i32) -> Result<()> {
    if (700006..840008).contains(&version) {
        return Ok(());
    }
    Err(BessemerError::new(
        "legacy hidden-parts migration",
        "explicit-field migration version",
        FailureClass::Unsupported,
        format!(
            "{version}; expected version700006..840007; raw-class and later direct-read paths are separate"
        ),
    ))
}

fn hidden_pair_values(original: i32) -> SimulationHiddenPartsMigrationValues {
    use SimulationHiddenPartsStore::*;
    let (hidden_option_raw, runtime_word_1f0_raw, store) = match original {
        0 => (0, 0, ClearOptionWord),
        1..=3 => (1, 0, WriteAdjacentPairOne),
        _ => (1, 1, WriteBothWordsOne),
    };
    SimulationHiddenPartsMigrationValues {
        hidden_option_raw,
        runtime_word_1f0_raw,
        store,
    }
}
