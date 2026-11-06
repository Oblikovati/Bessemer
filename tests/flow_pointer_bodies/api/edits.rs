use super::*;
#[path = "edits/cells.rs"]
mod cells;
use cells::*;

#[test]
fn owned_values_round_trip_on_isolated_carriers_with_complete_history() {
    for case in cases().into_iter().skip(1) {
        let bodies = read_case(&case);
        for cell in edit_cells(&bodies, case.frame.decoded.len() <= 4096) {
            verify_edit(&bodies, cell);
        }
    }
}

fn verify_edit(bodies: &FlowPointerBodies, cell: BodyCell) {
    let (offset, old) = cell.original(bodies);
    let new = cell.replacement(old);
    verify_noop(bodies, cell, old);
    let edit = cell.apply(bodies, new).unwrap();
    let mut expected = bodies.decoded_bytes().to_vec();
    let width = if cell.is_wide() { 8 } else { 1 };
    expected[offset..offset + width].copy_from_slice(&new.to_le_bytes()[..width]);
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(
        edit.source().source().as_bytes(),
        bodies.source().as_bytes()
    );
    verify_history(&edit);
    verify_reverse(bodies, cell, old, &edit);
    assert_eq!(edit.result().coverage().achieved_error, 0);
}

fn verify_noop(bodies: &FlowPointerBodies, cell: BodyCell, old: u64) {
    let unchanged = cell.apply(bodies, old).unwrap();
    assert_eq!(
        unchanged.result().source().as_bytes(),
        bodies.source().as_bytes()
    );
    assert_eq!(unchanged.result().record(), bodies.record());
    verify_history(&unchanged);
}

fn verify_history(edit: &FlowPointerBodyEdit) {
    let carrier = edit.carrier_edit();
    assert_eq!(carrier.history().len(), 3);
    let changed = carrier.source().as_bytes() != carrier.result().as_bytes();
    for (ordinal, ((before, after), history)) in carrier
        .source()
        .records()
        .zip(carrier.result().records())
        .zip(carrier.history())
        .enumerate()
    {
        check_relation(before, after, history, ordinal == 1 && changed);
    }
}

fn check_relation(
    before: FlowRecord<'_>,
    after: FlowRecord<'_>,
    history: &FlowRecordHistory,
    derived: bool,
) {
    assert_eq!(history.source, before.id);
    assert_eq!(history.result, after.id);
    assert_eq!(before.identifier, after.identifier);
    let expected = if derived {
        FlowPayloadRelation::Derived
    } else {
        FlowPayloadRelation::Copied
    };
    assert_eq!(history.relation, expected);
    if before.id.ordinal() != 1 {
        assert_eq!(before.payload, after.payload);
    }
}

fn verify_reverse(
    bodies: &FlowPointerBodies,
    cell: BodyCell,
    old: u64,
    edit: &FlowPointerBodyEdit,
) {
    let reverse = cell.apply(edit.result(), old).unwrap();
    assert_eq!(reverse.result().decoded_bytes(), bodies.decoded_bytes());
    if bodies.options().compression == FlowLegacyTableTransport::Raw {
        assert_eq!(
            reverse.result().source().as_bytes(),
            bodies.source().as_bytes()
        );
    }
}

#[test]
fn native_payload_samples_preserve_all_other_bytes_and_physical_records() {
    let case = cases().remove(0);
    let bodies = read_case(&case);
    let cells = edit_cells(&bodies, false);
    for cell in cells.iter().take(8).chain(cells.iter().rev().take(8)) {
        verify_edit(&bodies, *cell);
    }
}

#[test]
#[ignore = "complete native existing-field census; run explicitly with --ignored --nocapture"]
fn every_native_existing_pointer_body_field() {
    let case = cases().remove(0);
    let bodies = read_case(&case);
    let cells = edit_cells(&bodies, true);
    assert_eq!(cells.len(), 107712);
    for (index, cell) in cells.into_iter().enumerate() {
        verify_edit(&bodies, cell);
        if index % 10000 == 0 {
            println!("{{\"native_pointer_body_requests\":{}}}", index + 1);
        }
    }
    println!("{{\"completed_native_pointer_body_requests\":107712}}");
}

#[test]
fn missing_fields_bad_indices_and_branch_changing_tags_keep_source_isolated() {
    let case = cases()
        .into_iter()
        .find(|case| case.version == 0 && case.frame.counts[2] == 3)
        .unwrap();
    let bodies = read_case(&case);
    let before = bodies.source().as_bytes().to_vec();
    assert!(bodies.replace_pool_word(usize::MAX, 0, 0).is_err());
    assert!(bodies.replace_pool_word(0, usize::MAX, 0).is_err());
    assert!(bodies.replace_entry_word(3, 0, 0, 0).is_err());
    assert!(bodies.replace_entry_word(0, usize::MAX, 0, 0).is_err());
    assert!(bodies.replace_entry_word(0, 0, 0, 0).is_err());
    assert!(bodies.replace_raw_byte(0, 0, 0, 0).is_err());
    assert!(bodies.replace_tag(0, 0, 2).is_err());
    assert!(bodies.replace_vector_byte(0, 0, usize::MAX, 0, 0).is_err());
    assert!(bodies.replace_vector_byte(0, 0, 0, usize::MAX, 0).is_err());
    assert_eq!(bodies.source().as_bytes(), before);
    check_tag_rejection();
}

