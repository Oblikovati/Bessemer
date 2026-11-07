use super::{ModelArchiveLimits, context_snapshot_tests, manager_context_tests, resolved_features};
use crate::Document;

const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

#[test]
fn configuration_table_folder_uses_the_complete_common_folder_serializer() {
    use crate::archive_codec::{ArchiveClass, ObjectWriter};
    let mut bytes = Vec::new();
    ObjectWriter::new()
        .object(
            &ArchiveClass {
                name: "moConfigTableFolder_c".into(),
                schema: 1,
            },
            &mut bytes,
        )
        .unwrap();
    // The independent common-folder oracle in the reference tests fixes this body at 147 bytes.
    bytes.extend_from_slice(&[0; 139]);
    bytes.extend_from_slice(&(-7i32).to_le_bytes());
    bytes.extend_from_slice(&i32::MIN.to_le_bytes());
    let fields = super::annotation_view_tests::decode(19000, &bytes).unwrap();
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
    assert!(
        fields
            .iter()
            .any(|field| field.path.as_ref() == "root.object.unknown_358"
                && field.value == super::ModelFieldValue::I32(-7))
    );
    for length in 0..bytes.len() {
        assert!(super::annotation_view_tests::decode(19000, &bytes[..length]).is_err());
    }
    assert!(super::annotation_view_tests::decode(19000, &[bytes, vec![0]].concat()).is_err());
}

#[test]
fn complete_owned_feature_stream_reconstructs_with_the_actual_inherited_map_and_mbsm_tail() {
    let source = context_snapshot_tests::owned_configuration();
    let manager = manager_context_tests::decode(&manager_context_tests::manager_bytes()).unwrap();
    let doc = Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
    let bytes = doc
        .read_stream("Contents/Config-0-ResolvedFeatures")
        .unwrap();
    let parsed = resolved_features::parse(&bytes, &source, &manager, LIMITS).unwrap();
    assert_eq!(bytes.len(), 5558);
    assert_eq!(parsed.roots, 17);
    assert_eq!(parsed.final_map.slot_count(), 161);
    let rebuilt = super::encode_fields(&parsed.fields, LIMITS.maximum_bytes).unwrap();
    assert_eq!(rebuilt, bytes);
    let forced = resolved_features::parse(&rebuilt, &source, &manager, LIMITS).unwrap();
    assert_eq!(forced.fields, parsed.fields);
    assert_eq!(source.parsed.object_map.slot_count(), 97);
    super::manager_oracle_tests::assert_reference_fields(
        &parsed.fields,
        &bytes,
        include_str!("../fixtures/resolved-features/owned-features-primitives.tsv"),
        1518,
    );
}

#[test]
fn plane_name_edit_matches_independent_bytes_and_rejects_wrong_field_paths() {
    let source = context_snapshot_tests::owned_configuration();
    let manager = manager_context_tests::decode(&manager_context_tests::manager_bytes()).unwrap();
    let original = include_bytes!("../fixtures/resolved-features/owned-features.bin");
    let parsed = resolved_features::parse(original, &source, &manager, LIMITS).unwrap();
    let mut fields = renamed_plane(&parsed.fields);
    let path = "root.features[13].model_feature.feature.node.name.name";
    let changed = resolved_features::construct(fields.clone(), &source, &manager, LIMITS).unwrap();
    let expected = include_bytes!("../fixtures/resolved-features/owned-features-plane-edited.bin");
    assert_eq!(
        super::encode_fields(&changed.fields, LIMITS.maximum_bytes).unwrap(),
        expected
    );
    assert_eq!(changed.roots, 17);
    assert_eq!(changed.final_map.slot_count(), 161);
    assert_eq!(
        super::encode_fields(&parsed.fields, LIMITS.maximum_bytes).unwrap(),
        original
    );
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .path = "wrong.plane.name".into();
    assert!(resolved_features::construct(fields, &source, &manager, LIMITS).is_err());
}

