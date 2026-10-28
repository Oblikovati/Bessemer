use super::ModelFieldValue;
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/sketch-isocurve-information/cases.bin"),
        b"ISO1",
        162,
    )
}

#[test]
fn native_isocurve_information_regression_accepts_complete_external_entity() {
    assert_complete_records(
        records()
            .into_iter()
            .filter(|r| r.0 == 1 && r.1 == 19000)
            .collect(),
    );
}

#[test]
fn complete_isocurve_graphs_match_all_independent_fields_slots_and_bytes() {
    assert_complete_records(records());
}

#[test]
fn isocurve_graphs_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn isocurve_body_is_exactly_twenty_bytes_without_owner_or_string_fields() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_eq!(fields.len(), 7);
        assert_eq!(super::encode_fields(&fields[1..], 30000).unwrap().len(), 20);
        assert!(
            !fields
                .iter()
                .any(|f| f.path.contains("owner") || matches!(f.value, ModelFieldValue::Text(_)))
        );
    }
}

#[test]
fn signed_dimension_id_stays_unaligned_and_skip_holes_is_serialized_last() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let body = super::encode_fields(&fields[1..], 30000).unwrap();
        let expected = [
            i32::MIN,
            i32::MAX,
            -1,
            0,
            1,
            -32769,
            65536,
            -12345,
            0x12345678,
        ][usize::from(kind / 6)];
        assert_eq!(&body[14..18], expected.to_le_bytes());
        assert_eq!(fields[5].value, ModelFieldValue::I32(expected));
        assert!(fields[6].path.ends_with(".skip_holes_14"));
    }
}

#[test]
fn exact_ratio_bits_retain_signed_zero_nan_payloads_and_infinities() {
    let expected = [
        0,
        0x8000000000000000,
        0x7ff8000000001234,
        0x7ff0000000000042,
        0x7ff0000000000000,
        0xfff0000000000000,
        0x3ff0000000000000,
        0xbff0000000000000,
        0x3fc5555555555555,
    ];
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let ModelFieldValue::F64(ratio) = fields[1].value else {
            panic!("expected ratio bits")
        };
        assert_eq!(ratio.bits(), expected[usize::from(kind / 6)]);
    }
}

#[test]
fn parent_isocurve_roles_still_reject_cached_unrelated_arrays() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 3) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[1].isocurve",
            &2u16.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected unrelated array rejection")
        };
        assert_eq!(error.class, crate::FailureClass::Invalid);
    }
}
