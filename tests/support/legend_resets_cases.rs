use super::load_projection::read_word;
use bessemer::{FailureClass, simulation::*};

pub fn reset_result(version: SimulationPersistVersion) -> [u32; 10] {
    let plan = match version.legend_read_zero_assignments() {
        Ok(plan) => plan,
        Err(error) => {
            assert_eq!(error.class, FailureClass::Unsupported);
            assert_eq!(error.operation, "legend read-time zero assignments");
            assert_eq!(error.entity, "explicit-field reader version");
            assert!(error.cause.contains(&version.value().to_string()));
            return [10, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        }
    };
    assert_reset_coverage(version, &plan);
    reset_plan_words(&plan)
}

fn assert_reset_coverage(version: SimulationPersistVersion, plan: &SimulationLegendResetPlan) {
    assert_eq!(plan.history().version, version);
    assert_eq!(plan.history().relation, SimulationThermalRelation::Derived);
    let coverage = plan.coverage();
    assert!(coverage.version_inspected && coverage.exact_version_policy);
    assert_eq!(coverage.stored_words_read_for_reset_policy, 0);
    assert_eq!(coverage.additional_wire_bytes, 0);
    assert_eq!(coverage.achieved_pattern_error_bits, 0);
    assert!(!coverage.wire_bytes_modified && !coverage.full_reader_emulated);
    assert!(!coverage.constructor_state_inferred && !coverage.native_object_pointer_validated);
    assert_eq!(coverage.runtime_words_assigned, plan.assignments().len());
    assert_eq!(coverage.native_store_instructions, plan.assignments().len());
    assert_eq!(coverage.runtime_store_bytes, plan.assignments().len() * 4);
    assert_reset_diagnostics(plan);
}

fn assert_reset_diagnostics(plan: &SimulationLegendResetPlan) {
    use SimulationLegendResetDiagnostic::*;
    assert_eq!(
        plan.diagnostics(),
        [
            NativeNotExecuted,
            ExplicitReaderPolicyOnly,
            DerivedAssignmentsHaveNoWireOffsets,
            OtherSkippedFieldsRemainUnknown,
            ConstructorAndPriorStateNotInferred
        ]
    );
}

fn reset_plan_words(plan: &SimulationLegendResetPlan) -> [u32; 10] {
    let completion = match plan.completion() {
        SimulationLegendResetCompletion::CompletedNoAssignments => 1,
        SimulationLegendResetCompletion::CompletedZeroAssignments => 2,
    };
    let mut words = empty_reset_words(plan, completion);
    for (index, assignment) in plan.assignments().iter().enumerate() {
        words[2 + index * 2] = u32::from(assignment.member.member_offset());
        words[3 + index * 2] = assignment.value_raw as u32;
    }
    words
}

fn empty_reset_words(plan: &SimulationLegendResetPlan, completion: u32) -> [u32; 10] {
    [
        completion,
        plan.assignments().len() as u32,
        u32::MAX,
        0,
        u32::MAX,
        0,
        u32::MAX,
        0,
        completion,
        plan.coverage().runtime_store_bytes as u32,
    ]
}

pub fn assert_reset_case(cursor: &mut &[u8]) -> [u32; 10] {
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let actual = reset_result(version);
    let expected: [u32; 10] = std::array::from_fn(|_| read_word(cursor));
    assert_eq!(actual, expected, "version {}", version.value());
    actual
}

pub fn replay_reset_cases(mut cursor: &[u8]) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), 4137);
    let mut words = Vec::new();
    for _ in 0..4137 {
        words.extend(assert_reset_case(&mut cursor));
    }
    assert!(cursor.is_empty());
    words
}
