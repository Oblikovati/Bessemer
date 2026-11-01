use super::super::{
    SimulationPostDialog, SimulationPostStyle, SimulationRgbaColor,
    load_snapshot::BinaryLoadSnapshot, post_render::views::render_word, rgba::collect_wire_colors,
};
use super::SimulationTriangleColorControls as Controls;
use crate::{BessemerError, FailureClass, Result};

impl SimulationPostDialog {
    /// Require actual hidden selector/color fields; e.g. `dialog.hidden_parts_triangle_controls()?`.
    pub fn hidden_parts_triangle_controls(&self) -> Result<Controls> {
        hidden_triangle_snapshot(self)
    }
    /// Require actual model override/selector/color fields; e.g. `dialog.model_tessellation_triangle_controls()?`.
    pub fn model_tessellation_triangle_controls(&self) -> Result<Controls> {
        model_triangle_snapshot(self)
    }
}

impl SimulationPostStyle {
    /// Preserve hidden selector and all stored RGBA bits; e.g. `style.hidden_parts_triangle_controls()?`.
    pub fn hidden_parts_triangle_controls(&self) -> Result<Controls> {
        hidden_triangle_snapshot(self)
    }
    /// Preserve model override, selector and all RGBA bits; e.g. `style.model_tessellation_triangle_controls()?`.
    pub fn model_tessellation_triangle_controls(&self) -> Result<Controls> {
        model_triangle_snapshot(self)
    }
}

fn hidden_triangle_snapshot<T: BinaryLoadSnapshot>(snapshot: &T) -> Result<Controls> {
    Ok(Controls::HiddenParts {
        selector_raw: render_word(snapshot, "style.word_1f0")?,
        color: triangle_snapshot_color(snapshot, "style.color_1f4")?,
    })
}

fn model_triangle_snapshot<T: BinaryLoadSnapshot>(snapshot: &T) -> Result<Controls> {
    Ok(Controls::ModelTessellation {
        override_raw: render_word(snapshot, "style.word_21c")?,
        selector_raw: render_word(snapshot, "style.word_230")?,
        color: triangle_snapshot_color(snapshot, "style.color_220")?,
    })
}

fn triangle_snapshot_color<T: BinaryLoadSnapshot>(
    snapshot: &T,
    path: &str,
) -> Result<SimulationRgbaColor> {
    collect_wire_colors(snapshot.fields(), snapshot.field_offsets())?
        .into_iter()
        .find(|color| color.path.as_ref() == path)
        .map(|color| color.color)
        .ok_or_else(|| {
            BessemerError::new(
                "triangle color controls",
                path,
                FailureClass::Missing,
                "color None; expected a complete stored16-byte RGBA group",
            )
        })
}
