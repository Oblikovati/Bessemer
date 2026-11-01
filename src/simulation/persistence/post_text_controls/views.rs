use super::super::{
    SimulationPostDialog, SimulationPostStyle, SimulationPostStyleText,
    load_snapshot::BinaryLoadSnapshot, post_render::views::render_word,
    thermal::SimulationThermalValue,
};
use super::SimulationPostTextControls;
use crate::{BessemerError, FailureClass, Result};

impl SimulationPostDialog {
    /// Require canonical stored controls and complete counted text; e.g. `dialog.text_display_controls()?`.
    pub fn text_display_controls(&self) -> Result<SimulationPostTextControls> {
        text_control_snapshot(self)
    }
}

impl SimulationPostStyle {
    /// Preserve raw display words and all counted code units; e.g. `style.text_display_controls()?`.
    pub fn text_display_controls(&self) -> Result<SimulationPostTextControls> {
        text_control_snapshot(self)
    }
}

fn text_control_snapshot<T: BinaryLoadSnapshot>(
    snapshot: &T,
) -> Result<SimulationPostTextControls> {
    Ok(SimulationPostTextControls {
        primary_enabled_raw: render_word(snapshot, "style.word_23c")?,
        seven_lines_enabled_raw: render_word(snapshot, "style.word_240")?,
        stored_text: text_control_wire(snapshot)?,
    })
}

fn text_control_wire<T: BinaryLoadSnapshot>(snapshot: &T) -> Result<SimulationPostStyleText> {
    let field = snapshot
        .fields()
        .iter()
        .find(|field| field.path.as_ref() == "style.text");
    match field.map(|field| &field.value) {
        Some(SimulationThermalValue::Bytes(frame)) => {
            SimulationPostStyleText::from_frame(frame, frame.len())
        }
        value => Err(BessemerError::new(
            "post text controls",
            "style.text",
            if value.is_none() {
                FailureClass::Missing
            } else {
                FailureClass::Invalid
            },
            format!("{value:?}; expected a complete counted UTF16 byte frame"),
        )),
    }
}
