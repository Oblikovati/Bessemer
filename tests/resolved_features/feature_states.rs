use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};
use super::{ModelFieldValue, encode_fields};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/feature-states/cases.bin"),
        b"FST1",
        42,
    )
}

#[test]
fn complete_feature_states_match_independent_signed_words_bytes_and_slots() {
    assert_complete_records(records());
}

#[test]
fn feature_states_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn feature_states_have_exactly_two_signed_words_without_an_inherited_prefix() {
    for (_, version, bytes, _, slots) in records().into_iter().filter(|r| r.0 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_eq!(fields.len(), 3);
        assert_eq!(fields[1].value, ModelFieldValue::I32(i32::MIN));
        assert_eq!(fields[2].value, ModelFieldValue::I32(0));
        assert_eq!(encode_fields(&fields[1..], 30000).unwrap().len(), 8);
        assert_eq!(slots, 3);
    }
}
