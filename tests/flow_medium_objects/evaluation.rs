#[path = "evaluation/api.rs"]
mod api;
use super::{fixture, packet::*, rebuild, types::*};
#[path = "evaluation/fixture.rs"]
mod evaluation_fixture;
#[path = "evaluation/types.rs"]
mod evaluation_types;
use evaluation_types::*;
#[test]
fn every_numeric_request_rebuilds_all_body_fields_and_matches_complete_existing_body_views() {
    let cases = evaluation_fixture::cases();
    assert_eq!(cases.len(), 5424);
    for case in cases {
        assert_eq!(rebuild::body(&case.body), case.body.raw);
        let body = crate::api::open(&case.body.raw, crate::api::options()).unwrap();
        crate::api::read::check_body(&body, &case.body);
    }
}
#[test]
fn frozen_numeric_outcomes_preserve_rejections_endpoints_constants_and_all_arithmetic_classes() {
    let cases = evaluation_fixture::cases();
    let mut accepted = 0;
    let mut classes = std::collections::BTreeSet::new();
    for case in &cases {
        if let Some(expected) = &case.expected {
            accepted += 1;
            classes.insert(expected.classification);
            check_accepted(case, expected);
        } else {
            check_rejected(case);
        }
    }
    assert_eq!(accepted, 5184);
    assert_eq!(cases.len() - accepted, 240);
    assert_eq!(classes, [0, 1, 2, 3].into());
}
fn check_accepted(case: &EvaluationCase, expected: &ExpectedEvaluation) {
    assert!(matches!(case.ordinal, 1 | 2 | 4));
    let query = f64::from_bits(case.query);
    assert!(query.is_finite());
    let curve = &case.body.prefix.curves[case.ordinal as usize];
    assert_eq!(
        classify(f64::from_bits(expected.bits)),
        expected.classification
    );
    assert!(f64::from_bits(expected.reference).is_finite());
    check_measured_error(expected);
    if curve.count == 0 {
        assert_eq!(expected.branch, 0);
        assert!(expected.bracket.is_none());
        assert_eq!(expected.bits, curve.scalar);
        assert_eq!(expected.measured_error_bound, Some(0));
        return;
    }
    check_finite_arrays(curve);
    check_bracket(curve, query, expected);
}
fn check_measured_error(expected: &ExpectedEvaluation) {
    if expected.classification == 0 {
        let bound = f64::from_bits(expected.measured_error_bound.unwrap());
        assert!(!bound.is_nan() && bound >= 0.);
    } else {
        assert!(expected.measured_error_bound.is_none());
    }
}
fn classify(value: f64) -> u8 {
    if value.is_nan() {
        return 3;
    }
    if value.is_infinite() {
        return if value.is_sign_positive() { 1 } else { 2 };
    }
    0
}
fn check_finite_arrays(curve: &BodyCurve) {
    assert!(
        curve
            .first
            .iter()
            .chain(&curve.second)
            .all(|bits| f64::from_bits(*bits).is_finite())
    );
    assert!(
        curve
            .first
            .windows(2)
            .all(|pair| f64::from_bits(pair[0]) < f64::from_bits(pair[1]))
    );
}
fn check_bracket(curve: &BodyCurve, query: f64, expected: &ExpectedEvaluation) {
    let (lower, upper) = expected.bracket.unwrap();
    let count = curve.count;
    match expected.branch {
        1 => {
            assert_eq!((lower, upper), (0, 0));
            assert!(query <= f64::from_bits(curve.first[0]));
        }
        2 => {
            assert_eq!((lower, upper), (count - 1, count - 1));
            assert!(query >= f64::from_bits(*curve.first.last().unwrap()));
        }
        3 => check_linear(curve, query, lower, upper),
        other => panic!("Branch {other}; expected1/2/3 for populated table"),
    }
    if lower == upper {
        assert_eq!(expected.bits, curve.second[lower as usize]);
    }
}
fn check_rejected(case: &EvaluationCase) {
    if !matches!(case.ordinal, 1 | 2 | 4) || !f64::from_bits(case.query).is_finite() {
        return;
    }
    let curve = &case.body.prefix.curves[case.ordinal as usize];
    if curve.count == 0 {
        assert!(!f64::from_bits(curve.scalar).is_finite());
        return;
    }
    let nonfinite = curve
        .first
        .iter()
        .chain(&curve.second)
        .any(|bits| !f64::from_bits(*bits).is_finite());
    let nonordered = curve
        .first
        .windows(2)
        .any(|pair| f64::from_bits(pair[0]) >= f64::from_bits(pair[1]));
    assert!(nonfinite || nonordered);
}
#[test]
fn every_affine_numeric_request_matches_an_independent_closed_form_with_clamped_endpoints() {
    for case in evaluation_fixture::cases()
        .iter()
        .filter(|case| case.body.name.starts_with("affine-"))
    {
        let curve = &case.body.prefix.curves[case.ordinal as usize];
        let query = f64::from_bits(case.query);
        let first = f64::from_bits(curve.first[0]);
        let last = f64::from_bits(*curve.first.last().unwrap());
        let expected = 3. * query.clamp(first, last) - 5.;
        assert_eq!(
            case.expected.as_ref().unwrap().bits,
            expected.to_bits(),
            "{}",
            case.body.name
        );
    }
}

fn check_linear(curve: &BodyCurve, query: f64, lower: u32, upper: u32) {
    assert_eq!(lower + 1, upper);
    assert!(upper < curve.count);
    assert!(query >= f64::from_bits(curve.first[lower as usize]));
    assert!(query < f64::from_bits(curve.first[upper as usize]));
}
