use super::load_projection::read_word;
use bessemer::{FailureClass, simulation::*};

pub fn read_inverse_source(cursor: &mut &[u8]) -> SimulationLegendPreferenceSource {
    let mode = read_word(cursor);
    let kind = read_word(cursor);
    let plot = read_word(cursor) as i32;
    let present = read_word(cursor);
    let word = read_word(cursor) as i32;
    let original = inverse_optional_word(present, word);
    if mode == 0 {
        assert_eq!((kind, plot), (0, 0));
        return SimulationLegendPreferenceSource::StoredA8(SimulationLegendPreferenceInputs {
            stored_a8_raw: original,
        });
    }
    assert_eq!(mode, 1);
    let plot_context = decode_inverse_plot(kind, plot);
    SimulationLegendPreferenceSource::RuntimePreference(SimulationLegendPreferenceRuntime {
        plot_context,
        preference_word_1528_raw: original,
    })
}

fn inverse_optional_word(present: u32, word: i32) -> Option<i32> {
    assert!(present <= 1);
    if present == 0 {
        assert_eq!(word, 0);
    }
    (present == 1).then_some(word)
}

fn decode_inverse_plot(kind: u32, plot: i32) -> Option<SimulationLegendPreferencePlot> {
    if kind != 2 {
        assert_eq!(plot, 0);
    }
    match kind {
        0 => None,
        1 => Some(SimulationLegendPreferencePlot::NullPlot),
        2 => Some(SimulationLegendPreferencePlot::PresentType(plot)),
        _ => panic!("{kind}; expected unknown0/null1/present2 plot context"),
    }
}

pub fn inverse_source_words(source: SimulationLegendPreferenceSource) -> [u32; 5] {
    match source {
        SimulationLegendPreferenceSource::StoredA8(inputs) => [
            0,
            0,
            0,
            u32::from(inputs.stored_a8_raw.is_some()),
            inputs.stored_a8_raw.unwrap_or(0) as u32,
        ],
        SimulationLegendPreferenceSource::RuntimePreference(runtime) => {
            inverse_runtime_words(runtime)
        }
    }
}

fn inverse_runtime_words(runtime: SimulationLegendPreferenceRuntime) -> [u32; 5] {
    let (kind, plot) = match runtime.plot_context {
        None => (0, 0),
        Some(SimulationLegendPreferencePlot::NullPlot) => (1, 0),
        Some(SimulationLegendPreferencePlot::PresentType(word)) => (2, word as u32),
    };
    [
        1,
        kind,
        plot,
        u32::from(runtime.preference_word_1528_raw.is_some()),
        runtime.preference_word_1528_raw.unwrap_or(0) as u32,
    ]
}

pub fn project_inverse_source(
    source: SimulationLegendPreferenceSource,
) -> bessemer::Result<SimulationLegendPreferencePlan> {
    match source {
        SimulationLegendPreferenceSource::StoredA8(inputs) => inputs.project_preference_word(),
        SimulationLegendPreferenceSource::RuntimePreference(runtime) => {
            runtime.project_local_a8_assignment()
        }
    }
}

fn inverse_error_words(error: bessemer::BessemerError) -> [u32; 10] {
    assert_eq!(error.class, FailureClass::Missing);
    assert!(error.cause.contains("None; expected Some("));
    let status = match (error.operation, error.entity.as_str()) {
        ("legend preference zero-test export", "actual legend A8") => 10,
        ("legend local preference assignment", "actual plot context") => 11,
        ("legend local preference assignment", "actual preference1528") => 12,
        unexpected => panic!("{unexpected:?}; expected a typed missing local conversion fact"),
    };
    [status, 0, 0, 0, 0, 0, 0, 0, 0, 0]
}

pub fn inverse_plan_words(plan: &SimulationLegendPreferencePlan) -> [u32; 10] {
    let (status, effect, word) = inverse_effect_words(plan.effect());
    let coverage = plan.coverage();
    let completion = match plan.completion() {
        SimulationLegendPreferenceCompletion::CompletedWord => 1,
        SimulationLegendPreferenceCompletion::CompletedNoAssignment => 2,
    };
    [
        status,
        inverse_route_code(plan.route()),
        effect,
        word,
        u32::from(coverage.source_word_read),
        u32::from(coverage.plot_context_inspected),
        u32::from(coverage.plot_type_word_read),
        coverage.runtime_words_assigned as u32,
        coverage.runtime_store_bytes as u32,
        completion,
    ]
}

fn inverse_effect_words(effect: SimulationLegendPreferenceEffect) -> (u32, u32, u32) {
    match effect {
        SimulationLegendPreferenceEffect::PreferenceWord0(word) => (1, 0, word as u32),
        SimulationLegendPreferenceEffect::LegendWordA8(word) => (1, 1, word as u32),
        SimulationLegendPreferenceEffect::NoAssignment => (2, 2, 0),
    }
}

fn inverse_route_code(route: SimulationLegendPreferenceRoute) -> u32 {
    match route {
        SimulationLegendPreferenceRoute::ExportA8 => 0,
        SimulationLegendPreferenceRoute::ApplyOrdinary => 1,
        SimulationLegendPreferenceRoute::ApplyType70 => 2,
        SimulationLegendPreferenceRoute::SkipPlotType => 3,
    }
}

fn assert_inverse_coverage(plan: &SimulationLegendPreferencePlan) {
    let coverage = plan.coverage();
    assert_eq!(
        coverage.native_store_instructions,
        coverage.runtime_words_assigned
    );
    assert_eq!(coverage.additional_wire_bytes, 0);
    assert_eq!(coverage.achieved_pattern_error_bits, 0);
    assert!(coverage.exact_integer_policy);
    assert!(!coverage.wire_bytes_modified && !coverage.whole_methods_emulated);
    assert!(!coverage.native_object_pointer_validated);
    assert_eq!(plan.history().relation, SimulationThermalRelation::Derived);
    assert_inverse_diagnostics(plan);
}

fn assert_inverse_diagnostics(plan: &SimulationLegendPreferencePlan) {
    use SimulationLegendPreferenceDiagnostic::*;
    assert_eq!(
        plan.diagnostics(),
        [
            NativeNotExecuted,
            LocalZeroTestOnly,
            OriginalWordsRetained,
            WholePreferenceMethodsNotEmulated,
            NativePointerStateNotValidated
        ]
    );
}

pub fn assert_inverse_case(cursor: &mut &[u8]) -> [u32; 15] {
    let source = read_inverse_source(cursor);
    let (history, actual) = match project_inverse_source(source) {
        Ok(plan) => {
            assert_eq!(plan.history().source, source);
            assert_inverse_coverage(&plan);
            (plan.history().source, inverse_plan_words(&plan))
        }
        Err(error) => (source, inverse_error_words(error)),
    };
    let expected: [u32; 10] = std::array::from_fn(|_| read_word(cursor));
    assert_eq!(actual, expected, "{source:?}");
    let mut output = [0; 15];
    output[..5].copy_from_slice(&inverse_source_words(history));
    output[5..].copy_from_slice(&actual);
    output
}

pub fn replay_inverse_cases(mut cursor: &[u8]) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), 12678);
    let mut words = Vec::new();
    for _ in 0..12678 {
        words.extend(assert_inverse_case(&mut cursor));
    }
    assert!(cursor.is_empty());
    words
}
