use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};
use super::{ModelField, ModelFieldValue};
use crate::archive_codec::ObjectRecord;

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/intersection-wrapper-roles/cases.bin"),
        b"IWR1",
        1824,
    )
}

fn sketch_path(style: u8) -> &'static str {
    match style {
        1 | 5 => "root.object.items[4]",
        2 => "root.object.items[0]",
        _ => "root.object",
    }
}

fn reference_path(style: u8, position: usize) -> String {
    let offset = ["10", "18", "20", "28"][position];
    format!(
        "{}.styled_intersections.intersections[1].reference_{offset}",
        sketch_path(style)
    )
}

fn object_tag<'a>(fields: &'a [ModelField], path: &str) -> &'a ObjectRecord {
    let field = fields.iter().find(|f| f.path.as_ref() == path).unwrap();
    let ModelFieldValue::ObjectTag(tag) = &field.value else {
        panic!("{path}: expected object tag")
    };
    &tag.record
}

#[test]
fn native_single_face_intersection_reference_regression_accepts_complete_sketch() {
    assert_complete_records(
        records()
            .into_iter()
            .filter(|r| r.0 == 18 && r.1 == 19000)
            .collect(),
    );
}

#[test]
fn every_complete_intersection_graph_matches_independent_fields_slots_and_bytes() {
    assert_complete_records(records());
}

#[test]
fn all_new_class_styles_reject_every_prefix_extra_byte_and_old_context() {
    // Existing complete sketch/wrapper corpora retain every inherited version-boundary check.
    assert_rejected_boundaries(records().into_iter().filter(|r| r.1 == 19000).collect());
}

fn observed_family(position: usize) -> std::collections::BTreeSet<String> {
    records()
        .into_iter()
        .filter(|r| r.0 % 6 == 0 && r.1 == 19000)
        .map(|(kind, version, bytes, _, _)| {
            let fields = decode_graph(version, bytes, false).unwrap().0;
            let ObjectRecord::New { class, .. } =
                object_tag(&fields, &reference_path(kind % 6, position))
            else {
                panic!("expected complete fresh wrapper")
            };
            class.name.clone()
        })
        .collect()
}

#[test]
fn each_of_the_four_fields_accepts_exactly_the_reviewed_nineteen_class_family() {
    let expected: std::collections::BTreeSet<String> = super::model_references::WRAPPER_CLASSES
        .iter()
        .map(|s| s.to_string())
        .collect();
    for position in 0..4 {
        let actual = observed_family(position);
        assert_eq!(actual, expected);
        assert_eq!(actual.len(), 19);
    }
}

#[test]
fn cached_wrapper_references_preserve_the_actual_seed_identities() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 1) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for position in 0..4 {
            assert_seed_identity(&fields, kind % 6, position);
        }
    }
}

fn assert_seed_identity(fields: &[ModelField], style: u8, position: usize) {
    let seed = object_tag(fields, &format!("root.object.items[{position}]"));
    let reference = object_tag(fields, &reference_path(style, position));
    let ObjectRecord::New {
        index: expected, ..
    } = seed
    else {
        panic!("expected fresh seed")
    };
    let ObjectRecord::Reference { index: actual, .. } = reference else {
        panic!("expected cached wrapper")
    };
    assert_eq!(actual, expected);
}

#[test]
fn nullable_intersection_fields_retain_all_four_null_positions() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| matches!(r.0 % 6, 3 | 4)) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for position in 0..4 {
            let null = (kind % 6 == 3 && position % 2 == 1) || (kind % 6 == 4 && position % 2 == 0);
            let actual = object_tag(&fields, &reference_path(kind % 6, position));
            assert_eq!(matches!(actual, ObjectRecord::Null), null);
        }
    }
}

const UNRELATED: [&[u8]; 4] = [
    include_bytes!("../fixtures/intersection-wrapper-roles/unrelated-0.bin"),
    include_bytes!("../fixtures/intersection-wrapper-roles/unrelated-1.bin"),
    include_bytes!("../fixtures/intersection-wrapper-roles/unrelated-2.bin"),
    include_bytes!("../fixtures/intersection-wrapper-roles/unrelated-3.bin"),
];

#[test]
fn complete_fresh_unrelated_objects_are_rejected_at_all_four_fields() {
    for bytes in UNRELATED {
        let Err(error) = decode_graph(19000, bytes, false) else {
            panic!("expected unrelated wrapper rejection")
        };
        assert_eq!(error.class, crate::FailureClass::Invalid);
    }
}

#[test]
fn complete_cached_unrelated_arrays_are_rejected_at_all_four_fields() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| r.0 % 6 == 2 && r.1 == 19000)
    {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for position in 0..4 {
            let changed = super::manager_context_tests::replace_field(
                &fields,
                &reference_path(kind % 6, position),
                &2u16.to_le_bytes(),
            );
            let Err(error) = decode_graph(version, &changed, false) else {
                panic!("expected cached array rejection")
            };
            assert_eq!(error.class, crate::FailureClass::Invalid);
        }
    }
}
