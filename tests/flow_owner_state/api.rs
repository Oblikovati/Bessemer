use super::*;
#[path = "contracts.rs"]
mod contracts;
#[path = "edit.rs"]
mod edit;
#[path = "replay.rs"]
mod replay;
#[path = "selection.rs"]
mod selection;
fn options() -> FlowOwnerStateOptions {
    FlowOwnerStateOptions {
        maximum_payload_bytes: 1024,
    }
}
fn open(
    identifier: u32,
    raw: &[u8],
    options: FlowOwnerStateOptions,
) -> bessemer::Result<FlowOwnerState> {
    let source = state_carrier::carrier(identifier, raw);
    let id = source.records().nth(1).unwrap().id;
    match identifier {
        122 => source.record_122_owner_state(&id, options),
        434 => source.record_434_owner_state(&id, options),
        other => panic!("identifier {other}; expected 122 or 434"),
    }
}
fn check_case(row: &OwnerCase) -> FlowOwnerState {
    let view = open(row.identifier, &row.raw, options()).unwrap();
    assert_eq!(view.decoded_bytes(), row.raw);
    assert_eq!(view.options(), options());
    assert_eq!(view.record(), &view.source().records().nth(1).unwrap().id);
    assert_eq!(view.version(), row.version);
    assert_eq!(view.state_byte().map(i32::from), row.state);
    assert_eq!(view.updates_enabled(), row.gate);
    assert_eq!(projection(&view), row.updates);
    let kind = if row.identifier == 122 {
        FlowOwnerStateKind::Legacy122
    } else {
        FlowOwnerStateKind::Modern434
    };
    assert_eq!(view.kind(), kind);
    check_words(&view, row);
    check_coverage(&view, row);
    view
}
fn sample(identifier: u32) -> OwnerCase {
    cases()
        .0
        .into_iter()
        .find(|row| row.identifier == identifier)
        .unwrap()
}

fn projection(view: &FlowOwnerState) -> Updates {
    view.updates()
        .map(|updates| updates.members().map(Some))
        .unwrap_or([None; 7])
}
fn members() -> [FlowOwnerMember; 7] {
    use FlowOwnerMember::*;
    [
        Member08, Member0C, Member10, Member14, Member18, Member1C, Member20,
    ]
}
fn check_words(view: &FlowOwnerState, row: &OwnerCase) {
    let array = view.words();
    let start = if row.identifier == 122 { 0 } else { 4 };
    assert_eq!(array.as_bytes(), row.values);
    assert_eq!(array.word_count(), row.values.len() / 4);
    assert_eq!(array.payload_interval(), (start, start + row.values.len()));
    for (index, raw) in row.values.as_chunks::<4>().0.iter().enumerate() {
        assert_eq!(array.raw_word(index), Some(u32::from_le_bytes(*raw)));
        if row.identifier == 434 {
            assert_eq!(view.modern_member(index).unwrap(), i32::from_le_bytes(*raw));
        }
    }
    assert!(array.raw_word(array.word_count()).is_none() && array.raw_word(usize::MAX).is_none());
    assert!(
        view.modern_member(array.word_count()).is_err() && view.modern_member(usize::MAX).is_err()
    );
    check_members(view, row);
}
fn check_members(view: &FlowOwnerState, row: &OwnerCase) {
    for (index, member) in members().into_iter().enumerate() {
        assert_eq!(member.native_offset(), 8 + index as u32 * 4);
        if let Some(updates) = view.updates() {
            assert_eq!(Some(updates.member(member)), row.updates[index]);
        }
    }
}
fn check_coverage(view: &FlowOwnerState, row: &OwnerCase) {
    let proof = view.coverage();
    assert_eq!(proof.mode, FlowLegacyTableCheckMode::ExactRawTable);
    assert_eq!(proof.payload_bytes, row.raw.len());
    assert_eq!(proof.raw_words, row.values.len() / 4);
    assert_eq!(proof.updates_enabled, row.gate);
    assert_eq!(proof.projected_updates, if row.gate { 7 } else { 0 });
    assert_eq!(proof.achieved_error, 0);
    check_diagnostics(view);
}
fn check_diagnostics(view: &FlowOwnerState) {
    use FlowSimpleTableDiagnostic::*;
    assert_eq!(
        view.diagnostics(),
        &[
            NativeRuntimeNotReproduced,
            EngineeringRolesNotResolved,
            NativeApplicationNotChecked
        ]
    );
}
#[test]
fn all_stored_fields_optional_presence_and_native_member_updates_match_owned_source_results() {
    for row in cases().0 {
        check_case(&row);
    }
}
