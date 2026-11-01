use super::super::SimulationRgbaColor;
use super::{
    SimulationLegend3DBackgroundPlan, SimulationLegend3DBackgroundRuntime,
    SimulationLegend3DBackgroundValues, required_3d_fact,
};
use crate::Result;

impl SimulationLegend3DBackgroundRuntime {
    /// Project constructor/factor/lighting inputs with explicit nearest-even rounding; e.g. `runtime.project_background_nearest_even()?`.
    pub fn project_background_nearest_even(self) -> Result<SimulationLegend3DBackgroundPlan> {
        let packed = required_3d_fact(self.packed_color_raw, "packed background argument")?;
        let constructed_color = SimulationRgbaColor::from_colorref_nearest_even(packed);
        let precision =
            required_3d_fact(self.precision_at_width_raw, "precision BC at width read")?;
        let lighting = required_3d_fact(self.lighting_result_raw, "lighting GLboolean result")?;
        let values = SimulationLegend3DBackgroundValues {
            constructed_color,
            precision_factor_raw: precision.wrapping_add(10),
            begin_primitive_raw: 9,
            lighting_capability_raw: 0xb50,
            restore_lighting: lighting != 0,
        };
        Ok(SimulationLegend3DBackgroundPlan::complete(self, values))
    }
}
