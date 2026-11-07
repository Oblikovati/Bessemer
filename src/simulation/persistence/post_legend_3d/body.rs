use super::super::SimulationLegendHeightInputs;
use super::{
    SimulationLegend3DBodyPlan, SimulationLegend3DBodyRuntime, SimulationLegend3DBodyValues,
    required_3d_fact,
};
use crate::Result;

impl SimulationLegendHeightInputs {
    /// Project conditional local buffer/GL requests with an explicit later count; e.g. `inputs.project_3d_body_requests(runtime)?`.
    pub fn project_3d_body_requests(
        self,
        runtime: SimulationLegend3DBodyRuntime,
    ) -> Result<SimulationLegend3DBodyPlan> {
        let entry = required_3d_fact(self.cell_count_raw, "allocation-entry count14")?;
        let later = required_3d_fact(runtime.cell_count_at_mapping_raw, "post-vertex count14")?;
        let argument = required_3d_fact(runtime.rendering_argument_raw, "rendering argument")?;
        let values = body_request_values(entry, later, argument);
        Ok(SimulationLegend3DBodyPlan::complete(self, runtime, values))
    }
}

fn body_request_values(entry: i32, later: i32, argument: i32) -> SimulationLegend3DBodyValues {
    let pairs = entry.wrapping_add(1);
    let vertex_floats = pairs.wrapping_mul(6);
    let requested = if argument == 2 {
        later.wrapping_add(1)
    } else {
        later
    };
    let color_floats = requested.wrapping_add(1).wrapping_mul(3);
    SimulationLegend3DBodyValues {
        allocation_pairs_raw: pairs,
        vertex_float_count_raw: vertex_floats,
        vertex_allocation_bytes_raw: native_float_byte_request(vertex_floats),
        mapping_request_count_raw: requested,
        color_float_count_raw: color_floats,
        color_allocation_bytes_raw: native_float_byte_request(color_floats),
        initial_shade_model_raw: if argument == 2 { 0x1d01 } else { 0x1d00 },
        begin_primitive_raw: if argument == 2 { 8 } else { 7 },
        later_smooth_argument_raw: 0x1d01,
    }
}

fn native_float_byte_request(floats_raw: i32) -> u64 {
    let unsigned = i64::from(floats_raw) as u64;
    unsigned.saturating_mul(4)
}
