use super::super::{
    SimulationPostDialog, SimulationPostStyle, load_snapshot::BinaryLoadSnapshot,
    post_render::views::optional_render_signed_words,
};
use super::SimulationLegendPaletteInputs;
use crate::Result;

impl SimulationPostStyle {
    /// Read only actually stored raw palette controls; e.g. `style.legend_palette_inputs()?`.
    pub fn legend_palette_inputs(&self) -> Result<SimulationLegendPaletteInputs> {
        palette_inputs_from_snapshot(self)
    }
}

impl SimulationPostDialog {
    /// Preserve embedded legend control absence without initializing it; e.g. `dialog.legend_palette_inputs()?`.
    pub fn legend_palette_inputs(&self) -> Result<SimulationLegendPaletteInputs> {
        palette_inputs_from_snapshot(self)
    }
}

fn palette_inputs_from_snapshot<T: BinaryLoadSnapshot>(
    snapshot: &T,
) -> Result<SimulationLegendPaletteInputs> {
    let words = optional_render_signed_words(
        snapshot,
        [
            "legend.word_8",
            "legend.word_c",
            "legend.word_10",
            "legend.word_d4",
        ],
    )?;
    Ok(SimulationLegendPaletteInputs {
        mapping_mode_raw: words[0],
        reverse_base_palette_raw: words[1],
        base_palette_count_raw: words[2],
        gray_above_yield_raw: words[3],
    })
}
