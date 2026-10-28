//! Complete edge-flange documents with independent scalar edit goldens (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::ModelFieldValue;

const ACTIVE: [(&[u8], &[u8], usize, &str); 3] = [
    (
        include_bytes!("fixtures/edge-flanges/active-0.bin"),
        include_bytes!("fixtures/edge-flanges/active-0-edited.bin"),
        111,
        "root.features[0].data.raw_c0",
    ),
    (
        include_bytes!("fixtures/edge-flanges/active-1.bin"),
        include_bytes!("fixtures/edge-flanges/active-1-edited.bin"),
        113,
        "root.features[0].raw_c0",
    ),
    (
        include_bytes!("fixtures/edge-flanges/active-2.bin"),
        include_bytes!("fixtures/edge-flanges/active-2-edited.bin"),
        111,
        "root.features[0].body_tail.raw_a58",
    ),
];
const CASES: [(&[u8], usize); 42] = [
    (include_bytes!("fixtures/edge-flanges/owned-0-0.bin"), 101),
    (include_bytes!("fixtures/edge-flanges/owned-0-1.bin"), 109),
    (include_bytes!("fixtures/edge-flanges/owned-0-2.bin"), 115),
    (include_bytes!("fixtures/edge-flanges/owned-0-3.bin"), 111),
    (include_bytes!("fixtures/edge-flanges/owned-0-4.bin"), 101),
    (include_bytes!("fixtures/edge-flanges/owned-0-5.bin"), 119),
    (include_bytes!("fixtures/edge-flanges/owned-1-0.bin"), 103),
    (include_bytes!("fixtures/edge-flanges/owned-1-1.bin"), 111),
    (include_bytes!("fixtures/edge-flanges/owned-1-2.bin"), 115),
    (include_bytes!("fixtures/edge-flanges/owned-1-3.bin"), 113),
    (include_bytes!("fixtures/edge-flanges/owned-1-4.bin"), 103),
    (include_bytes!("fixtures/edge-flanges/owned-1-5.bin"), 117),
    (include_bytes!("fixtures/edge-flanges/owned-2-0.bin"), 101),
    (include_bytes!("fixtures/edge-flanges/owned-2-1.bin"), 111),
    (include_bytes!("fixtures/edge-flanges/owned-2-2.bin"), 115),
    (include_bytes!("fixtures/edge-flanges/owned-2-3.bin"), 113),
    (include_bytes!("fixtures/edge-flanges/owned-2-4.bin"), 103),
    (include_bytes!("fixtures/edge-flanges/owned-2-5.bin"), 121),
    (include_bytes!("fixtures/edge-flanges/owned-3-0.bin"), 99),
    (include_bytes!("fixtures/edge-flanges/owned-3-1.bin"), 107),
    (include_bytes!("fixtures/edge-flanges/owned-3-2.bin"), 107),
    (include_bytes!("fixtures/edge-flanges/owned-3-3.bin"), 105),
    (include_bytes!("fixtures/edge-flanges/owned-3-4.bin"), 101),
    (include_bytes!("fixtures/edge-flanges/owned-3-5.bin"), 111),
    (include_bytes!("fixtures/edge-flanges/owned-4-0.bin"), 99),
    (include_bytes!("fixtures/edge-flanges/owned-4-1.bin"), 99),
    (include_bytes!("fixtures/edge-flanges/owned-4-2.bin"), 103),
    (include_bytes!("fixtures/edge-flanges/owned-4-3.bin"), 99),
    (include_bytes!("fixtures/edge-flanges/owned-4-4.bin"), 99),
    (include_bytes!("fixtures/edge-flanges/owned-4-5.bin"), 103),
    (include_bytes!("fixtures/edge-flanges/owned-5-0.bin"), 99),
    (include_bytes!("fixtures/edge-flanges/owned-5-1.bin"), 99),
    (include_bytes!("fixtures/edge-flanges/owned-5-2.bin"), 103),
    (include_bytes!("fixtures/edge-flanges/owned-5-3.bin"), 99),
    (include_bytes!("fixtures/edge-flanges/owned-5-4.bin"), 99),
    (include_bytes!("fixtures/edge-flanges/owned-5-5.bin"), 103),
    (
        include_bytes!("fixtures/edge-flanges/owned-0-2-edited.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/edge-flanges/owned-1-2-edited.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/edge-flanges/owned-2-2-edited.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/edge-flanges/owned-3-2-edited.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/edge-flanges/owned-4-2-edited.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/edge-flanges/owned-5-2-edited.bin"),
        103,
    ),
];
const PATHS: [&str; 6] = [
    "root.features[0].items[0].data.raw_c0",
    "root.features[0].items[0].raw_c0",
    "root.features[0].items[0].body_tail.raw_a58",
    "root.features[0].items[0].raw_20",
    "root.features[0].items[0].raw_08",
    "root.features[0].items[0].raw_08",
];

#[test]
fn complete_edge_documents_reconstruct_and_isolate_inputs() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, slots);
    }
}

#[test]
fn independent_edge_edits_preserve_source_documents() {
    for (kind, path) in PATHS.into_iter().enumerate() {
        contract::check_word_edit(CASES[kind * 6 + 2].0, CASES[36 + kind].0, path);
    }
}

#[test]
fn rejected_edge_scalar_types_preserve_source_documents() {
    for (kind, path) in PATHS.into_iter().enumerate() {
        contract::check_rejected_edit(CASES[kind * 6 + 2].0, path);
    }
}

#[test]
fn active_saved_manager_selects_complete_edge_and_bend_layouts() {
    for (original, edited, slots, path) in ACTIVE {
        let document = active_document(original);
        let archive = contract::parsed_with_kernel(&document, None);
        assert_eq!(archive.object_slots(), (97, slots));
        assert_eq!(archive.to_bytes().unwrap(), original);
        check_active_edit(&archive, edited, path);
        assert_eq!(
            document
                .read_stream("Contents/Config-0-ResolvedFeatures")
                .unwrap(),
            original
        );
    }
}

fn active_document(original: &[u8]) -> bessemer::Document {
    let mut document = contract::document_with(original);
    document
        .replace_stream(
            "Contents/CMgr",
            include_bytes!("fixtures/edge-flanges/manager-1.bin").to_vec(),
        )
        .unwrap();
    document
}

fn check_active_edit(
    archive: &bessemer::model_archives::PartResolvedFeaturesArchive,
    edited: &[u8],
    path: &str,
) {
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value = ModelFieldValue::I32(12345);
    let changed = archive.edit_fields(fields).unwrap();
    assert_eq!(changed.archive().to_bytes().unwrap(), edited);
}
