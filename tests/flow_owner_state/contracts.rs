use super::*;
use bessemer::FailureClass;
#[test]
fn every_exact_payload_bound_accepts_and_one_below_rejects() {
    for row in cases().0 {
        let mut limits = FlowOwnerStateOptions {
            maximum_payload_bytes: row.raw.len(),
        };
        assert!(open(row.identifier, &row.raw, limits).is_ok());
        limits.maximum_payload_bytes -= 1;
        assert!(open(row.identifier, &row.raw, limits).is_err());
    }
}
#[test]
fn all_small_prefixes_suffixes_and_unsupported_modern_versions_are_rejected() {
    for identifier in [122, 434] {
        let row = sample(identifier);
        check_malformed(&row);
    }
    for version in [i32::MIN, -1, 2, i32::MAX] {
        check_unsupported_version(version);
    }
}
fn check_unsupported_version(version: i32) {
    let row = sample(434);
    let mut raw = row.raw;
    raw[..4].copy_from_slice(&version.to_le_bytes());
    assert_eq!(
        open(434, &raw, options()).unwrap_err().class,
        FailureClass::Unsupported
    );
    assert_eq!(
        open(434, &raw[..4], options()).unwrap_err().class,
        FailureClass::Unsupported
    );
}

fn check_malformed(row: &OwnerCase) {
    for end in 0..row.raw.len() {
        assert!(open(row.identifier, &row.raw[..end], options()).is_err());
    }
    for suffix in [vec![0], vec![255], vec![0; 8]] {
        let mut raw = row.raw.clone();
        raw.extend(suffix);
        assert!(open(row.identifier, &raw, options()).is_err());
    }
}
#[test]
fn wrong_layout_fields_out_of_range_edits_and_foreign_ids_preserve_sources() {
    for identifier in [122, 434] {
        let row = sample(identifier);
        let view = check_case(&row);
        let before = view.source().as_bytes().to_vec();
        assert!(
            view.replace_word(view.words().word_count(), 0).is_err()
                && view.replace_word(usize::MAX, 0).is_err()
        );
        assert!(view.replace_modern_member(usize::MAX, 0).is_err());
        check_occurrences(&view, &row);
        if identifier == 122 {
            check_legacy_absence(&view);
        }
        assert_eq!(view.source().as_bytes(), before);
    }
}
fn check_legacy_absence(view: &FlowOwnerState) {
    assert_eq!(
        view.modern_member(0).unwrap_err().class,
        FailureClass::Missing
    );
    assert!(view.replace_modern_member(0, 1).is_err() && view.replace_state_byte(1).is_err());
}

fn check_occurrences(view: &FlowOwnerState, row: &OwnerCase) {
    let foreign = open(row.identifier, &row.raw, options()).unwrap();
    reject_id(view.source(), foreign.record());
    let wrong = view.source().records().nth(2).unwrap().id;
    reject_id(view.source(), &wrong);
    let edit = view.replace_word(0, 7).unwrap();
    reject_id(edit.result().source(), view.record());
}
fn reject_id(source: &FlowCarrier, id: &FlowRecordId) {
    assert!(source.record_122_owner_state(id, options()).is_err());
    assert!(source.record_434_owner_state(id, options()).is_err());
}
#[test]
fn every_new_and_reused_public_type_is_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<FlowOwnerStateOptions>();
    check::<FlowOwnerStateKind>();
    check::<FlowOwnerState>();
    check::<FlowOwnerMember>();
    check::<FlowOwnerUpdates>();
    check::<FlowOwnerStateCoverage>();
    check::<FlowBlockWordArray<'static>>();
    check::<FlowSimpleTableDiagnostic>();
    check::<FlowSimpleTableEdit<FlowOwnerState>>();
    check::<FlowCarrierEdit>();
}
