use super::super::{
    SimulationPostDialog, SimulationPostStyle, load_snapshot::BinaryLoadSnapshot,
    post_render::views::optional_render_signed_words,
};
use super::SimulationLegendHeightInputs;
use crate::Result;

impl SimulationPostDialog {
    /// Read only the actual stored cell-count multiplier; e.g. `dialog.legend_height_inputs()?`.
    pub fn legend_height_inputs(&self) -> Result<SimulationLegendHeightInputs> {
        height_inputs_from_snapshot(self)
    }
}

impl SimulationPostStyle {
    /// Preserve absence without inferring fonts or tick count; e.g. `style.legend_height_inputs()?`.
    pub fn legend_height_inputs(&self) -> Result<SimulationLegendHeightInputs> {
        height_inputs_from_snapshot(self)
    }
}

fn height_inputs_from_snapshot<T: BinaryLoadSnapshot>(
    snapshot: &T,
) -> Result<SimulationLegendHeightInputs> {
    let [cell_count_raw] = optional_render_signed_words(snapshot, ["legend.word_14"])?;
    Ok(SimulationLegendHeightInputs { cell_count_raw })
}
