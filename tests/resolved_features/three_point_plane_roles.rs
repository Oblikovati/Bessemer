use super::ModelFieldValue;
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};
use crate::archive_codec::ObjectRecord;

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/three-point-plane-roles/cases.bin"),
        b"TPP1",
        78,
    )
}

fn prefix(style: u8) -> &'static str {
    match style {
        2 => "root.object.items[1]",
        4 => "root.object.plane_data",
        5 => "root.object.items[1].plane_data",
        _ => "root.object",
    }
}

#[test]
fn all_three_point_planes_accept_complete_finite_point_family_bodies() {
    assert_complete_records(records());
}

#[test]
fn three_point_graphs_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn all_three_pointer_occurrences_remain_present_including_nulls() {
    for (style, version, bytes, _, _) in records() {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for offset in ["88", "90", "98"] {
            let path = format!("{}.point_reference_{offset}", prefix(style));
            let field = fields.iter().find(|f| f.path.as_ref() == path).unwrap();
            let ModelFieldValue::ObjectTag(tag) = &field.value else {
                panic!("{path}: expected tagged point")
            };
            assert_eq!(matches!(tag.record, ObjectRecord::Null), style == 0);
        }
    }
}

#[test]
fn complete_unrelated_fresh_and_cached_wrappers_still_fail_at_the_point_role() {
    for bytes in [
        include_bytes!("../fixtures/three-point-plane-roles/unrelated-fresh.bin").as_slice(),
        include_bytes!("../fixtures/three-point-plane-roles/unrelated-cached.bin").as_slice(),
    ] {
        let Err(error) = decode_graph(19000, bytes, false) else {
            panic!("expected rejected complete unrelated wrapper")
        };
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(
            error.cause.contains("point_reference_88") && error.cause.contains("moRefWrapper_c")
        );
    }
}

#[test]
fn each_point_occurrence_rejects_the_cached_enclosing_array() {
    for (style, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 2 || r.0 == 5) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for offset in ["88", "90", "98"] {
            let path = format!("{}.point_reference_{offset}", prefix(style));
            let changed =
                super::manager_context_tests::replace_field(&fields, &path, &2u16.to_le_bytes());
            let Err(error) = decode_graph(version, &changed, false) else {
                panic!("expected rejected array at {path}")
            };
            assert_eq!(error.class, crate::FailureClass::Invalid);
        }
    }
}

#[test]
fn cached_endpoint_and_loft_points_keep_the_actual_earlier_identity() {
    for (style, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 2 || r.0 == 5) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_seed_identity(&fields, style);
    }
}

fn point_role_tag<'a>(fields: &'a [super::ModelField], path: &str) -> &'a ObjectRecord {
    let field = fields.iter().find(|f| f.path.as_ref() == path).unwrap();
    let ModelFieldValue::ObjectTag(tag) = &field.value else {
        panic!("{path}: expected point tag")
    };
    &tag.record
}

fn assert_seed_identity(fields: &[super::ModelField], style: u8) {
    let pair = (
        point_role_tag(fields, "root.object.items[0]"),
        point_role_tag(fields, &format!("{}.point_reference_88", prefix(style))),
    );
    let (ObjectRecord::New { index: first, .. }, ObjectRecord::Reference { index: next, .. }) =
        pair
    else {
        panic!("expected a fresh seed and its actual cached point target")
    };
    assert_eq!(first, next);
}
