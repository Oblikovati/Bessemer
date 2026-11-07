use super::super::SimulationRgbaColor;
use super::{
    SimulationLegendYieldBase, SimulationLegendYieldCall, SimulationLegendYieldInputs,
    SimulationLegendYieldPlan, SimulationLegendYieldRuntime, required_yield_fact,
};
use crate::Result;

impl SimulationLegendYieldInputs {
    /// Plan exact local calls with explicit nearest-even white; e.g. `inputs.plan_gray_calls_nearest_even(runtime)?`.
    pub fn plan_gray_calls_nearest_even(
        self,
        runtime: SimulationLegendYieldRuntime,
    ) -> Result<SimulationLegendYieldPlan> {
        let copied = required_yield_fact(self.gray_base_color, "legend.color_d8")?;
        let pair = [
            copied,
            SimulationRgbaColor::from_colorref_nearest_even(0xffffff),
        ];
        let index = required_yield_fact(runtime.split_index_raw, "runtime split index178")?;
        let calls = select_yield_calls(self, runtime, index)?;
        Ok(SimulationLegendYieldPlan::complete(
            self, runtime, pair, calls,
        ))
    }
}

fn select_yield_calls(
    inputs: SimulationLegendYieldInputs,
    runtime: SimulationLegendYieldRuntime,
    index: i32,
) -> Result<Vec<SimulationLegendYieldCall>> {
    let count = runtime.requested_output_count_raw;
    if index == -2 {
        return Ok(vec![yield_pair_call(inputs, 0, count)?]);
    }
    if index < 0 || index >= count {
        return Ok(Vec::new());
    }
    partial_yield_calls(inputs, runtime, index)
}

fn yield_pair_call(
    inputs: SimulationLegendYieldInputs,
    destination_byte_offset: i64,
    output_count_raw: i32,
) -> Result<SimulationLegendYieldCall> {
    Ok(SimulationLegendYieldCall {
        destination_byte_offset,
        output_count_raw,
        base_count_raw: 2,
        reversal_raw: required_yield_fact(inputs.reverse_base_palette_raw, "legend.word_c")?,
        base: SimulationLegendYieldBase::CopiedYieldPair,
    })
}

fn partial_yield_calls(
    inputs: SimulationLegendYieldInputs,
    runtime: SimulationLegendYieldRuntime,
    index: i32,
) -> Result<Vec<SimulationLegendYieldCall>> {
    let displacement = i64::from(index.wrapping_mul(3)) * 4;
    let suffix = yield_pair_call(
        inputs,
        displacement,
        runtime.requested_output_count_raw - index,
    )?;
    let state = required_yield_fact(runtime.after_suffix, "post-suffix reread state")?;
    let prefix = SimulationLegendYieldCall {
        destination_byte_offset: 0,
        output_count_raw: state.split_index_raw,
        base_count_raw: state.base_palette_count_raw,
        reversal_raw: state.reverse_base_palette_raw,
        base: SimulationLegendYieldBase::OrdinaryLegendArea,
    };
    Ok(vec![suffix, prefix])
}
