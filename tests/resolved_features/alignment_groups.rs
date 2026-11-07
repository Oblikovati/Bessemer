use super::ModelFieldValue;
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records_with_kernel, assert_rejected_boundaries_with_kernel,
    records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/alignment-groups/cases.bin"),
        b"ALG1",
        612,
    )
}
#[test]
fn alignment_graphs_match_independent_fields_bytes_and_map_slots() {
    assert_complete_records_with_kernel(records(), true);
}
#[test]
fn alignment_graphs_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}
#[test]
fn alignment_version_gates_preserve_legacy_handles_and_optional_points() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 < 6) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        for (label, expected) in [
            ("raw_3f4", version > 0x2f4d),
            ("legacy_handles_450[0]", (0x2f50..0x2fa0).contains(&version)),
            ("handles_450.count.count_word", version >= 0x2fa0),
            ("point_4d8[0]", version >= 0x2fa0 && !matches!(kind, 0 | 4)),
        ] {
            assert_eq!(
                fields
                    .iter()
                    .any(|f| f.path.as_ref() == format!("root.object.payload.{label}")),
                expected,
                "kind {kind}, version {version:#x}, {label}"
            );
        }
    }
}
#[test]
fn alignment_second_handle_array_enforces_native_limit_before_entries() {
    let (_, v, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 2 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(v, bytes, true).unwrap().0;
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.payload.handles_450.count.count_word",
        &3_u16.to_le_bytes(),
    );
    let Err(error) = decode_graph(v, &changed, true) else {
        panic!("three handles accepted")
    };
    assert!(error.cause.contains("expected <= 2"));
}
#[test]
fn alignment_typed_references_reject_cached_unrelated_roots() {
    for (kind, label) in [
        (1, "payload.handles_370.items[0]"),
        (1, "payload.component_4c0"),
        (1, "payload.entity_4a8"),
        (7, "suffix.list_ff8"),
        (7, "suffix.component_1000"),
        (7, "suffix.reference_fb8"),
        (7, "suffix.references_fc8[0]"),
    ] {
        let (_, v, bytes, _, _) = records()
            .into_iter()
            .find(|r| r.0 == kind && r.1 == 19000)
            .unwrap();
        let fields = decode_graph(v, bytes, true).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.{label}"),
            &2_u16.to_le_bytes(),
        );
        assert!(decode_graph(v, &changed, true).is_err());
    }
}
#[test]
fn text_kernel_frames_require_schema_while_suppression_preserves_its_byte() {
    for (kind, v, bytes, _, _) in records().into_iter().filter(|r| r.0 >= 30 && r.1 == 19000) {
        if matches!(kind, 32 | 35) {
            let Err(error) = decode_graph(v, bytes, false) else {
                panic!("compressed text accepted without schema")
            };
            assert_eq!(error.class, crate::FailureClass::Missing);
            continue;
        }
        let fields = decode_graph(v, bytes, false).unwrap().0;
        if kind == 30 {
            continue;
        }
        let field = fields
            .iter()
            .find(|f| f.path.ends_with("kernel_body.suppressed"))
            .unwrap();
        assert_eq!(
            field.value,
            ModelFieldValue::U8(if kind == 31 {
                1
            } else if kind == 33 {
                255
            } else {
                128
            })
        );
    }
}

#[test]
fn alignment_layouts_keep_sheet_metal_helper_850_independent_of_profile_834() {
    for (_, v, bytes, _, slots) in records()
        .into_iter()
        .filter(|r| r.0 % 6 == 2 && r.1 == 19000)
    {
        let mut reader = super::ModelDecoder::new_in_context(
            bytes,
            crate::header::ArchiveVersion::new(v),
            super::annotation_view_tests::LIMITS,
            super::reader::ModelArchiveContext::OrdinaryPartExternalXml,
        )
        .unwrap()
        .with_kernel(Some(super::kernel_frame_tests::context()))
        .with_saved_helper_state(super::configuration_manager::SavedHelperState::Active);
        let mut map = crate::archive_codec::ObjectReader::new();
        reader
            .object("object", &mut map, super::ModelObjectRole::Any)
            .unwrap();
        assert_eq!(map.into_snapshot().slot_count(), slots);
        assert_eq!(
            super::encode_fields(&reader.finish().unwrap(), 30000).unwrap(),
            bytes
        );
    }
}
