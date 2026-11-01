use super::super::{
    SimulationPostDialog, SimulationPostStyle, load_snapshot::BinaryLoadSnapshot,
    post_render::views::optional_render_signed_words,
};
use super::SimulationLegendStepInputs;
use crate::Result;

impl SimulationPostDialog {
    /// Read actual FC presence without supplying runtime words; e.g. `dialog.legend_step_inputs()?`.
    pub fn legend_step_inputs(&self) -> Result<SimulationLegendStepInputs> {
        step_inputs_from_snapshot(self)
    }
}

impl SimulationPostStyle {
    /// Preserve older FC absence and every signed layout pattern; e.g. `style.legend_step_inputs()?`.
    pub fn legend_step_inputs(&self) -> Result<SimulationLegendStepInputs> {
        step_inputs_from_snapshot(self)
    }
}

fn step_inputs_from_snapshot<T: BinaryLoadSnapshot>(
    snapshot: &T,
) -> Result<SimulationLegendStepInputs> {
    let [layout_switch_raw] = optional_render_signed_words(snapshot, ["legend.word_fc"])?;
    Ok(SimulationLegendStepInputs { layout_switch_raw })
}
