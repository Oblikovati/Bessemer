use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/mitered-flanges/cases.bin"),
        b"MTF1",
        54,
    )
}

#[test]
fn complete_mitered_flanges_match_independent_fields_bytes_and_map_slots() {
    assert_complete_records(records());
}

#[test]
fn mitered_flanges_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn each_direct_handle_group_rejects_negative_and_impossible_signed_counts() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 0 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    for offset in ["9f0", "a10", "a30", "a50"] {
        for count in [-1_i32, 2147483647] {
            let path = format!("root.object.payload.handles_{offset}.count");
            let changed =
                super::manager_context_tests::replace_field(&fields, &path, &count.to_le_bytes());
            assert!(decode_graph(version, &changed, false).is_err());
        }
    }
}

#[test]
fn mitered_profile_and_dimension_roles_reject_the_cached_enclosing_feature() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 0 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    for suffix in ["profile_918", "dimension_c38", "dimension_c40"] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.payload.{suffix}"),
            &2_u16.to_le_bytes(),
        );
        assert!(decode_graph(version, &changed, false).is_err());
    }
}

#[test]
fn mitered_profile_rejects_a_complete_profile_feature_instead_of_its_wrapper() {
    let bytes = include_bytes!("../fixtures/mitered-flanges/unrelated-profile-feature.bin");
    let Err(error) = decode_graph(19000, bytes, false) else {
        panic!("expected rejected complete profile feature")
    };
    assert_eq!(error.class, crate::FailureClass::Invalid);
    assert!(error.cause.contains("moProfileFeature_c"));
    assert!(error.cause.contains("moProfileRef_w"));
}
