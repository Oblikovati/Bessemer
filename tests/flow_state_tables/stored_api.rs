use super::fixture;
use FlowStoredStateMember::*;
use bessemer::flow::*;
#[path = "stored_replay.rs"]
mod replay;
const MEMBERS: [FlowStoredStateMember; 4] = [Member5E0, Member5D8, Member5DC, Discarded];
fn options(maximum: usize) -> FlowStoredStateOptions {
    FlowStoredStateOptions {
        maximum_payload_bytes: maximum,
    }
}
fn carrier(payload: &[u8]) -> FlowCarrier {
    super::state_carrier::carrier(432, payload)
}
fn open(raw: &[u8], maximum: usize) -> bessemer::Result<FlowStoredState> {
    let source = carrier(raw);
    let id = source.latest(432).unwrap().unwrap().id;
    source.record_432_stored_state(&id, options(maximum))
}
fn role(member: FlowStoredStateMember) -> &'static str {
    match member {
        Member5E0 => "[\"stored_member_5e0\"]",
        Member5D8 => "[\"stored_member_5d8\"]",
        Member5DC => "[\"stored_member_5dc\"]",
        Discarded => "[\"discarded_i32\"]",
    }
}
#[test]
fn stored_state_matches_every_complete_source_member_and_version_projection() {
    for case in fixture::cases()
        .into_iter()
        .filter(|case| case.identifier == 432)
    {
        let state = open(&case.raw, case.raw.len()).unwrap();
        assert_eq!(state.version(), case.version);
        assert_eq!(state.decoded_bytes(), case.raw);
        assert_eq!(state.options(), options(case.raw.len()));
        assert_eq!(state.record().ordinal(), 1);
        for member in MEMBERS {
            check_member(&state, &case, member);
        }
        check_coverage(&state);
    }
}
fn check_member(state: &FlowStoredState, case: &fixture::StateCase, member: FlowStoredStateMember) {
    let expected = case.fields.iter().find(|field| field.path == role(member));
    assert_eq!(
        state.member(member),
        expected.map(|field| i32::from_le_bytes(field.raw.as_slice().try_into().unwrap()))
    );
    if let Some(field) = expected {
        assert_eq!(member.payload_offset(), field.offset as usize);
    }
}
fn check_coverage(state: &FlowStoredState) {
    let coverage = state.coverage();
    assert_eq!(coverage.mode, FlowLegacyTableCheckMode::ExactRawTable);
    assert_eq!(coverage.payload_bytes, state.decoded_bytes().len());
    assert_eq!(coverage.stored_members, state.decoded_bytes().len() / 4 - 1);
    assert_eq!(coverage.achieved_error, 0);
    assert_eq!(coverage.native_recomputation_required, state.version() < 2);
    assert_eq!(
        coverage.native_recomputation_required,
        state.native_recomputation_required()
    );
    assert_eq!(
        state.diagnostics(),
        &[
            FlowStoredStateDiagnostic::NativeRecomputationNotPerformed,
            FlowStoredStateDiagnostic::NativeRuntimeNotReproduced,
            FlowStoredStateDiagnostic::EngineeringRolesNotResolved,
            FlowStoredStateDiagnostic::NativeApplicationNotChecked,
        ]
    );
}
#[test]
fn every_stored_member_edit_preserves_source_noops_history_and_fresh_exact_reversal() {
    for case in fixture::cases()
        .into_iter()
        .filter(|case| case.identifier == 432)
    {
        let state = open(&case.raw, 20).unwrap();
        for member in MEMBERS {
            if let Some(old) = state.member(member) {
                check_edit(&state, &case, member, old);
            } else {
                assert!(state.replace_member(member, 7).is_err());
            }
        }
    }
}
fn check_edit(
    state: &FlowStoredState,
    case: &fixture::StateCase,
    member: FlowStoredStateMember,
    old: i32,
) {
    let noop = state.replace_member(member, old).unwrap();
    assert_eq!(noop.result().source().as_bytes(), state.source().as_bytes());
    assert_eq!(noop.result().record(), state.record());
    let edit = state.replace_member(member, !old).unwrap();
    let mut expected = case.raw.clone();
    expected[member.payload_offset()..member.payload_offset() + 4]
        .copy_from_slice(&(!old).to_le_bytes());
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(edit.source().source().as_bytes(), state.source().as_bytes());
    assert_eq!(state.decoded_bytes(), case.raw);
    check_carrier_history(edit.carrier_edit());
    let fresh = open(&expected, 20).unwrap();
    let reverse = fresh.replace_member(member, old).unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        state.source().as_bytes()
    );
    assert_eq!(edit.result().coverage(), state.coverage());
}
use crate::state_carrier::check_history as check_carrier_history;
#[test]
fn complete_state_rejects_every_prefix_extension_negative_version_and_bad_limit() {
    for version in [0i32, 1, 2, 3, i32::MAX] {
        let mut raw = version.to_le_bytes().to_vec();
        raw.resize(if [1, 2].contains(&version) { 20 } else { 16 }, 0);
        for end in 0..raw.len() {
            assert!(open(&raw[..end], 20).is_err());
        }
        for maximum in [0, 1, 3, raw.len() - 1] {
            assert!(open(&raw, maximum).is_err());
        }
        let mut extension = raw.clone();
        extension.push(0);
        assert!(open(&extension, 100).is_err());
        raw[..4].copy_from_slice(&(-1i32).to_le_bytes());
        assert_eq!(
            open(&raw, 20).unwrap_err().class,
            bessemer::FailureClass::Unsupported
        );
    }
}
#[test]
fn native_offset_order_and_discarded_presence_have_an_independent_signed_oracle() {
    let raw: Vec<_> = [1i32, i32::MIN, -1, i32::MAX, 0x12345678]
        .into_iter()
        .flat_map(i32::to_le_bytes)
        .collect();
    let state = open(&raw, 20).unwrap();
    for (member, value) in MEMBERS
        .into_iter()
        .zip([i32::MIN, -1, i32::MAX, 0x12345678])
    {
        assert_eq!(state.member(member), Some(value));
    }
    check_occurrence_ids(&raw);
}
fn check_occurrence_ids(raw: &[u8]) {
    let source = carrier(raw);
    let wrong = source.latest(999).unwrap().unwrap().id;
    assert!(source.record_432_stored_state(&wrong, options(20)).is_err());
    let stale = source.latest(432).unwrap().unwrap().id;
    let changed = source.replace_payload(&stale, &[0; 16]).unwrap();
    assert!(
        changed
            .result()
            .record_432_stored_state(&stale, options(20))
            .is_err()
    );
}
fn assert_send_sync<T: Send + Sync>() {}
#[test]
fn published_states_are_isolated_from_input_mutation_and_concurrent_edits() {
    assert_send_sync::<FlowStoredState>();
    assert_send_sync::<FlowStoredStateOptions>();
    assert_send_sync::<FlowStoredStateMember>();
    assert_send_sync::<FlowStoredStateCoverage>();
    assert_send_sync::<FlowStoredStateDiagnostic>();
    assert_send_sync::<FlowStoredStateEdit>();
    let mut raw = vec![0; 16];
    let state = open(&raw, 20).unwrap();
    raw.fill(255);
    drop(raw);
    check_concurrent_edits(&state);
    assert_eq!(state.decoded_bytes(), &[0; 16]);
    assert!(state.replace_member(Discarded, 1).is_err());
    assert_eq!(state.decoded_bytes(), &[0; 16]);
}
fn check_concurrent_edits(state: &FlowStoredState) {
    let serial: Vec<_> = (0..8).map(|index| worker_bytes(state, index)).collect();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|index| scope.spawn(move || worker_bytes(state, index)))
            .collect();
        for (worker, expected) in workers.into_iter().zip(serial) {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
}
fn worker_bytes(state: &FlowStoredState, index: i32) -> Vec<Vec<u8>> {
    (0..32)
        .map(|step| {
            state
                .replace_member(Member5E0, index * 32 + step)
                .unwrap()
                .result()
                .source()
                .as_bytes()
                .to_vec()
        })
        .collect()
}
