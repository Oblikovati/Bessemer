use super::entity_representations::{RepresentationLayout, layout};
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

const CLASSES: [&str; 2] = ["moFromSktTopSurfIdRep_c", "moEndCapChamferSurfIdRep_c"];
fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/surface-identifier-variants/cases.bin"),
        b"SIV1",
        108,
    )
}

#[test]
fn native_surface_identifier_variants_accept_complete_typed_entity_owners() {
    assert_complete_records(
        records()
            .into_iter()
            .filter(|r| r.0 % 6 == 1 && r.1 == 19000)
            .collect(),
    );
}

#[test]
fn every_surface_identifier_graph_matches_independent_fields_slots_and_bytes() {
    assert_complete_records(records());
}

#[test]
fn surface_identifier_graphs_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn exact_native_classes_select_only_their_documented_signed_suffixes() {
    assert_eq!(
        layout(CLASSES[0]),
        Some(RepresentationLayout::Local(&["38"]))
    );
    assert_eq!(
        layout(CLASSES[1]),
        Some(RepresentationLayout::Local(&["38", "3c"]))
    );
    assert_eq!(layout("moFutureTopSurfIdRep_c"), None);
    assert_eq!(layout("moFutureEndCapIdRep_c"), None);
}

#[test]
fn minimum_identifier_bodies_have_only_two_base_pointers_and_signed_suffix_words() {
    for (kind, version, bytes, _, slots) in records().into_iter().filter(|r| r.0 % 6 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_eq!(
            super::encode_fields(&fields[1..], 30000).unwrap().len(),
            8 + usize::from(kind / 6) * 4
        );
        assert_eq!(fields.len(), 4 + usize::from(kind / 6));
        assert_eq!(slots, 3);
        assert!(!fields.iter().any(|f| f.path.contains("source_entity")));
    }
}

#[test]
fn cached_entity_representations_reject_their_unrelated_containing_array() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 3) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[1].representation",
            &2u16.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected unrelated array rejection")
        };
        assert!(error.to_string().contains("representation"));
    }
}

#[test]
fn cached_external_feature_references_reject_unrelated_arrays() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 2) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[0].external_feature_reference",
            &2u16.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected unrelated external-reference rejection")
        };
        assert!(error.to_string().contains("external_feature_reference"));
    }
}
