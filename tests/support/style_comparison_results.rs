use super::load_projection::read_word;
use super::state_support::{read_style_state, style_input_path};
use bessemer::{FailureClass, simulation::*};

pub fn check_code(check: SimulationStyleComparisonCheck) -> u32 {
    match check {
        SimulationStyleComparisonCheck::Word(member) => member as u32,
        SimulationStyleComparisonCheck::Scalar(SimulationStyleComparisonScalar::Alpha22C) => 11,
        SimulationStyleComparisonCheck::Scalar(SimulationStyleComparisonScalar::Alpha2B8) => 12,
        SimulationStyleComparisonCheck::Color(color) => {
            assert_ne!(color, SimulationStyleComparisonColor::Color2AC);
            13 + color as u32
        }
        SimulationStyleComparisonCheck::Legend => 19,
    }
}

fn assert_style_plan(
    plan: &SimulationStyleComparisonPlan,
    left: &SimulationStyleComparisonState,
    right: &SimulationStyleComparisonState,
) {
    assert_eq!(&plan.history().left, left);
    assert_eq!(&plan.history().right, right);
    assert_eq!(plan.history().relation, SimulationThermalRelation::Derived);
    let coverage = plan.coverage();
    assert_eq!(
        coverage.check_mode,
        SimulationStyleComparisonCheckMode::ExactDependenciesMaskedExceptionsDazOff
    );
    assert_eq!(coverage.compared_entities, plan.checks().len());
    assert!(coverage.exact_dependency_order);
    assert_eq!(coverage.achieved_pattern_error_bits, 0);
    assert!(!coverage.unread_inputs_validated && !coverage.full_object_validated);
    assert!(!coverage.wire_bytes_modified && !coverage.native_object_pointer_validated);
    assert_style_diagnostics(plan);
}

fn assert_style_diagnostics(plan: &SimulationStyleComparisonPlan) {
    use SimulationStyleComparisonDiagnostic::*;
    assert_eq!(
        plan.diagnostics(),
        [
            NativeNotExecuted,
            RuntimeFactsRequired,
            WireInputsNotMigrated,
            NotUniversalRenderEquivalence
        ]
    );
}

fn style_plan_words(plan: &SimulationStyleComparisonPlan) -> [u32; 25] {
    let mut words = [u32::MAX; 25];
    words[0] = 0;
    if let SimulationStyleComparisonOutcome::CompletedDifferent(first) = plan.outcome() {
        words[0] = 1;
        words[1] = check_code(first);
        assert_eq!(plan.checks().last(), Some(&first));
    }
    assert!(plan.checks().len() <= 20);
    words[4] = plan.checks().len() as u32;
    for (index, check) in plan.checks().iter().enumerate() {
        words[5 + index] = check_code(*check);
    }
    words
}

fn error_identity(entity: &str) -> (u32, u32) {
    if entity.starts_with("legend.") {
        return (39, 2);
    }
    let (side, path) = if let Some(path) = entity.strip_prefix("left.") {
        (0, path)
    } else {
        (1, entity.strip_prefix("right.").unwrap())
    };
    let index = (0..39)
        .find(|index| style_input_path(*index) == path)
        .unwrap();
    (index as u32, side)
}

fn style_error_words(error: bessemer::BessemerError) -> [u32; 25] {
    assert_eq!(error.operation, "conditional PostStyle comparison");
    assert!(error.cause.contains("expected"));
    let status = match error.class {
        FailureClass::Missing => 2,
        FailureClass::Invalid => 3,
        unexpected => panic!(
            "{unexpected:?}; expected missing required fact or invalid delegated palette count"
        ),
    };
    let (member, side) = error_identity(&error.entity);
    let mut words = [u32::MAX; 25];
    words[0] = status;
    words[2] = member;
    words[3] = side;
    words[4] = 0;
    words
}

pub fn assert_style_pair(
    left: &SimulationStyleComparisonState,
    right: &SimulationStyleComparisonState,
    cursor: &mut &[u8],
) -> [u32; 25] {
    let original_left = left.clone();
    let original_right = right.clone();
    let actual = match left.compare_masked_exceptions_daz_off(right) {
        Ok(plan) => {
            assert_style_plan(&plan, left, right);
            style_plan_words(&plan)
        }
        Err(error) => style_error_words(error),
    };
    assert_eq!(left, &original_left);
    assert_eq!(right, &original_right);
    let expected: [u32; 25] = std::array::from_fn(|_| read_word(cursor));
    assert_eq!(actual, expected, "left={left:?}, right={right:?}");
    actual
}

pub fn replay_style_cases(mut cursor: &[u8], count: u32) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), count);
    let mut words = Vec::new();
    for _ in 0..count {
        let left = read_style_state(&mut cursor);
        let right = read_style_state(&mut cursor);
        words.extend(assert_style_pair(&left, &right, &mut cursor));
    }
    assert!(cursor.is_empty());
    words
}
