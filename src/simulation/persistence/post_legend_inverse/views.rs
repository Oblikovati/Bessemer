use super::super::{
    SimulationPostDialog, SimulationPostStyle, load_snapshot::BinaryLoadSnapshot,
    post_render::views::optional_render_signed_words,
};
use super::SimulationLegendPreferenceInputs;
use crate::Result;

impl SimulationPostDialog {
    /// Preserve optional canonical A8 without a runtime preference default; e.g. `dialog.legend_preference_inputs()?`.
    pub fn legend_preference_inputs(&self) -> Result<SimulationLegendPreferenceInputs> {
        inverse_inputs_from_snapshot(self)
    }
}

impl SimulationPostStyle {
    /// Keep original stored A8 distinct from its lossy zero-test view; e.g. `style.legend_preference_inputs()?`.
    pub fn legend_preference_inputs(&self) -> Result<SimulationLegendPreferenceInputs> {
        inverse_inputs_from_snapshot(self)
    }
}

fn inverse_inputs_from_snapshot<T: BinaryLoadSnapshot>(
    snapshot: &T,
) -> Result<SimulationLegendPreferenceInputs> {
    let [stored_a8_raw] = optional_render_signed_words(snapshot, ["legend.word_a8"])?;
    Ok(SimulationLegendPreferenceInputs { stored_a8_raw })
}
