use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records_with_kernel, assert_rejected_boundaries_with_kernel,
    records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/pattern-extensions/cases.bin"),
        b"PEX1",
        468,
    )
}

#[test]
fn pattern_extensions_match_all_independent_fields_bytes_and_map_slots() {
    assert_complete_records_with_kernel(records(), true);
}

#[test]
fn pattern_extensions_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}

use super::{ModelFieldValue, solid_reference_tests::decode_graph};

#[test]
fn sketch_pattern_maps_follow_containing_flags_and_keep_raw_transform_bits() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| (6..12).contains(&r.0)) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let style = kind - 6;
        let map = fields
            .iter()
            .any(|f| f.path.ends_with("payload.declared_count_1158"));
        assert_eq!(map, matches!(style, 1 | 3 | 5));
        let words = fields
            .iter()
            .filter(|f| f.path.contains(".payload.transforms_1148.words["))
            .count();
        assert_eq!(words, [0, 0, 120, 30, 0, 180][usize::from(style)]);
    }
}

#[test]
fn pattern_signed_nonpositive_counts_keep_empty_collections() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| matches!(r.0, 13 | 16)) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let expected = if kind == 13 { -1 } else { i32::MIN };
        let count = fields
            .iter()
            .find(|f| f.path.ends_with("variation.declared_index_count_f88"))
            .unwrap();
        assert_eq!(count.value, ModelFieldValue::I32(expected));
        assert!(
            !fields
                .iter()
                .any(|f| f.path.contains(".indices_f88[") || f.path.contains(".statuses_1600["))
        );
    }
}

#[test]
fn pattern_transform_width_and_count_bounds_reject_impossible_extents() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 8 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    for (path, value) in [
        ("element_width", 119_u32),
        ("element_width", u32::MAX),
        ("count", u32::MAX),
    ] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.payload.transforms_1148.{path}"),
            &value.to_le_bytes(),
        );
        assert!(decode_graph(version, &changed, true).is_err());
    }
}
