use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/end-spec-wrapper-roles/cases.bin"),
        b"EWR1",
        1140,
    )
}

#[test]
fn native_solid_wrapper_end_reference_regression_accepts_its_complete_body() {
    let selected = records()
        .into_iter()
        .filter(|r| r.0 == 78 && r.1 == 19000)
        .collect();
    assert_complete_records(selected);
}

use super::solid_reference_tests::decode_graph;
use super::{ModelField, ModelFieldValue};
use crate::archive_codec::ObjectRecord;

fn reference_path(style: u8) -> &'static str {
    match style {
        0 => "root.object.reference_78",
        1 => "root.object.items[1].extrusion_suffix.end_spec.reference_78",
        2 => "root.object.reference",
        3 => "root.object.items[1].reference",
        4 => "root.object.reference_118",
        5 => "root.object.items[1].revolution_suffix.end_spec.reference_118",
        _ => panic!("style {style}; expected 0..=5"),
    }
}

fn object_tag<'a>(fields: &'a [ModelField], path: &str) -> &'a ObjectRecord {
    let field = fields.iter().find(|f| f.path.as_ref() == path).unwrap();
    let ModelFieldValue::ObjectTag(tag) = &field.value else {
        panic!("{path}: expected object tag")
    };
    &tag.record
}

#[test]
fn all_end_callers_match_independent_primitives_slots_and_bytes_for_all_wrappers() {
    assert_complete_records(records());
}

#[test]
fn all_new_caller_class_styles_reject_every_prefix_extra_byte_and_old_context() {
    // Existing wrapper/body corpora retain their exhaustive version-boundary checks.
    assert_rejected_boundaries(records().into_iter().filter(|r| r.1 == 19000).collect());
}

#[test]
fn each_caller_fixture_set_equals_the_exact_twenty_class_wrapper_family() {
    use std::collections::BTreeSet;
    let expected: BTreeSet<_> = super::model_references::WRAPPER_CLASSES
        .iter()
        .map(|s| s.to_string())
        .collect();
    for style in [0, 2, 4] {
        let mut actual = observed_wrapper_family(style);
        actual.insert(super::cone_reference_tests::fresh_wrapper_class(
            style / 2,
            reference_path(style),
        ));
        assert_eq!(actual, expected);
        assert_eq!(actual.len(), 20);
    }
}

fn observed_wrapper_family(style: u8) -> std::collections::BTreeSet<String> {
    let mut classes = std::collections::BTreeSet::new();
    for (_, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| r.1 == 19000 && r.0 % 6 == style)
    {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let ObjectRecord::New { class, .. } = object_tag(&fields, reference_path(style)) else {
            panic!("expected complete fresh wrapper")
        };
        classes.insert(class.name.clone());
    }
    classes
}

#[test]
fn cached_references_keep_the_actual_seeded_target_through_complete_parent_features() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 2 == 1) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let first = object_tag(&fields, "root.object.items[0]");
        let next = object_tag(&fields, reference_path(kind % 6));
        let (
            ObjectRecord::New {
                index: expected, ..
            },
            ObjectRecord::Reference { index: actual, .. },
        ) = (first, next)
        else {
            panic!("expected a fresh seed and cached actual reference")
        };
        assert_eq!(expected, actual);
    }
}

#[test]
fn all_three_caller_kinds_preserve_null_reference_occurrences() {
    for (style, bytes) in [
        (
            0,
            include_bytes!("../fixtures/end-spec-wrapper-roles/null-0.bin").as_slice(),
        ),
        (
            2,
            include_bytes!("../fixtures/end-spec-wrapper-roles/null-2.bin").as_slice(),
        ),
        (
            4,
            include_bytes!("../fixtures/end-spec-wrapper-roles/null-4.bin").as_slice(),
        ),
    ] {
        assert_null_reference(style, bytes);
    }
}

fn assert_null_reference(style: u8, bytes: &[u8]) {
    let fields = decode_graph(19000, bytes, false).unwrap().0;
    assert!(matches!(
        object_tag(&fields, reference_path(style)),
        ObjectRecord::Null
    ));
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
}

#[test]
fn complete_unrelated_fresh_and_cached_node_names_remain_rejected() {
    for bytes in [
        include_bytes!("../fixtures/end-spec-wrapper-roles/unrelated-0.bin").as_slice(),
        include_bytes!("../fixtures/end-spec-wrapper-roles/unrelated-1.bin").as_slice(),
        include_bytes!("../fixtures/end-spec-wrapper-roles/unrelated-2.bin").as_slice(),
        include_bytes!("../fixtures/end-spec-wrapper-roles/unrelated-3.bin").as_slice(),
        include_bytes!("../fixtures/end-spec-wrapper-roles/unrelated-4.bin").as_slice(),
        include_bytes!("../fixtures/end-spec-wrapper-roles/unrelated-5.bin").as_slice(),
    ] {
        let Err(error) = decode_graph(19000, bytes, false) else {
            panic!("expected rejected complete unrelated node name")
        };
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("moNodeName_c"));
    }
}
