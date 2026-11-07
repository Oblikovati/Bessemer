use super::super::super::{
    SimulationPostDialog, SimulationPostStyle, load_snapshot::BinaryLoadSnapshot,
    post_render::views::optional_render_word_bits,
};
use super::super::SimulationLegendComparisonState;
use super::state::STYLE_CHANNELS;
use super::{
    SimulationStyleComparisonColor as Color, SimulationStyleComparisonState,
    SimulationStyleComparisonWord as Word,
};
use crate::Result;

impl SimulationPostStyle {
    /// Preserve stored comparison facts and unknown runtime state; e.g. `style.style_comparison_state()?`.
    pub fn style_comparison_state(&self) -> Result<SimulationStyleComparisonState> {
        style_state_from_snapshot(self)
    }
}

impl SimulationPostDialog {
    /// Project the embedded style without applying migrations or defaults; e.g. `dialog.style_comparison_state()?`.
    pub fn style_comparison_state(&self) -> Result<SimulationStyleComparisonState> {
        style_state_from_snapshot(self)
    }
}

fn style_state_from_snapshot<T: BinaryLoadSnapshot>(
    snapshot: &T,
) -> Result<SimulationStyleComparisonState> {
    let legend = SimulationLegendComparisonState::from_word_reader(|path| {
        optional_render_word_bits(snapshot, path)
    })?;
    let mut state = SimulationStyleComparisonState::from_legend_state(legend);
    for member in Word::ALL {
        state.set_word_bits(
            member,
            optional_render_word_bits(snapshot, member.field_path())?,
        );
    }
    for color in Color::ALL {
        for channel in STYLE_CHANNELS {
            let path = format!("{}.component_{}", color.field_path(), channel as usize);
            state.set_component_bits(color, channel, optional_render_word_bits(snapshot, &path)?);
        }
    }
    Ok(state)
}
