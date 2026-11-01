//! Complete bounded C889 caller/helper requests with explicit runtime and floating-point assumptions (#166).
mod admission;
mod plan;
mod types;
mod views;
use super::{
    SimulationRgbaColor,
    rgba::cap_alpha_one_masked_daz_ftz_off as triangle_alpha_masked_daz_ftz_off,
};
pub use plan::SimulationTriangleColorPlan;
pub use types::*;

impl SimulationTriangleColorControls {
    pub(super) fn components(self) -> (i32, SimulationRgbaColor) {
        match self {
            Self::HiddenParts {
                selector_raw,
                color,
            }
            | Self::ModelTessellation {
                selector_raw,
                color,
                ..
            } => (selector_raw, color),
        }
    }

    pub(super) fn hidden_class(self) -> bool {
        matches!(self, Self::HiddenParts { .. })
    }
}
