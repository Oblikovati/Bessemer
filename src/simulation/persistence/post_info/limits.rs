use super::super::{load_reader::outer_limits, thermal::SimulationThermalLimits};

/// Independent complete PostInfo budgets; the imported reader's default character cap is 0x100000.
#[derive(Debug, Clone, Copy)]
pub struct SimulationPostInfoLimits {
    /// Entire input and reconstructed image budget.
    pub maximum_bytes: usize,
    /// Every fixed scalar and atomic text field.
    pub maximum_fields: usize,
    /// Aggregate declared characters.
    pub maximum_entries: usize,
    /// Maximum payload bytes of the optional text, excluding its count.
    pub maximum_text_bytes: usize,
    /// Explicit safety cap on the version700002 character count.
    pub maximum_characters: usize,
}

impl SimulationPostInfoLimits {
    pub(super) fn wire_limits(self) -> SimulationThermalLimits {
        outer_limits(
            self.maximum_bytes,
            self.maximum_fields,
            self.maximum_entries,
            self.maximum_text_bytes,
        )
    }
}
