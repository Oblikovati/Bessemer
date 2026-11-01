use super::super::{SimulationPostDialog, post_render::views::render_word};
use super::SimulationUnitTypeInputs;
use crate::{FailureClass, Result};

impl SimulationPostDialog {
    /// Capture stored controls while requiring explicit BE8/global runtime state; e.g. `dialog.unit_type_inputs(None, None)?.unit_type_code()`.
    pub fn unit_type_inputs(
        &self,
        word_be8: Option<i32>,
        actual_freq_disp_flag: Option<i32>,
    ) -> Result<SimulationUnitTypeInputs> {
        let word_bec = match render_word(self, "dialog.word_bec") {
            Ok(word) => Some(word),
            Err(error) if error.class == FailureClass::Missing => None,
            Err(error) => return Err(error),
        };
        Ok(SimulationUnitTypeInputs {
            analysis: render_word(self, "fourteen_words0c_28_30_3c_518_51c[0]")?,
            plot_type: render_word(self, "fourteen_words0c_28_30_3c_518_51c[1]")?,
            component: render_word(self, "fourteen_words0c_28_30_3c_518_51c[3]")?,
            word_be8,
            word_bec,
            actual_freq_disp_flag,
        })
    }
}
