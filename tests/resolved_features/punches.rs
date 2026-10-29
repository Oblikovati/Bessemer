use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

const REVISIONS: [i32; 10] = [-1, 0, 1, 2, 3, 4, 5, 6, 7, 8];

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/punches/cases.bin"),
        b"PCH1",
        300,
    )
}

#[test]
fn complete_punch_revisions_match_independent_fields_bytes_and_map_slots() {
    assert_complete_records(records());
}

#[test]
fn punch_records_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn internal_revision_gates_remain_independent_of_file_version() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let revision = REVISIONS[usize::from(kind / 6)];
        let has = |path: &str| fields.iter().any(|f| f.path.as_ref() == path);
        for (minimum, path) in [
            (1, "raw_864"),
            (4, "raw_910"),
            (5, "raw_914"),
            (6, "raw_918"),
            (6, "raw_85c"),
            (7, "tagged_array_920"),
        ] {
            assert_eq!(
                has(&format!("root.object.payload.{path}")),
                revision >= minimum
            );
        }
        assert_eq!(has("root.object.payload.legacy_integer_830"), revision < 2);
        assert_eq!(
            has("root.object.payload.points_848.element_width"),
            revision >= 2
        );
    }
}

#[test]
fn punch_dynamic_arrays_reject_wrong_units_and_impossible_counts() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 20 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    for (path, word) in [
        ("points_848.element_width", 12_u32),
        ("integers_830.element_width", 6),
        ("points_848.count", u32::MAX),
    ] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.payload.{path}"),
            &word.to_le_bytes(),
        );
        assert!(decode_graph(version, &changed, false).is_err());
    }
}

#[test]
fn huge_zero_count_dynamic_widths_do_not_allocate_payload_values() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 22 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    assert!(
        !fields
            .iter()
            .any(|f| f.path.starts_with("root.object.payload.points_848.values["))
    );
    assert!(!fields.iter().any(|f| {
        f.path
            .starts_with("root.object.payload.integers_830.values[")
    }));
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
}

#[test]
fn revision_seven_array_role_rejects_a_cached_unrelated_punch() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 48 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.payload.tagged_array_920",
        &2_u16.to_le_bytes(),
    );
    assert!(decode_graph(version, &changed, false).is_err());
}
