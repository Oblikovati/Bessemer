use super::*;
#[path = "edits/cells.rs"]
mod cells;
use cells::*;

#[test]
fn every_small_owned_value_has_isolated_noop_patch_history_and_fresh_reverse() {
    for case in fixture::cases().into_iter().skip(3) {
        let view = read_case(&case);
        for cell in edit_cells(&view, case.frame.decoded.len() <= 4096) {
            verify_edit(&view, cell);
        }
    }
}

fn verify_edit(view: &FlowFieldValues, cell: FieldCell) {
    let (offset, old) = cell.original(view);
    let noop = cell.apply(view, old).unwrap();
    assert_eq!(noop.result().source().as_bytes(), view.source().as_bytes());
    assert_eq!(noop.result().record(), view.record());
    check_history(&noop);
    let new = cell.replacement(view, old);
    let edit = cell.apply(view, new).unwrap();
    let bytes = new.bytes();
    let mut expected = view.body_bytes().to_vec();
    expected[offset..offset + bytes.len()].copy_from_slice(&bytes);
    assert_eq!(edit.result().body_bytes(), expected);
    assert_eq!(edit.source().source().as_bytes(), view.source().as_bytes());
    assert_eq!(edit.result().version(), view.version());
    check_history(&edit);
    check_reverse(view, cell, old, &edit);
    assert_prefix(&edit);
}

fn assert_prefix(edit: &FlowFieldValueEdit) {
    let payload = edit.result().source().latest(108).unwrap().unwrap().payload;
    assert_eq!(&payload[..4], edit.source().version().to_le_bytes());
    assert_eq!(edit.result().coverage().achieved_error, 0);
}

fn check_history(edit: &FlowFieldValueEdit) {
    let carrier = edit.carrier_edit();
    assert_eq!(carrier.history().len(), 3);
    let changed = carrier.source().as_bytes() != carrier.result().as_bytes();
    for ((before, after), history) in carrier
        .source()
        .records()
        .zip(carrier.result().records())
        .zip(carrier.history())
    {
        check_relation(before, after, history, changed);
    }
}

fn check_relation(
    before: FlowRecord<'_>,
    after: FlowRecord<'_>,
    history: &FlowRecordHistory,
    changed: bool,
) {
    assert_eq!(history.source, before.id);
    assert_eq!(history.result, after.id);
    assert_eq!(before.identifier, after.identifier);
    let relation = if before.id.ordinal() == 1 && changed {
        FlowPayloadRelation::Derived
    } else {
        FlowPayloadRelation::Copied
    };
    assert_eq!(history.relation, relation);
    if before.id.ordinal() != 1 {
        assert_eq!(before.payload, after.payload);
    }
}

#[test]
fn native_fields_have_deterministic_samples_under_all_three_expected_table_contexts() {
    for case in fixture::cases().into_iter().take(3) {
        let view = read_case(&case);
        let cells = edit_cells(&view, false);
        for cell in cells.iter().take(8).chain(cells.iter().rev().take(8)) {
            verify_edit(&view, *cell);
        }
    }
}

#[test]
#[ignore = "complete native record-108 existing-field census; run explicitly with --ignored --nocapture"]
fn every_native_existing_field_value() {
    let view = read_case(&fixture::cases().remove(0));
    let cells = edit_cells(&view, true);
    assert_eq!(cells.len(), 47153);
    for (index, cell) in cells.into_iter().enumerate() {
        verify_edit(&view, cell);
        if index % 5000 == 0 {
            println!("{{\"native_field_value_requests\":{}}}", index + 1);
        }
    }
    println!("{{\"completed_native_field_value_requests\":47153}}");
}

#[test]
fn checked_table_edits_reject_mismatches_while_wildcards_preserve_signed_words() {
    for case in fixture::cases().into_iter().filter(|case| {
        case.version == 5 && case.frame.name.contains("-rich-") && !case.frame.compressed
    }) {
        let view = read_case(&case);
        assert!(view.replace_width_table_word(0, -1).is_err());
        assert_eq!(view.body_bytes(), case.frame.decoded);
    }
    let case = fixture::cases()
        .into_iter()
        .find(|case| case.version == 5 && case.frame.name.contains("-wildcards-"))
        .unwrap();
    let view = read_case(&case);
    let edit = view.replace_width_table_word(0, -12345).unwrap();
    assert_eq!(edit.result().width_table_word(0), Some(-12345));
    assert_eq!(edit.result().objects().len(), view.objects().len());
    check_bad_indices(&view, &edit);
}

