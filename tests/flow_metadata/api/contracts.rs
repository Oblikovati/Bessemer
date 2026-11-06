use super::*;

#[test]
fn every_small_prefix_extension_negative_and_impossible_count_is_rejected() {
    let mut seen = std::collections::BTreeSet::new();
    for case in fixture::state_cases() {
        if !seen.insert(case.raw.clone()) {
            continue;
        }
        for end in 0..case.raw.len() {
            assert_invalid(open(&case.raw[..end], limits()));
        }
        let mut extra = case.raw.clone();
        extra.push(0);
        assert_invalid(open(&extra, limits()));
    }
    for count in [-1_i32, i32::MAX] {
        let raw: Vec<u8> = [0, count].into_iter().flat_map(i32::to_le_bytes).collect();
        let mut limits = limits();
        limits.maximum_entries = usize::MAX;
        assert_invalid(open(&raw, limits));
    }
}

#[test]
fn exact_row_and_payload_budgets_are_independent_including_completed_empty_updates() {
    for case in fixture::state_cases() {
        let exact = FlowIdStateLimits {
            maximum_payload_bytes: case.raw.len(),
            maximum_entries: case.entries.len().max(1),
        };
        assert!(open(&case.raw, exact).is_ok());
        let mut shorter = exact;
        shorter.maximum_payload_bytes -= 1;
        assert_invalid(open(&case.raw, shorter));
        let mut zero = exact;
        zero.maximum_entries = 0;
        assert_invalid(open(&case.raw, zero));
        if !case.entries.is_empty() {
            let mut below = exact;
            below.maximum_entries = case.entries.len() - 1;
            assert_invalid(open(&case.raw, below));
        }
    }
}

#[test]
fn wrong_foreign_and_stale_occurrences_and_invalid_row_indices_leave_source_unchanged() {
    let case = fixture::state_cases().remove(0);
    let view = read_case(&case);
    let wrong = carrier(&case.raw, 411);
    let wrong_id = wrong.latest(411).unwrap().unwrap().id;
    assert_invalid(wrong.record_410_states(&wrong_id, limits()));
    assert_invalid(wrong.record_410_states(view.record(), limits()));
    assert!(view.replace_identifier(usize::MAX, -1).is_err());
    assert!(view.replace_state(case.entries.len(), 255).is_err());
    let edit = view.replace_header_word(u32::MAX).unwrap();
    assert_invalid(
        edit.result()
            .source()
            .record_410_states(view.record(), limits()),
    );
    assert_eq!(view.payload_bytes(), case.raw);
}

#[test]
fn every_existing_wire_field_has_complete_patch_noop_history_and_fresh_reverse() {
    let mut seen = std::collections::BTreeSet::new();
    for case in fixture::state_cases() {
        if !seen.insert(case.raw.clone()) {
            continue;
        }
        let view = read_case(&case);
        verify_edit(&view, StateCell::Header);
        for entry in view.entries() {
            verify_edit(&view, StateCell::Identifier(entry.ordinal()));
            verify_edit(&view, StateCell::State(entry.ordinal()));
        }
    }
}

#[derive(Clone, Copy)]
enum StateCell {
    Header,
    Identifier(usize),
    State(usize),
}

impl StateCell {
    fn original(self, view: &FlowIdStates) -> (usize, Vec<u8>) {
        match self {
            Self::Header => (0, view.header_word().to_le_bytes().to_vec()),
            Self::Identifier(ordinal) => {
                let entry = view.entry(ordinal).unwrap();
                (
                    entry.payload_interval().0,
                    entry.identifier().to_le_bytes().to_vec(),
                )
            }
            Self::State(ordinal) => {
                let entry = view.entry(ordinal).unwrap();
                (entry.payload_interval().0 + 4, vec![entry.state_byte()])
            }
        }
    }
    fn apply(self, view: &FlowIdStates, bytes: &[u8]) -> FlowIdStateEdit {
        match self {
            Self::Header => view.replace_header_word(u32::from_le_bytes(bytes.try_into().unwrap())),
            Self::Identifier(ordinal) => {
                view.replace_identifier(ordinal, i32::from_le_bytes(bytes.try_into().unwrap()))
            }
            Self::State(ordinal) => view.replace_state(ordinal, bytes[0]),
        }
        .unwrap()
    }
}

fn verify_edit(view: &FlowIdStates, cell: StateCell) {
    let (offset, old) = cell.original(view);
    let noop = cell.apply(view, &old);
    assert_eq!(noop.result().record(), view.record());
    assert_eq!(noop.result().source().as_bytes(), view.source().as_bytes());
    check_history(&noop);
    let mut new = old.clone();
    new[0] ^= 255;
    let edit = cell.apply(view, &new);
    let mut expected = view.payload_bytes().to_vec();
    expected[offset..offset + new.len()].copy_from_slice(&new);
    assert_eq!(edit.result().payload_bytes(), expected);
    assert_eq!(edit.source().source().as_bytes(), view.source().as_bytes());
    assert_eq!(edit.result().entries().len(), view.entries().len());
    check_history(&edit);
    let reverse = cell.apply(edit.result(), &old);
    assert_eq!(
        reverse.result().source().as_bytes(),
        view.source().as_bytes()
    );
}

fn check_history(edit: &FlowIdStateEdit) {
    let history = edit.carrier_edit();
    assert_eq!(history.history().len(), 3);
    let changed = history.source().as_bytes() != history.result().as_bytes();
    for ((before, after), relation) in history
        .source()
        .records()
        .zip(history.result().records())
        .zip(history.history())
    {
        check_relation(before, after, relation, changed);
    }
}

