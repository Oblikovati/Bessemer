use super::{
    SimulationTriangleColorCaller as Caller, SimulationTriangleColorControls as Controls,
    SimulationTriangleColorRuntime, SimulationTriangleColorStage as Stage,
};
use crate::{BessemerError, FailureClass, Result};

pub(super) fn triangle_color_stage(
    controls: Controls,
    runtime: SimulationTriangleColorRuntime,
) -> Result<Stage> {
    if let Some(stage) = triangle_caller_stage(controls, runtime.caller)? {
        return Ok(stage);
    }
    if !runtime.core_object_present {
        return Ok(Stage::HelperCoreMissing);
    }
    if runtime.group_count_raw == 0 {
        return Ok(Stage::HelperCountZero);
    }
    if !runtime.group_array_present {
        return Ok(Stage::HelperArrayMissing);
    }
    Ok(Stage::HelperEntered)
}

fn triangle_caller_stage(controls: Controls, caller: Caller) -> Result<Option<Stage>> {
    match caller {
        Caller::HiddenParts {
            option_result,
            core_678_raw,
        } => hidden_controls_stage(controls, caller, option_result, core_678_raw),
        Caller::ModelTessellation {
            renderer_type_code,
            precheck_result,
        } => model_controls_stage(controls, caller, renderer_type_code, precheck_result),
    }
}

fn hidden_controls_stage(
    controls: Controls,
    caller: Caller,
    option: Option<i32>,
    core: Option<i32>,
) -> Result<Option<Stage>> {
    match controls {
        Controls::HiddenParts { selector_raw, .. } => {
            hidden_caller_stage(selector_raw, option, core)
        }
        _ => Err(triangle_caller_mismatch(controls, caller)),
    }
}

fn model_controls_stage(
    controls: Controls,
    caller: Caller,
    renderer: i32,
    precheck: Option<i32>,
) -> Result<Option<Stage>> {
    match controls {
        Controls::ModelTessellation { override_raw, .. } => {
            model_caller_stage(override_raw, renderer, precheck)
        }
        _ => Err(triangle_caller_mismatch(controls, caller)),
    }
}

fn triangle_caller_mismatch(controls: Controls, caller: Caller) -> BessemerError {
    BessemerError::new(
        "triangle color call plan",
        "caller kind",
        FailureClass::Invalid,
        format!(
            "controls {controls:?}, caller {caller:?}; expected matching hidden-parts or model-tessellation variants"
        ),
    )
}

fn hidden_caller_stage(
    selector: i32,
    option: Option<i32>,
    core_678: Option<i32>,
) -> Result<Option<Stage>> {
    if required_caller_word(option, "HiddenPartsOptionOn result")? == 0 {
        return Ok(Some(Stage::HiddenDisabled));
    }
    if selector as u32 > 1 {
        return Ok(Some(Stage::HiddenSelectorBypassed));
    }
    if required_caller_word(core_678, "core678")? != 0 {
        return Ok(Some(Stage::HiddenOtherPath));
    }
    Ok(None)
}

fn model_caller_stage(
    override_raw: i32,
    renderer: i32,
    precheck: Option<i32>,
) -> Result<Option<Stage>> {
    if renderer == 5 {
        return Ok(Some(Stage::ModelRendererExcluded));
    }
    if required_caller_word(precheck, "1800FF240 result")? != 0 {
        return Ok(Some(Stage::ModelPrecheckRejected));
    }
    if override_raw == 0 {
        return Ok(Some(Stage::ModelOtherPath));
    }
    Ok(None)
}

fn required_caller_word(value: Option<i32>, entity: &str) -> Result<i32> {
    value.ok_or_else(|| BessemerError::new("triangle color call plan", entity, FailureClass::Missing,
        "runtime value None; expected an explicit known signed32 result for the reached caller branch"))
}
