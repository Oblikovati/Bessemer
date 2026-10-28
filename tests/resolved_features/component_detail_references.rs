use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/component-detail-references/cases.bin"),
        b"DRF1",
        108,
    )
}

#[test]
fn native_component_detail_regression_accepts_the_complete_null_component_branch() {
    assert_complete_records(
        records()
            .into_iter()
            .filter(|r| r.0 == 6 && r.1 == 19000)
            .collect(),
    );
}

#[test]
fn complete_detail_graphs_match_independent_primitives_slots_and_bytes() {
    assert_complete_records(records());
}

#[test]
fn detail_graphs_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn negative_detail_integer_counts_never_publish_partial_graphs() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.raw_values_60.count",
            &(-1i32).to_le_bytes(),
        );
        assert!(decode_graph(version, &changed, false).is_err());
    }
}

#[test]
fn component_detail_targets_reject_cached_unrelated_arrays() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 9) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[1].detail_e8",
            &2u16.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected unrelated detail rejection")
        };
        assert!(error.to_string().contains("detail_e8"));
    }
}

#[test]
fn detail_attachment_roles_reject_cached_unrelated_arrays() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 3) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[1].attachments_50[0]",
            &2u16.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected unrelated attachment rejection")
        };
        assert!(error.to_string().contains("attachments_50"));
    }
}
