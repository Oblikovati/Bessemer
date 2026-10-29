use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_complete_records_with_kernel,
    assert_rejected_boundaries_with_kernel, records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/derived-feature-references/cases.bin"),
        b"DRF1",
        132,
    )
}

fn wrapper_records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/derived-reference-wrappers/cases.bin"),
        b"DRW1",
        180,
    )
}

pub(super) fn fresh_wrapper_class(target: u8, caller: u8, path: &str) -> String {
    let (_, version, bytes, _, _) = wrapper_records()
        .into_iter()
        .find(|r| r.0 == (target * 5 + caller) * 6 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let field = fields.iter().find(|f| f.path.as_ref() == path).unwrap();
    let super::ModelFieldValue::ObjectTag(tag) = &field.value else {
        panic!("expected typed reference tag")
    };
    let crate::archive_codec::ObjectRecord::New { class, .. } = &tag.record else {
        panic!("expected complete fresh reference")
    };
    assert_eq!(
        class.name,
        ["moDerivedFeatRef_w", "moSurfaceBodyRef_w"][usize::from(target)]
    );
    class.name.clone()
}

#[test]
fn complete_derived_and_surface_body_references_match_independent_primitives_bytes_and_slots() {
    assert_complete_records_with_kernel(records(), true);
}

#[test]
fn references_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}

#[test]
fn all_generic_callers_match_independent_reference_fields_bytes_and_slots() {
    assert_complete_records(wrapper_records());
}

#[test]
fn point_presence_is_absent_through_the_exact_version_boundary() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 / 6 == 0) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        assert_eq!(
            fields
                .iter()
                .any(|f| f.path.as_ref() == "root.object.point_presence"),
            version > 0x2f85
        );
        let present = version > 0x2f85 && !matches!(kind % 6, 0 | 4);
        assert_eq!(
            fields
                .iter()
                .filter(|f| f.path.starts_with("root.object.point_d8["))
                .count(),
            if present { 3 } else { 0 }
        );
    }
}

#[test]
fn nullable_data_roles_reject_an_unrelated_cached_reference() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 0 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    for path in ["plane_data_c0", "axis_data_c8", "coordinate_data_d0"] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.{path}"),
            &2_u16.to_le_bytes(),
        );
        assert!(decode_graph(version, &changed, true).is_err());
    }
}
