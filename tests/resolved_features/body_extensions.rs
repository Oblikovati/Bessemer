use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records_with_kernel, assert_rejected_boundaries_with_kernel,
    records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/body-extensions/cases.bin"),
        b"BEX1",
        270,
    )
}

#[test]
fn complete_extension_bodies_match_independent_fields_bytes_and_map_slots() {
    assert_complete_records_with_kernel(records(), true);
}

#[test]
fn extension_bodies_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}

#[test]
fn cavity_reference_roles_reject_a_cached_unrelated_root() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 1 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    for name in ["face_838", "coordinate_848"] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.payload.{name}"),
            &2_u16.to_le_bytes(),
        );
        assert!(decode_graph(version, &changed, true).is_err());
    }
}

#[test]
fn sketch_hole_retains_complete_extrusion_suffix_without_extra_fields() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 6) {
        assert_eq!(kind, 6);
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let last = fields.last().unwrap();
        assert_eq!(last.path.as_ref(), "root.object.extrusion_suffix.word_808");
        assert_eq!(last.value, super::ModelFieldValue::I32(i32::MIN));
    }
}
