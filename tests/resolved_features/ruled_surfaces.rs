use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records_with_kernel, assert_rejected_boundaries_with_kernel,
    records_with_count,
};
use crate::FailureClass;

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/ruled-surfaces/cases.bin"),
        b"RUL1",
        156,
    )
}

fn prefix(kind: u8) -> &'static str {
    if matches!(kind % 6, 2 | 5) {
        "root.object.items[0]"
    } else {
        "root.object"
    }
}

fn payload(kind: u8) -> String {
    if kind < 6 {
        format!("{}.direct_data_7b0", prefix(kind))
    } else {
        prefix(kind).into()
    }
}

#[test]
fn ruled_graphs_match_independent_bytes_fields_map_slots_and_kernel_frames() {
    assert_complete_records_with_kernel(records(), true);
}

#[test]
fn ruled_graphs_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}

#[test]
fn direct_data_has_no_class_header_presence_or_constructor_only_embedded_body() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 < 6) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        assert!(!fields.iter().any(|f| f.path.as_ref() == payload(kind)));
        assert!(!fields.iter().any(|f| f.path.contains("7d8")));
        assert!(
            fields
                .iter()
                .any(|f| f.path.as_ref() == format!("{}.owner", payload(kind)))
        );
    }
}

#[test]
fn direct_word_arrays_retain_unsigned_sixteen_bit_values_and_independent_counts() {
    for (kind, version, bytes, _, _) in records() {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let path = format!("{}.words_f0.items[", payload(kind));
        let values: Vec<_> = fields
            .iter()
            .filter(|f| f.path.starts_with(&path))
            .collect();
        assert_eq!(values.len(), [0, 3, 1, 4, 0, 2][usize::from(kind % 6)]);
        for (field, expected) in values.into_iter().zip([0xffff, 0, 0x8000, 0x1234]) {
            assert_eq!(field.value, super::ModelFieldValue::U16(expected));
        }
    }
}

fn assert_rejected_pointer(record: TrimEdgeRecord, suffix: &str, wire: &[u8], class: FailureClass) {
    let (kind, version, bytes, _, _) = record;
    let fields = decode_graph(version, bytes, true).unwrap().0;
    let changed = super::manager_context_tests::replace_field(
        &fields,
        &format!("{}.{suffix}", payload(kind)),
        wire,
    );
    let Err(error) = decode_graph(version, &changed, true) else {
        panic!("expected rejected {suffix}")
    };
    assert_eq!(error.class, class);
}

#[test]
fn required_feature_owners_reject_null_and_cached_unrelated_arrays() {
    for record in records().into_iter().filter(|r| r.0 % 6 == 2) {
        assert_rejected_pointer(record, "owner", &0u16.to_le_bytes(), FailureClass::Invalid);
        assert_rejected_pointer(record, "owner", &2u16.to_le_bytes(), FailureClass::Invalid);
    }
}

#[test]
fn display_and_direction_roles_reject_cached_unrelated_arrays() {
    for record in records().into_iter().filter(|r| r.0 % 6 == 2) {
        for suffix in ["distance_18", "angle_20", "direction_e8"] {
            assert_rejected_pointer(record, suffix, &2u16.to_le_bytes(), FailureClass::Invalid);
        }
    }
}

#[test]
fn impossible_direct_word_and_object_counts_fail_before_entries() {
    for record in records().into_iter().filter(|r| r.0 % 6 == 5) {
        for suffix in [
            "words_f0.items.count.count_dword",
            "array_30.count.count_dword",
            "array_50.count.count_dword",
        ] {
            let (kind, version, bytes, _, _) = record;
            let fields = decode_graph(version, bytes, true).unwrap().0;
            let changed = super::manager_context_tests::replace_field(
                &fields,
                &format!("{}.{suffix}", payload(kind)),
                &u32::MAX.to_le_bytes(),
            );
            assert!(decode_graph(version, &changed, true).is_err());
        }
    }
}

#[test]
fn serialized_surface_bodies_require_explicit_kernel_context() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 4) {
        assert!(decode_graph(version, bytes, false).is_err());
    }
}
