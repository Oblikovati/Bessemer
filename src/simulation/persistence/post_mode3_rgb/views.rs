use super::super::{
    SimulationPostDialog, SimulationPostStyle, load_snapshot::BinaryLoadSnapshot,
    post_render::views::render_word, rgba::collect_wire_colors,
};
use super::SimulationMode3RgbInputs;
use crate::{FailureClass, Result};

impl SimulationPostDialog {
    /// Retain actual stored mode/color and supplied parent plot code; e.g. `dialog.mode3_geometry_rgb_inputs()?`.
    pub fn mode3_geometry_rgb_inputs(&self) -> Result<SimulationMode3RgbInputs> {
        let mut inputs = mode3_snapshot_inputs(self)?;
        inputs.plot_type_raw = Some(render_word(self, "fourteen_words0c_28_30_3c_518_51c[1]")?);
        Ok(inputs)
    }
}
impl SimulationPostStyle {
    /// Keep older named-field absence and standalone parent context explicit; e.g. `style.mode3_geometry_rgb_inputs()?`.
    pub fn mode3_geometry_rgb_inputs(&self) -> Result<SimulationMode3RgbInputs> {
        mode3_snapshot_inputs(self)
    }
}
fn mode3_snapshot_inputs<T: BinaryLoadSnapshot>(snapshot: &T) -> Result<SimulationMode3RgbInputs> {
    let mode = match render_word(snapshot, "style.word_1b8") {
        Ok(mode) => Some(mode),
        Err(error) if error.class == FailureClass::Missing => None,
        Err(error) => return Err(error),
    };
    let colors = collect_wire_colors(snapshot.fields(), snapshot.field_offsets())?;
    let color = colors
        .iter()
        .find(|color| color.path.as_ref() == "style.color_1dc")
        .map(|color| color.color);
    Ok(SimulationMode3RgbInputs {
        mode_1b8_raw: mode,
        color_1dc: color,
        plot_type_raw: None,
    })
}
