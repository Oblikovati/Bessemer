use super::*;
#[test]
fn exact_payload_and_transferred_unit_bounds_accept_but_one_below_rejects() {
    for row in cases() {
        check_limits(&row);
    }
    let raw = vec![0; 204];
    let limits = FlowTextSlotOptions {
        maximum_payload_bytes: 204,
        maximum_units: 0,
        ..options()
    };
    assert_eq!(open(&raw, limits).unwrap().coverage().empty_prefixes, 50);
}
fn check_limits(row: &TextSlotCase) {
    let count = row.slots.iter().map(|slot| slot.units as usize).sum();
    let mut limits = FlowTextSlotOptions {
        maximum_payload_bytes: row.raw.len(),
        maximum_units: count,
        ..options()
    };
    assert!(open(&row.raw, limits).is_ok());
    limits.maximum_payload_bytes -= 1;
    assert!(open(&row.raw, limits).is_err());
    limits.maximum_payload_bytes += 1;
    if count == 0 {
        return;
    }
    limits.maximum_units -= 1;
    assert!(open(&row.raw, limits).is_err());
}
#[test]
fn every_small_prefix_extra_bytes_negative_count_and_nonterminal_positive_text_is_rejected() {
    for first in [vec![], vec![0], vec![65, 0], vec![0, 65, 0]] {
        check_malformed(&first_slot_payload(0, &first));
    }
    for count in [i32::MIN, -1, i32::MAX] {
        let mut raw = vec![0; 204];
        raw[4..8].copy_from_slice(&count.to_le_bytes());
        assert!(open(&raw, options()).is_err());
    }
    assert!(open(&first_slot_payload(0, &[65]), options()).is_err());
}
fn check_malformed(raw: &[u8]) {
    for end in 0..raw.len() {
        assert!(open(&raw[..end], options()).is_err());
    }
    for suffix in [vec![0], vec![255], vec![0; 4]] {
        let mut extended = raw.to_vec();
        extended.extend(suffix);
        assert!(open(&extended, options()).is_err());
    }
}
#[test]
fn failed_indices_and_foreign_wrong_and_stale_occurrences_preserve_input() {
    let view = open(&vec![0; 204], options()).unwrap();
    let before = view.source().as_bytes().to_vec();
    for (ordinal, index) in [(0, 0), (49, 0), (50, 0), (usize::MAX, 0), (0, usize::MAX)] {
        assert!(view.replace_unit(ordinal, index, 0).is_err());
    }
    check_occurrences(&view);
    assert_eq!(view.source().as_bytes(), before);
}
fn check_occurrences(view: &FlowTextSlots) {
    let foreign = open(view.decoded_bytes(), options()).unwrap();
    assert!(
        view.source()
            .record_311_text_slots(foreign.record(), options())
            .is_err()
    );
    let wrong = view.source().records().nth(2).unwrap().id;
    assert!(
        view.source()
            .record_311_text_slots(&wrong, options())
            .is_err()
    );
    let edit = view.replace_header(-1).unwrap();
    assert!(
        edit.result()
            .source()
            .record_311_text_slots(view.record(), options())
            .is_err()
    );
}
#[test]
fn failed_projection_and_caller_mutation_or_drop_leave_retained_results_isolated() {
    let mut bytes = vec![0; 204];
    let view = open(&bytes, options()).unwrap();
    let mut values: [Vec<u16>; 50] = std::array::from_fn(|_| vec![0xd800, 0, 66]);
    let previous = FlowTextSlotState::from_units(values.clone(), 150).unwrap();
    let projected = view.project(&previous, 150).unwrap();
    assert!(view.project(&previous, 149).is_err());
    bytes.fill(255);
    drop(bytes);
    for value in &mut values {
        value.fill(255);
    }
    drop(values);
    drop(previous);
    drop(view);
    for index in 0..50 {
        assert_eq!(projected.result().units(index).unwrap(), [0xd800, 0, 66]);
    }
    assert_eq!(projected.previous(), projected.result());
    assert_eq!(projected.source().header(), 0);
    assert_eq!(projected.clone().result(), projected.result());
}
#[test]
fn all_new_and_reused_public_types_are_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<FlowTextSlotOptions>();
    check::<FlowTextSlots>();
    check::<FlowTextSlot<'static>>();
    check::<FlowTextSlotCoverage>();
    check::<FlowTextSlotState>();
    check::<FlowTextSlotRelation>();
    check::<FlowTextSlotProjection>();
    check::<FlowTextSlotProjectionCoverage>();
    check::<FlowTextSlotDiagnostic>();
    check::<FlowMetadataStringMode>();
    check::<FlowSimpleTableEdit<FlowTextSlots>>();
    check::<FlowCarrierEdit>();
}
