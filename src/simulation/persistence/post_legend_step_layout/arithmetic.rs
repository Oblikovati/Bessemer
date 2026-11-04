use super::super::SimulationLegendStepInputs;
use super::{
    SimulationLegendStepLayoutAfterHelpers, SimulationLegendStepLayoutRuntime,
    SimulationLegendStepLayoutValues, required_layout_fact,
};
use crate::Result;

pub(super) fn step_layout_values(
    inputs: SimulationLegendStepInputs,
    runtime: SimulationLegendStepLayoutRuntime,
) -> Result<SimulationLegendStepLayoutValues> {
    let fc = required_layout_fact(inputs.layout_switch_raw, "entry legend.word_fc")?;
    let initial_y_raw = initial_step_y(fc, runtime)?;
    let point_x_raw = required_layout_fact(runtime.point_x_raw, "pointX")?;
    let after = required_layout_fact(runtime.after_helpers, "post-helper state")?;
    let spacing_raw = adjusted_step_spacing(after)?;
    Ok(SimulationLegendStepLayoutValues {
        point_x_raw,
        initial_y_raw,
        spacing_raw,
        vertical_after_spacing_raw: initial_y_raw.wrapping_sub(spacing_raw),
    })
}

fn initial_step_y(fc: i32, runtime: SimulationLegendStepLayoutRuntime) -> Result<i32> {
    if fc == 0 {
        let point = required_layout_fact(runtime.point_y_raw, "pointY")?;
        let size = required_layout_fact(runtime.size_y_raw, "sizeY")?;
        return Ok(point.wrapping_sub(size));
    }
    let upper = required_layout_fact(runtime.word_12c_raw, "runtime word12C")?;
    let lower = required_layout_fact(runtime.word_124_raw, "runtime word124")?;
    let quotient = upper.wrapping_sub(lower) / 20;
    let size = required_layout_fact(runtime.size_y_raw, "sizeY")?;
    let point = required_layout_fact(runtime.point_y_raw, "pointY")?;
    Ok(quotient.wrapping_sub(size).wrapping_add(point))
}

fn adjusted_step_spacing(after: SimulationLegendStepLayoutAfterHelpers) -> Result<i32> {
    let mut spacing = required_layout_fact(after.spacing_raw, "post-helper spacing")?;
    let fc = required_layout_fact(after.layout_switch_raw, "post-helper legend.word_fc")?;
    if fc != 0 {
        spacing = spacing.wrapping_mul(2).wrapping_sub(4);
    }
    let first = required_layout_fact(after.word_1b0_raw, "runtime word1B0")?;
    if first != 0 {
        return Ok(spacing.wrapping_mul(3));
    }
    let second = required_layout_fact(after.word_1b4_raw, "runtime word1B4")?;
    Ok(if second != 0 {
        spacing.wrapping_mul(3)
    } else {
        spacing
    })
}
