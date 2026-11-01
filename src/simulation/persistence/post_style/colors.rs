use super::super::rgba::{SimulationPostColorView, collect_wire_colors};
use super::SimulationPostStyle;
use crate::Result;

impl SimulationPostStyle {
    /// Read every stored color with its complete body span; e.g. `body.colors()?`.
    pub fn colors(&self) -> Result<Vec<SimulationPostColorView>> {
        collect_wire_colors(&self.wire_fields, &self.offsets)
    }
}
