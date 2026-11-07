use super::super::{load_reader::outer_limits, thermal::SimulationThermalLimits};

/// Explicit budgets for one historical helper body, independent of runtime dispatch.
#[derive(Debug, Clone, Copy)]
pub struct SimulationLegacyPlotLimits {
    /// Maximum complete input and reconstructed output bytes.
    pub maximum_bytes: usize,
    /// Maximum complete physical fields, counting a text frame once.
    pub maximum_fields: usize,
    /// Maximum aggregate declared text characters; fixed buffers do not charge entries.
    pub maximum_entries: usize,
    /// Maximum payload bytes for each counted text, excluding its four-byte count.
    pub maximum_text_bytes: usize,
    /// Per-text character budget, additionally bounded by the documented 512-character profile.
    pub maximum_characters: usize,
}

impl SimulationLegacyPlotLimits {
    pub(super) fn wire_limits(self) -> SimulationThermalLimits {
        outer_limits(
            self.maximum_bytes,
            self.maximum_fields,
            self.maximum_entries,
            self.maximum_text_bytes,
        )
    }
}
