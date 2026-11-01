use super::load_projection::read_word;
use bessemer::{FailureClass, simulation::*};

pub fn read_width_runtime(cursor: &mut &[u8]) -> SimulationLegendWidthRuntime {
    let mask = read_word(cursor);
    let font = read_word(cursor) as i32;
    let selector = read_word(cursor) as i32;
    assert!(mask < 4);
    SimulationLegendWidthRuntime {
        font_size_at_entry_raw: (mask & 1 != 0).then_some(font),
        selector_at_entry_raw: (mask & 2 != 0).then_some(selector),
    }
}

fn assert_width_plan(runtime: SimulationLegendWidthRuntime, plan: &SimulationLegendWidthPlan) {
    assert_eq!(plan.history().runtime, runtime);
    assert_eq!(plan.history().relation, SimulationThermalRelation::Derived);
    let coverage = plan.coverage();
    assert_eq!(coverage.runtime_words_read, 2);
    assert_eq!(coverage.stored_snapshot_words_read, 0);
    assert_eq!(coverage.achieved_pattern_error_bits, 0);
    assert!(coverage.exact_nearest_even_patterns);
    assert!(!coverage.ambient_sse_state_validated && !coverage.pointer_geometry_renderer_validated);
    assert_eq!(
        coverage.addition_reached,
        plan.values().operation == SimulationLegendWidthOperation::AddInitialToItself
    );
    assert_width_diagnostics(plan);
}

fn assert_width_diagnostics(plan: &SimulationLegendWidthPlan) {
    use SimulationLegendWidthDiagnostic::*;
    assert_eq!(
        plan.diagnostics(),
        [
            NativeNotExecuted,
            ReadTimeFactsExplicit,
            NearestEvenPolicyExplicit,
            PointerGeometryNotValidated,
            RendererAndUnitsNotEstablished
        ]
    );
}

fn width_error_words(error: bessemer::BessemerError) -> [u32; 8] {
    assert_eq!(error.operation, "legend eDrawings width projection");
    assert_eq!(error.class, FailureClass::Missing);
    assert!(
        error
            .cause
            .contains("None; expected Some(explicit signed32")
    );
    let status = match error.entity.as_str() {
        "read-time font68" => 10,
        "read-time legend B4" => 11,
        unexpected => panic!("{unexpected:?}; expected exact missing width read entity"),
    };
    [status, 0, 0, 0, 0, 0, 0, 0]
}

pub fn width_result_words(runtime: SimulationLegendWidthRuntime) -> [u32; 8] {
    let plan = match runtime.project_width_nearest_even() {
        Ok(plan) => plan,
        Err(error) => return width_error_words(error),
    };
    assert_width_plan(runtime, &plan);
    width_plan_words(&plan)
}

fn width_plan_words(plan: &SimulationLegendWidthPlan) -> [u32; 8] {
    let values = plan.values();
    let completion = match plan.completion() {
        SimulationLegendWidthCompletion::CompletedZeroWidth => 1,
        SimulationLegendWidthCompletion::CompletedNonzeroWidth => 2,
    };
    let operation = match values.operation {
        SimulationLegendWidthOperation::KeepInitial => 0,
        SimulationLegendWidthOperation::AddInitialToItself => 1,
        SimulationLegendWidthOperation::MultiplyByFour => 2,
    };
    [
        completion,
        values.converted_font_bits,
        values.scaled_font_bits,
        values.initial_width_bits,
        values.selected_width_bits,
        operation,
        completion,
        plan.coverage().rounded_multiplications as u32,
    ]
}

pub fn assert_width_case(cursor: &mut &[u8]) -> [u32; 8] {
    let runtime = read_width_runtime(cursor);
    let expected: [u32; 8] = std::array::from_fn(|_| read_word(cursor));
    let actual = width_result_words(runtime);
    assert_eq!(actual, expected, "{runtime:?}");
    actual
}

pub fn replay_width_operations(mut cursor: &[u8]) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), 18040);
    let mut words = Vec::new();
    for _ in 0..18040 {
        words.extend(assert_width_case(&mut cursor));
    }
    assert!(cursor.is_empty());
    words
}
