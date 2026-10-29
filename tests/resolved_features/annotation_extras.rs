use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records_with_kernel, assert_rejected_boundaries_with_kernel,
    records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/annotation-extras/cases.bin"),
        b"ANX1",
        660,
    )
}
#[test]
fn annotation_extras_match_independent_fields_bytes_and_slots() {
    assert_complete_records_with_kernel(records(), true);
}
#[test]
fn annotation_extras_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}
#[test]
fn annotation_symbol_suffixes_follow_both_independent_version_boundaries() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| (6..12).contains(&r.0)) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        for (suffix, present) in [
            ("raw_84", version > 0x2bca),
            ("raw_58", version >= 0x2be2),
            ("raw_68", version >= 0x2be2),
        ] {
            assert_eq!(
                fields
                    .iter()
                    .any(|f| f.path.as_ref() == format!("root.object.{suffix}")),
                present,
                "kind={kind} version={version}"
            );
        }
        assert!(
            !fields
                .iter()
                .any(|f| f.path.as_ref() == "root.object.base.raw_40")
        );
    }
}
#[test]
fn annotation_thread_array_bounds_reject_impossible_packed_counts() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 50 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    let path = fields
        .iter()
        .find(|f| f.path.ends_with("count_word"))
        .unwrap()
        .path
        .clone();
    let changed =
        super::manager_context_tests::replace_field(&fields, &path, &65534_u16.to_le_bytes());
    assert!(decode_graph(version, &changed, true).is_err());
}
#[test]
fn annotation_pattern_counts_end_after_one_word_without_entity_suffixes() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| (12..24).contains(&r.0)) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        assert!(
            fields
                .last()
                .unwrap()
                .path
                .ends_with("pattern_position_130")
        );
        assert!(
            !fields
                .iter()
                .any(|f| f.path.contains("entity0") || f.path.contains("entity1"))
        );
    }
}
