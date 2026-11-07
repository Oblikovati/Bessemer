//! Complete corner and hem documents with independent scalar edit goldens (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::header::StoredDouble;
use bessemer::model_archives::{ModelFieldValue, PartResolvedFeaturesArchive};
const CASES: [(&[u8], usize); 61] = [
    (include_bytes!("fixtures/corner-hems/owned-0-0.bin"), 101),
    (include_bytes!("fixtures/corner-hems/owned-0-1.bin"), 107),
    (include_bytes!("fixtures/corner-hems/owned-0-2.bin"), 113),
    (include_bytes!("fixtures/corner-hems/owned-0-3.bin"), 109),
    (include_bytes!("fixtures/corner-hems/owned-0-4.bin"), 101),
    (include_bytes!("fixtures/corner-hems/owned-0-5.bin"), 117),
    (include_bytes!("fixtures/corner-hems/owned-1-0.bin"), 99),
    (include_bytes!("fixtures/corner-hems/owned-1-1.bin"), 99),
    (include_bytes!("fixtures/corner-hems/owned-1-2.bin"), 103),
    (include_bytes!("fixtures/corner-hems/owned-1-3.bin"), 99),
    (include_bytes!("fixtures/corner-hems/owned-1-4.bin"), 99),
    (include_bytes!("fixtures/corner-hems/owned-1-5.bin"), 103),
    (include_bytes!("fixtures/corner-hems/owned-2-0.bin"), 99),
    (include_bytes!("fixtures/corner-hems/owned-2-1.bin"), 99),
    (include_bytes!("fixtures/corner-hems/owned-2-2.bin"), 103),
    (include_bytes!("fixtures/corner-hems/owned-2-3.bin"), 99),
    (include_bytes!("fixtures/corner-hems/owned-2-4.bin"), 99),
    (include_bytes!("fixtures/corner-hems/owned-2-5.bin"), 103),
    (include_bytes!("fixtures/corner-hems/owned-3-0.bin"), 101),
    (include_bytes!("fixtures/corner-hems/owned-3-1.bin"), 117),
    (include_bytes!("fixtures/corner-hems/owned-3-2.bin"), 123),
    (include_bytes!("fixtures/corner-hems/owned-3-3.bin"), 119),
    (include_bytes!("fixtures/corner-hems/owned-3-4.bin"), 101),
    (include_bytes!("fixtures/corner-hems/owned-3-5.bin"), 127),
    (include_bytes!("fixtures/corner-hems/owned-4-0.bin"), 103),
    (include_bytes!("fixtures/corner-hems/owned-4-1.bin"), 119),
    (include_bytes!("fixtures/corner-hems/owned-4-2.bin"), 123),
    (include_bytes!("fixtures/corner-hems/owned-4-3.bin"), 121),
    (include_bytes!("fixtures/corner-hems/owned-4-4.bin"), 103),
    (include_bytes!("fixtures/corner-hems/owned-4-5.bin"), 125),
    (include_bytes!("fixtures/corner-hems/owned-5-0.bin"), 99),
    (include_bytes!("fixtures/corner-hems/owned-5-1.bin"), 109),
    (include_bytes!("fixtures/corner-hems/owned-5-2.bin"), 113),
    (include_bytes!("fixtures/corner-hems/owned-5-3.bin"), 109),
    (include_bytes!("fixtures/corner-hems/owned-5-4.bin"), 99),
    (include_bytes!("fixtures/corner-hems/owned-5-5.bin"), 113),
    (include_bytes!("fixtures/corner-hems/owned-6-0.bin"), 99),
    (include_bytes!("fixtures/corner-hems/owned-6-1.bin"), 103),
    (include_bytes!("fixtures/corner-hems/owned-6-2.bin"), 107),
    (include_bytes!("fixtures/corner-hems/owned-6-3.bin"), 103),
    (include_bytes!("fixtures/corner-hems/owned-6-4.bin"), 99),
    (include_bytes!("fixtures/corner-hems/owned-6-5.bin"), 107),
    (include_bytes!("fixtures/corner-hems/owned-7-0.bin"), 99),
    (include_bytes!("fixtures/corner-hems/owned-7-1.bin"), 101),
    (include_bytes!("fixtures/corner-hems/owned-7-2.bin"), 105),
    (include_bytes!("fixtures/corner-hems/owned-7-3.bin"), 101),
    (include_bytes!("fixtures/corner-hems/owned-7-4.bin"), 99),
    (include_bytes!("fixtures/corner-hems/owned-7-5.bin"), 105),
    (include_bytes!("fixtures/corner-hems/owned-8-0.bin"), 99),
    (include_bytes!("fixtures/corner-hems/owned-8-1.bin"), 99),
    (include_bytes!("fixtures/corner-hems/owned-8-2.bin"), 103),
    (include_bytes!("fixtures/corner-hems/owned-8-3.bin"), 99),
    (include_bytes!("fixtures/corner-hems/owned-8-4.bin"), 99),
    (include_bytes!("fixtures/corner-hems/owned-8-5.bin"), 103),
    (
        include_bytes!("fixtures/corner-hems/owned-0-2-edited.bin"),
        113,
    ),
    (
        include_bytes!("fixtures/corner-hems/owned-1-2-edited.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/corner-hems/owned-2-2-edited.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/corner-hems/owned-3-2-edited.bin"),
        123,
    ),
    (
        include_bytes!("fixtures/corner-hems/owned-4-2-edited.bin"),
        123,
    ),
    (
        include_bytes!("fixtures/corner-hems/owned-5-2-edited.bin"),
        113,
    ),
    (
        include_bytes!("fixtures/corner-hems/owned-8-2-edited.bin"),
        103,
    ),
];
const EDITS: [(usize, &str); 7] = [
    (0, "root.features[0].items[0].payload.raw_9a0"),
    (1, "root.features[0].items[0].point_08[0]"),
    (2, "root.features[0].items[0].point_08[0]"),
    (3, "root.features[0].items[0].payload.data.raw_80"),
    (4, "root.features[0].items[0].raw_80"),
    (5, "root.features[0].items[0].raw_28"),
    (8, "root.features[0].items[0].raw_08"),
];
const ACTIVE: [(&[u8], &[u8], usize, &str); 3] = [
    (
        include_bytes!("fixtures/corner-hems/active-0.bin"),
        include_bytes!("fixtures/corner-hems/active-0-edited.bin"),
        109,
        "root.features[0].payload.raw_9a0",
    ),
    (
        include_bytes!("fixtures/corner-hems/active-3.bin"),
        include_bytes!("fixtures/corner-hems/active-3-edited.bin"),
        119,
        "root.features[0].payload.data.raw_80",
    ),
    (
        include_bytes!("fixtures/corner-hems/active-4.bin"),
        include_bytes!("fixtures/corner-hems/active-4-edited.bin"),
        121,
        "root.features[0].raw_80",
    ),
];
fn edited_value(kind: usize) -> ModelFieldValue {
    if matches!(kind, 1 | 2) {
        return ModelFieldValue::F64(StoredDouble::from_bits(123.5f64.to_bits()));
    }
    ModelFieldValue::I32(12345)
}
#[test]
fn complete_corner_hem_documents_reconstruct_and_isolate_inputs() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, slots);
    }
}
#[test]
fn independent_corner_hem_edits_match_frozen_goldens_and_preserve_inputs() {
    for (edit, (kind, path)) in EDITS.into_iter().enumerate() {
        contract::check_value_edit(
            CASES[kind * 6 + 2].0,
            CASES[54 + edit].0,
            path,
            edited_value(kind),
            None,
        );
    }
}
#[test]
fn rejected_corner_hem_scalar_types_preserve_original_documents() {
    for (kind, path) in EDITS {
        contract::check_rejected_edit(CASES[kind * 6 + 2].0, path);
    }
}
fn active_document(original: &[u8]) -> bessemer::Document {
    let mut document = contract::document_with(original);
    document
        .replace_stream(
            "Contents/CMgr",
            include_bytes!("fixtures/corner-hems/manager-1.bin").to_vec(),
        )
        .unwrap();
    document
}
fn check_active_edit(archive: &PartResolvedFeaturesArchive, edited: &[u8], path: &str) {
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value = ModelFieldValue::I32(12345);
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), edited);
    assert!(
        !archive
            .fields()
            .iter()
            .any(|field| field.path.ends_with("sheet_metal_18"))
    );
}
#[test]
fn actual_saved_manager_selects_complete_alternate_hem_and_corner_layouts() {
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
