use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records_with_kernel, assert_rejected_boundaries_with_kernel,
    records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/mate-planes/cases.bin"),
        b"MPL1",
        312,
    )
}

#[test]
fn complete_mates_sections_edges_and_keepers_match_independent_fields_bytes_and_slots() {
    assert_complete_records_with_kernel(records(), true);
}

#[test]
fn mate_plane_records_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}

#[test]
fn mate_version_and_presence_gates_preserve_exact_native_boundaries() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 < 6) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        for (path, expected) in [
            ("mate.raw_380", version > 0x2bb4),
            ("mate.words_368.count", version > 0x3afc),
            ("mate.vector_3b8[0]", !matches!(kind, 0 | 4)),
            ("payload.raw_4c0", !matches!(kind, 0 | 4)),
        ] {
            assert_eq!(
                fields
                    .iter()
                    .any(|f| f.path.as_ref() == format!("root.object.{path}")),
                expected
            );
        }
    }
}

fn assert_bounded_counts(kind: u8, labels: &[&str]) {
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
fn mate_and_section_counts_reject_negative_or_impossible_work_before_iteration() {
    assert_bounded_counts(
        0,
        &[
            "mate.words_368.count",
            "mate.references_3c0.count",
            "payload.entities_408.count",
            "payload.lists_440.count",
        ],
    );
    assert_bounded_counts(
        6,
        &[
            "section_data.bodies.count",
            "edges.count",
            "map_8b8.count",
            "map_8c8.count",
            "map_8e8.count",
            "bodies_940.count",
            "visuals_960.count",
        ],
    );
}

#[test]
fn suppressed_section_and_edge_bodies_remain_explicitly_unsupported() {
    for (kind, path) in [
        (7, "section_data.bodies[0].body.suppressed"),
        (13, "bodies[0].suppressed"),
    ] {
        let (_, version, bytes, _, _) = records()
            .into_iter()
            .find(|r| r.0 == kind && r.1 == 19000)
            .unwrap();
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.{path}"),
            &[1],
        );
        let Err(error) = decode_graph(version, &changed, true) else {
            panic!("suppressed section body was accepted")
        };
        assert_eq!(error.class, crate::FailureClass::Unsupported);
        assert!(error.cause.contains("Suppressed"));
    }
}

#[test]
fn typed_mate_and_section_references_reject_unrelated_cached_roots() {
    for (kind, label) in [
        (1, "payload.array_428"),
        (1, "payload.entities_408[0]"),
        (1, "payload.lists_440[0]"),
        (1, "payload.data_468"),
        (1, "payload.data_460"),
        (1, "payload.data_470"),
        (7, "plane.plane_data"),
        (7, "edges[0]"),
        (7, "line_reference"),
        (13, "surface"),
    ] {
        let (_, version, bytes, _, _) = records()
            .into_iter()
            .find(|r| r.0 == kind && r.1 == 19000)
            .unwrap();
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.{label}"),
            &2_u16.to_le_bytes(),
        );
        assert!(decode_graph(version, &changed, true).is_err());
    }
}

#[test]
fn undocumented_populated_mate_data_is_not_treated_as_an_empty_record() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 0 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    let class = b"moMateBushingData_c";
    let mut framing = vec![255, 255, 1, 0];
    framing.extend_from_slice(&(class.len() as u16).to_le_bytes());
    framing.extend_from_slice(class);
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.payload.data_468",
        &framing,
    );
    let Err(error) = decode_graph(version, &changed, true) else {
        panic!("undocumented mate data was accepted")
    };
    assert_eq!(error.class, crate::FailureClass::Unsupported);
    assert!(error.cause.contains("moMateBushingData_c"));
}

#[test]
fn section_ordered_maps_keep_duplicate_keys_and_shared_values() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 7 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    for label in ["map_8c8", "map_8e8"] {
        let first = fields
            .iter()
            .find(|f| f.path.as_ref() == format!("root.object.{label}[0].key"))
            .unwrap();
        let second = fields
            .iter()
            .find(|f| f.path.as_ref() == format!("root.object.{label}[1].key"))
            .unwrap();
        assert_eq!(first.value, second.value);
    }
    let nulls = fields
        .iter()
        .filter(|f| f.path.starts_with("root.object.visuals_960["))
        .count();
    assert_eq!(nulls, 2);
}

#[test]
fn undocumented_section_body_references_fail_instead_of_accepting_empty_stubs() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 7 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    let class = b"moCompPlaneSectionBody_c";
    let mut framing = vec![255, 255, 1, 0];
    framing.extend_from_slice(&(class.len() as u16).to_le_bytes());
    framing.extend_from_slice(class);
    let changed =
        super::manager_context_tests::replace_field(&fields, "root.object.bodies_940[0]", &framing);
    let Err(error) = decode_graph(version, &changed, true) else {
        panic!("undocumented section body was accepted")
    };
    assert_eq!(error.class, crate::FailureClass::Unsupported);
    assert!(error.cause.contains("moCompPlaneSectionBody_c"));
}
