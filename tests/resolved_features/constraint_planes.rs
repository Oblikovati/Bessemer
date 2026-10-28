use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};
use super::{ModelField, ModelFieldValue};
use crate::archive_codec::ObjectRecord;

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/constraint-plane-families/cases.bin"),
        b"CPL1",
        1620,
    )
}

fn root(kind: u8) -> String {
    let prefix = if matches!(kind % 6, 2 | 5) {
        "root.object.items[0]"
    } else {
        "root.object"
    };
    if kind / 6 < 9 && matches!(kind % 6, 4 | 5) {
        format!("{prefix}.plane_data")
    } else {
        prefix.into()
    }
}

fn record<'a>(fields: &'a [ModelField], path: &str) -> &'a ObjectRecord {
    let field = fields.iter().find(|f| f.path.as_ref() == path).unwrap();
    let ModelFieldValue::ObjectTag(tag) = &field.value else {
        panic!("{path}: expected object tag")
    };
    &tag.record
}

fn reject_target(fields: &[ModelField], version: u32, path: &str, tag: u16) {
    let bytes = super::manager_context_tests::replace_field(fields, path, &tag.to_le_bytes());
    let Err(error) = decode_graph(version, &bytes, false) else {
        panic!("{path}: expected rejected target {tag}")
    };
    assert_eq!(error.class, crate::FailureClass::Invalid);
}

#[test]
fn complete_planes_match_independent_primitives_slots_and_bytes() {
    assert_complete_records(records());
}

#[test]
fn complete_planes_reject_every_prefix_appended_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn minimal_geometry_and_interface_bodies_have_exact_native_lengths() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| r.0 % 6 == 0 && (9..17).contains(&(r.0 / 6)))
    {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let body = super::encode_fields(&fields[1..], 30000).unwrap();
        let expected = match kind / 6 {
            9 => 6,
            10..=12 => 42,
            13..=15 => 43,
            16 => 51,
            _ => panic!("unexpected kind {kind}"),
        };
        assert_eq!(body.len(), expected, "kind {kind} version {version}");
    }
}

#[test]
fn point_surface_dependencies_require_both_finite_typed_targets() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 < 6) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for label in ["surface_88", "point_90"] {
            for tag in [0, 2] {
                reject_target(&fields, version, &format!("{}.{label}", root(kind)), tag);
            }
        }
    }
}

#[test]
fn constraint_interface_and_geometry_roles_reject_cached_arrays() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| r.0 % 6 == 2 && (1..17).contains(&(r.0 / 6)))
    {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let labels: &[&str] = match kind / 6 {
            1..=8 => &["interface_88"],
            9 => &["geometry_08", "geometry_10", "geometry_18"],
            _ => &["reference_08"],
        };
        for label in labels {
            reject_target(&fields, version, &format!("{}.{label}", root(kind)), 2);
        }
    }
}

#[test]
fn all_new_data_classes_work_through_complete_plane_parent_roles() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 < 54 && r.0 % 6 == 5) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        reject_target(&fields, version, &root(kind), 2);
        let ObjectRecord::New { class, .. } = record(&fields, &root(kind)) else {
            panic!("expected fresh plane data")
        };
        assert!(super::reference_planes::CLASSES.contains(&class.name.as_str()));
    }
}

#[test]
fn interfaces_retain_shared_identity_without_a_count_prefix() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| r.0 / 6 == 9 && r.0 % 6 > 0)
    {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let prefix = root(kind);
        assert_shared_geometry(&fields, &prefix);
        assert!(
            !fields
                .iter()
                .any(|f| f.path.as_ref() == format!("{prefix}.count"))
        );
    }
}

fn assert_shared_geometry(fields: &[ModelField], prefix: &str) {
    let ObjectRecord::New { index: first, .. } = record(fields, &format!("{prefix}.geometry_08"))
    else {
        panic!("expected fresh geometry")
    };
    let ObjectRecord::Reference { index: last, .. } =
        record(fields, &format!("{prefix}.geometry_18"))
    else {
        panic!("expected cached geometry")
    };
    assert_eq!(first, last);
}

