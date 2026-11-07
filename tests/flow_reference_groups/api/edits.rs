use super::*;
#[path = "edits/cells.rs"]
mod cells;
use FlowBlockFieldValue as Value;
use FlowReferenceEntryField as Field;
use cells::*;

#[test]
fn owned_existing_fields_preserve_full_patches_noops_history_and_fresh_reversals() {
    for case in cases().into_iter().skip(1) {
        let view = read_case(&case);
        for cell in edit_cells(&view, case.frame.decoded.len() <= 4096) {
            verify_edit(&view, cell);
        }
    }
}

#[test]
fn every_native_existing_reference_group_field_has_complete_edit_contracts() {
    let view = read_case(&cases().remove(0));
    let cells = edit_cells(&view, true);
    assert_eq!(cells.len(), 1369);
    for cell in cells {
        verify_edit(&view, cell);
    }
}

fn verify_edit(view: &FlowReferenceGroups, cell: ReferenceCell) {
    let (offset, width, old) = cell.original(view);
    let noop = cell.apply(view, old).unwrap();
    assert_eq!(noop.result().source().as_bytes(), view.source().as_bytes());
    assert_eq!(noop.result().record(), view.record());
    check_history(&noop);
    let new = cell.replacement(view, old);
    let edit = cell.apply(view, new).unwrap();
    let mut expected = view.decoded_bytes().to_vec();
    expected[offset..offset + width].copy_from_slice(&new.bytes()[..width]);
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(edit.source().source().as_bytes(), view.source().as_bytes());
    check_history(&edit);
    let reverse = cell.apply(edit.result(), old).unwrap();
    assert_eq!(reverse.result().decoded_bytes(), view.decoded_bytes());
    if view.options().compression == FlowLegacyTableTransport::Raw {
        assert_eq!(
            reverse.result().source().as_bytes(),
            view.source().as_bytes()
        );
    }
}

