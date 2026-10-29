use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/selection-sets/cases.bin"),
        b"SEL1",
        84,
    )
}

#[test]
fn complete_selection_sets_and_items_match_independent_fields_bytes_and_slots() {
    assert_complete_records(records());
}

#[test]
fn selection_records_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn selection_comment_occurrence_obeys_the_strict_version_boundary() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 < 6) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_eq!(
            fields
                .iter()
                .any(|f| f.path.as_ref() == "root.object.comment"),
            version > 0x3720
        );
    }
}

#[test]
fn selection_comments_and_wrappers_reject_cached_unrelated_roots() {
    for (kind, path) in [(0, "root.object.comment"), (6, "root.object.reference_40")] {
        let (_, version, bytes, _, _) = records()
            .into_iter()
            .find(|r| r.0 == kind && r.1 == 19000)
            .unwrap();
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed =
            super::manager_context_tests::replace_field(&fields, path, &2_u16.to_le_bytes());
        assert!(decode_graph(version, &changed, false).is_err());
    }
}
