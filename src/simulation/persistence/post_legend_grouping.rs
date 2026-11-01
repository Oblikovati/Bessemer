//! C858 stored locale grouping control, separate from runtime locale/string state (#166).
use super::{
    SimulationPostDialog, SimulationPostStyle, load_snapshot::BinaryLoadSnapshot,
    post_render::views::optional_render_signed_words,
};
use crate::Result;

/// Actual optional C0 wire control; no locale characters or rendered output are inferred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendGroupingInputs {
    /// Nonzero enables conditional grouping; None preserves absence before840008.
    pub locale_thousands_grouping_enabled_raw: Option<i32>,
}

impl SimulationPostDialog {
    /// Preserve stored C0 independently of read-time resets; e.g. `dialog.legend_grouping_inputs()?`.
    pub fn legend_grouping_inputs(&self) -> Result<SimulationLegendGroupingInputs> {
        grouping_inputs_from_snapshot(self)
    }
}

impl SimulationPostStyle {
    /// Preserve arbitrary signed controls without choosing locale state; e.g. `style.legend_grouping_inputs()?`.
    pub fn legend_grouping_inputs(&self) -> Result<SimulationLegendGroupingInputs> {
        grouping_inputs_from_snapshot(self)
    }
}

fn grouping_inputs_from_snapshot<T: BinaryLoadSnapshot>(
    snapshot: &T,
) -> Result<SimulationLegendGroupingInputs> {
    let [locale_thousands_grouping_enabled_raw] =
        optional_render_signed_words(snapshot, ["legend.word_c0"])?;
    Ok(SimulationLegendGroupingInputs {
        locale_thousands_grouping_enabled_raw,
    })
}
