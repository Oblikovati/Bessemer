use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_from,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_from(
        include_bytes!("../fixtures/frontier-edge-identifiers/cases.bin"),
        b"FEI1",
    )
}

#[test]
fn frontier_edges_match_independent_primitives_slots_and_bytes() {
    assert_complete_records(records());
}

#[test]
fn frontier_edges_reject_all_truncated_appended_and_old_contexts() {
    assert_rejected_boundaries(records());
}

#[test]
fn frontier_edge_minima_preserve_direct_member_map_accounting() {
    for (kind, version, bytes, _, slots) in records().into_iter().filter(|row| row.0 % 6 == 0) {
        let name_length = usize::from(u16::from_le_bytes([bytes[4], bytes[5]]));
        let minimum = if kind < 6 { 12 } else { 14 };
        assert_eq!(bytes.len() - 6 - name_length, minimum);
        assert_eq!(slots, 3);
        assert_eq!(decode_graph(version, bytes, false).unwrap().1, 3);
    }
}

#[test]
fn frontier_edge_pointer_roles_reject_unrelated_cached_array() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|row| row.0 % 6 == 2) {
        for label in ["feature", "external_feature_reference"] {
            let path = format!("root.object.items[0].{label}");
            assert_wrong_role(version, bytes, &path);
        }
        if kind >= 6 {
            assert_wrong_role(
                version,
                bytes,
                "root.object.items[0].source_entity.representation",
            );
        }
    }
}

fn assert_wrong_role(version: u32, bytes: &[u8], path: &str) {
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let changed = super::manager_context_tests::replace_field(&fields, path, &2u16.to_le_bytes());
    let Err(error) = decode_graph(version, &changed, false) else {
        panic!("{path}: expected rejected cached suObArray");
    };
    assert_eq!(error.class, crate::FailureClass::Invalid);
}

#[test]
fn frontier_edges_use_only_exact_native_bound_literal_layouts() {
    use super::entity_representations::{RepresentationLayout, layout};
    for (name, expected) in [
        (
            "moTrimWireExtEdgeIdRep_c",
            RepresentationLayout::Local(&["38", "3c"]),
        ),
        (
            "moPttnBodyEdgeIdRep_c",
            RepresentationLayout::Source(&["48", "4c"]),
        ),
    ] {
        assert_eq!(layout(name), Some(expected));
        assert_eq!(layout(&format!("{name}_unknown")), None);
    }
}

#[test]
fn frontier_edge_tagged_entity_owners_reject_array_representations() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 % 6 == 5) {
        assert_wrong_role(version, bytes, "root.object.items[0].representation");
    }
}
