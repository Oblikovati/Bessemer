use super::*;
fn send_sync<T: Send + Sync>() {}
#[test]
fn every_public_evaluation_type_is_send_sync_and_eight_workers_match_all_owned_request_results() {
    send_sync::<FlowMediumCurveAccessor>();
    send_sync::<FlowMediumEvaluationBehavior>();
    send_sync::<FlowMediumEvaluationBranch>();
    send_sync::<FlowMediumArithmeticClass>();
    send_sync::<FlowMediumEvaluationCheckMode>();
    send_sync::<FlowMediumEvaluationCoverage>();
    send_sync::<FlowMediumEvaluationDiagnostic>();
    send_sync::<FlowMediumCurveEvaluation>();
    let cases = evaluation_fixture::cases();
    let expected = transcript(&cases);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8).map(|_| scope.spawn(|| transcript(&cases))).collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
}
fn transcript(cases: &[EvaluationCase]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for case in cases {
        let mut raw = case.body.raw.clone();
        let body = crate::api::open(&raw, crate::api::options()).unwrap();
        raw.fill(255);
        let result = FlowMediumCurveAccessor::try_from(case.ordinal)
            .and_then(|accessor| body.evaluate_curve(accessor, f64::from_bits(case.query)));
        drop(body);
        append_outcome(&mut bytes, result);
    }
    bytes
}
fn append_outcome(bytes: &mut Vec<u8>, result: bessemer::Result<FlowMediumCurveEvaluation>) {
    match result {
        Ok(value) => {
            bytes.push(1);
            bytes.push(class_code(value.classification()));
            bytes.extend_from_slice(&value.query_bits().to_le_bytes());
            bytes.extend_from_slice(&value.result_bits().to_le_bytes());
        }
        Err(error) => {
            bytes.push(0);
            bytes.push(u8::from(error.class == bessemer::FailureClass::Unsupported));
        }
    }
}
fn constant_body() -> FlowMediumKindOne {
    let cases = evaluation_fixture::cases();
    let case = zero_case(&cases);
    open(case)
}
#[test]
fn published_evaluations_keep_their_input_identity_after_success_failure_and_source_edits() {
    let body = constant_body();
    let result = body
        .evaluate_curve(FlowMediumCurveAccessor::CurveTwo, 0.)
        .unwrap();
    let source_record = result.source_record().clone();
    let before = body.body_bytes().to_vec();
    assert!(
        body.evaluate_curve(FlowMediumCurveAccessor::CurveTwo, f64::NAN)
            .is_err()
    );
    check_source_change(&body, &result);
    assert_eq!(result.source_record(), &source_record);
    assert_eq!(body.body_bytes(), before);
    drop(body);
    assert_eq!(result.value(), 0.);
}
fn check_source_change(body: &FlowMediumKindOne, original: &FlowMediumCurveEvaluation) {
    let edit = body
        .replace_wide_word(
            FlowMediumWideField::CurveScalar { curve: 2 },
            0,
            9f64.to_bits(),
        )
        .unwrap();
    let changed = edit
        .result()
        .evaluate_curve(FlowMediumCurveAccessor::CurveTwo, 0.)
        .unwrap();
    assert_eq!(original.value(), 0.);
    assert_eq!(changed.value(), 9.);
    assert_ne!(changed.source_record(), original.source_record());
}
#[test]
fn finite_input_guards_run_before_clamps_and_internal_knots_retain_the_documented_arithmetic() {
    let cases = evaluation_fixture::cases();
    for case in cases
        .iter()
        .filter(|case| case.body.name.starts_with("bad-x-") || case.body.name.starts_with("bad-y-"))
    {
        assert!(outcome(case).is_err());
    }
    check_nan_knot(
        cases
            .iter()
            .find(|case| case.body.name == "extreme-c2-1")
            .unwrap(),
    );
    check_lost_accuracy(
        cases
            .iter()
            .find(|case| case.body.name == "extreme-c2-2")
            .unwrap(),
    );
}
fn check_nan_knot(case: &EvaluationCase) {
    let result = outcome(case).unwrap();
    assert_eq!(
        result.classification(),
        FlowMediumArithmeticClass::NotANumber
    );
    assert!(matches!(
        result.branch(),
        FlowMediumEvaluationBranch::Linear { lower: 1, upper: 2 }
    ));
}
fn check_lost_accuracy(case: &EvaluationCase) {
    let result = outcome(case).unwrap();
    assert_eq!(result.value(), 0.);
    assert_eq!(
        f64::from_bits(case.expected.as_ref().unwrap().reference),
        0.5
    );
    assert_eq!(result.achieved_error_bound(), None);
}
