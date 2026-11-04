use super::super::{
    SimulationPostDialog, SimulationPostStyle, SimulationRgbaColor,
    load_snapshot::BinaryLoadSnapshot, post_render::views::render_word, rgba::collect_wire_colors,
};
use super::SimulationAuxiliaryTriangleControls;
use crate::{BessemerError, FailureClass, Result};

impl SimulationPostDialog {
    /// Require the actual840008 group without defaults; e.g. `dialog.auxiliary_triangle_controls()?`.
    pub fn auxiliary_triangle_controls(&self) -> Result<SimulationAuxiliaryTriangleControls> {
        aux_from_snapshot(self)
    }
}

impl SimulationPostStyle {
    /// Preserve stored pass controls and all RGBA bits; e.g. `style.auxiliary_triangle_controls()?`.
    pub fn auxiliary_triangle_controls(&self) -> Result<SimulationAuxiliaryTriangleControls> {
        aux_from_snapshot(self)
    }
}

fn aux_from_snapshot<T: BinaryLoadSnapshot>(
    snapshot: &T,
) -> Result<SimulationAuxiliaryTriangleControls> {
    Ok(SimulationAuxiliaryTriangleControls {
        enable_raw: render_word(snapshot, "style.word_204")?,
        group_selector_raw: render_word(snapshot, "style.word_208")?,
        color: aux_color(snapshot)?,
    })
}

fn aux_color<T: BinaryLoadSnapshot>(snapshot: &T) -> Result<SimulationRgbaColor> {
    collect_wire_colors(snapshot.fields(), snapshot.field_offsets())?
        .into_iter()
        .find(|color| color.path.as_ref() == "style.color_20c")
        .map(|color| color.color)
        .ok_or_else(|| {
            BessemerError::new(
                "auxiliary triangle controls",
                "style.color_20c",
                FailureClass::Missing,
                "color None; expected a complete stored16-byte RGBA group",
            )
        })
}
