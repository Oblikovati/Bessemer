//! Complete OneBend documents with shared conic state and raw scalar edits (#218).
#[path = "support/owned_resolved_document.rs"]
mod owned_resolved_document;
#[path = "support/owned_saved_manager_document.rs"]
mod owned_saved_manager_document;
use bessemer::Document;
use bessemer::model_archives::{
    ModelArchiveLimits, ModelEditCheckMode, ModelEditCompletion, ModelEditFidelity, ModelField,
    ModelFieldRelation, ModelFieldValue, PartResolvedFeaturesArchive, PartResolvedFeaturesEdit,
    StoredDouble,
};
use owned_resolved_document::{OWNED_RESOLVED_STREAM, read_owned_resolved_document};
use owned_saved_manager_document::prepared_saved_manager_document;

const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
const CASES: [[&[u8]; 3]; 6] = [
    [
        include_bytes!("fixtures/conic-bend/owned-empty-0.bin"),
        include_bytes!("fixtures/conic-bend/owned-empty-1.bin"),
        include_bytes!("fixtures/conic-bend/owned-empty-2.bin"),
    ],
    [
        include_bytes!("fixtures/conic-bend/owned-rich-0.bin"),
        include_bytes!("fixtures/conic-bend/owned-rich-1.bin"),
        include_bytes!("fixtures/conic-bend/owned-rich-2.bin"),
    ],
    [
        include_bytes!("fixtures/conic-bend/owned-aliases-0.bin"),
        include_bytes!("fixtures/conic-bend/owned-aliases-1.bin"),
        include_bytes!("fixtures/conic-bend/owned-aliases-2.bin"),
    ],
    [
        include_bytes!("fixtures/conic-bend/owned-helper-empty-0.bin"),
        include_bytes!("fixtures/conic-bend/owned-helper-empty-1.bin"),
        include_bytes!("fixtures/conic-bend/owned-helper-empty-2.bin"),
    ],
    [
        include_bytes!("fixtures/conic-bend/owned-helper-rich-0.bin"),
        include_bytes!("fixtures/conic-bend/owned-helper-rich-1.bin"),
        include_bytes!("fixtures/conic-bend/owned-helper-rich-2.bin"),
    ],
    [
        include_bytes!("fixtures/conic-bend/owned-helper-aliases-0.bin"),
        include_bytes!("fixtures/conic-bend/owned-helper-aliases-1.bin"),
        include_bytes!("fixtures/conic-bend/owned-helper-aliases-2.bin"),
    ],
];

fn conic_document(raw: &[u8], context: usize) -> Document {
    let mut document = prepared_saved_manager_document(raw);
    if context >= 3 {
        document
            .replace_stream(
                "Contents/CMgr",
                include_bytes!("fixtures/edge-flanges/manager-1.bin").to_vec(),
            )
            .unwrap();
    }
    Document::from_bytes(&document.to_bytes().unwrap()).unwrap()
}

fn conic_archive(document: &Document) -> PartResolvedFeaturesArchive {
    let archive = read_owned_resolved_document(document, LIMITS);
    assert_eq!(archive.object_slots().0, 97);
    archive
}

fn conic_replacement(context: usize, edit: usize) -> (String, ModelFieldValue) {
    let root = if context % 3 == 2 {
        "root.features[0].items[0]"
    } else {
        "root.features[0]"
    };
    let (suffix, value) = match edit {
        1 => ("raw_48", ModelFieldValue::I32(12345)),
        2 => (
            "raw_38",
            ModelFieldValue::F64(StoredDouble::from_bits(0x7ff0000000000001)),
        ),
        _ => panic!("edit {edit}; expected state-word1 or scalar2"),
    };
    (format!("{root}.conic_ad0.{suffix}"), value)
}

fn conic_fields(
    source: &PartResolvedFeaturesArchive,
    context: usize,
    edit: usize,
) -> Vec<ModelField> {
    let (path, value) = conic_replacement(context, edit);
    let mut fields = source.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value = value;
    fields
}

