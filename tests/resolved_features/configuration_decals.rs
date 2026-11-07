use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};
use super::{ModelFieldValue, solid_reference_tests::decode_graph};
use crate::archive_codec::ObjectRecord;
fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/configuration-decals/cases.bin"),
        b"DCL1",
        360,
    )
}
#[test]
fn configuration_decals_match_independent_primitives_slots_and_reconstruction() {
    assert_complete_records(records());
}
#[test]
fn configuration_decals_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries(records());
}
#[test]
fn configuration_decal_minimum_properties_occupy_exactly_423_bytes() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 0) {
        assert_eq!(bytes.len(), 423 + 6 + b"moDecalProperties_c".len());
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_eq!(
            fields
                .iter()
                .find(|f| f.path.ends_with(".raw_48") && !f.path.contains("object_fields"))
                .unwrap()
                .value,
            ModelFieldValue::I32(0)
        );
        let text = &fields
            .iter()
            .find(|f| f.path.ends_with("object_fields.raw_48"))
            .unwrap()
            .value;
        assert!(matches!(text,ModelFieldValue::Text(value) if value.text().is_empty()));
    }
}
#[test]
fn configuration_decal_negative_index_counts_consume_no_indices() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| matches!(r.0 % 6, 1 | 3)) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let expected = if kind % 6 == 1 { -1 } else { i32::MIN };
        assert_eq!(
            fields
                .iter()
                .find(|f| f.path.ends_with(".index_count"))
                .unwrap()
                .value,
            ModelFieldValue::I32(expected)
        );
        assert!(!fields.iter().any(|f| f.path.contains(".indices[")));
    }
}
#[test]
fn configuration_decal_nested_fresh_classes_equal_the_complete_scoped_family() {
    let expected = [
        "moUnitComponent_c",
        "moCompFace_c",
        "moFaceRef_c",
        "moVertexRef_c",
        "moEndFaceSurfIdRep_c",
        "moNetSideSurfIdRep_c",
        "moFR_c",
        "moFromSktEntSurfIdRep_c",
        "moSimpleSurfIdRep_c",
        "moFromSktEnt3IntSurfIdRep_c",
    ];
    let mut observed = std::collections::BTreeSet::new();
    for (_, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| r.0 % 6 == 2 && r.1 == 19000)
    {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let selected = fields
            .iter()
            .find(|f| f.path.as_ref() == "root.object.list_08.items[0]")
            .unwrap();
        let ModelFieldValue::ObjectTag(pointer) = &selected.value else {
            panic!("expected fresh dependency");
        };
        let ObjectRecord::New { class, .. } = &pointer.record else {
            panic!("expected fresh class");
        };
        observed.insert(class.name.clone());
    }
    assert_eq!(observed, expected.into_iter().map(str::to_owned).collect());
}

#[test]
fn configuration_decal_fresh_scope_rejects_unverified_recursive_entity_classes() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 14 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let name = b"moEntityIdRep_c";
    let mut replacement = vec![0xff, 0xff, 1, 0];
    replacement.extend_from_slice(&(name.len() as u16).to_le_bytes());
    replacement.extend_from_slice(name);
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.list_08.items[0].entities[0].representation",
        &replacement,
    );
    let Err(error) = decode_graph(version, &changed, false) else {
        panic!("expected recursive scope rejection");
    };
    assert_eq!(error.operation, "configuration decal");
    assert_eq!(error.class, crate::FailureClass::Unsupported);
    assert!(error.cause.contains("moEntityIdRep_c") && error.cause.contains("expected"));
}
