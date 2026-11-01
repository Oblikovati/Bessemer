use super::super::rgba::{SimulationPostColorView, collect_wire_colors};
use super::SimulationPostDialog;
use crate::Result;

impl SimulationPostDialog {
    /// Read every stored color with its complete body span; e.g. `body.colors()?`.
    pub fn colors(&self) -> Result<Vec<SimulationPostColorView>> {
        collect_wire_colors(&self.wire_fields, &self.offsets)
    }
}
