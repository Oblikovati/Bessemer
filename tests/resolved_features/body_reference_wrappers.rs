use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/body-reference-wrappers/cases.bin"),
        b"BWR1",
        270,
    )
}

pub(super) fn fresh_wrapper_class(kind: u8, path: &str) -> String {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == kind * 6 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let field = fields.iter().find(|f| f.path.as_ref() == path).unwrap();
    let super::ModelFieldValue::ObjectTag(tag) = &field.value else {
        panic!("expected body-reference tag")
    };
    let crate::archive_codec::ObjectRecord::New { class, .. } = &tag.record else {
        panic!("expected fresh body reference")
    };
    assert_eq!(class.name, "moBodyRef_w");
    class.name.clone()
}

#[test]
fn body_reference_callers_match_independent_fields_slots_and_bytes() {
    assert_complete_records(records());
}

#[test]
fn body_reference_callers_reject_truncations_trailing_bytes_and_old_contexts() {
    assert_rejected_boundaries(records().into_iter().filter(|r| r.1 == 19000).collect());
}
