use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records_with_kernel, assert_rejected_boundaries_with_kernel,
    records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/curve-files/cases.bin"),
        b"CRF1",
        42,
    )
}

#[test]
fn complete_curve_files_match_independent_fields_bytes_kernel_and_map_slots() {
    assert_complete_records_with_kernel(records(), true);
}

#[test]
fn curve_files_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}

#[test]
fn signed_counts_preserve_nonpositive_empty_lists_and_exact_pointer_gate() {
    for (kind, version, bytes, _, _) in records() {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let count = fields
            .iter()
            .filter(|f| f.path.starts_with("root.object.payload.points["))
            .count();
        assert_eq!(count, [0, 0, 6, 9, 0, 3][usize::from(kind)]);
        assert_eq!(
            fields
                .iter()
                .any(|f| f.path.as_ref() == "root.object.payload.reference_798"),
            version > 0x42cd
        );
    }
}

#[test]
fn curve_file_counts_are_bounded_before_point_iteration() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 0 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.payload.declared_count",
        &2147483647_i32.to_le_bytes(),
    );
    assert!(decode_graph(version, &changed, true).is_err());
}

#[test]
fn curve_file_point_reference_rejects_an_unrelated_cached_curve() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 0 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.payload.reference_798",
        &2_u16.to_le_bytes(),
    );
    assert!(decode_graph(version, &changed, true).is_err());
}
