//! C851 local layout integers with explicit post-helper facts (#166).
mod arithmetic;
mod plan;
mod types;
use super::SimulationLegendStepInputs;
use crate::{BessemerError, FailureClass, Result};
pub use plan::SimulationLegendStepLayoutPlan;
pub use types::*;

impl SimulationLegendStepInputs {
    /// Project local layout integers across explicit helper boundaries; e.g. `inputs.project_step_layout(runtime)?`.
    pub fn project_step_layout(
        self,
        runtime: SimulationLegendStepLayoutRuntime,
    ) -> Result<SimulationLegendStepLayoutPlan> {
        if runtime.enable_raw == 0 {
            return Ok(SimulationLegendStepLayoutPlan::complete(
                self, runtime, None,
            ));
        }
        let values = arithmetic::step_layout_values(self, runtime)?;
        Ok(SimulationLegendStepLayoutPlan::complete(
            self,
            runtime,
            Some(values),
        ))
    }
}

fn required_layout_fact<T: Copy + std::fmt::Debug>(value: Option<T>, entity: &str) -> Result<T> {
    value.ok_or_else(|| {
        BessemerError::new(
            "legend step layout projection",
            entity,
            FailureClass::Missing,
            format!(
                "{value:?}; expected Some(explicit {entity} value) at the reached native dependency"
            ),
        )
    })
}
