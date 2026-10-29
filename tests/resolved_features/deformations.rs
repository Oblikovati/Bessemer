use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records_with_kernel, assert_rejected_boundaries_with_kernel,
    records_with_count,
};
use super::{ModelField, ModelFieldValue, solid_reference_tests::decode_graph};
const FEATURE_VERSIONS: [i32; 15] = [i32::MIN, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, i32::MAX];
const GEOMETRY_VERSIONS: [i32; 6] = [0, 7, 3, 2, 4, 7];
fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/deformations/cases.bin"),
        b"DFM1",
        384,
    )
}
#[test]
fn deformations_match_independent_fields_bytes_and_object_slots() {
    assert_complete_records_with_kernel(records(), true);
}
#[test]
fn deformations_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}
fn stored_deform_value<'a>(fields: &'a [ModelField], suffix: &str) -> &'a ModelFieldValue {
    &fields
        .iter()
        .find(|f| f.path.ends_with(suffix))
        .unwrap()
        .value
}
fn assert_stored_versions(fields: &[ModelField], feature: i32, geometry: i32) {
    assert_eq!(
        *stored_deform_value(fields, "payload.stored_version_8d0"),
        ModelFieldValue::I32(feature)
    );
    assert_eq!(
        *stored_deform_value(fields, "geometry_8f8.words_f8_134[15]"),
        ModelFieldValue::I32(geometry)
    );
}
fn assert_versioned_fields(fields: &[ModelField], version: i32) {
    for (suffix, present) in [
        ("direction_820", version >= 1),
        ("array_838.presence", version >= 2),
        ("array_848.presence", version >= 3),
        ("raw_8d4", version >= 4),
        ("raw_8d8", version >= 6),
        ("array_850.presence", version >= 8),
    ] {
        assert_eq!(
            fields
                .iter()
                .any(|f| f.path.ends_with(&format!("payload.{suffix}"))),
            present
        );
    }
}
fn assert_matching_passes(fields: &[ModelField], kind: u8, version: i32) {
    let matches = matches!(kind % 6, 1 | 2 | 3 | 5);
    let populated_subrecords = matches!(kind % 6, 2 | 3 | 5);
    assert_eq!(
        fields.iter().any(|f| f.path.contains("second_sides[")),
        version >= 9 && matches
    );
    assert_eq!(
        fields.iter().any(|f| f.path.contains("vectors_88_a0[")),
        version >= 10 && populated_subrecords
    );
}
#[test]
fn deform_feature_and_geometry_versions_remain_independent() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 < 90) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let feature = FEATURE_VERSIONS[usize::from(kind / 6)];
        let geometry = GEOMETRY_VERSIONS[usize::from(kind % 6)];
        assert_stored_versions(&fields, feature, geometry);
        assert_versioned_fields(&fields, feature);
        assert_matching_passes(&fields, kind, feature);
    }
}
#[test]
fn deform_negative_matching_counts_keep_empty_native_loops() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 < 90 && r.0 % 6 == 4) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        assert_eq!(
            fields
                .iter()
                .find(|f| f.path.ends_with("declared_match_count"))
                .unwrap()
                .value,
            ModelFieldValue::I32(i32::MIN)
        );
        assert!(
            !fields.iter().any(|f| f.path.contains(".matches_8c8[")),
            "kind={kind}"
        );
    }
}
#[test]
fn deform_extension_terminators_retain_the_exact_native_signed_word() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| r.0 < 90 && FEATURE_VERSIONS[usize::from(r.0 / 6)] >= 7)
    {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let terminal = fields
            .iter()
            .rfind(|f| f.path.contains(".extensions[") && f.path.ends_with(".code"))
            .unwrap();
        assert_eq!(
            terminal.value,
            ModelFieldValue::I32([0, -1, 0x12345678, -8, i32::MIN, -8][usize::from(kind % 6)])
        );
    }
}
#[test]
fn deform_counts_and_curve_point_roles_reject_impossible_shapes() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 80 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    for suffix in ["declared_match_count", "matches_8c8[0].declared_subcount"] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.payload.{suffix}"),
            &i32::MAX.to_le_bytes(),
        );
        assert!(decode_graph(version, &changed, true).is_err());
    }
    let path = "root.object.payload.matches_8c8[0].subdata[0].sides[0].curve_point";
    let changed = super::manager_context_tests::replace_field(&fields, path, &2_u16.to_le_bytes());
    assert!(decode_graph(version, &changed, true).is_err());
}
