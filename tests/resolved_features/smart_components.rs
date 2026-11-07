use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records_with_kernel, assert_rejected_boundaries_with_kernel,
    records_with_count,
};
use super::{ModelField, ModelFieldValue, solid_reference_tests::decode_graph};
use crate::archive_codec::ObjectRecord;

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/smart-components/cases.bin"),
        b"SMC1",
        540,
    )
}
#[test]
fn smart_components_match_all_independent_fields_bytes_and_map_slots() {
    assert_complete_records_with_kernel(records(), true);
}
#[test]
fn smart_components_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}
fn root_index(fields: &[ModelField]) -> u16 {
    match &fields[0].value {
        ModelFieldValue::ObjectTag(tag) => match &tag.record {
            ObjectRecord::New { index, .. } => u16::try_from(index.value()).unwrap(),
            _ => panic!("expected actual fresh root identity"),
        },
        _ => panic!("expected actual fresh root tag"),
    }
}
#[test]
fn smart_feature_requires_nonnull_data_before_reading_external_object() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 0 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.data_368",
        &0_u16.to_le_bytes(),
    );
    assert!(decode_graph(version, &changed, true).is_err());
}
#[test]
fn smart_typed_pointers_reject_their_cached_unrelated_containing_root() {
    for (kind, path) in [
        (2, "data_368"),
        (8, "context_88"),
        (8, "feature_folder_e8"),
        (8, "component_folder_e0"),
        (8, "reference_folder_f0"),
        (38, "feature_40"),
        (44, "transform_40"),
        (44, "transform_48"),
        (50, "smart_feature_78"),
    ] {
        let (_, version, bytes, _, _) = records()
            .into_iter()
            .find(|r| r.0 == kind && r.1 == 19000)
            .unwrap();
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.{path}"),
            &root_index(&fields).to_le_bytes(),
        );
        assert!(
            decode_graph(version, &changed, true).is_err(),
            "kind={kind} path={path}"
        );
    }
}
#[test]
fn smart_items_keep_their_original_containing_feature_owner_identity() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 2 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    let root = u32::from(root_index(&fields));
    let mut owners = 0;
    for field in fields
        .iter()
        .filter(|f| f.path.ends_with("smart_feature_60") || f.path.ends_with("smart_feature_78"))
    {
        let ModelFieldValue::ObjectTag(tag) = &field.value else {
            panic!("expected typed owner tag")
        };
        assert!(matches!(&tag.record,ObjectRecord::Reference {index,..} if index.value()==root));
        owners += 1;
    }
    assert_eq!(owners, 3);
}
#[test]
fn smart_sensor_maps_preserve_duplicate_keys_nulls_and_shared_values() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| matches!(r.0, 55 | 56 | 57 | 59))
    {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let first = fields
            .iter()
            .find(|f| f.path.as_ref() == "root.object.entries[0].key")
            .unwrap();
        let second = fields
            .iter()
            .find(|f| f.path.as_ref() == "root.object.entries[1].key")
            .unwrap();
        assert_eq!(first.value, second.value, "kind={kind}");
        let value = fields
            .iter()
            .find(|f| f.path.as_ref() == "root.object.entries[1].object")
            .unwrap();
        assert!(
            matches!(&value.value,ModelFieldValue::ObjectTag(tag) if matches!(tag.record,ObjectRecord::Null))
        );
    }
}
#[test]
fn smart_sensor_maps_reject_impossible_readcount_extents() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 56 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.entries.count.count_word",
        &65534_u16.to_le_bytes(),
    );
    assert!(decode_graph(version, &changed, true).is_err());
}
