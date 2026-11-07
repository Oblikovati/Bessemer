use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records_with_kernel, assert_rejected_boundaries_with_kernel,
    records_with_count,
};
use super::{ModelField, ModelFieldValue, split_maps};
use std::collections::BTreeMap;

const POINTER_LABELS: [&[&str]; 2] = [
    &[
        "payload.data_868.profile_18",
        "payload.data_868.direction_40",
        "payload.chooser_870.owner",
    ],
    &["visual_10"],
];

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/split-features/cases.bin"),
        b"SPL1",
        312,
    )
}

#[test]
fn split_records_match_all_independent_primitives_bytes_and_slots() {
    assert_complete_records_with_kernel(records(), true);
}

#[test]
fn split_records_reject_every_truncated_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}

#[test]
fn split_items_and_empty_tagged_records_have_exact_minimal_bodies() {
    for (kind, version, bytes, _, slots) in
        records().into_iter().filter(|r| r.0 % 6 == 0 && r.0 >= 6)
    {
        let name_length = usize::from(u16::from_le_bytes([bytes[4], bytes[5]]));
        assert_eq!(
            bytes.len() - 6 - name_length,
            if kind < 12 { 247 } else { 0 }
        );
        assert_eq!(slots, 3);
        assert_eq!(decode_graph(version, bytes, true).unwrap().1, 3);
    }
}

#[test]
fn negative_split_map_counts_preserve_the_raw_word_and_consume_no_pairs() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 1 || r.0 == 3) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let maps = split_maps::read(&fields).unwrap();
        assert_eq!(maps.len(), 1);
        assert_eq!(maps[0].raw_count(), if kind == 1 { -1 } else { i32::MIN });
        assert!(maps[0].raw_pairs().is_empty());
        assert!(maps[0].loaded().is_empty());
    }
}

#[test]
fn derived_split_maps_ignore_negative_entries_and_keep_last_accepted_duplicate() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 2) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let maps = split_maps::read(&fields).unwrap();
        assert_eq!(maps.len(), 2);
        for map in maps {
            assert_eq!(map.raw_count(), 8);
            assert_eq!(map.raw_pairs().len(), 8);
            assert_eq!(
                map.loaded(),
                &BTreeMap::from([(0, 0), (3, 20), (i32::MAX, i32::MAX)])
            );
            assert!(map.path().ends_with(".payload.map_9a8"));
        }
    }
}

#[test]
fn split_positive_map_and_direct_keeper_counts_are_bounded() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 2) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        for (suffix, value) in [
            ("map_9a8_count", i32::MAX as u32),
            ("data_868.entity_keeper_78.count.count_dword", u32::MAX),
        ] {
            let path = format!("root.object.items[0].payload.{suffix}");
            let changed =
                super::manager_context_tests::replace_field(&fields, &path, &value.to_le_bytes());
            assert!(decode_graph(version, &changed, true).is_err());
        }
    }
}

#[test]
fn split_profile_direction_chooser_and_visual_roles_reject_cached_arrays() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 2 || r.0 == 8) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        for label in POINTER_LABELS[usize::from(kind != 2)] {
            let path = format!("root.object.items[0].{label}");
            let changed =
                super::manager_context_tests::replace_field(&fields, &path, &2u16.to_le_bytes());
            assert!(
                matches!(decode_graph(version,&changed,true),Err(e) if e.class==crate::FailureClass::Invalid)
            );
        }
    }
}

#[test]
fn present_split_item_bodies_require_kernel_context_and_reject_suppressed_returns() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 8) {
        assert!(
            matches!(decode_graph(version,bytes,false),Err(e) if e.class==crate::FailureClass::Missing)
        );
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[0].body_18.suppressed",
            &[255],
        );
        assert!(
            matches!(decode_graph(version,&changed,true),Err(e) if e.class==crate::FailureClass::Unsupported && e.cause.contains("Suppressed"))
        );
    }
}

#[test]
fn both_split_item_fixed_transforms_retain_high_bits_and_reject_special_kinds() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 6 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    for label in ["transform_20", "transform_a8"] {
        for flags in 0..=255u32 {
            let word = 0xa5a50000 | flags;
            let path = format!("root.object.{label}[28]");
            check_fixed_kind(&fields, version, &path, word);
        }
    }
}

fn check_fixed_kind(fields: &[ModelField], version: u32, path: &str, word: u32) {
    let changed = super::manager_context_tests::replace_field(fields, path, &word.to_le_bytes());
    let result = decode_graph(version, &changed, true);
    if word & 0xf0 == 0 {
        assert_eq!(
            super::encode_fields(&result.unwrap().0, 30000).unwrap(),
            changed
        );
        return;
    }
    assert!(
        matches!(result,Err(e) if e.class==crate::FailureClass::Unsupported && e.cause.contains(&format!("{word:#010x}")))
    );
}

fn map_field(path: &str, value: ModelFieldValue) -> ModelField {
    ModelField {
        path: path.into(),
        value,
    }
}

fn map_fields() -> Vec<ModelField> {
    vec![
        map_field("root.payload.map_9a8_count", ModelFieldValue::I32(1)),
        map_field("root.payload.raw_map_9a8[0].key", ModelFieldValue::I32(7)),
        map_field("root.payload.raw_map_9a8[0].value", ModelFieldValue::I32(8)),
    ]
}

#[test]
fn derived_maps_reject_missing_wrong_type_and_wrong_path_fields_without_partial_success() {
    for index in 0..3 {
        let mut fields = map_fields();
        fields[index].value = ModelFieldValue::U32(99);
        assert!(
            matches!(split_maps::read(&fields),Err(e) if e.class==crate::FailureClass::Invalid)
        );
    }
}

#[test]
fn derived_maps_reject_wrong_paths_missing_pairs_and_partial_map_collections() {
    let mut fields = map_fields();
    fields[1].path = "wrong.key".into();
    assert!(split_maps::read(&fields).is_err());
    let fields = map_fields();
    assert!(split_maps::read(&fields[..2]).is_err());
    let mut mixed = map_fields();
    mixed.push(map_field(
        "other.payload.map_9a8_count",
        ModelFieldValue::U16(7),
    ));
    assert!(split_maps::read(&mixed).is_err());
    assert!(split_maps::read(&[]).unwrap().is_empty());
}

#[test]
fn derived_split_map_results_are_send_sync_and_isolated_from_source_field_mutations() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<super::SplitIntegerMap>();
    let mut fields = map_fields();
    let maps = split_maps::read(&fields).unwrap();
    fields[1].value = ModelFieldValue::I32(-1);
    assert_eq!(maps[0].loaded(), &BTreeMap::from([(7, 8)]));
    let copied = maps[0].clone();
    assert_eq!(std::thread::spawn(move || copied).join().unwrap(), maps[0]);
}

#[test]
fn inherited_split_keepers_reject_null_and_unrelated_cached_targets() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 5) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let path = "root.object.items[0].body.id_keeper";
        for tag in [0u16, 2] {
            let changed =
                super::manager_context_tests::replace_field(&fields, path, &tag.to_le_bytes());
            assert!(
                matches!(decode_graph(version,&changed,true),Err(e) if e.class==crate::FailureClass::Invalid)
            );
        }
    }
}
