use super::super::{SimulationPostDialog, SimulationPostStyle};
use super::SimulationLegendResetPlan;
use crate::Result;

impl SimulationPostDialog {
    /// Project reader resets from the existing explicit version, without editing bytes; e.g. `dialog.legend_read_zero_assignments()?`.
    pub fn legend_read_zero_assignments(&self) -> Result<SimulationLegendResetPlan> {
        self.version().legend_read_zero_assignments()
    }
}

impl SimulationPostStyle {
    /// Keep absent fields separate from explicit derived zero events; e.g. `style.legend_read_zero_assignments()?`.
    pub fn legend_read_zero_assignments(&self) -> Result<SimulationLegendResetPlan> {
        self.version().legend_read_zero_assignments()
    }
}
