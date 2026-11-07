use super::load_projection::read_word;
use bessemer::{FailureClass, simulation::*};

pub fn read_hidden_inputs(cursor: &mut &[u8]) -> SimulationHiddenPartsMigrationInputs {
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let present = read_word(cursor);
    let word = read_word(cursor) as i32;
    assert!(present <= 1);
    SimulationHiddenPartsMigrationInputs {
        version,
        stored_word_1f0_raw: (present == 1).then_some(word),
    }
}

fn assert_hidden_plan(
    inputs: SimulationHiddenPartsMigrationInputs,
    plan: &SimulationHiddenPartsMigrationPlan,
) {
    assert_eq!(plan.history().inputs, inputs);
    assert_eq!(plan.history().relation, SimulationThermalRelation::Derived);
    let coverage = plan.coverage();
    assert_eq!(coverage.stored_words_read, 1);
    assert_eq!(coverage.additional_wire_bytes, 0);
    assert_eq!(coverage.achieved_pattern_error_bits, 0);
    assert!(coverage.exact_integer_policy);
    assert!(!coverage.wire_bytes_modified && !coverage.full_visibility_validated);
    assert!(!coverage.native_object_pointer_validated);
    use SimulationHiddenPartsMigrationDiagnostic::*;
    assert_eq!(
        plan.diagnostics(),
        [
            NativeNotExecuted,
            ExplicitFieldVersionOnly,
            DerivedPairNotStoredField,
            OriginalWordRetained,
            FullVisibilityNotModeled
        ]
    );
}

fn hidden_error_words(error: bessemer::BessemerError) -> [u32; 8] {
    assert_eq!(error.operation, "legacy hidden-parts migration");
    let status = match (error.class, error.entity.as_str()) {
        (FailureClass::Unsupported, "explicit-field migration version") => {
            assert!(error.cause.contains("expected version700006..840007"));
            10
        }
        (FailureClass::Missing, "stored style1F0") => {
            assert!(
                error
                    .cause
                    .contains("None; expected Some(original signed32")
            );
            11
        }
        unexpected => panic!("{unexpected:?}; expected unsupported scope or missing stored1F0"),
    };
    [status, 0, 0, 0, 0, 0, 0, 0]
}

fn hidden_plan_words(plan: &SimulationHiddenPartsMigrationPlan) -> [u32; 8] {
    let values = plan.values();
    let coverage = plan.coverage();
    let store = hidden_store_code(values.store);
    let completion = match plan.completion() {
        SimulationHiddenPartsMigrationCompletion::CompletedClearedPair => 1,
        SimulationHiddenPartsMigrationCompletion::CompletedOptionOne => 2,
    };
    [
        completion,
        values.hidden_option_raw as u32,
        values.runtime_word_1f0_raw as u32,
        store,
        coverage.runtime_words_assigned as u32,
        coverage.runtime_store_bytes as u32,
        coverage.native_store_instructions as u32,
        completion,
    ]
}

fn hidden_store_code(store: SimulationHiddenPartsStore) -> u32 {
    match store {
        SimulationHiddenPartsStore::ClearOptionWord => 0,
        SimulationHiddenPartsStore::WriteAdjacentPairOne => 1,
        SimulationHiddenPartsStore::WriteBothWordsOne => 2,
    }
}

pub fn hidden_result_words(inputs: SimulationHiddenPartsMigrationInputs) -> [u32; 8] {
    let plan = match inputs.project_legacy_hidden_parts() {
        Ok(plan) => plan,
        Err(error) => return hidden_error_words(error),
    };
    assert_hidden_plan(inputs, &plan);
    hidden_plan_words(&plan)
}

pub fn assert_hidden_case(cursor: &mut &[u8]) -> [u32; 8] {
    let inputs = read_hidden_inputs(cursor);
    let expected: [u32; 8] = std::array::from_fn(|_| read_word(cursor));
    let actual = hidden_result_words(inputs);
    assert_eq!(actual, expected, "{inputs:?}");
    actual
}

pub fn replay_hidden_operations(mut cursor: &[u8]) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), 4336);
    let mut words = Vec::new();
    for _ in 0..4336 {
        words.extend(assert_hidden_case(&mut cursor));
    }
    assert!(cursor.is_empty());
    words
}
