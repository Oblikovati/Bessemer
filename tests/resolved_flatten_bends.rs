//! Complete flatten/process documents with independent scalar edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::{
    CompressedKernelContext, ModelField, ModelFieldValue, PartResolvedFeaturesArchive,
};
use bessemer::parasolid::{BaseSchema, TransmitLimits};
const CASES: [(&[u8], usize); 14] = [
    (include_bytes!("fixtures/flatten-bends/owned-0-0.bin"), 101),
    (include_bytes!("fixtures/flatten-bends/owned-0-1.bin"), 101),
    (include_bytes!("fixtures/flatten-bends/owned-0-2.bin"), 107),
    (include_bytes!("fixtures/flatten-bends/owned-0-3.bin"), 103),
    (include_bytes!("fixtures/flatten-bends/owned-0-4.bin"), 101),
    (include_bytes!("fixtures/flatten-bends/owned-0-5.bin"), 111),
    (include_bytes!("fixtures/flatten-bends/owned-1-0.bin"), 101),
    (include_bytes!("fixtures/flatten-bends/owned-1-1.bin"), 107),
    (include_bytes!("fixtures/flatten-bends/owned-1-2.bin"), 113),
    (include_bytes!("fixtures/flatten-bends/owned-1-3.bin"), 109),
    (include_bytes!("fixtures/flatten-bends/owned-1-4.bin"), 101),
    (include_bytes!("fixtures/flatten-bends/owned-1-5.bin"), 117),
    (
        include_bytes!("fixtures/flatten-bends/owned-0-2-edited.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/flatten-bends/owned-1-2-edited.bin"),
        113,
    ),
];
const PATHS: [&str; 2] = [
    "root.features[0].items[0].payload.raw_8c8",
    "root.features[0].items[0].payload.raw_890",
];
const ACTIVE: [(&[u8], &[u8], usize, &str); 2] = [
    (
        include_bytes!("fixtures/flatten-bends/active-0.bin"),
        include_bytes!("fixtures/flatten-bends/active-0-edited.bin"),
        103,
        "root.features[0].payload.raw_8c8",
    ),
    (
        include_bytes!("fixtures/flatten-bends/active-1.bin"),
        include_bytes!("fixtures/flatten-bends/active-1-edited.bin"),
        109,
        "root.features[0].payload.raw_890",
    ),
];
fn kernel() -> CompressedKernelContext {
    CompressedKernelContext {
        base: BaseSchema::from_bytes(include_bytes!("fixtures/parasolid/base-schema.s_t")).unwrap(),
        maximum_expanded: 1024 * 1024,
        transmit_limits: TransmitLimits {
            maximum_records: 1000,
            maximum_field_elements: 10000,
        },
    }
}
#[test]
fn complete_flatten_process_documents_reconstruct_and_isolate_inputs() {
    contract::check_document(CASES[0].0, CASES[0].1);
    for (bytes, slots) in CASES {
        contract::check_document_with_kernel(bytes, slots, Some(kernel()));
    }
}
#[test]
fn independent_flatten_process_edits_match_frozen_goldens() {
    for (kind, path) in PATHS.into_iter().enumerate() {
        contract::check_word_edit_with_kernel(
            CASES[kind * 6 + 2].0,
            CASES[12 + kind].0,
            path,
            Some(kernel()),
        );
    }
}
#[test]
fn rejected_flatten_process_scalar_types_preserve_originals() {
    contract::check_rejected_edit(CASES[2].0, PATHS[0]);
    for (kind, path) in PATHS.into_iter().enumerate() {
        contract::check_rejected_edit_with_kernel(CASES[kind * 6 + 2].0, path, Some(kernel()));
    }
}
fn active_document(original: &[u8]) -> bessemer::Document {
    let mut document = contract::document_with(original);
    document
        .replace_stream(
            "Contents/CMgr",
            include_bytes!("fixtures/flatten-bends/manager-1.bin").to_vec(),
        )
        .unwrap();
    document
}
fn replacement_fields(
    archive: &PartResolvedFeaturesArchive,
    path: &str,
    value: ModelFieldValue,
) -> Vec<ModelField> {
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value = value;
    fields
}
#[test]
fn saved_helper_state_remains_independent_of_node_body_presence_and_archive_mode() {
    for (original, edited, slots, path) in ACTIVE {
        let document = active_document(original);
        let archive = contract::parsed_with_kernel(&document, Some(kernel()));
        assert_eq!(archive.object_slots(), (97, slots));
        assert_eq!(archive.to_bytes().unwrap(), original);
        let fields = replacement_fields(&archive, path, ModelFieldValue::I32(12345));
        let changed = archive.edit_fields(fields).unwrap();
        assert_eq!(changed.archive().to_bytes().unwrap(), edited);
        assert_eq!(archive.to_bytes().unwrap(), original);
        assert_eq!(
            document
                .read_stream("Contents/Config-0-ResolvedFeatures")
                .unwrap(),
            original
        );
    }
}
#[test]
fn selecting_a_missing_presence_word_is_rejected_without_mutating_inputs() {
    let document = contract::document_with(CASES[0].0);
    let archive = contract::parsed_with_kernel(&document, Some(kernel()));
    let path = "root.features[0].body.model_feature.feature.node.flags_28";
    let fields = replacement_fields(&archive, path, ModelFieldValue::U32(0x800));
    assert!(archive.edit_fields(fields).is_err());
    assert_eq!(archive.to_bytes().unwrap(), CASES[0].0);
    assert_eq!(
        document
            .read_stream("Contents/Config-0-ResolvedFeatures")
            .unwrap(),
        CASES[0].0
    );
}
