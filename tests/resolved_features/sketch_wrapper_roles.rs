use super::ModelFieldValue;
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};
use crate::archive_codec::ObjectRecord;
use std::collections::BTreeSet;

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/sketch-wrapper-roles/cases.bin"),
        b"SWR1",
        1188,
    )
}

#[test]
fn single_face_sketch_references_accept_fresh_and_cached_complete_bodies() {
    let cases = records()
        .into_iter()
        .filter(|r| [18, 19].contains(&r.0))
        .collect();
    assert_complete_records(cases);
}

#[test]
fn all_known_sketch_wrapper_graphs_match_independent_primitives_slots_and_bytes() {
    assert_complete_records(records());
}

#[test]
fn sketch_wrapper_graphs_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

fn fresh_reference_class(version: u32, bytes: &[u8]) -> String {
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let field = fields
        .iter()
        .find(|f| f.path.as_ref() == "root.object.reference")
        .unwrap();
    let ModelFieldValue::ObjectTag(pointer) = &field.value else {
        panic!("sketch reference: expected a typed object tag");
    };
    let ObjectRecord::New { class, .. } = &pointer.record else {
        panic!("sketch reference: expected a complete fresh body");
    };
    class.name.to_string()
}

#[test]
fn sketch_reference_coverage_equals_the_existing_finite_wrapper_family() {
    let actual: BTreeSet<_> = records()
        .into_iter()
        .filter(|r| r.0 % 6 == 0 && r.1 == 19000)
        .map(|(_, version, bytes, _, _)| fresh_reference_class(version, bytes))
        .collect();
    let expected: BTreeSet<_> = super::model_references::WRAPPER_CLASSES
        .iter()
        .map(|name| name.to_string())
        .collect();
    assert_eq!(actual, expected);
    assert_eq!(actual.len(), 18);
}

fn assert_invalid_array_role(version: u32, bytes: &[u8], path: &str) {
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let changed = super::manager_context_tests::replace_field(&fields, path, &2u16.to_le_bytes());
    let Err(error) = decode_graph(version, &changed, false) else {
        panic!("{path}: expected rejected cached suObArray");
    };
    assert_eq!(error.class, crate::FailureClass::Invalid);
}

#[test]
fn sketch_external_entity_pointer_roles_still_reject_cached_unrelated_arrays() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 2) {
        for label in ["reference", "view", "intersection", "isocurve"] {
            assert_invalid_array_role(version, bytes, &format!("root.object.items[0].{label}"));
        }
    }
}

#[test]
fn sketch_reference_rejects_a_complete_unrelated_fresh_node_name() {
    let bytes = include_bytes!("../fixtures/sketch-wrapper-roles/unrelated-fresh.bin");
    let Err(error) = decode_graph(19000, bytes, false) else {
        panic!("sgExtEnt.reference: expected rejected complete moNodeName_c body");
    };
    assert_eq!(error.class, crate::FailureClass::Invalid);
    assert!(error.cause.contains("moNodeName_c"));
}
