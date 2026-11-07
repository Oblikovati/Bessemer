use super::{SimulationLegendStepPlan, SimulationLegendStepRuntime};

impl SimulationLegendStepRuntime {
    /// Evaluate the complete explicit-word policy, retaining raw BEC; e.g. `runtime.step_availability().availability_raw()`.
    pub fn step_availability(self) -> SimulationLegendStepPlan {
        let (result, mask) = step_policy(self);
        SimulationLegendStepPlan::complete(self, result, mask)
    }
}

fn step_policy(runtime: SimulationLegendStepRuntime) -> (i32, u8) {
    let mut mask = 0b11;
    let ordinary = runtime.step_count_raw > 1 && runtime.word_bd8_raw == 0;
    if runtime.step_count_raw > 1 {
        mask |= 1 << 3;
    }
    if runtime.plot_type_raw == 21 && runtime.step_count_raw > 1 {
        return (1, mask);
    }
    if !ordinary {
        return (0, mask);
    }
    analysis_step_policy(runtime, mask)
}

fn analysis_step_policy(runtime: SimulationLegendStepRuntime, mut mask: u8) -> (i32, u8) {
    mask |= 1 << 2;
    if runtime.analysis_type_raw != 8 {
        return (1, mask);
    }
    mask |= 1 << 4;
    if runtime.word_be8_raw != 2 {
        return (1, mask);
    }
    (runtime.word_bec_raw, mask | (1 << 5))
}
