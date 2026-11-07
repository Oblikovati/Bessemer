use super::{SimulationUnitTypeInputs, families};
use crate::{BessemerError, FailureClass, Result};

pub(super) fn frequency_code(
    inputs: SimulationUnitTypeInputs,
    component: u32,
    fallback: i32,
) -> Result<i32> {
    if inputs.plot_type != 1 {
        return Ok(fallback);
    }
    let code = match component {
        0..=3 => 0,
        4..=6 if inputs.analysis == 2 => 9,
        _ => return Ok(fallback),
    };
    match inputs.actual_freq_disp_flag {
        Some(1) => Ok(code),
        Some(_) => Ok(9999),
        None => Err(BessemerError::new(
            "component unit type",
            "ActualFreqDispFlag",
            FailureClass::Missing,
            "None; expected an explicit loaded signed32 ActualFreqDispFlag at provider global57B68",
        )),
    }
}

fn modified(inputs: SimulationUnitTypeInputs) -> Result<bool> {
    if inputs.word_be8.is_some_and(|word| word != 2) || inputs.word_bec == Some(0) {
        return Ok(false);
    }
    if inputs.word_be8.is_some() && inputs.word_bec.is_some() {
        return Ok(true);
    }
    Err(BessemerError::new(
        "component unit type",
        "loaded BE8/BEC",
        FailureClass::Missing,
        format!(
            "BE8 {:?}, BEC {:?}; expected explicit loaded words unless BE8 differs from2 or BEC equals0",
            inputs.word_be8, inputs.word_bec
        ),
    ))
}

pub(super) fn analysis_eight(
    inputs: SimulationUnitTypeInputs,
    component: u32,
    fallback: i32,
) -> Result<i32> {
    match inputs.plot_type {
        1 => displacement_eight(inputs, component, fallback),
        2 => Ok(if modified(inputs)? { 21 } else { 4 }),
        19 | 20 => stress_eight(inputs, component),
        3 => Ok(families::shear_code(component)),
        _ => Ok(fallback),
    }
}

fn stress_eight(inputs: SimulationUnitTypeInputs, component: u32) -> Result<i32> {
    let interval = (4..=6).contains(&component);
    let pair = match (inputs.plot_type, interval) {
        (19, true) => (27, 28),
        (19, false) => (14, 23),
        (_, true) => (29, 30),
        (_, false) => (18, 24),
    };
    Ok(if modified(inputs)? { pair.1 } else { pair.0 })
}

fn displacement_eight(
    inputs: SimulationUnitTypeInputs,
    component: u32,
    fallback: i32,
) -> Result<i32> {
    let pair = match component {
        0..=3 => (0, 22),
        4..=7 => (2, 26),
        8..=10 => return Ok(9),
        11..=14 => (3, 25),
        _ => return Ok(fallback),
    };
    Ok(if modified(inputs)? { pair.1 } else { pair.0 })
}
