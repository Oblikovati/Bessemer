use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/sketch-bends/cases.bin"),
        b"SKB1",
        66,
    )
}

#[test]
fn complete_sketch_bends_match_independent_fields_bytes_and_shared_slots() {
    assert_complete_records(records());
}

#[test]
fn sketch_bends_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn kind_seven_stops_before_consuming_any_derived_owned_payload() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 0 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.one_bend.raw_ac8",
        &7_i32.to_le_bytes(),
    );
    for end in [changed.len() - 14, changed.len()] {
        let Err(error) = decode_graph(version, &changed[..end], false) else {
            panic!("expected kind-7 owned-data stop")
        };
        assert_eq!(error.class, crate::FailureClass::Unsupported);
        assert!(error.cause.contains("kind 7"));
    }
}

#[test]
fn sketch_bend_curve_role_rejects_the_cached_enclosing_feature() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 0 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.payload.curve_de0",
        &2_u16.to_le_bytes(),
    );
    assert!(decode_graph(version, &changed, false).is_err());
}
