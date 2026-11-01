use super::load_projection::read_word;
use bessemer::{FailureClass, simulation::*};

pub fn read_end_cell_runtime(cursor: &mut &[u8]) -> SimulationLegendEndCellRuntime {
    let mask = read_word(cursor);
    let plot = read_word(cursor) as i32;
    let flag = read_word(cursor) as i32;
    let color = SimulationRgbaColor::from_bits(std::array::from_fn(|_| read_word(cursor)));
    assert!(mask < 8);
    SimulationLegendEndCellRuntime {
        plot_word_524_raw: (mask & 1 != 0).then_some(plot),
        enabled_at_gate_raw: (mask & 2 != 0).then_some(flag),
        color_at_rgb_copy: (mask & 4 != 0).then_some(color),
    }
}

fn assert_end_cell_plan(
    runtime: SimulationLegendEndCellRuntime,
    plan: &SimulationLegendEndCellPlan,
) {
    assert_eq!(plan.history().runtime, runtime);
    assert_eq!(plan.history().relation, SimulationThermalRelation::Derived);
    assert_end_cell_coverage(plan);
    assert_end_cell_diagnostics(plan);
}

fn assert_end_cell_coverage(plan: &SimulationLegendEndCellPlan) {
    let coverage = plan.coverage();
    assert_eq!(coverage.stored_snapshot_words_read, 0);
    assert_eq!(
        coverage.rgb_words_copied,
        if plan.rgb_words().is_some() { 3 } else { 0 }
    );
    assert_eq!(coverage.achieved_pattern_error_bits, 0);
    assert!(coverage.exact_gate_and_rgb_bits);
    assert!(!coverage.runtime_pointer_validated && !coverage.geometry_renderer_modeled);
}

fn assert_end_cell_diagnostics(plan: &SimulationLegendEndCellPlan) {
    use SimulationLegendEndCellDiagnostic::*;
    assert_eq!(
        plan.diagnostics(),
        [
            NativeNotExecuted,
            ReadTimeFactsExplicit,
            StoredRuntimeIdentityNotInferred,
            AlphaNotConsumed,
            GeometryRendererNotModeled
        ]
    );
}

fn end_cell_error_words(error: bessemer::BessemerError) -> [u32; 6] {
    assert_eq!(error.operation, "legend end-cell projection");
    assert_eq!(error.class, FailureClass::Missing);
    assert!(error.cause.contains("None; expected Some(explicit"));
    let status = match error.entity.as_str() {
        "runtime plot524" => 10,
        "read-time legend E8" => 11,
        "read-time legend EC color" => 12,
        unexpected => panic!("{unexpected:?}; expected an exact reached end-cell dependency"),
    };
    [status, 0, 0, 0, 0, 0]
}

pub fn end_cell_result_words(runtime: SimulationLegendEndCellRuntime) -> [u32; 6] {
    let plan = match runtime.project_end_cell() {
        Ok(plan) => plan,
        Err(error) => return end_cell_error_words(error),
    };
    assert_end_cell_plan(runtime, &plan);
    let status = match plan.completion() {
        SimulationLegendEndCellCompletion::CompletedInactive => 1,
        SimulationLegendEndCellCompletion::CompletedRgbCopy => 2,
    };
    let [red, green, blue] = plan.rgb_words().unwrap_or([0; 3]);
    [
        status,
        u32::from(plan.coverage().reached_read_mask),
        red,
        green,
        blue,
        status,
    ]
}

pub fn assert_end_cell_case(cursor: &mut &[u8]) -> [u32; 6] {
    let runtime = read_end_cell_runtime(cursor);
    let expected: [u32; 6] = std::array::from_fn(|_| read_word(cursor));
    let actual = end_cell_result_words(runtime);
    assert_eq!(actual, expected, "{runtime:?}");
    actual
}

pub fn replay_end_cell_operations(mut cursor: &[u8]) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), 7232);
    let mut words = Vec::new();
    for _ in 0..7232 {
        words.extend(assert_end_cell_case(&mut cursor));
    }
    assert!(cursor.is_empty());
    words
}
