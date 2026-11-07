//! C849 stored count and final integer product from an explicit helper result (#166).
mod plan;
mod types;
mod views;
use crate::{BessemerError, FailureClass, Result};
pub use plan::SimulationLegendHeightPlan;
pub use types::*;

impl SimulationLegendHeightInputs {
    /// Project the final signed32 product from an explicit helper result; e.g. `inputs.total_height_from_cell_height(Some(24))?`.
    pub fn total_height_from_cell_height(
        self,
        cell_height_raw: Option<i32>,
    ) -> Result<SimulationLegendHeightPlan> {
        let cell = required_height_word(cell_height_raw, "cell-height helper result")?;
        let count = required_height_word(self.cell_count_raw, "legend.word_14")?;
        Ok(SimulationLegendHeightPlan::complete(
            self,
            cell,
            cell.wrapping_mul(count),
        ))
    }
}

fn required_height_word(word: Option<i32>, entity: &str) -> Result<i32> {
    word.ok_or_else(|| {
        BessemerError::new(
            "legend height product",
            entity,
            FailureClass::Missing,
            "None; expected Some(explicit signed32 value) at the reached multiplication dependency",
        )
    })
}