#[test]
fn raw_flags_keep_all_bytes_and_derive_only_equality_with_one() {
    let flags = [0, 1, 2, 128, 254, 255];
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| (13..17).contains(&(r.0 / 6)))
    {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let derived = super::plane_geometry_flags::read(&fields).unwrap();
        assert!(!derived.is_empty());
        for flag in derived {
            assert_eq!(flag.raw(), flags[usize::from(kind % 6)]);
            assert_eq!(flag.loaded(), flag.raw() == 1);
            assert!(flag.path().starts_with("root.object"));
        }
    }
}

#[test]
fn angle_data_has_its_own_suffix_without_a_phantom_runtime_word() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 / 6 == 3) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_angle_suffix(&fields, &root(kind));
        if kind % 6 == 2 {
            reject_target(
                &fields,
                version,
                &format!("{}.angular_dimension_150", root(kind)),
                2,
            );
        }
    }
}

fn assert_angle_suffix(fields: &[ModelField], prefix: &str) {
    for label in ["raw_148", "angular_dimension_150"] {
        assert!(
            fields
                .iter()
                .any(|f| f.path.as_ref() == format!("{prefix}.{label}"))
        );
    }
    assert!(
        !fields
            .iter()
            .any(|f| f.path.as_ref() == format!("{prefix}.raw_e8"))
    );
}

pub(super) fn fresh_base_surface_wrapper_class() -> String {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 102 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let ObjectRecord::New { class, .. } = record(&fields, "root.object.reference") else {
        panic!("expected fresh surface wrapper")
    };
    class.name.clone()
}

#[test]
fn base_surface_works_through_the_existing_sketch_wrapper_caller() {
    assert_eq!(fresh_base_surface_wrapper_class(), "moSurfaceRef_w");
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 104) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        reject_target(&fields, version, "root.object.items[0].reference", 2);
    }
}

#[test]
fn derived_flags_are_send_sync_owned_and_validate_recognized_field_types() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<super::PlaneGeometryFlag>();
    let mut fields = vec![ModelField {
        path: "root.raw_plane_geometry_flag_38".into(),
        value: ModelFieldValue::U8(2),
    }];
    let original = super::plane_geometry_flags::read(&fields).unwrap();
    fields[0].value = ModelFieldValue::U8(1);
    assert!(!original[0].loaded());
    assert!(super::plane_geometry_flags::read(&fields).unwrap()[0].loaded());
    fields[0].value = ModelFieldValue::U16(1);
    assert!(super::plane_geometry_flags::read(&fields).is_err());
    assert!(super::plane_geometry_flags::read(&[]).unwrap().is_empty());
    let joined = std::thread::spawn(move || original[0].raw())
        .join()
        .unwrap();
    assert_eq!(joined, 2);
}

#[test]
fn combined_owned_fixture_families_equal_the_exact_plane_data_role() {
    use std::collections::BTreeSet;
    let mut observed: BTreeSet<String> = super::body_feature_tests::ORIGINAL_PLANE_DATA_CLASSES
        .into_iter()
        .map(String::from)
        .collect();
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| r.0 < 54 && r.0 % 6 == 4 && r.1 == 19000)
    {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let ObjectRecord::New { class, .. } = record(&fields, &root(kind)) else {
            panic!("expected complete plane data")
        };
        assert!(observed.insert(class.name.clone()));
    }
    assert_exact_plane_data_role(observed);
}

fn assert_exact_plane_data_role(observed: std::collections::BTreeSet<String>) {
    let expected: std::collections::BTreeSet<String> = super::reference_planes::CLASSES
        .iter()
        .map(|name| name.to_string())
        .collect();
    assert_eq!(observed, expected);
    assert_eq!(observed.len(), 18);
}
