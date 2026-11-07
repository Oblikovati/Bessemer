use super::super::{
    SimulationPostDialog, SimulationPostStyle, load_snapshot::BinaryLoadSnapshot,
    post_render::views::optional_render_signed_words,
};
use super::SimulationLegendWidthInputs;
use crate::Result;

impl SimulationPostDialog {
    /// Preserve optional B4 without manufacturing runtime font state; e.g. `dialog.legend_width_inputs()?`.
    pub fn legend_width_inputs(&self) -> Result<SimulationLegendWidthInputs> {
        width_inputs_from_snapshot(self)
    }
}

impl SimulationPostStyle {
    /// Preserve raw signed width selectors and older absence; e.g. `style.legend_width_inputs()?`.
    pub fn legend_width_inputs(&self) -> Result<SimulationLegendWidthInputs> {
        width_inputs_from_snapshot(self)
    }
}

fn width_inputs_from_snapshot<T: BinaryLoadSnapshot>(
    snapshot: &T,
) -> Result<SimulationLegendWidthInputs> {
    let [selector_raw] = optional_render_signed_words(snapshot, ["legend.word_b4"])?;
    Ok(SimulationLegendWidthInputs { selector_raw })
}
