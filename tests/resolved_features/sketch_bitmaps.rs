use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/sketch-bitmaps/cases.bin"),
        b"BMP1",
        60,
    )
}

#[test]
fn complete_bitmap_graphs_match_independent_fields_bytes_slots_and_raw_planes() {
    assert_complete_records(records());
}

#[test]
fn bitmaps_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn bitmap_late_word_and_nonzero_presence_fields_obey_their_exact_gates() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 >= 6) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_eq!(
            fields.iter().any(|f| f.path.ends_with("raw_80")),
            version >= 0x2bd3
        );
        let count = fields
            .iter()
            .filter(|f| f.path.contains(".byte_plane_40["))
            .count();
        assert_eq!(count, if matches!(kind % 6, 1 | 2) { 6 } else { 0 });
    }
}

#[test]
fn present_bitmap_plane_rejects_negative_and_impossible_extents() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 7 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    for value in [-1_i32, 2147483647] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.raw_48",
            &value.to_le_bytes(),
        );
        assert!(decode_graph(version, &changed, false).is_err());
    }
}

#[test]
fn bitmap_reference_roles_reject_cached_unrelated_payloads() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 7 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    for path in ["line_b8", "dimension_reference_c0"] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.{path}"),
            &2_u16.to_le_bytes(),
        );
        assert!(decode_graph(version, &changed, false).is_err());
    }
}