fn check_tag_rejection() {
    let case = cases()
        .into_iter()
        .find(|case| case.version == 3 && case.frame.counts[0] > 0)
        .unwrap();
    let bodies = read_case(&case);
    assert!(bodies.replace_tag(0, 0, 255).is_err());
    assert!(bodies.replace_tag(0, 0, 2).is_err());
    assert!(bodies.replace_entry_word(0, 0, 0, 0).is_err());
    assert!(bodies.replace_raw_byte(0, 0, usize::MAX, 0).is_err());
}

#[test]
fn complete_alternate_tag_parse_is_rejected_when_entry_boundaries_change() {
    let case = cases()
        .into_iter()
        .find(|case| case.version == 3 && case.frame.counts == [0; 3])
        .unwrap();
    let mut context = options(&case.frame);
    context.group_counts = FlowPointerBodyCounts::try_from([2, 0, 0]).unwrap();
    let raw = [3, 0, 0, 0, 2, 0, 0, 0, 0, 1, 4, 0, 0, 0, 9, 8, 7, 255];
    let bodies = open(&raw, context.clone()).unwrap();
    let mut alternate = raw;
    alternate[4] = 1;
    let other = open(&alternate, context).unwrap();
    assert_ne!(
        bodies.entry(0, 0).unwrap().payload_interval(),
        other.entry(0, 0).unwrap().payload_interval()
    );
    let error = bodies.replace_tag(0, 0, 1).unwrap_err();
    assert!(error.cause.contains("shape"));
    assert_eq!(bodies.decoded_bytes(), raw);
}

#[test]
fn later_recognized_tags_can_change_without_decoding_or_changing_spans() {
    let case = cases()
        .into_iter()
        .find(|case| case.version == 4 && case.frame.counts[0] > 0)
        .unwrap();
    let bodies = read_case(&case);
    let edit = bodies.replace_tag(0, 0, 2).unwrap();
    assert_eq!(edit.result().entry(0, 0).unwrap().tag_byte(), Some(2));
    assert_eq!(
        edit.result().entry(0, 0).unwrap().raw_payload(),
        bodies.entry(0, 0).unwrap().raw_payload()
    );
    assert_eq!(
        edit.result().entry(0, 0).unwrap().packed_payload_decoded(),
        Some(false)
    );
}

#[test]
fn immutable_results_and_context_are_send_sync_and_eight_threads_match_serial() {
    check_thread_traits();
    for case in cases()
        .into_iter()
        .filter(|case| case.frame.name.ends_with("-rich"))
    {
        check_input_isolation(&case);
    }
}

fn check_input_isolation(case: &PointerCase) {
    let mut input = case.frame.stored.clone();
    let mut context = options(&case.frame);
    let bodies = open(&input, context.clone()).unwrap();
    input.fill(0);
    if let Some(codes) = &mut context.entry_codes {
        Arc::make_mut(codes).fill(255);
    }
    check_workers(&bodies);
    assert_eq!(bodies.decoded_bytes(), case.frame.decoded);
    assert_eq!(
        bodies.options().entry_codes.as_deref(),
        case.frame.codes.as_deref()
    );
}

fn check_thread_traits() {
    assert_send_sync::<FlowPointerBodies>();
    assert_send_sync::<FlowPointerBodyOptions>();
    assert_send_sync::<FlowPointerBodyCounts>();
    assert_send_sync::<FlowPointerBodyKind>();
    assert_send_sync::<FlowPointerBodyPool<'static>>();
    assert_send_sync::<FlowPointerBodyEntry<'static>>();
    assert_send_sync::<FlowPointerBodyCoverage>();
    assert_send_sync::<FlowPointerBodyDiagnostic>();
    assert_send_sync::<FlowPointerBodyEdit>();
}

fn check_workers(bodies: &FlowPointerBodies) {
    let cell = edit_cells(bodies, false)[0];
    let serial = cell.apply(bodies, 0x7ff800000000abcd).unwrap();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| cell.apply(bodies, 0x7ff800000000abcd).unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(
                worker.join().unwrap().result().source().as_bytes(),
                serial.result().source().as_bytes()
            );
        }
    });
}

fn assert_send_sync<T: Send + Sync>() {}