#[test]
fn every_feature_prefix_trailing_byte_and_saved_helper_tail_mismatch_rejects() {
    let source = context_snapshot_tests::owned_configuration();
    let manager = manager_context_tests::decode(&manager_context_tests::manager_bytes()).unwrap();
    let bytes = include_bytes!("../fixtures/resolved-features/owned-features.bin");
    for end in 0..bytes.len() {
        assert!(
            resolved_features::parse(&bytes[..end], &source, &manager, LIMITS).is_err(),
            "prefix {end}"
        );
    }
    let mut trailing = bytes.to_vec();
    trailing.push(0);
    assert!(resolved_features::parse(&trailing, &source, &manager, LIMITS).is_err());
    let inactive_bytes = manager_context_tests::replace_field(
        &manager.fields,
        "root.manager.persisted_state_650",
        &0_i32.to_le_bytes(),
    );
    let inactive = manager_context_tests::decode(&inactive_bytes).unwrap();
    assert!(resolved_features::parse(bytes, &source, &inactive, LIMITS).is_err());
    assert_eq!(source.parsed.object_map.slot_count(), 97);
}

pub(super) fn renamed_plane(source: &[super::ModelField]) -> Vec<super::ModelField> {
    use super::{ArchiveString, ModelFieldValue, TextEncoding};
    let mut fields = source.to_vec();
    let plane = fields
        .iter_mut()
        .find(|field| {
            field.path.as_ref() == "root.features[13].model_feature.feature.node.name.name"
        })
        .unwrap();
    plane.value =
        ModelFieldValue::Text(ArchiveString::new("Bessemer Plane Ω", TextEncoding::Utf16).unwrap());
    fields
}

#[test]
fn edited_feature_document_reopens_preserves_other_streams_and_reverts_exactly() {
    let original = Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
    let source = context_snapshot_tests::configuration_from(&original);
    let manager = manager_context_tests::decode(&manager_context_tests::manager_bytes()).unwrap();
    let stream = "Contents/Config-0-ResolvedFeatures";
    let parsed = resolved_features::parse(
        &original.read_stream(stream).unwrap(),
        &source,
        &manager,
        LIMITS,
    )
    .unwrap();
    let edited =
        resolved_features::construct(renamed_plane(&parsed.fields), &source, &manager, LIMITS)
            .unwrap();
    let mut changed = original.clone();
    changed
        .replace_stream(
            stream,
            super::encode_fields(&edited.fields, LIMITS.maximum_bytes).unwrap(),
        )
        .unwrap();
    let reopened = Document::from_bytes(&changed.to_bytes().unwrap()).unwrap();
    let reopened_source = context_snapshot_tests::configuration_from(&reopened);
    let reopened_manager =
        manager_context_tests::decode(&reopened.read_stream("Contents/CMgr").unwrap()).unwrap();
    let read_back = resolved_features::parse(
        &reopened.read_stream(stream).unwrap(),
        &reopened_source,
        &reopened_manager,
        LIMITS,
    )
    .unwrap();
    assert_eq!(read_back.fields, edited.fields);
    for other in original.streams().filter(|entry| entry.name != stream) {
        assert_eq!(
            original.read_stream(other.name).unwrap(),
            reopened.read_stream(other.name).unwrap()
        );
    }
    changed
        .replace_stream(stream, original.read_stream(stream).unwrap())
        .unwrap();
    assert_eq!(
        changed.to_bytes().unwrap(),
        include_bytes!("../fixtures/blank-part.SLDPRT")
    );
}

