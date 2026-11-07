use super::arithmetic::{initial_width_patterns, selected_width_pattern};
use super::{SimulationLegendWidthPlan, SimulationLegendWidthRuntime, SimulationLegendWidthValues};
use crate::{BessemerError, FailureClass, Result};

impl SimulationLegendWidthRuntime {
    /// Project each width stage under explicit nearest-even rounding; e.g. `runtime.project_width_nearest_even()?.values()`.
    pub fn project_width_nearest_even(self) -> Result<SimulationLegendWidthPlan> {
        let font = required_width_word(self.font_size_at_entry_raw, "read-time font68")?;
        let [converted_font_bits, scaled_font_bits, initial_width_bits] =
            initial_width_patterns(font);
        let selector = required_width_word(self.selector_at_entry_raw, "read-time legend B4")?;
        let (operation, selected_width_bits) = selected_width_pattern(initial_width_bits, selector);
        let values = SimulationLegendWidthValues {
            converted_font_bits,
            scaled_font_bits,
            initial_width_bits,
            selected_width_bits,
            operation,
        };
        Ok(SimulationLegendWidthPlan::complete(self, values))
    }
}

fn required_width_word(value: Option<i32>, entity: &str) -> Result<i32> {
    value.ok_or_else(|| {
        BessemerError::new(
            "legend eDrawings width projection",
            entity,
            FailureClass::Missing,
            format!(
                "{value:?}; expected Some(explicit signed32 {entity}) at the reached native read"
            ),
        )
    })
}
