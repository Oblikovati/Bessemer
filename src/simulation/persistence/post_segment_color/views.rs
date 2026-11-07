use super::super::{
    SimulationPostDialog, SimulationPostStyle, load_snapshot::BinaryLoadSnapshot,
    rgba::collect_wire_colors,
};
use super::SimulationSegmentColorInputs;
use crate::Result;

impl SimulationPostDialog {
    /// Retain stored1BC availability without filling older opaque regions; e.g. `dialog.element_segment_color_inputs()?`.
    pub fn element_segment_color_inputs(&self) -> Result<SimulationSegmentColorInputs> {
        segment_snapshot_color(self)
    }
}
impl SimulationPostStyle {
    /// Retain standalone original1BC and unresolved alpha; e.g. `style.element_segment_color_inputs()?`.
    pub fn element_segment_color_inputs(&self) -> Result<SimulationSegmentColorInputs> {
        segment_snapshot_color(self)
    }
}
fn segment_snapshot_color<T: BinaryLoadSnapshot>(
    snapshot: &T,
) -> Result<SimulationSegmentColorInputs> {
    let colors = collect_wire_colors(snapshot.fields(), snapshot.field_offsets())?;
    Ok(SimulationSegmentColorInputs {
        color_1bc: colors
            .iter()
            .find(|color| color.path.as_ref() == "style.color_1bc")
            .map(|color| color.color),
    })
}
