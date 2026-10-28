use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};
use super::{ModelField, ModelFieldValue};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/edge-flanges/cases.bin"),
        b"EFL1",
        684,
    )
}

fn root(style: u8) -> &'static str {
    if matches!(style, 2 | 5) {
        "root.object.items[0]"
    } else {
        "root.object"
    }
}

fn field<'a>(fields: &'a [ModelField], path: &str) -> &'a ModelField {
    fields
        .iter()
        .find(|f| f.path.as_ref() == path)
        .unwrap_or_else(|| {
            panic!("missing field {path}; expected complete edge-flange observation")
        })
}

#[test]
fn complete_edge_families_match_independent_fields_bytes_and_map_slots() {
    assert_complete_records(records());
}

#[test]
fn edge_families_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn direct_feature_data_has_the_exact_three_late_word_gates() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let direct: Vec<_> = fields
            .into_iter()
            .filter(|f| f.path.starts_with("root.object.data."))
            .collect();
        let expected = 156
            + usize::from(version > 0x2f5f) * 4
            + usize::from(version > 0x3356) * 4
            + usize::from(version > 0x42cb) * 4;
        assert_eq!(
            super::encode_fields(&direct, 30000).unwrap().len(),
            expected
        );
    }
}

#[test]
fn both_geometry_bindings_are_one_hundred_twenty_four_bytes_without_child_slots() {
    for (_, version, bytes, _, slots) in records().into_iter().filter(|r| r.0 >= 24 && r.0 % 6 == 0)
    {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_eq!(
            super::encode_fields(&fields[1..], 30000).unwrap().len(),
            124
        );
        assert_eq!(slots, 3);
    }
}

#[test]
fn single_bends_preserve_negative_empty_raw_body_counts() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 12) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_eq!(
            field(&fields, "root.object.body_tail.raw_body_count_ae0").value,
            ModelFieldValue::I32(i32::MIN)
        );
    }
}

#[test]
fn positive_raw_body_counts_and_nonzero_presences_stop_explicitly() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 12 && r.1 == 19000) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for label in [
            "raw_body_count_ae0",
            "raw_body_count_b00",
            "body_presence_b40",
            "body_presence_b50",
        ] {
            let path = format!("root.object.body_tail.{label}");
            let changed =
                super::manager_context_tests::replace_field(&fields, &path, &1i32.to_le_bytes());
            let Err(error) = decode_graph(version, &changed, false) else {
                panic!("expected unsupported raw body at {path}")
            };
            assert!(error.to_string().contains("Unsupported"));
            assert!(error.to_string().contains(label));
        }
    }
}

#[test]
fn direct_line_handles_reserve_no_outer_object_tags() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 15) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for label in ["direct_handle_878", "direct_handle_880"] {
            let prefix = format!("root.object.{label}.");
            let handle: Vec<_> = fields
                .iter()
                .filter(|f| f.path.starts_with(&prefix))
                .cloned()
                .collect();
            assert_eq!(handle.len(), 4);
            assert_eq!(super::encode_fields(&handle, 30000).unwrap().len(), 14);
        }
    }
}

#[test]
fn per_edge_dimension_presence_follows_nonzero_signed_flag_and_version() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 / 6 == 3) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let prefix = root(kind % 6);
        let flag = fields
            .iter()
            .find(|f| f.path.as_ref() == format!("{prefix}.raw_c8"));
        assert_eq!(flag.is_some(), version >= 0x42cc);
        let present =
            flag.is_some_and(|f| matches!(f.value, ModelFieldValue::I32(value) if value != 0));
        assert_eq!(
            fields
                .iter()
                .any(|f| f.path.as_ref() == format!("{prefix}.dimension_d0")),
            present
        );
    }
}

#[test]
fn direct_edge_data_rejects_a_null_required_owner() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 0 && r.1 == 19000) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.data.owner_08",
            &0u16.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected missing edge-flange data owner rejection")
        };
        assert!(error.to_string().contains("owner_08"));
    }
}