#[test]
fn inactive_regions_zero_width_positions_and_wrong_layout_edits_do_not_mutate_source() {
    for case in fixture::cases().into_iter().filter(|case| {
        case.frame.name.contains("-inactive-max-") || case.frame.name.contains("-zero-objects-")
    }) {
        let view = read_case(&case);
        let before = view.source().as_bytes().to_vec();
        assert!(view.replace_object_word(0, 0, 1).is_err());
        assert!(view.replace_region_word(0, 0, 1).is_err());
        if view.regions().len() > 0 {
            assert!(view.replace_width_table_word(0, -1).is_err());
        }
        assert_eq!(view.source().as_bytes(), before);
    }
}

#[test]
fn every_public_field_type_is_send_sync_and_eight_workers_match_after_input_mutation() {
    check_traits();
    for case in fixture::cases()
        .into_iter()
        .filter(|case| case.frame.name.contains("-rich-"))
    {
        let mut input = case.frame.stored.clone();
        let mut context = options(&case.frame);
        let view = open(&input, context.clone()).unwrap();
        input.fill(0);
        context.compatibility_restriction = !context.compatibility_restriction;
        check_workers(&view);
        assert_eq!(
            view.options().compatibility_restriction,
            case.frame.restriction
        );
        assert_eq!(view.body_bytes(), case.frame.decoded);
    }
}

fn check_workers(view: &FlowFieldValues) {
    let cell = if view.objects().len() > 0 {
        FieldCell::Object(0, 0)
    } else {
        FieldCell::Region(0, 0)
    };
    let bits = FieldBits::Wide(0x7ff0000000000001);
    let serial = cell.apply(view, bits).unwrap();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| cell.apply(view, bits).unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(
                worker.join().unwrap().result().source().as_bytes(),
                serial.result().source().as_bytes()
            );
        }
    });
}

fn check_traits() {
    assert_send_sync::<FlowFieldCount>();
    assert_send_sync::<FlowLegacyFieldLayout>();
    assert_send_sync::<FlowFieldRegionContext>();
    assert_send_sync::<FlowRegionFieldLayout>();
    assert_send_sync::<FlowFieldLayout>();
    assert_send_sync::<FlowFieldValueOptions>();
    assert_send_sync::<FlowFieldValues>();
    assert_send_sync::<FlowFieldObject<'static>>();
    assert_send_sync::<FlowFieldRegion<'static>>();
    assert_send_sync::<FlowFieldValueCheckMode>();
    assert_send_sync::<FlowFieldValueCoverage>();
    assert_send_sync::<FlowFieldValueDiagnostic>();
    assert_send_sync::<FlowFieldValueEdit>();
    assert_send_sync::<FlowFieldRegionType>();
    assert_send_sync::<FlowFieldWidthInput>();
    assert_send_sync::<FlowFieldWidthRequest>();
    assert_send_sync::<FlowFieldWidthOrigin>();
    assert_send_sync::<FlowFieldWidthProjection>();
    assert_send_sync::<FlowFieldWidthDiagnostic>();
}

fn assert_send_sync<T: Send + Sync>() {}

fn check_reverse(
    view: &FlowFieldValues,
    cell: FieldCell,
    old: FieldBits,
    edit: &FlowFieldValueEdit,
) {
    let reverse = cell.apply(edit.result(), old).unwrap();
    assert_eq!(reverse.result().body_bytes(), view.body_bytes());
    if view.options().body_compression == FlowLegacyTableTransport::Raw {
        assert_eq!(
            reverse.result().source().as_bytes(),
            view.source().as_bytes()
        );
    }
}

fn check_bad_indices(view: &FlowFieldValues, edit: &FlowFieldValueEdit) {
    assert!(view.replace_object_word(usize::MAX, 0, 0).is_err());
    assert!(view.replace_region_word(0, 0, 0).is_err());
    assert!(view.replace_width_table_word(usize::MAX, 0).is_err());
    assert!(view.replace_object_word(0, usize::MAX, 0).is_err());
    assert_invalid(
        edit.result()
            .source()
            .record_108_values(view.record(), view.options().clone()),
    );
}
