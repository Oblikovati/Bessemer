use super::{SimulationLegendEndCellPlan, SimulationLegendEndCellRuntime};
use crate::{BessemerError, FailureClass, Result};

impl SimulationLegendEndCellRuntime {
    /// Evaluate reached gates and copy explicit RGB words; e.g. `runtime.project_end_cell()?.rgb_words()`.
    pub fn project_end_cell(self) -> Result<SimulationLegendEndCellPlan> {
        let plot = required_end_cell_fact(self.plot_word_524_raw, "runtime plot524")?;
        if plot == 0 {
            return Ok(SimulationLegendEndCellPlan::complete(self, None, 1));
        }
        let enabled = required_end_cell_fact(self.enabled_at_gate_raw, "read-time legend E8")?;
        if enabled == 0 {
            return Ok(SimulationLegendEndCellPlan::complete(self, None, 3));
        }
        let color = required_end_cell_fact(self.color_at_rgb_copy, "read-time legend EC color")?;
        let [red, green, blue, _] = color.bits();
        Ok(SimulationLegendEndCellPlan::complete(
            self,
            Some([red, green, blue]),
            7,
        ))
    }
}

fn required_end_cell_fact<T: Copy + std::fmt::Debug>(value: Option<T>, entity: &str) -> Result<T> {
    value.ok_or_else(|| {
        BessemerError::new(
            "legend end-cell projection",
            entity,
            FailureClass::Missing,
            format!("{value:?}; expected Some(explicit {entity} value) at the reached native read"),
        )
    })
}
