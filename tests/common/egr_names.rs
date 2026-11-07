use super::Cursor;
use bessemer::simulation::*;
use std::sync::Arc;

pub fn name_case(cursor: &mut Cursor<'_>, totals: &mut [usize; 2]) {
    let code = SimulationSolverElementCode::from_signed(cursor.word() as i32);
    let destination: Arc<[u8]> = cursor.span().into();
    let mut expected = Cursor(cursor.span());
    let result = code.project_destination(destination.clone());
    if expected.word() == 0 {
        assert!(result.is_err());
        assert!(expected.0.is_empty());
        totals[1] += 1;
        return;
    }
    let result = result.unwrap();
    assert_projection(&result, &mut expected);
    assert_eq!(result.destination, destination);
    assert_eq!(result.code.signed(), code.signed());
    assert_eq!(result.lookup, code.lookup());
    assert_state_contract(&result);
    totals[0] += 1;
}

fn assert_projection(result: &SimulationSolverNameProjection, cursor: &mut Cursor<'_>) {
    let kind = match result.lookup {
        SimulationSolverNameState::Known(_) => 1,
        SimulationSolverNameState::MatchedNullSource => 2,
        SimulationSolverNameState::UnmatchedNoWrite => 3,
    };
    assert_eq!(kind, cursor.word());
    assert_eq!(result.output.is_some(), cursor.word() != 0);
    assert_eq!(result.native_return.is_some(), cursor.word() != 0);
    assert_eq!(result.native_return.unwrap_or(0) as u32, cursor.word());
    assert_eq!(result.output.as_deref().unwrap_or(&[]), cursor.span());
    assert!(!result.wire_field_binding);
    assert!(!result.native_execution);
    assert!(cursor.0.is_empty());
}

fn assert_state_contract(result: &SimulationSolverNameProjection) {
    match result.lookup {
        SimulationSolverNameState::Known(name) => assert_known(result, name),
        SimulationSolverNameState::UnmatchedNoWrite => assert_unmatched(result),
        SimulationSolverNameState::MatchedNullSource => assert_null(result),
    }
}

fn assert_known(result: &SimulationSolverNameProjection, name: &str) {
    assert_eq!(
        result.completion,
        SimulationSolverNameCompletion::ProjectedKnownCopy
    );
    assert_eq!(
        result.check_mode,
        SimulationSolverNameCheckMode::ExactNamePrefixAndRetainedDestination
    );
    assert_eq!(result.copied_prefix_bytes, name.len() + 1);
    assert_eq!(
        result.preserved_destination_bytes,
        Some(result.destination.len() - name.len() - 1)
    );
    assert_eq!(result.requested_pattern_error_bits, Some(0));
    assert_known_bytes(result, name);
}

fn assert_unmatched(result: &SimulationSolverNameProjection) {
    assert_eq!(
        result.completion,
        SimulationSolverNameCompletion::ProjectedUnmatchedNoWrite
    );
    assert_eq!(
        result.check_mode,
        SimulationSolverNameCheckMode::ExactUnmatchedDestinationUnchanged
    );
    assert_eq!(result.copied_prefix_bytes, 0);
    assert_eq!(
        result.preserved_destination_bytes,
        Some(result.destination.len())
    );
    assert_eq!(result.requested_pattern_error_bits, Some(0));
    assert!(Arc::ptr_eq(
        result.output.as_ref().unwrap(),
        &result.destination
    ));
}

fn assert_null(result: &SimulationSolverNameProjection) {
    assert_eq!(
        result.completion,
        SimulationSolverNameCompletion::IncompleteMatchedNullSource
    );
    assert_eq!(
        result.check_mode,
        SimulationSolverNameCheckMode::ExactLookupOnlyNativeCopyIncomplete
    );
    assert_eq!(result.copied_prefix_bytes, 0);
    assert_eq!(result.preserved_destination_bytes, None);
    assert_eq!(result.requested_pattern_error_bits, None);
    assert_eq!(result.native_return, None);
    assert_eq!(result.output, None);
}

fn assert_known_bytes(result: &SimulationSolverNameProjection, name: &str) {
    let output = result.output.as_ref().unwrap();
    assert_eq!(&output[..name.len()], name.as_bytes());
    assert_eq!(output[name.len()], 0);
    assert_eq!(
        &output[name.len() + 1..],
        &result.destination[name.len() + 1..]
    );
    if output.as_ref() == result.destination.as_ref() {
        assert!(Arc::ptr_eq(output, &result.destination));
    }
}
