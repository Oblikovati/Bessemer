use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records_with_kernel, assert_rejected_boundaries_with_kernel,
    records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/library-features/cases.bin"),
        b"LIB1",
        756,
    )
}

#[test]
fn complete_library_graphs_match_independent_fields_bytes_and_slots() {
    assert_complete_records_with_kernel(records(), true);
}

#[test]
fn library_graphs_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}

#[test]
fn library_feature_gates_match_native_version_boundaries() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 0) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        for (label, expected) in [
            ("raw_dc0", version > 0x2b5c),
            ("raw_dd4", version > 0x2b5f),
            ("component_map_e18.count", version > 0x2b60),
            (
                "legacy_atom_ids.ids.count",
                version > 0x2b7b && version < 0x2bac,
            ),
            ("atom_nodes.count", version > 0x2bab),
            ("raw_e10", version > 0x42df),
        ] {
            assert_eq!(
                fields
                    .iter()
                    .any(|f| f.path.as_ref() == format!("root.object.{label}")),
                expected
            );
        }
    }
}

fn reject_counts(kind: u8, labels: &[&str]) {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == kind && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    for label in labels {
        for count in [-1_i32, i32::MAX] {
            let changed = super::manager_context_tests::replace_field(
                &fields,
                &format!("root.object.{label}"),
                &count.to_le_bytes(),
            );
            assert!(decode_graph(version, &changed, true).is_err());
        }
    }
}

#[test]
fn library_signed_counts_bound_nested_work_before_iteration() {
    reject_counts(1, &["component_map_e18.count", "atom_nodes.count"]);
    reject_counts(7, &["features_70.count"]);
    reject_counts(
        37,
        &[
            "ids.count",
            "map_18.count",
            "map_18[0].values.count",
            "bodies_28.count",
        ],
    );
}

#[test]
fn library_typed_references_reject_cached_unrelated_roots() {
    for (kind, path) in [
        (0, "plane_da0"),
        (1, "atom_nodes[0]"),
        (7, "features_70[0].feature"),
        (25, "reference_10"),
        (31, "ids_10"),
        (37, "bodies_28[0]"),
    ] {
        let (_, version, bytes, _, _) = records()
            .into_iter()
            .find(|r| r.0 == kind && r.1 == 19000)
            .unwrap();
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.{path}"),
            &2_u16.to_le_bytes(),
        );
        assert!(decode_graph(version, &changed, true).is_err());
    }
}
