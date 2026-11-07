use super::*;
use std::sync::Arc;

fn projected(case: &WidthCase) -> bessemer::Result<FlowFieldWidthProjection> {
    let request = FlowFieldWidthRequest {
        region_type: FlowFieldRegionType::try_from(case.code)?,
        cached_width: case.cached,
        region_index: case.index,
        numeric_widths: case.numeric.as_ref().map(|values| {
            values
                .iter()
                .map(|value| match value {
                    WidthNumber::Signed(value) => FlowFieldWidthInput::Signed(*value),
                    WidthNumber::FloatBits(bits) => FlowFieldWidthInput::Binary64(*bits),
                })
                .collect::<Vec<_>>()
                .into()
        }),
    };
    project_flow_region_width(&request)
}

#[test]
fn every_isolated_width_request_matches_source_and_independent_integer_oracle() {
    for case in fixture::width_cases() {
        assert_eq!(case.expected, rebuild::projected_width(&case));
        match (projected(&case), case.expected) {
            (Ok(actual), Some(expected)) => {
                assert_eq!(actual.width.value(), expected);
                assert_eq!(actual.achieved_error, 0);
                assert_eq!(actual.request().region_type.code(), case.code as i8);
                assert_eq!(actual.request().cached_width, case.cached);
                assert_eq!(actual.request().region_index, case.index);
                assert_eq!(actual.diagnostics().len(), 3);
                assert_eq!(actual.origin, expected_origin(&case));
            }
            (Err(error), None) => {
                assert_eq!(error.class, bessemer::FailureClass::Invalid);
                assert!(!error.cause.is_empty());
            }
            (actual, expected) => panic!("width result {actual:?}; expected {expected:?}"),
        }
    }
}

fn expected_origin(case: &WidthCase) -> FlowFieldWidthOrigin {
    match case.code {
        -1 => FlowFieldWidthOrigin::Cached,
        1 => {
            let index = case.index.unwrap();
            if index < 0 || index as usize >= case.numeric.as_ref().unwrap().len() {
                FlowFieldWidthOrigin::MissingRegionZero
            } else {
                FlowFieldWidthOrigin::SelectedNumeric(index as usize)
            }
        }
        _ => FlowFieldWidthOrigin::TypeDefaultZero,
    }
}

#[test]
fn admitted_domains_and_projection_checkpoint_preserve_unused_and_mutated_inputs() {
    check_width_domains();
    let mut request = FlowFieldWidthRequest {
        region_type: FlowFieldRegionType::try_from(1).unwrap(),
        cached_width: Some(-1),
        region_index: Some(0),
        numeric_widths: Some(Arc::from([FlowFieldWidthInput::Binary64(
            14.9_f64.to_bits(),
        )])),
    };
    let result = project_flow_region_width(&request).unwrap();
    Arc::make_mut(request.numeric_widths.as_mut().unwrap())[0] = FlowFieldWidthInput::Signed(-1);
    assert_eq!(result.width.value(), 14);
    assert_eq!(
        result.request().numeric_widths.as_deref(),
        Some(&[FlowFieldWidthInput::Binary64(14.9_f64.to_bits())][..])
    );
    assert!(project_flow_region_width(&request).is_err());
}

fn check_width_domains() {
    for value in [-1, i64::MIN, i64::from(i32::MAX) + 1, i64::MAX] {
        assert!(FlowFieldCount::try_from(value).is_err());
    }
    for value in [-129, 128, i32::MIN, i32::MAX] {
        assert!(FlowFieldRegionType::try_from(value).is_err());
    }
}

#[test]
fn eight_workers_match_every_success_and_failure_of_the_complete_width_corpus() {
    let cases = fixture::width_cases();
    let serial = width_outcomes(&cases);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| width_outcomes(&cases)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}

fn width_outcomes(cases: &[WidthCase]) -> Vec<bessemer::Result<(u32, FlowFieldWidthOrigin)>> {
    cases
        .iter()
        .map(|case| projected(case).map(|projection| (projection.width.value(), projection.origin)))
        .collect()
}
