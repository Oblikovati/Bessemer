use super::super::{
    SimulationPostDialog, SimulationPostStyle,
    load_snapshot::BinaryLoadSnapshot,
    post_render::views::render_word,
    rgba::{SimulationRgbaColor, collect_wire_colors},
};
use super::SimulationVectorStyleControls;
use crate::{BessemerError, FailureClass, Result};

impl SimulationPostDialog {
    /// Require the complete stored vector group without filling older absence; e.g. `dialog.vector_style_controls()?`.
    pub fn vector_style_controls(&self) -> Result<SimulationVectorStyleControls> {
        vector_style_from_snapshot(self)
    }
}

impl SimulationPostStyle {
    /// Preserve raw vector words and all RGBA bits through the canonical snapshot; e.g. `style.vector_style_controls()?`.
    pub fn vector_style_controls(&self) -> Result<SimulationVectorStyleControls> {
        vector_style_from_snapshot(self)
    }
}

fn vector_style_from_snapshot<T: BinaryLoadSnapshot>(
    snapshot: &T,
) -> Result<SimulationVectorStyleControls> {
    let size_raw = render_word(snapshot, "style.word_28c")?;
    Ok(SimulationVectorStyleControls {
        size_raw,
        density_first_raw: render_word(snapshot, "style.word_290")?,
        density_second_raw: render_word(snapshot, "style.word_294")?,
        color_selector_raw: render_word(snapshot, "style.word_298")?,
        color: vector_style_color(snapshot)?,
    })
}

fn vector_style_color<T: BinaryLoadSnapshot>(snapshot: &T) -> Result<SimulationRgbaColor> {
    collect_wire_colors(snapshot.fields(), snapshot.field_offsets())?
        .into_iter()
        .find(|color| color.path.as_ref() == "style.color_29c")
        .map(|color| color.color)
        .ok_or_else(|| {
            BessemerError::new(
                "vector style controls",
                "style.color_29c",
                FailureClass::Missing,
                "color None; expected a complete stored 16-byte RGBA group",
            )
        })
}
