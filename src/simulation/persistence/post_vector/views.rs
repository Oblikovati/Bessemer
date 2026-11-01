use super::super::SimulationPostDialog;
use super::super::post_render::views::render_word;
use super::{SimulationComponentVectorInputs, SimulationVectorDirectionInputs};
use crate::Result;

impl SimulationPostDialog {
    /// Capture raw controls with explicit loaded-string state; e.g. `dialog.vector_direction_inputs(true)?.vector_direction()`.
    pub fn vector_direction_inputs(
        &self,
        loaded_text_nonempty: bool,
    ) -> Result<SimulationVectorDirectionInputs> {
        Ok(SimulationVectorDirectionInputs {
            provider: SimulationComponentVectorInputs {
                analysis: render_word(self, "fourteen_words0c_28_30_3c_518_51c[0]")?,
                plot_type: render_word(self, "fourteen_words0c_28_30_3c_518_51c[1]")?,
                component: render_word(self, "fourteen_words0c_28_30_3c_518_51c[3]")?,
                fourth_word: render_word(self, "fourteen_words0c_28_30_3c_518_51c[2]")?,
            },
            word_570: render_word(self, "three_words568_570[2]")?,
            loaded_text_nonempty,
        })
    }
}
