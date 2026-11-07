//! Bounded C-888 local call planning; no native execution or triangle validity claim (#166).
mod plan;
mod types;
mod views;
use super::{SimulationRgbaColor, rgba::is_binary32_nan};
pub use plan::SimulationAuxTrianglePlan;
pub use types::*;

/// Original stored pass controls without inferred UI restrictions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationAuxiliaryTriangleControls {
    /// Style204, tested as signed nonzero.
    pub enable_raw: i32,
    /// Style208, selecting group RGB only when exactly one.
    pub group_selector_raw: i32,
    /// Every original style20C component.
    pub color: SimulationRgbaColor,
}

impl SimulationAuxiliaryTriangleControls {
    /// Model the caller under masked comparison exceptions; e.g. `controls.helper_requested_masked_exceptions()`.
    pub fn helper_requested_masked_exceptions(self) -> bool {
        let alpha = self.color.bits()[3];
        self.enable_raw != 0
            && alpha & 0x80000000 == 0
            && !is_binary32_nan(alpha)
            && alpha >= 0x3f7d70a4
    }

    /// Preserve exact-one selection independently of admission; e.g. `controls.per_group_rgb_requested()`.
    pub fn per_group_rgb_requested(self) -> bool {
        self.group_selector_raw == 1
    }

    /// Cap alpha only inside the caller's admitted domain; e.g. `controls.renderer_color_masked_exceptions()`.
    pub fn renderer_color_masked_exceptions(self) -> Option<SimulationRgbaColor> {
        if !self.helper_requested_masked_exceptions() {
            return None;
        }
        let mut bits = self.color.bits();
        bits[3] = bits[3].min(0x3f800000);
        Some(SimulationRgbaColor::from_bits(bits))
    }
}
