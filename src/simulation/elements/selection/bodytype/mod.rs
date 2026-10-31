mod outcome;
mod scan;
use super::*;
pub use outcome::*;

/// Three explicitly supplied runtime inputs, never a serialized descriptor grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationBodyTypeInputs {
    /// Full caller body-ID bits, compared with zero-extended stored keys.
    pub body_id: SimulationBodySelectionId,
    /// Signed zero-based start, with negative values taking the native guard.
    pub start: i32,
    /// Signed exclusive upper endpoint, never silently converted to inclusive.
    pub exclusive_upper: i32,
}

impl SimulationBodyQueryMesh {
    /// Project the exclusive body-type query, e.g. `mesh.project_body_type(inputs, 3, Some(-1))?`; no callback or descriptor deserialization executes.
    pub fn project_body_type(
        &self,
        inputs: SimulationBodyTypeInputs,
        mode: i32,
        callback: Option<i16>,
    ) -> Result<SimulationBodyTypeResult> {
        if inputs.start < 0 || inputs.exclusive_upper < 0 {
            return Ok(outcome::early_result(
                self,
                inputs,
                mode,
                None,
                -1,
                SimulationBodyTypeCompletion::GuardedNegativeBounds,
            ));
        }
        let callback = callback.ok_or_else(|| invalid("body-type callback",format!("bounds {}, {}, mode {mode}, callback None; expected a supplied signed-i16 result after nonnegative bounds",inputs.start,inputs.exclusive_upper)))?;
        dispatch_type(self, inputs, mode, callback)
    }
}

fn dispatch_type(
    mesh: &SimulationBodyQueryMesh,
    inputs: SimulationBodyTypeInputs,
    mode: i32,
    callback: i16,
) -> Result<SimulationBodyTypeResult> {
    if callback != -1 {
        return Ok(callback_override(mesh, inputs, mode, callback));
    }
    if mode != 3 {
        return Ok(fixed_result(mesh, inputs, mode));
    }
    scan::scan_type(mesh, inputs)
}

fn callback_override(
    mesh: &SimulationBodyQueryMesh,
    inputs: SimulationBodyTypeInputs,
    mode: i32,
    callback: i16,
) -> SimulationBodyTypeResult {
    let completion = SimulationBodyTypeCompletion::CompletedCallbackOverride;
    outcome::early_result(mesh, inputs, mode, Some(callback), 16, completion)
}

fn fixed_result(
    mesh: &SimulationBodyQueryMesh,
    inputs: SimulationBodyTypeInputs,
    mode: i32,
) -> SimulationBodyTypeResult {
    let flag = fixed_type_flag(mode);
    let completion = SimulationBodyTypeCompletion::CompletedFixedMode;
    outcome::early_result(mesh, inputs, mode, Some(-1), flag, completion)
}

fn fixed_type_flag(mode: i32) -> i32 {
    match mode {
        0 => 1,
        1 | 2 => 2,
        4 => 4,
        6 => 8,
        _ => -1,
    }
}