#[test]
fn input_maps_and_payloads_are_isolated_and_eight_workers_match_all_projection_and_edit_bytes() {
    check_traits();
    let case = fixture::state_cases()
        .into_iter()
        .find(|case| case.entries.len() == 256 && !case.existing.is_empty())
        .unwrap();
    let mut raw = case.raw.clone();
    let mut before: BTreeMap<i32, u8> = case.existing.iter().copied().collect();
    let existing = FlowIdStateMap::from(before.clone());
    let view = open(&raw, limits()).unwrap();
    raw.fill(0);
    before.clear();
    let serial = view.apply_to(&existing);
    assert_eq!(
        serial.existing().entries().collect::<Vec<_>>(),
        case.existing
    );
    check_workers(&view, &existing, &serial);
    assert_eq!(view.payload_bytes(), case.raw);
    assert_eq!(existing.entries().collect::<Vec<_>>(), case.existing);
}

fn check_workers(view: &FlowIdStates, existing: &FlowIdStateMap, serial: &FlowIdStateProjection) {
    let edit = view.replace_state(0, 3).unwrap();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| (view.apply_to(existing), view.replace_state(0, 3).unwrap())))
            .collect();
        for worker in workers {
            let (projection, edited) = worker.join().unwrap();
            assert_eq!(
                projection.result().entries().collect::<Vec<_>>(),
                serial.result().entries().collect::<Vec<_>>()
            );
            assert_eq!(projection.assignments(), serial.assignments());
            assert_eq!(
                edited.result().source().as_bytes(),
                edit.result().source().as_bytes()
            );
        }
    });
}

fn check_traits() {
    assert_send_sync::<FlowIdStateLimits>();
    assert_send_sync::<FlowIdStates>();
    assert_send_sync::<FlowIdStateEntry<'static>>();
    assert_send_sync::<FlowIdStateCoverage>();
    assert_send_sync::<FlowIdStateDiagnostic>();
    assert_send_sync::<FlowIdStateEdit>();
    assert_send_sync::<FlowIdStateMap>();
    assert_send_sync::<FlowIdStateAssignmentRelation>();
    assert_send_sync::<FlowIdStateAssignment>();
    assert_send_sync::<FlowIdStateProjectionCoverage>();
    assert_send_sync::<FlowIdStateProjection>();
}
fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn edited_duplicate_identifiers_use_current_rows_and_preserve_unmentioned_existing_keys() {
    let case = fixture::state_cases().remove(0);
    let source = read_case(&case);
    let renamed = source.replace_identifier(1, 410).unwrap();
    let changed = renamed.result().replace_state(1, 255).unwrap();
    let existing = map(&[(410, 0), (413, 3), (42, 9)]);
    let projection = changed.result().apply_to(&existing);
    assert_eq!(
        projection.result().entries().collect::<Vec<_>>(),
        [(42, 9), (410, 255), (413, 3)]
    );
    check_duplicate_predecessors(&projection);
    assert_eq!(projection.source().record(), changed.result().record());
    assert_eq!(
        existing.entries().collect::<Vec<_>>(),
        [(42, 9), (410, 0), (413, 3)]
    );
    assert_eq!(source.payload_bytes(), case.raw);
}

#[test]
fn eight_workers_apply_the_complete_corpus_with_identical_map_and_history_transcripts() {
    let cases = fixture::state_cases();
    let serial = projection_transcript(&cases);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| projection_transcript(&cases)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}

fn projection_transcript(cases: &[StateCase]) -> Vec<u8> {
    let mut raw = Vec::new();
    for case in cases {
        let projection = read_case(case).apply_to(&map(&case.existing));
        raw.extend_from_slice(&(projection.result().len() as u32).to_le_bytes());
        for (identifier, state) in projection.result().entries() {
            raw.extend_from_slice(&identifier.to_le_bytes());
            raw.push(state);
        }
        for assignment in projection.assignments() {
            append_assignment(&mut raw, assignment);
        }
    }
    raw
}

fn append_assignment(raw: &mut Vec<u8>, assignment: &FlowIdStateAssignment) {
    raw.extend_from_slice(&(assignment.entry_ordinal as u32).to_le_bytes());
    raw.extend_from_slice(&assignment.identifier.to_le_bytes());
    raw.push(assignment.previous.is_some() as u8);
    if let Some(previous) = assignment.previous {
        raw.push(previous);
    }
    raw.push(assignment.state);
    raw.push((assignment.relation == FlowIdStateAssignmentRelation::Created) as u8);
}

fn check_relation(
    before: FlowRecord<'_>,
    after: FlowRecord<'_>,
    relation: &FlowRecordHistory,
    changed: bool,
) {
    assert_eq!(relation.source, before.id);
    assert_eq!(relation.result, after.id);
    assert_eq!(before.identifier, after.identifier);
    let derived = changed && before.id.ordinal() == 1;
    assert_eq!(
        relation.relation,
        if derived {
            FlowPayloadRelation::Derived
        } else {
            FlowPayloadRelation::Copied
        }
    );
    if before.id.ordinal() != 1 {
        assert_eq!(before.payload, after.payload);
    }
}

fn check_duplicate_predecessors(projection: &FlowIdStateProjection) {
    let previous: Vec<_> = projection
        .assignments()
        .iter()
        .map(|assignment| assignment.previous)
        .collect();
    assert_eq!(previous, [Some(0), Some(1)]);
}
