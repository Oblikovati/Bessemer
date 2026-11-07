use super::*;
use bessemer::FailureClass;
fn raw(version: i32, count: usize) -> Vec<u8> {
    let mut raw = version.to_le_bytes().to_vec();
    raw.extend_from_slice(&(count as i32).to_le_bytes());
    let stride = [36, 36, 36, 36, 40, 24, 25, 81][version as usize];
    raw.resize(
        8 + count * stride + if count > 0 && version < 5 { 20 } else { 0 },
        0,
    );
    raw
}
#[test]
fn actual_zero_budgets_and_exact_limits_accept_but_one_below_rejects() {
    for row in cases() {
        check_limits(&row);
    }
    for maximum in 0..8 {
        let limits = FlowObjectRecordOptions {
            maximum_payload_bytes: maximum,
            maximum_rows: 0,
        };
        assert_eq!(
            open(&raw(0, 0), limits).unwrap_err().class,
            FailureClass::Invalid
        );
    }
}
fn check_limits(row: &ObjectCase) {
    let mut limits = FlowObjectRecordOptions {
        maximum_payload_bytes: row.raw.len(),
        maximum_rows: row.items.len(),
    };
    assert!(open(&row.raw, limits).is_ok());
    limits.maximum_payload_bytes -= 1;
    assert!(open(&row.raw, limits).is_err());
    limits.maximum_payload_bytes += 1;
    if row.items.is_empty() {
        return;
    }
    limits.maximum_rows -= 1;
    assert!(open(&row.raw, limits).is_err());
}
#[test]
fn every_small_layout_prefix_extension_and_invalid_count_or_version_is_rejected() {
    for version in 0..8 {
        for count in [0, 1, 2, 7] {
            check_malformed_layout(&raw(version, count));
        }
        for count in [i32::MIN, -1, 1, i32::MAX] {
            let mut bytes = version.to_le_bytes().to_vec();
            bytes.extend_from_slice(&count.to_le_bytes());
            assert!(open(&bytes, options()).is_err());
        }
    }
    for version in [i32::MIN, -1, 8, i32::MAX] {
        let mut bytes = version.to_le_bytes().to_vec();
        bytes.extend([0; 4]);
        assert_eq!(
            open(&bytes, options()).unwrap_err().class,
            FailureClass::Unsupported
        );
    }
}
fn check_malformed_layout(bytes: &[u8]) {
    for end in 0..bytes.len() {
        assert!(open(&bytes[..end], options()).is_err());
    }
    for suffix in [vec![0], vec![255], vec![0; 20]] {
        let mut extended = bytes.to_vec();
        extended.extend(suffix);
        assert!(open(&extended, options()).is_err());
    }
}
#[test]
fn absent_fields_and_out_of_range_row_trailer_arrays_have_typed_errors() {
    for version in 0..8 {
        let view = open(&raw(version, 1), options()).unwrap();
        let row = view.row(0).unwrap();
        for field in [
            FlowObjectField::DiscardedWord(4),
            FlowObjectField::BlockByte(16),
            FlowObjectField::ExtensionByte(56),
            FlowObjectField::BlockByte(usize::MAX),
        ] {
            assert!(row.field_id(field).is_err());
        }
        check_field_presence(&row, version);
        check_trailer_bounds(&view);
    }
}
fn check_field_presence(row: &FlowObjectRow<'_>, version: i32) {
    assert_eq!(
        row.field_id(FlowObjectField::Member7C).is_ok(),
        version >= 4
    );
    assert_eq!(
        row.field_id(FlowObjectField::StateByte).is_ok(),
        version >= 6
    );
    assert_eq!(
        row.field_id(FlowObjectField::DiscardedWord(0)).is_ok(),
        version < 5
    );
    assert_eq!(
        row.field_id(FlowObjectField::ExtensionByte(0)).is_ok(),
        version == 7
    );
}
fn check_trailer_bounds(view: &FlowObjectRecords) {
    if let Some(trailer) = view.legacy_trailer() {
        assert!(
            trailer
                .field_id(FlowObjectTrailerField::BlockByte(16))
                .is_err()
        );
        assert!(
            trailer
                .field_id(FlowObjectTrailerField::BlockByte(usize::MAX))
                .is_err()
        );
    }
}
#[test]
fn foreign_stale_wrong_encoding_and_wrong_occurrence_inputs_leave_sources_unchanged() {
    let view = open(&raw(7, 1), options()).unwrap();
    let foreign = open(&raw(7, 1), options()).unwrap();
    let id = foreign
        .row(0)
        .unwrap()
        .field_id(FlowObjectField::Member10)
        .unwrap();
    let before = view.source().as_bytes().to_vec();
    assert!(view.signed(&id).is_err() && view.replace_signed(&id, 1).is_err());
    check_stale_and_encoding(&view);
    let wrong = view.source().records().nth(2).unwrap().id;
    assert!(view.source().record_304_objects(&wrong, options()).is_err());
    assert!(
        view.source()
            .record_304_objects(foreign.record(), options())
            .is_err()
    );
    assert_eq!(view.source().as_bytes(), before);
}
fn check_stale_and_encoding(view: &FlowObjectRecords) {
    let id = view
        .row(0)
        .unwrap()
        .field_id(FlowObjectField::Member10)
        .unwrap();
    assert!(view.replace_byte(&id, 1).is_err());
    let edit = view.replace_signed(&id, 1).unwrap();
    assert!(edit.result().replace_signed(&id, 2).is_err());
    let byte = view
        .row(0)
        .unwrap()
        .field_id(FlowObjectField::BlockByte(0))
        .unwrap();
    assert!(view.replace_signed(&byte, 1).is_err());
}
#[test]
fn retained_views_isolate_caller_buffers_and_unrelated_carrier_edits() {
    let mut bytes = raw(7, 2);
    let view = open(&bytes, options()).unwrap();
    let retained = view.clone();
    let original = view.source().as_bytes().to_vec();
    bytes.fill(255);
    drop(bytes);
    let tail = view.source().records().nth(2).unwrap().id;
    let unrelated = view
        .source()
        .replace_payload(&tail, b"modified opaque tail")
        .unwrap();
    assert_ne!(unrelated.result().as_bytes(), original);
    drop(view);
    assert_eq!(retained.source().as_bytes(), original);
    check_retained_edit(&retained);
}
fn check_retained_edit(retained: &FlowObjectRecords) {
    let id = retained
        .row(1)
        .unwrap()
        .field_id(FlowObjectField::Member10)
        .unwrap();
    assert_eq!(retained.signed(&id).unwrap(), 0);
    let edit = retained.replace_signed(&id, i32::MIN).unwrap();
    assert_eq!(retained.signed(&id).unwrap(), 0);
    assert_eq!(edit.result().row(1).unwrap().member_10(), i32::MIN);
}
#[test]
fn all_new_public_types_and_reused_result_contracts_are_send_and_sync() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<FlowObjectRecords>();
    send_sync::<FlowObjectRecordOptions>();
    send_sync::<FlowObjectRow<'static>>();
    send_sync::<FlowObjectTrailer<'static>>();
    send_sync::<FlowObjectField>();
    send_sync::<FlowObjectTrailerField>();
    send_sync::<FlowObjectFieldId>();
    send_sync::<FlowObjectRecordCoverage>();
    send_sync::<FlowObjectRecordDiagnostic>();
    send_sync::<FlowSimpleTableEdit<FlowObjectRecords>>();
    send_sync::<FlowCarrierEdit>();
}