fn assert_conic_contract(source: &PartResolvedFeaturesArchive, edit: &PartResolvedFeaturesEdit) {
    assert_eq!(edit.completion(), ModelEditCompletion::Changed);
    assert_eq!(edit.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(edit.coverage().mode, ModelEditCheckMode::ExactStoredArchive);
    assert_eq!(edit.coverage(), edit.archive().coverage());
    assert_eq!(edit.diagnostics(), source.diagnostics());
    assert_eq!(edit.unchanged_fields(), source.fields().len() - 1);
    assert_eq!(edit.changes().len(), 1);
    let change = &edit.changes()[0];
    assert_eq!(change.relation(), ModelFieldRelation::Derived);
    assert_eq!(change.parents().len(), 1);
    let parent = source.resolve_field(&change.parents()[0]).unwrap();
    let result = edit.archive();
    let child = result.resolve_field(change.result().unwrap()).unwrap();
    assert_eq!(parent.path, child.path);
    assert_ne!(parent.value, child.value);
    assert!(source.resolve_field(change.result().unwrap()).is_err());
    assert!(edit.archive().resolve_field(&change.parents()[0]).is_err());
}

#[test]
fn all_conic_documents_reconstruct_whole_carriers_and_shared_maps() {
    for (context, cases) in CASES.iter().enumerate() {
        for raw in cases {
            let mut document = conic_document(raw, context);
            let before = document.to_bytes().unwrap();
            let archive = conic_archive(&document);
            assert_eq!(archive.to_bytes().unwrap(), *raw);
            assert_eq!(
                archive.object_slots(),
                (97, [103, 115, 123, 103, 113, 121][context])
            );
            document
                .replace_stream(OWNED_RESOLVED_STREAM, archive.to_bytes().unwrap())
                .unwrap();
            assert_eq!(document.to_bytes().unwrap(), before);
        }
    }
}

fn check_conic_edit(source: &PartResolvedFeaturesArchive, context: usize, edit_index: usize) {
    let edit = source
        .edit_fields(conic_fields(source, context, edit_index))
        .unwrap();
    assert_eq!(
        edit.archive().to_bytes().unwrap(),
        CASES[context][edit_index]
    );
    assert_conic_contract(source, &edit);
    let fresh = conic_archive(&conic_document(CASES[context][edit_index], context));
    let reverse = fresh.edit_fields(source.fields().to_vec()).unwrap();
    assert_eq!(reverse.archive().to_bytes().unwrap(), CASES[context][0]);
    assert_conic_contract(&fresh, &reverse);
    assert_eq!(source.to_bytes().unwrap(), CASES[context][0]);
}

#[test]
fn twelve_conic_edits_and_fresh_reversals_preserve_history_and_inputs() {
    for (context, cases) in CASES.iter().enumerate() {
        let document = conic_document(cases[0], context);
        let before = document.to_bytes().unwrap();
        let source = conic_archive(&document);
        for edit in 1..=2 {
            check_conic_edit(&source, context, edit);
        }
        assert_eq!(document.to_bytes().unwrap(), before);
    }
}

fn repeat_conic_edits(sources: &[PartResolvedFeaturesArchive]) {
    for (context, source) in sources.iter().enumerate() {
        for edit in 1..=2 {
            check_conic_edit(source, context, edit);
        }
    }
}

#[test]
fn conic_snapshots_isolate_eight_concurrent_workers() {
    let sources: Vec<_> = CASES
        .iter()
        .enumerate()
        .map(|(index, cases)| conic_archive(&conic_document(cases[0], index)))
        .collect();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| repeat_conic_edits(&sources)))
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
    repeat_conic_edits(&sources);
}

#[test]
fn rejected_conic_edits_and_source_mutation_do_not_change_published_archives() {
    for (context, cases) in CASES.iter().enumerate() {
        let mut document = conic_document(cases[0], context);
        let before = document.to_bytes().unwrap();
        let source = conic_archive(&document);
        let mut fields = conic_fields(&source, context, 2);
        fields
            .iter_mut()
            .find(|field| field.path.ends_with("conic_ad0.raw_38"))
            .unwrap()
            .value = ModelFieldValue::I32(0);
        assert!(source.edit_fields(fields).is_err());
        assert_eq!(document.to_bytes().unwrap(), before);
        document
            .replace_stream(OWNED_RESOLVED_STREAM, vec![0])
            .unwrap();
        check_conic_edit(&source, context, 1);
        assert_eq!(document.read_stream(OWNED_RESOLVED_STREAM).unwrap(), &[0]);
    }
}
