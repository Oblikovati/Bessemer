use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records_with_kernel, assert_rejected_boundaries_with_kernel,
    records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/mid-surfaces/cases.bin"),
        b"MSF1",
        432,
    )
}

#[test]
fn complete_mid_surfaces_match_independent_fields_bytes_and_map_slots() {
    assert_complete_records_with_kernel(records(), true);
}

#[test]
fn mid_surfaces_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}

#[test]
fn mid_reference_extra_body_requires_presence_exactly_one() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| (12..18).contains(&r.0)) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let present = fields
            .iter()
            .any(|f| f.path.starts_with("root.object.payload.body_7b0"));
        assert_eq!(present, matches!(kind % 6, 1 | 5));
    }
}

#[test]
fn mid_reference_face_arrays_preserve_nonpositive_signed_counts() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| (12..18).contains(&r.0)) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let count = [0_i32, -1, 2, 1, i32::MIN, 3][usize::from(kind % 6)];
        for offset in ["7e8", "7f8"] {
            let prefix = format!("root.object.payload.faces_{offset}");
            let field = fields
                .iter()
                .find(|f| f.path.as_ref() == format!("{prefix}.declared_count"))
                .unwrap();
            assert_eq!(field.value, super::ModelFieldValue::I32(count));
            assert_eq!(
                fields
                    .iter()
                    .filter(|f| f
                        .path
                        .strip_prefix(&format!("{prefix}.items["))
                        .is_some_and(|tail| !tail.contains('.')))
                    .count(),
                count.max(0) as usize
            );
        }
    }
}

#[test]
fn huge_external_sheet_count_is_metadata_without_inline_sheet_fields() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 10 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    let count = fields
        .iter()
        .find(|f| f.path.as_ref() == "root.object.surface.sheet_count")
        .unwrap();
    assert_eq!(count.value, super::ModelFieldValue::I32(i32::MAX));
    assert!(
        !fields
            .iter()
            .any(|f| f.path.starts_with("root.object.surface.sheets["))
    );
    assert!(
        fields
            .iter()
            .any(|f| f.path.starts_with("root.object.surface.extra_body."))
    );
    let changed =
        super::manager_context_tests::replace_field(&fields, &count.path, &(-1_i32).to_le_bytes());
    assert!(decode_graph(version, &changed, true).is_err());
}

#[test]
fn signed_face_counts_are_bounded_before_iteration() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 12 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.payload.faces_7e8.declared_count",
        &i32::MAX.to_le_bytes(),
    );
    assert!(decode_graph(version, &changed, true).is_err());
}

#[test]
fn mid_surface_typed_slots_reject_unrelated_cached_roots() {
    for (kind, path) in [
        (13, "payload.data_808"),
        (25, "face_08"),
        (25, "face_10"),
        (25, "dimension_18"),
        (31, "face_08"),
        (37, "edge_08"),
        (37, "edge_10"),
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

#[test]
fn edge_pairs_retain_fresh_and_cached_silhouette_edge_subtypes() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 41 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    for (label, fresh) in [("edge_08", true), ("edge_10", false)] {
        let field = fields
            .iter()
            .find(|f| f.path.as_ref() == format!("root.object.{label}"))
            .unwrap();
        let super::ModelFieldValue::ObjectTag(tag) = &field.value else {
            panic!("edge slot lost tagged framing")
        };
        match &tag.record {
            crate::archive_codec::ObjectRecord::New { class, .. } => {
                assert!(fresh);
                assert_eq!(class.name, "moCompSilhEdge_c");
            }
            crate::archive_codec::ObjectRecord::Reference { class, .. } => {
                assert!(!fresh);
                assert_eq!(class.name, "moCompSilhEdge_c");
            }
            _ => panic!("silhouette edge unexpectedly became null"),
        }
    }
}

#[test]
fn mid_reference_keeps_inline_sheets_with_a_nonzero_backend_marker() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 15 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    for (label, expected) in [("archive_backend_marker", i32::MIN), ("sheet_count", 2)] {
        let field = fields
            .iter()
            .find(|f| f.path.as_ref() == format!("root.object.surface.{label}"))
            .unwrap();
        assert_eq!(field.value, super::ModelFieldValue::I32(expected));
    }
    for index in 0..2 {
        assert!(fields.iter().any(|f| {
            f.path
                .starts_with(&format!("root.object.surface.sheets[{index}].body."))
        }));
    }
}
