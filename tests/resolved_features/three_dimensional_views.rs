use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};
use super::{ModelFieldValue, encode_fields, solid_reference_tests::decode_graph};
use crate::archive_codec::ObjectRecord;
fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/three-dimensional-views/cases.bin"),
        b"TDV1",
        36,
    )
}
#[test]
fn three_dimensional_views_match_all_independent_fields_bytes_and_slots() {
    assert_complete_records(records());
}
#[test]
fn three_dimensional_views_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries(records());
}
#[test]
fn three_dimensional_views_keep_duplicate_member_words_and_direct_record_slots() {
    for (_, version, bytes, _, slots) in records() {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_eq!(slots, 3);
        for (path, value) in [("raw_3fc_first", -1), ("raw_3fc_second", i32::MAX)] {
            assert_eq!(
                fields
                    .iter()
                    .find(|f| f.path.as_ref() == format!("root.object.{path}"))
                    .unwrap()
                    .value,
                ModelFieldValue::I32(value)
            );
        }
        assert_eq!(fields.iter().filter(|f|matches!(&f.value,ModelFieldValue::ObjectTag(tag) if matches!(tag.record,ObjectRecord::New{..}))).count(),1);
    }
}
#[test]
fn three_dimensional_signed_counts_preserve_negative_empty_vectors_and_keys() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| matches!(r.0, 0 | 4)) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let expected = if kind == 0 { -1 } else { i32::MIN };
        assert_eq!(
            fields
                .iter()
                .find(|f| f.path.ends_with("vector_420.declared_count"))
                .unwrap()
                .value,
            ModelFieldValue::I32(expected)
        );
        assert!(!fields.iter().any(|f| f.path.contains("vector_420.values[")));
        if kind == 4 {
            assert!(!fields.iter().any(|f| f.path.contains(".groups[")));
        }
    }
}
#[test]
fn three_dimensional_counts_combine_high_word_before_low_word() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 2 && r.1 == 19000)
        .unwrap();
    let mut fields = decode_graph(version, bytes, false).unwrap().0;
    for (suffix, value) in [("map_488.count_high", 1), ("map_488.count_low", 0)] {
        fields
            .iter_mut()
            .find(|f| f.path.ends_with(suffix))
            .unwrap()
            .value = ModelFieldValue::U32(value);
    }
    let changed = encode_fields(&fields, 30000).unwrap();
    let Err(error) = decode_graph(version, &changed, false) else {
        panic!("expected impossible 4294967296 map entries")
    };
    assert!(error.cause.contains("4294967296"));
}
#[test]
fn three_dimensional_group_counts_reject_impossible_work_before_iteration() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 2 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    for path in [
        "vector_420.declared_count",
        "map_488.entries[0].data.declared_count",
        "map_488.entries[0].data.groups[0].items.count_high",
    ] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.{path}"),
            &i32::MAX.to_le_bytes(),
        );
        assert!(decode_graph(version, &changed, false).is_err());
    }
}
#[test]
fn three_dimensional_legacy_layout_stops_before_reading_the_base() {
    let (_, _, bytes, _, _) = records().into_iter().next().unwrap();
    let Err(error) = decode_graph(0x3b06, bytes, false) else {
        panic!("expected unsupported legacy view")
    };
    assert_eq!(error.class, crate::FailureClass::Unsupported);
    assert!(error.cause.contains("expected >= 0x3B07"));
}
const MANAGERS: &[(&[u8], &str, usize)] = &[
    (
        include_bytes!("../fixtures/three-dimensional-views/owned-0-0.bin"),
        include_str!("../fixtures/three-dimensional-views/owned-0-0.tsv"),
        575,
    ),
    (
        include_bytes!("../fixtures/three-dimensional-views/owned-0-1.bin"),
        include_str!("../fixtures/three-dimensional-views/owned-0-1.tsv"),
        595,
    ),
    (
        include_bytes!("../fixtures/three-dimensional-views/owned-0-2.bin"),
        include_str!("../fixtures/three-dimensional-views/owned-0-2.tsv"),
        643,
    ),
    (
        include_bytes!("../fixtures/three-dimensional-views/owned-0-3.bin"),
        include_str!("../fixtures/three-dimensional-views/owned-0-3.tsv"),
        668,
    ),
    (
        include_bytes!("../fixtures/three-dimensional-views/owned-0-4.bin"),
        include_str!("../fixtures/three-dimensional-views/owned-0-4.tsv"),
        584,
    ),
    (
        include_bytes!("../fixtures/three-dimensional-views/owned-0-5.bin"),
        include_str!("../fixtures/three-dimensional-views/owned-0-5.tsv"),
        665,
    ),
    (
        include_bytes!("../fixtures/three-dimensional-views/owned-0-2-edited.bin"),
        include_str!("../fixtures/three-dimensional-views/owned-0-2-edited.tsv"),
        643,
    ),
];
#[test]
fn three_dimensional_complete_managers_match_independent_oracles_and_reconstruction() {
    for (bytes, oracle, reads) in MANAGERS {
        let parsed = super::manager_context_tests::decode(bytes).unwrap();
        super::manager_oracle_tests::assert_reference_fields(&parsed.fields, bytes, oracle, *reads);
        assert_eq!(encode_fields(&parsed.fields, 30000).unwrap(), *bytes);
    }
}
#[test]
fn three_dimensional_manager_edit_matches_the_independent_owned_golden() {
    let mut parsed = super::manager_context_tests::decode(MANAGERS[2].0).unwrap();
    let selected = parsed
        .fields
        .iter_mut()
        .find(|f| f.path.ends_with(".raw_400"))
        .unwrap();
    selected.value = ModelFieldValue::I32(12345);
    assert_eq!(encode_fields(&parsed.fields, 30000).unwrap(), MANAGERS[6].0);
}
#[test]
fn three_dimensional_managers_reject_every_prefix_and_appended_byte() {
    for (bytes, _, _) in MANAGERS {
        for end in 0..bytes.len() {
            assert!(super::manager_context_tests::decode(&bytes[..end]).is_err());
        }
        let mut changed = bytes.to_vec();
        changed.push(0);
        assert!(super::manager_context_tests::decode(&changed).is_err());
    }
}
