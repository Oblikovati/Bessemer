use super::super::{
    SimulationPostDialog, SimulationPostStyle, load_snapshot::BinaryLoadSnapshot,
    post_render::views::render_word,
};
use super::SimulationExtremaAnnotationControls;
use crate::Result;

impl SimulationPostDialog {
    /// Read both raw annotation controls without initializer fallbacks; e.g. `dialog.extrema_annotation_controls()?`.
    pub fn extrema_annotation_controls(&self) -> Result<SimulationExtremaAnnotationControls> {
        annotations_from_snapshot(self)
    }
}

impl SimulationPostStyle {
    /// Preserve missing older controls rather than inferring migration values; e.g. `style.extrema_annotation_controls()?`.
    pub fn extrema_annotation_controls(&self) -> Result<SimulationExtremaAnnotationControls> {
        annotations_from_snapshot(self)
    }
}

fn annotations_from_snapshot<T: BinaryLoadSnapshot>(
    snapshot: &T,
) -> Result<SimulationExtremaAnnotationControls> {
    Ok(SimulationExtremaAnnotationControls {
        minimum_raw: render_word(snapshot, "style.word_234")?,
        maximum_raw: render_word(snapshot, "style.word_238")?,
    })
}
