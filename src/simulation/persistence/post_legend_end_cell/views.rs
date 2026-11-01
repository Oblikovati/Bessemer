use super::super::{
    SimulationPostDialog, SimulationPostStyle, load_snapshot::BinaryLoadSnapshot,
    post_render::views::optional_render_signed_words, rgba::collect_wire_colors,
};
use super::SimulationLegendEndCellInputs;
use crate::Result;

impl SimulationPostDialog {
    /// Preserve stored E8/EC presence without live activation defaults; e.g. `dialog.legend_end_cell_inputs()?`.
    pub fn legend_end_cell_inputs(&self) -> Result<SimulationLegendEndCellInputs> {
        end_cell_inputs_from_snapshot(self)
    }
}

impl SimulationPostStyle {
    /// Preserve all versioned end-cell flag/color bits; e.g. `style.legend_end_cell_inputs()?`.
    pub fn legend_end_cell_inputs(&self) -> Result<SimulationLegendEndCellInputs> {
        end_cell_inputs_from_snapshot(self)
    }
}

fn end_cell_inputs_from_snapshot<T: BinaryLoadSnapshot>(
    snapshot: &T,
) -> Result<SimulationLegendEndCellInputs> {
    let [enabled_raw] = optional_render_signed_words(snapshot, ["legend.word_e8"])?;
    let colors = collect_wire_colors(snapshot.fields(), snapshot.field_offsets())?;
    let color = colors
        .iter()
        .find(|color| color.path.as_ref() == "legend.color_ec")
        .map(|color| color.color);
    Ok(SimulationLegendEndCellInputs { enabled_raw, color })
}
