use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records_with_kernel, assert_rejected_boundaries_with_kernel,
    records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/stock-features/cases.bin"),
        b"STK1",
        216,
    )
}

#[test]
fn stock_graphs_match_complete_independent_primitives_bytes_and_slots() {
    assert_complete_records_with_kernel(records(), true);
}

#[test]
fn stock_graphs_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}

#[test]
fn stock_hole_records_move_between_layers_at_the_exact_version_gate() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| r.0 / 6 < 2 && r.0 % 6 == 1)
    {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        assert_eq!(
            fields
                .iter()
                .any(|f| f.path.starts_with("root.object.derived.records_950")),
            version >= 0x373c,
            "{kind}/{version:#x}"
        );
        assert_eq!(
            fields
                .iter()
                .any(|f| f.path.starts_with("root.object.late.legacy_hole_info_950")),
            0x3335 < version && version < 0x373c
        );
    }
}

#[test]
fn serialized_stock_and_reference_bodies_require_explicit_kernel_context() {
    for (_, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| r.0 / 6 < 3 && r.0 % 6 == 2)
    {
        assert!(decode_graph(version, bytes, false).is_err());
        assert!(decode_graph(version, bytes, true).is_ok());
    }
}

fn edited_field(kind: u8, path: &str, replacement: &[u8]) -> Vec<u8> {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == kind && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    super::manager_context_tests::replace_field(&fields, path, replacement)
}

#[test]
fn stock_dynamic_width_rejects_unaligned_values_without_a_partial_result() {
    let changed = edited_field(
        0,
        "root.object.late.array_de8.element_width",
        &3_u32.to_le_bytes(),
    );
    assert!(decode_graph(19000, &changed, true).is_err());
}

#[test]
fn body_reference_rejects_negative_and_impossible_body_counts() {
    for count in [i32::MIN, -1, i32::MAX] {
        let changed = edited_field(14, "root.object.bodies.count", &count.to_le_bytes());
        assert!(decode_graph(19000, &changed, true).is_err());
    }
}

#[test]
fn stock_fixed_operator_rejects_perspective_words() {
    let changed = edited_field(
        0,
        "root.object.transform_c30.words[28]",
        &0x10_u32.to_le_bytes(),
    );
    let Err(error) = decode_graph(19000, &changed, true) else {
        panic!("expected unsupported fixed perspective")
    };
    assert_eq!(error.class, crate::FailureClass::Unsupported);
    assert!(error.cause.contains("0x00000010"));
}

#[test]
fn component_body_retains_all_ten_box_doubles() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 / 6 == 3) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        assert_eq!(
            fields
                .iter()
                .filter(|f| f.path.starts_with("root.object.box_100["))
                .count(),
            10
        );
        assert!(
            fields
                .iter()
                .any(|f| f.path.as_ref() == "root.object.box_100[9]")
        );
    }
}
