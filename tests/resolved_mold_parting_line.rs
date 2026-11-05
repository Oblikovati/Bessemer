//! Parting-line documents with independent scalar edits and strict kernel context (#216).
#[path = "support/owned_resolved_document.rs"]
mod owned_resolved_document;
#[path = "support/owned_saved_manager_document.rs"]
mod owned_saved_manager_document;
use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::{
    CompressedKernelContext, ConfigurationId, ModelArchiveLimits, ModelEditCheckMode,
    ModelEditCompletion, ModelEditFidelity, ModelField, ModelFieldRelation, ModelFieldValue,
    PartResolvedFeaturesArchive, PartResolvedFeaturesEdit, StoredDouble,
};
use bessemer::parasolid::{BaseSchema, TransmitLimits};
use owned_resolved_document::{OWNED_RESOLVED_STREAM, read_owned_resolved_document};
use owned_saved_manager_document::prepared_saved_manager_document;

const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
const CASES: [[&[u8]; 3]; 4] = [
    [
        include_bytes!("fixtures/mold-parting-line/owned-empty-0.bin"),
        include_bytes!("fixtures/mold-parting-line/owned-empty-1.bin"),
        include_bytes!("fixtures/mold-parting-line/owned-empty-2.bin"),
    ],
    [
        include_bytes!("fixtures/mold-parting-line/owned-suppressed-0.bin"),
        include_bytes!("fixtures/mold-parting-line/owned-suppressed-1.bin"),
        include_bytes!("fixtures/mold-parting-line/owned-suppressed-2.bin"),
    ],
    [
        include_bytes!("fixtures/mold-parting-line/owned-serialized-0.bin"),
        include_bytes!("fixtures/mold-parting-line/owned-serialized-1.bin"),
        include_bytes!("fixtures/mold-parting-line/owned-serialized-2.bin"),
    ],
    [
        include_bytes!("fixtures/mold-parting-line/owned-negative-0.bin"),
        include_bytes!("fixtures/mold-parting-line/owned-negative-1.bin"),
        include_bytes!("fixtures/mold-parting-line/owned-negative-2.bin"),
    ],
];

fn line_kernel() -> CompressedKernelContext {
    CompressedKernelContext {
        base: BaseSchema::from_bytes(include_bytes!("fixtures/parasolid/base-schema.s_t")).unwrap(),
        maximum_expanded: 1024 * 1024,
        transmit_limits: TransmitLimits {
            maximum_records: 1000,
            maximum_field_elements: 10000,
        },
    }
}

fn line_archive(document: &Document, context: usize) -> PartResolvedFeaturesArchive {
    if context != 2 {
        return read_owned_resolved_document(document, LIMITS);
    }
    PartResolvedFeaturesArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        Some(line_kernel()),
        LIMITS,
    )
    .unwrap()
}

fn line_replacement(edit: usize) -> (&'static str, ModelFieldValue) {
    match edit {
        1 => (
            "root.features[0].data_800.raw_18",
            ModelFieldValue::F64(StoredDouble::from_bits(0x7ff0000000000001)),
        ),
        2 => (
            "root.features[0].data_800.raw_1d8",
            ModelFieldValue::I32(12345),
        ),
        _ => panic!("edit {edit}; expected Angle1 or split-flag2"),
    }
}

fn line_fields(source: &PartResolvedFeaturesArchive, edit: usize) -> Vec<ModelField> {
    let (path, value) = line_replacement(edit);
    let mut fields = source.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value = value;
    fields
}

fn assert_line_contract(source: &PartResolvedFeaturesArchive, edit: &PartResolvedFeaturesEdit) {
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
fn all_parting_line_documents_reconstruct_whole_carriers_and_shared_maps() {
    for (context, cases) in CASES.iter().enumerate() {
        for raw in cases {
            let mut document = prepared_saved_manager_document(raw);
            let before = document.to_bytes().unwrap();
            let archive = line_archive(&document, context);
            assert_eq!(archive.to_bytes().unwrap(), *raw);
            assert_eq!(
                archive.object_slots(),
                (97, if context == 0 { 103 } else { 117 })
            );
            document
                .replace_stream(OWNED_RESOLVED_STREAM, archive.to_bytes().unwrap())
                .unwrap();
            assert_eq!(document.to_bytes().unwrap(), before);
        }
    }
}

fn check_line_edit(source: &PartResolvedFeaturesArchive, context: usize, edit_index: usize) {
    let edit = source.edit_fields(line_fields(source, edit_index)).unwrap();
    assert_eq!(
        edit.archive().to_bytes().unwrap(),
        CASES[context][edit_index]
    );
    assert_line_contract(source, &edit);
    let fresh = line_archive(
        &prepared_saved_manager_document(CASES[context][edit_index]),
        context,
    );
    let reverse = fresh.edit_fields(source.fields().to_vec()).unwrap();
    assert_eq!(reverse.archive().to_bytes().unwrap(), CASES[context][0]);
    assert_line_contract(&fresh, &reverse);
    assert_eq!(source.to_bytes().unwrap(), CASES[context][0]);
}

#[test]
fn eight_parting_line_edits_and_fresh_reversals_preserve_history_and_inputs() {
    for (context, cases) in CASES.iter().enumerate() {
        let document = prepared_saved_manager_document(cases[0]);
        let before = document.to_bytes().unwrap();
        let source = line_archive(&document, context);
        for edit in 1..=2 {
            check_line_edit(&source, context, edit);
        }
        assert_eq!(document.to_bytes().unwrap(), before);
    }
}

fn repeat_line_edits(sources: &[PartResolvedFeaturesArchive]) {
    for (context, source) in sources.iter().enumerate() {
        for edit in 1..=2 {
            check_line_edit(source, context, edit);
        }
    }
}

#[test]
fn parting_line_snapshots_isolate_eight_concurrent_workers() {
    let sources: Vec<_> = CASES
        .iter()
        .enumerate()
        .map(|(index, cases)| line_archive(&prepared_saved_manager_document(cases[0]), index))
        .collect();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| repeat_line_edits(&sources)))
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
    repeat_line_edits(&sources);
}

#[test]
fn rejected_line_edits_and_source_mutation_do_not_change_published_archives() {
    for (context, cases) in CASES.iter().enumerate() {
        let mut document = prepared_saved_manager_document(cases[0]);
        let before = document.to_bytes().unwrap();
        let source = line_archive(&document, context);
        let mut fields = line_fields(&source, 1);
        fields
            .iter_mut()
            .find(|field| field.path.ends_with("data_800.raw_18"))
            .unwrap()
            .value = ModelFieldValue::I32(0);
        assert!(source.edit_fields(fields).is_err());
        assert_eq!(document.to_bytes().unwrap(), before);
        document
            .replace_stream(OWNED_RESOLVED_STREAM, vec![0])
            .unwrap();
        check_line_edit(&source, context, 1);
        assert_eq!(document.read_stream(OWNED_RESOLVED_STREAM).unwrap(), &[0]);
    }
}
