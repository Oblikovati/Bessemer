//! Complete UI-bend documents with independent edits and saved helper contexts (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::{ModelFieldValue, PartResolvedFeaturesArchive};
const CASES: [(&[u8], usize); 21] = [
    (include_bytes!("fixtures/ui-bends/owned-0-0.bin"), 101),
    (include_bytes!("fixtures/ui-bends/owned-0-1.bin"), 115),
    (include_bytes!("fixtures/ui-bends/owned-0-2.bin"), 119),
    (include_bytes!("fixtures/ui-bends/owned-0-3.bin"), 119),
    (include_bytes!("fixtures/ui-bends/owned-0-4.bin"), 101),
    (include_bytes!("fixtures/ui-bends/owned-0-5.bin"), 127),
    (include_bytes!("fixtures/ui-bends/owned-1-0.bin"), 99),
    (include_bytes!("fixtures/ui-bends/owned-1-1.bin"), 99),
    (include_bytes!("fixtures/ui-bends/owned-1-2.bin"), 103),
    (include_bytes!("fixtures/ui-bends/owned-1-3.bin"), 99),
    (include_bytes!("fixtures/ui-bends/owned-1-4.bin"), 99),
    (include_bytes!("fixtures/ui-bends/owned-1-5.bin"), 103),
    (include_bytes!("fixtures/ui-bends/owned-2-0.bin"), 99),
    (include_bytes!("fixtures/ui-bends/owned-2-1.bin"), 99),
    (include_bytes!("fixtures/ui-bends/owned-2-2.bin"), 103),
    (include_bytes!("fixtures/ui-bends/owned-2-3.bin"), 99),
    (include_bytes!("fixtures/ui-bends/owned-2-4.bin"), 99),
    (include_bytes!("fixtures/ui-bends/owned-2-5.bin"), 103),
    (
        include_bytes!("fixtures/ui-bends/owned-0-2-edited.bin"),
        119,
    ),
    (
        include_bytes!("fixtures/ui-bends/owned-1-2-edited.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/ui-bends/owned-2-2-edited.bin"),
        103,
    ),
];
const PATHS: [&str; 3] = [
    "root.features[0].items[0].payload.bend_808.body_tail.raw_a58",
    "root.features[0].items[0].raw_18",
    "root.features[0].items[0].raw_08",
];
const ACTIVE: [(&[u8], &[u8], usize, i32); 6] = [
    (
        include_bytes!("fixtures/ui-bends/active-0.bin"),
        include_bytes!("fixtures/ui-bends/active-0-edited.bin"),
        101,
        -2147483648,
    ),
    (
        include_bytes!("fixtures/ui-bends/active-1.bin"),
        include_bytes!("fixtures/ui-bends/active-1-edited.bin"),
        105,
        -1,
    ),
    (
        include_bytes!("fixtures/ui-bends/active-2.bin"),
        include_bytes!("fixtures/ui-bends/active-2-edited.bin"),
        105,
        0,
    ),
    (
        include_bytes!("fixtures/ui-bends/active-3.bin"),
        include_bytes!("fixtures/ui-bends/active-3-edited.bin"),
        107,
        1,
    ),
    (
        include_bytes!("fixtures/ui-bends/active-4.bin"),
        include_bytes!("fixtures/ui-bends/active-4-edited.bin"),
        101,
        2,
    ),
    (
        include_bytes!("fixtures/ui-bends/active-5.bin"),
        include_bytes!("fixtures/ui-bends/active-5-edited.bin"),
        107,
        3,
    ),
];
const COUNT_PATH: &str = "root.features[0].payload.declared_line_count";
const ID_PATH: &str = "root.features[0].payload.raw_810";
#[test]
fn complete_ui_documents_reconstruct_and_isolate_inputs() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, slots);
    }
}
#[test]
fn independent_ui_edits_match_frozen_goldens_and_preserve_inputs() {
    for (kind, path) in PATHS.into_iter().enumerate() {
        contract::check_word_edit(CASES[kind * 6 + 2].0, CASES[18 + kind].0, path);
    }
}
#[test]
fn rejected_ui_scalar_types_preserve_original_documents() {
    for (kind, path) in PATHS.into_iter().enumerate() {
        contract::check_rejected_edit(CASES[kind * 6 + 2].0, path);
    }
}
fn active_document(original: &[u8]) -> bessemer::Document {
    let mut document = contract::document_with(original);
    document
        .replace_stream(
            "Contents/CMgr",
            include_bytes!("fixtures/ui-bends/manager-1.bin").to_vec(),
        )
        .unwrap();
    document
}
fn check_direct_lines(archive: &PartResolvedFeaturesArchive, count: i32) {
    let fields: Vec<_> = archive
        .fields()
        .iter()
        .filter(|field| {
            field
                .path
                .starts_with("root.features[0].payload.direct_lines_828[")
        })
        .collect();
    assert_eq!(fields.len(), count.max(0) as usize * 2);
    assert!(fields.iter().all(|field| matches!(
        field.value,
        ModelFieldValue::F64(_) | ModelFieldValue::I32(_)
    )));
    check_stored_count(archive, count);
}

fn check_stored_count(archive: &PartResolvedFeaturesArchive, count: i32) {
    assert_eq!(
        archive
            .fields()
            .iter()
            .find(|field| field.path.as_ref() == COUNT_PATH)
            .unwrap()
            .value,
        ModelFieldValue::I32(count)
    );
    assert!(
        !archive
            .fields()
            .iter()
            .any(|field| field.path.ends_with("bend_808"))
    );
}
fn check_active_edit(archive: &PartResolvedFeaturesArchive, edited: &[u8]) {
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == ID_PATH)
        .unwrap()
        .value = ModelFieldValue::I32(12345);
    let changed = archive.edit_fields(fields).unwrap();
    assert_eq!(changed.archive().to_bytes().unwrap(), edited);
}
#[test]
fn actual_saved_manager_selects_all_signed_count_shapes_without_direct_object_tags() {
    for (original, edited, slots, count) in ACTIVE {
        let document = active_document(original);
        let archive = contract::parsed_with_kernel(&document, None);
        assert_eq!(archive.object_slots(), (97, slots));
        assert_eq!(archive.to_bytes().unwrap(), original);
        check_direct_lines(&archive, count);
        check_active_edit(&archive, edited);
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
fn impossible_direct_line_counts_fail_before_iteration_and_preserve_inputs() {
    for (original, _, _, _) in ACTIVE {
        let document = active_document(original);
        let archive = contract::parsed_with_kernel(&document, None);
        reject_impossible_count(&archive);
        assert_eq!(archive.to_bytes().unwrap(), original);
        assert_eq!(
            document
                .read_stream("Contents/Config-0-ResolvedFeatures")
                .unwrap(),
            original
        );
    }
}

fn reject_impossible_count(archive: &PartResolvedFeaturesArchive) {
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == COUNT_PATH)
        .unwrap()
        .value = ModelFieldValue::I32(i32::MAX);
    let Err(error) = archive.edit_fields(fields) else {
        panic!("expected bounded UI-bend line count")
    };
    assert!(error.to_string().contains("2147483647"));
}
