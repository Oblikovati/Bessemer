use super::super::{
    SimulationPostDialog, SimulationPostStyle, load_snapshot::BinaryLoadSnapshot,
    post_render::views::optional_render_signed_words, rgba::collect_wire_colors,
};
use super::SimulationLegendYieldInputs;
use crate::Result;

impl SimulationPostDialog {
    /// Read stored D8 and reversal without inventing runtime split state; e.g. `dialog.legend_yield_inputs()?`.
    pub fn legend_yield_inputs(&self) -> Result<SimulationLegendYieldInputs> {
        yield_inputs_from_snapshot(self)
    }
}

impl SimulationPostStyle {
    /// Preserve version-conditioned D8 absence and raw channels; e.g. `style.legend_yield_inputs()?`.
    pub fn legend_yield_inputs(&self) -> Result<SimulationLegendYieldInputs> {
        yield_inputs_from_snapshot(self)
    }
}

fn yield_inputs_from_snapshot<T: BinaryLoadSnapshot>(
    snapshot: &T,
) -> Result<SimulationLegendYieldInputs> {
    let [reverse_base_palette_raw] = optional_render_signed_words(snapshot, ["legend.word_c"])?;
    let colors = collect_wire_colors(snapshot.fields(), snapshot.field_offsets())?;
    let gray_base_color = colors
        .iter()
        .find(|color| color.path.as_ref() == "legend.color_d8")
        .map(|color| color.color);
    Ok(SimulationLegendYieldInputs {
        gray_base_color,
        reverse_base_palette_raw,
    })
}
