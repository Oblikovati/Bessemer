use super::load_projection::read_word;
use super::{assert_3d_coverage, assert_background_diagnostics, assert_body_diagnostics};
use bessemer::{FailureClass, simulation::*};

#[derive(Debug, Clone, Copy)]
pub enum Fixture3DCase {
    Body(SimulationLegendHeightInputs, SimulationLegend3DBodyRuntime),
    Background(SimulationLegend3DBackgroundRuntime),
}

pub fn read_3d_case(cursor: &mut &[u8]) -> Fixture3DCase {
    let words: [u32; 5] = std::array::from_fn(|_| read_word(cursor));
    assert!(words[1] < 8);
    match words[0] {
        0 => Fixture3DCase::Body(
            SimulationLegendHeightInputs {
                cell_count_raw: (words[1] & 1 != 0).then_some(words[2] as i32),
            },
            SimulationLegend3DBodyRuntime {
                cell_count_at_mapping_raw: (words[1] & 2 != 0).then_some(words[3] as i32),
                rendering_argument_raw: (words[1] & 4 != 0).then_some(words[4] as i32),
            },
        ),
        1 => Fixture3DCase::Background(background_case_words(words)),
        other => panic!("unexpected3D fixture kind {other}; expected0/1"),
    }
}

fn background_case_words(words: [u32; 5]) -> SimulationLegend3DBackgroundRuntime {
    assert!(words[4] <= 255);
    SimulationLegend3DBackgroundRuntime {
        packed_color_raw: (words[1] & 1 != 0).then_some(words[2]),
        precision_at_width_raw: (words[1] & 2 != 0).then_some(words[3] as i32),
        lighting_result_raw: (words[1] & 4 != 0).then_some(words[4] as u8),
    }
}

fn error_3d_words(error: bessemer::BessemerError) -> [u32; 12] {
    assert_eq!(error.operation, "legend 3D local projection");
    assert_eq!(error.class, FailureClass::Missing);
    assert!(error.cause.contains("None") && error.cause.contains("expected Some"));
    let status = match error.entity.as_str() {
        "allocation-entry count14" => 10,
        "post-vertex count14" => 11,
        "rendering argument" => 12,
        "packed background argument" => 20,
        "precision BC at width read" => 21,
        "lighting GLboolean result" => 22,
        other => panic!("unexpected {other}: {error:?}"),
    };
    let mut words = [0; 12];
    words[0] = status;
    words
}

fn body_3d_words(plan: &SimulationLegend3DBodyPlan) -> [u32; 12] {
    let status = match plan.completion() {
        SimulationLegend3DCompletion::CompletedZeroMappingRequest => 1,
        SimulationLegend3DCompletion::CompletedNonzeroMappingRequest => 2,
        other => panic!("unexpected body completion {other:?}"),
    };
    let values = plan.values();
    [
        status,
        values.allocation_pairs_raw as u32,
        values.vertex_float_count_raw as u32,
        values.vertex_allocation_bytes_raw as u32,
        (values.vertex_allocation_bytes_raw >> 32) as u32,
        values.mapping_request_count_raw as u32,
        values.color_float_count_raw as u32,
        values.color_allocation_bytes_raw as u32,
        (values.color_allocation_bytes_raw >> 32) as u32,
        values.initial_shade_model_raw,
        values.begin_primitive_raw,
        values.later_smooth_argument_raw,
    ]
}

fn background_3d_words(plan: &SimulationLegend3DBackgroundPlan) -> [u32; 12] {
    assert_eq!(
        plan.completion(),
        SimulationLegend3DCompletion::CompletedBackgroundInputs
    );
    let values = plan.values();
    let bits = values.constructed_color.bits();
    [
        3,
        bits[0],
        bits[1],
        bits[2],
        bits[3],
        values.precision_factor_raw as u32,
        values.begin_primitive_raw,
        values.lighting_capability_raw,
        values.restore_lighting as u32,
        0,
        0,
        0,
    ]
}

fn body_3d_result(
    inputs: SimulationLegendHeightInputs,
    runtime: SimulationLegend3DBodyRuntime,
) -> [u32; 12] {
    match inputs.project_3d_body_requests(runtime) {
        Ok(plan) => {
            assert_eq!(plan.history().inputs, inputs);
            assert_eq!(plan.history().runtime, runtime);
            assert_eq!(plan.history().relation, SimulationThermalRelation::Derived);
            assert_3d_coverage(plan.coverage(), true);
            assert_body_diagnostics(&plan);
            body_3d_words(&plan)
        }
        Err(error) => error_3d_words(error),
    }
}

fn background_3d_result(runtime: SimulationLegend3DBackgroundRuntime) -> [u32; 12] {
    match runtime.project_background_nearest_even() {
        Ok(plan) => {
            assert_eq!(plan.history().runtime, runtime);
            assert_eq!(plan.history().relation, SimulationThermalRelation::Derived);
            assert_3d_coverage(plan.coverage(), false);
            assert_background_diagnostics(&plan);
            background_3d_words(&plan)
        }
        Err(error) => error_3d_words(error),
    }
}

pub fn assert_3d_case(cursor: &mut &[u8]) -> [u32; 12] {
    let case = read_3d_case(cursor);
    let expected: [u32; 12] = std::array::from_fn(|_| read_word(cursor));
    let actual = match case {
        Fixture3DCase::Body(inputs, runtime) => body_3d_result(inputs, runtime),
        Fixture3DCase::Background(runtime) => background_3d_result(runtime),
    };
    assert_eq!(actual, expected, "{case:?}");
    actual
}

pub fn replay_3d_operations(mut cursor: &[u8]) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    let count = read_word(&mut cursor);
    assert_eq!(count, 26784);
    let mut words = Vec::new();
    for _ in 0..count {
        words.extend(assert_3d_case(&mut cursor));
    }
    assert!(cursor.is_empty());
    words
}
