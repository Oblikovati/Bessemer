use super::super::{
    SimulationPostDialog, SimulationPostStyle, load_snapshot::BinaryLoadSnapshot,
    post_render::views::optional_render_word_bits,
};
use super::SimulationLegendFormatInputs;
use crate::Result;

impl SimulationPostStyle {
    /// Read only present canonical format words without initializer defaults; e.g. `style.legend_format_inputs()?`.
    pub fn legend_format_inputs(&self) -> Result<SimulationLegendFormatInputs> {
        format_inputs_from_snapshot(self)
    }
}

impl SimulationPostDialog {
    /// Read the embedded legend's optional format words; e.g. `dialog.legend_format_inputs()?`.
    pub fn legend_format_inputs(&self) -> Result<SimulationLegendFormatInputs> {
        format_inputs_from_snapshot(self)
    }
}

fn format_inputs_from_snapshot<T: BinaryLoadSnapshot>(
    snapshot: &T,
) -> Result<SimulationLegendFormatInputs> {
    let mut words = [None; 4];
    for (slot, path) in words.iter_mut().zip([
        "legend.word_b8",
        "legend.word_bc",
        "legend.word_c4",
        "legend.word_c8",
    ]) {
        *slot = optional_render_word_bits(snapshot, path)?.map(|word| word as i32);
    }
    Ok(SimulationLegendFormatInputs {
        mode_raw: words[0],
        precision_raw: words[1],
        alternate_enabled_raw: words[2],
        alternate_selector_raw: words[3],
    })
}
