use super::{SimulationUnitTypeInputs, modifier};
use crate::Result;

pub(super) fn analysis_code(
    inputs: SimulationUnitTypeInputs,
    component: u32,
    fallback: i32,
) -> Result<i32> {
    match inputs.analysis {
        0 => Ok(analysis_zero(inputs.plot_type, component, fallback)),
        1 | 2 => modifier::frequency_code(inputs, component, fallback),
        3 => Ok(analysis_three(inputs.plot_type, component, fallback)),
        5 | 6 => Ok(analysis_five_six(inputs.plot_type, component, fallback)),
        7 => Ok(if inputs.plot_type == 12 && component == 4 {
            37
        } else {
            9999
        }),
        8 => modifier::analysis_eight(inputs, component, fallback),
        10 => Ok(analysis_ten(inputs.plot_type, component, fallback)),
        _ => Ok(fallback),
    }
}

fn displacement_code(component: u32, extra: u32, fallback: i32) -> i32 {
    if component == extra {
        return 9;
    }
    match component {
        0..=3 => 0,
        4..=7 => 2,
        8..=10 => 9,
        11..=14 => 3,
        _ => fallback,
    }
}

fn analysis_zero(plot: i32, component: u32, fallback: i32) -> i32 {
    match plot {
        1 => displacement_code(component, 15, fallback),
        2 | 57 | 59 => {
            if component == 12 {
                9999
            } else {
                4
            }
        }
        3 | 22 => shear_code(component),
        4 => 4,
        _ => fallback,
    }
}

fn analysis_five_six(plot: i32, component: u32, fallback: i32) -> i32 {
    match plot {
        1 => displacement_code(component, 29, fallback),
        2 | 4 | 59 => 4,
        19 => {
            if (4..=6).contains(&component) {
                27
            } else {
                14
            }
        }
        20 => {
            if (4..=6).contains(&component) {
                29
            } else {
                18
            }
        }
        3 => shear_code(component),
        _ => fallback,
    }
}

pub(super) fn shear_code(component: u32) -> i32 {
    match component {
        8 => 19,
        7 => 20,
        _ => 9999,
    }
}

fn analysis_three(plot: i32, component: u32, fallback: i32) -> i32 {
    if plot != 5 {
        return fallback;
    }
    match component {
        0 => 5,
        1..=4 => 6,
        5..=8 => 7,
        9 => 35,
        10 => 36,
        _ => fallback,
    }
}

fn analysis_ten(plot: i32, component: u32, fallback: i32) -> i32 {
    if plot == 7 {
        return if component == 4 { 9999 } else { 4 };
    }
    match plot {
        6 => 14,
        8 => plot_eight(component, fallback),
        9 => plot_nine(component, fallback),
        10 => plot_ten(component, fallback),
        11 => plot_eleven(component, fallback),
        _ => fallback,
    }
}

fn plot_eight(component: u32, fallback: i32) -> i32 {
    match component {
        0..=1 => 11,
        3 => 12,
        4 => 17,
        5 => 0,
        6 => 4,
        _ => fallback,
    }
}

fn plot_nine(component: u32, fallback: i32) -> i32 {
    match component {
        0 => 5,
        2..=5 => 6,
        6 => 11,
        _ => fallback,
    }
}

fn plot_ten(component: u32, fallback: i32) -> i32 {
    match component {
        0..=3 => 2,
        4..=6 => 4,
        _ => fallback,
    }
}

fn plot_eleven(component: u32, fallback: i32) -> i32 {
    match component {
        1 => 10,
        2 => 13,
        4 => 16,
        5 => 7,
        _ => fallback,
    }
}