fn check_history(edit: &FlowReferenceGroupEdit) {
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

#[test]
fn structural_count_changes_wrong_types_narrow_values_and_absent_fields_fail() {
    for case in cases().into_iter().filter(|case| {
        case.frame.name.contains("-rich-") && !case.frame.mapping && !case.frame.compressed
    }) {
        let view = read_case(&case);
        let before = view.source().as_bytes().to_vec();
        check_bad_entry_requests(&view);
        check_bad_group_requests(&view);
        assert_eq!(view.source().as_bytes(), before);
    }
    check_narrow_rejections();
    check_mapping_index();
}

fn check_narrow_rejections() {
    let case = cases()
        .into_iter()
        .find(|case| case.version == 1 && case.frame.name.contains("-rich-"))
        .unwrap();
    let view = read_case(&case);
    for value in [-129, 128, i32::MIN, i32::MAX] {
        assert!(
            view.replace_entry(0, 0, Field::SecondCount, Value::Signed32(value))
                .is_err()
        );
    }
    assert!(
        view.replace_entry(0, 0, Field::Descriptor(0), Value::Word16(256))
            .is_err()
    );
    assert!(
        view.replace_header(FlowReferenceHeaderField::Stored, 2)
            .is_err()
    );
}

#[test]
fn complete_alternate_count_parse_is_rejected_when_two_entry_boundaries_change() {
    let case = cases()
        .into_iter()
        .find(|case| case.version == 1 && !case.frame.compressed)
        .unwrap();
    let raw = alternate_count_payload();
    let view = open(&raw, options(&case.frame)).unwrap();
    let mut alternate = raw.clone();
    alternate[28] = 1;
    let other = open(&alternate, options(&case.frame)).unwrap();
    let group = other.group(0).unwrap();
    assert_eq!(group.entry(0).unwrap().second_words().word_count(), 1);
    assert_eq!(group.entry(1).unwrap().reference_count(), 0);
    let error = view
        .replace_entry(0, 0, Field::SecondCount, Value::Signed32(1))
        .unwrap_err();
    assert!(error.cause.contains("shape"));
    assert_eq!(view.decoded_bytes(), raw);
}

#[test]
fn every_public_reference_group_type_is_send_sync_and_all_versions_isolate_eight_workers() {
    check_traits();
    for case in cases()
        .into_iter()
        .filter(|case| case.frame.name.contains("-rich-"))
    {
        let mut input = case.frame.stored.clone();
        let view = open(&input, options(&case.frame)).unwrap();
        input.fill(0);
        check_workers(&view);
        assert_eq!(view.decoded_bytes(), case.frame.decoded);
    }
}

fn check_workers(view: &FlowReferenceGroups) {
    let cell = ReferenceCell::Entry(0, 0, FlowReferenceEntryField::State);
    let bits = ReferenceBits::Scalar(FlowBlockFieldValue::Byte(77));
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
    assert_invalid(
        serial
            .result()
            .source()
            .record_416_references(view.record(), view.options()),
    );
}

fn check_traits() {
    assert_send_sync::<FlowReferenceGroupOptions>();
    assert_send_sync::<FlowReferenceGroups>();
    assert_send_sync::<FlowReferenceGroup<'static>>();
    assert_send_sync::<FlowReferenceEntry<'static>>();
    assert_send_sync::<FlowReferenceHeaderField>();
    assert_send_sync::<FlowReferenceEntryField>();
    assert_send_sync::<FlowReferenceGroupCoverage>();
    assert_send_sync::<FlowReferenceGroupDiagnostic>();
    assert_send_sync::<FlowReferenceGroupEdit>();
}

fn assert_send_sync<T: Send + Sync>() {}

fn check_relation(
    before: FlowRecord<'_>,
    after: FlowRecord<'_>,
    history: &FlowRecordHistory,
    changed: bool,
) {
    assert_eq!(history.source, before.id);
    assert_eq!(history.result, after.id);
    assert_eq!(before.identifier, after.identifier);
    let expected = if before.id.ordinal() == 1 && changed {
        FlowPayloadRelation::Derived
    } else {
        FlowPayloadRelation::Copied
    };
    assert_eq!(history.relation, expected);
    if before.id.ordinal() != 1 {
        assert_eq!(before.payload, after.payload);
    }
}

fn check_bad_entry_requests(view: &FlowReferenceGroups) {
    for (field, value) in [
        (Field::Descriptor(6), Value::Word16(4)),
        (Field::SecondCount, Value::Signed32(3)),
        (Field::State, Value::Signed32(1)),
        (Field::Descriptor(7), Value::Word16(1)),
        (Field::Reference(usize::MAX), Value::Signed32(0)),
    ] {
        assert!(view.replace_entry(0, 0, field, value).is_err());
    }
    assert!(
        view.replace_entry(0, usize::MAX, Field::State, Value::Byte(0))
            .is_err()
    );
}

fn check_bad_group_requests(view: &FlowReferenceGroups) {
    assert!(view.replace_wide_word(2, 0, 0).is_err());
    assert!(view.replace_wide_word(1, usize::MAX, 0).is_err());
    if view.version() < 15 {
        assert!(view.replace_wide_word(0, 0, 0).is_err());
    }
    assert!(view.replace_mapping_word(0, 0, 0).is_err());
    assert!(view.replace_refinement_level(1, 0).is_err());
}

fn check_mapping_index() {
    let case = cases()
        .into_iter()
        .find(|case| case.frame.mapping && case.frame.name.contains("-rich-"))
        .unwrap();
    assert!(
        read_case(&case)
            .replace_mapping_word(0, usize::MAX, 0)
            .is_err()
    );
}

fn alternate_count_payload() -> Vec<u8> {
    let mut raw: Vec<u8> = [1_i32, 0, 2, 0, 0]
        .into_iter()
        .flat_map(i32::to_le_bytes)
        .collect();
    raw.extend_from_slice(&[0; 9]);
    raw.extend_from_slice(
        &[1_i32, 0, 0]
            .into_iter()
            .flat_map(i32::to_le_bytes)
            .collect::<Vec<_>>(),
    );
    raw.extend_from_slice(&[0; 13]);
    raw
}
