use super::*;
#[test]
fn every_frozen_numeric_request_matches_the_documented_bits_branches_provenance_and_check_scope() {
    for case in evaluation_fixture::cases() {
        check_request(&case);
    }
}
fn check_request(case: &EvaluationCase) {
    let body = open(case);
    let before = body
        .backend_prefix()
        .metadata_prefix()
        .source()
        .as_bytes()
        .to_vec();
    let result = FlowMediumCurveAccessor::try_from(case.ordinal)
        .and_then(|accessor| body.evaluate_curve(accessor, f64::from_bits(case.query)));
    match (&case.expected, result) {
        (Some(expected), Ok(result)) => check_result(&body, case, expected, &result),
        (None, Err(error)) => check_rejection(case, &error),
        _ => panic!("Evaluation outcome mismatch for {}", case.body.name),
    }
    assert_eq!(
        body.backend_prefix().metadata_prefix().source().as_bytes(),
        before
    );
}
fn check_result(
    body: &FlowMediumKindOne,
    case: &EvaluationCase,
    expected: &ExpectedEvaluation,
    result: &FlowMediumCurveEvaluation,
) {
    assert_eq!(class_code(result.classification()), expected.classification);
    assert_eq!(result.value().to_bits(), result.result_bits());
    if expected.classification == 3 {
        assert!(result.value().is_nan());
    } else {
        assert_eq!(result.result_bits(), expected.bits, "{}", case.body.name);
    }
    check_provenance(body, case, result);
    check_branch(result, expected);
    check_coverage(
        result,
        case.body.prefix.curves[case.ordinal as usize].count as usize,
    );
    check_diagnostics(result, expected);
}
fn check_branch(result: &FlowMediumCurveEvaluation, expected: &ExpectedEvaluation) {
    let branch = match expected.branch {
        0 => FlowMediumEvaluationBranch::Constant,
        1 => FlowMediumEvaluationBranch::LowerClamp,
        2 => FlowMediumEvaluationBranch::UpperClamp,
        3 => {
            let (lower, upper) = expected.bracket.unwrap();
            FlowMediumEvaluationBranch::Linear {
                lower: lower as usize,
                upper: upper as usize,
            }
        }
        other => panic!("Branch{other}; expected0..=3"),
    };
    assert_eq!(result.branch(), branch);
    assert_eq!(
        result.achieved_error_bound(),
        if expected.branch == 3 { None } else { Some(0.) }
    );
}
fn check_coverage(result: &FlowMediumCurveEvaluation, count: usize) {
    let coverage = result.coverage();
    assert_eq!(
        coverage.mode,
        FlowMediumEvaluationCheckMode::ExactSelectedFiniteOrderedV1
    );
    assert_eq!(coverage.table_points, count);
    assert_eq!(
        coverage.finite_values_checked,
        if count == 0 { 2 } else { 2 * count + 1 }
    );
    assert_eq!(coverage.ordered_pairs_checked, count.saturating_sub(1));
    assert!(coverage.all_selected_inputs_validated);
    assert_eq!(coverage.scalar_evaluated, count == 0);
    assert!(
        !coverage.extra_arrays_evaluated
            && !coverage.engineering_units_assigned
            && !coverage.native_application_checked
    );
}
fn check_diagnostics(result: &FlowMediumCurveEvaluation, expected: &ExpectedEvaluation) {
    let mut diagnostics = vec![
        FlowMediumEvaluationDiagnostic::EngineeringUnitsNotAssigned,
        FlowMediumEvaluationDiagnostic::NativeRuntimeNotReproduced,
        FlowMediumEvaluationDiagnostic::NativeApplicationNotChecked,
    ];
    if expected.branch == 3 {
        diagnostics.push(FlowMediumEvaluationDiagnostic::ForwardErrorNotBounded);
    }
    if expected.classification != 0 {
        diagnostics.push(FlowMediumEvaluationDiagnostic::NonFiniteArithmeticResult);
    }
    assert_eq!(result.diagnostics(), diagnostics);
}

fn check_rejection(case: &EvaluationCase, error: &bessemer::BessemerError) {
    let class = if matches!(case.ordinal, 1 | 2 | 4) {
        bessemer::FailureClass::Invalid
    } else {
        bessemer::FailureClass::Unsupported
    };
    assert_eq!(error.class, class);
    assert!(!error.operation.is_empty() && !error.entity.is_empty() && !error.cause.is_empty());
}

fn check_provenance(
    body: &FlowMediumKindOne,
    case: &EvaluationCase,
    result: &FlowMediumCurveEvaluation,
) {
    assert_eq!(result.accessor().as_i32(), case.ordinal);
    assert_eq!(result.query_bits(), case.query);
    assert_eq!(
        result.source_record(),
        body.backend_prefix().metadata_prefix().record()
    );
    assert_eq!(
        result.body_payload_interval(),
        (35, 35 + case.body.footer.end as usize)
    );
    let curve = &case.body.prefix.curves[case.ordinal as usize];
    assert_eq!(
        result.curve_payload_interval(),
        (35 + curve.start as usize, 35 + curve.end as usize)
    );
    assert_eq!(
        result.behavior(),
        FlowMediumEvaluationBehavior::FiniteOrderedV1
    );
}