#[test]
fn completed_empty_feature_lists_are_distinct_from_budget_failures() {
    let source = context_snapshot_tests::owned_configuration();
    let manager = manager_context_tests::decode(&manager_context_tests::manager_bytes()).unwrap();
    let empty = resolved_features::parse(&[0; 10], &source, &manager, LIMITS).unwrap();
    assert_eq!(empty.roots, 0);
    assert_eq!(empty.final_map.slot_count(), 97);
    let bytes = include_bytes!("../fixtures/resolved-features/owned-features.bin");
    let complete = resolved_features::parse(bytes, &source, &manager, LIMITS).unwrap();
    for limits in [
        ModelArchiveLimits {
            maximum_bytes: bytes.len() - 1,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_fields: complete.fields.len() - 1,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_entries: 0,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_objects: 0,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_depth: 0,
            ..LIMITS
        },
    ] {
        assert!(resolved_features::parse(bytes, &source, &manager, limits).is_err());
    }
}

#[test]
fn mismatched_context_versions_and_fresh_maps_cannot_substitute_for_the_source() {
    use crate::archive_codec::ObjectReader;
    let mut source = context_snapshot_tests::owned_configuration();
    let manager = manager_context_tests::decode(&manager_context_tests::manager_bytes()).unwrap();
    let bytes = include_bytes!("../fixtures/resolved-features/owned-features.bin");
    source.context.version = crate::header::ArchiveVersion::new(19001);
    let error = resolved_features::parse(bytes, &source, &manager, LIMITS)
        .err()
        .unwrap();
    assert!(error.cause.contains("19000"));
    assert!(
        error
            .cause
            .contains("expected source configuration version 19001")
    );
    source.context.version = manager.version;
    std::sync::Arc::get_mut(&mut source.parsed)
        .unwrap()
        .object_map = ObjectReader::new().into_snapshot();
    assert!(resolved_features::parse(bytes, &source, &manager, LIMITS).is_err());
}

#[test]
fn parsed_feature_values_own_their_input_and_failed_construction_leaves_context_unchanged() {
    let source = context_snapshot_tests::owned_configuration();
    let manager = manager_context_tests::decode(&manager_context_tests::manager_bytes()).unwrap();
    let mut bytes = include_bytes!("../fixtures/resolved-features/owned-features.bin").to_vec();
    let parsed = resolved_features::parse(&bytes, &source, &manager, LIMITS).unwrap();
    bytes.fill(0xa5);
    assert_eq!(
        super::encode_fields(&parsed.fields, LIMITS.maximum_bytes).unwrap(),
        include_bytes!("../fixtures/resolved-features/owned-features.bin")
    );
    let mut wrong = parsed.fields.clone();
    wrong[0].path = "wrong.prefix".into();
    assert!(resolved_features::construct(wrong, &source, &manager, LIMITS).is_err());
    assert_eq!(source.parsed.object_map.slot_count(), 97);
    assert_eq!(
        super::encode_fields(&manager.fields, LIMITS.maximum_bytes).unwrap(),
        manager_context_tests::manager_bytes()
    );
}

#[test]
fn operation_types_are_send_sync_and_parallel_feature_edits_are_deterministic() {
    fn shared<T: Send + Sync>() {}
    shared::<resolved_features::ParsedResolvedFeatures>();
    shared::<super::configuration_manager::ParsedConfigurationManager>();
    shared::<super::ModelDecoder<'static>>();
    let source = context_snapshot_tests::owned_configuration();
    let manager = manager_context_tests::decode(&manager_context_tests::manager_bytes()).unwrap();
    let original = include_bytes!("../fixtures/resolved-features/owned-features.bin");
    let parsed = resolved_features::parse(original, &source, &manager, LIMITS).unwrap();
    std::thread::scope(|scope| {
        let operations: Vec<_> = (0..4)
            .map(|_| {
                scope.spawn(|| {
                    let edited = resolved_features::construct(
                        renamed_plane(&parsed.fields),
                        &source,
                        &manager,
                        LIMITS,
                    )
                    .unwrap();
                    super::encode_fields(&edited.fields, LIMITS.maximum_bytes).unwrap()
                })
            })
            .collect();
        for operation in operations {
            assert_eq!(
                operation.join().unwrap(),
                include_bytes!("../fixtures/resolved-features/owned-features-plane-edited.bin")
            );
        }
    });
    assert_eq!(source.parsed.object_map.slot_count(), 97);
}
