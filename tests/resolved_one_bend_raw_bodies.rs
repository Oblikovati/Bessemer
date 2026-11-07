//! Complete raw-body list and optional-body edits retain carriers and history (#220).
#[path = "support/owned_resolved_document.rs"]
mod owned_resolved_document;
#[path = "support/owned_saved_manager_document.rs"]
mod owned_saved_manager_document;
use bessemer::model_archives::{
    CompressedKernelContext, ConfigurationId, ModelArchiveLimits, ModelEditCheckMode,
    ModelEditCompletion, ModelEditFidelity, ModelField, ModelFieldRelation, ModelFieldValue,
    PartConfigurationArchive, PartConfigurationContext, PartResolvedFeaturesArchive,
};
use bessemer::parasolid::{BaseSchema, FieldValues, TransmitBody, TransmitLimits};
use bessemer::{Document, header::ArchiveVersion};
use owned_resolved_document::OWNED_RESOLVED_STREAM;
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
        include_bytes!("fixtures/one-bend-raw-bodies/owned-empty-0.bin"),
        include_bytes!("fixtures/one-bend-raw-bodies/owned-empty-1.bin"),
        include_bytes!("fixtures/one-bend-raw-bodies/owned-empty-2.bin"),
    ],
    [
        include_bytes!("fixtures/one-bend-raw-bodies/owned-rich-0.bin"),
        include_bytes!("fixtures/one-bend-raw-bodies/owned-rich-1.bin"),
        include_bytes!("fixtures/one-bend-raw-bodies/owned-rich-2.bin"),
    ],
    [
        include_bytes!("fixtures/one-bend-raw-bodies/owned-aliases-0.bin"),
        include_bytes!("fixtures/one-bend-raw-bodies/owned-aliases-1.bin"),
        include_bytes!("fixtures/one-bend-raw-bodies/owned-aliases-2.bin"),
    ],
    [
        include_bytes!("fixtures/one-bend-raw-bodies/owned-helper-empty-0.bin"),
        include_bytes!("fixtures/one-bend-raw-bodies/owned-helper-empty-1.bin"),
        include_bytes!("fixtures/one-bend-raw-bodies/owned-helper-empty-2.bin"),
    ],
    [
        include_bytes!("fixtures/one-bend-raw-bodies/owned-helper-rich-0.bin"),
        include_bytes!("fixtures/one-bend-raw-bodies/owned-helper-rich-1.bin"),
        include_bytes!("fixtures/one-bend-raw-bodies/owned-helper-rich-2.bin"),
    ],
    [
        include_bytes!("fixtures/one-bend-raw-bodies/owned-helper-aliases-0.bin"),
        include_bytes!("fixtures/one-bend-raw-bodies/owned-helper-aliases-1.bin"),
        include_bytes!("fixtures/one-bend-raw-bodies/owned-helper-aliases-2.bin"),
    ],
];

fn kernel_context() -> CompressedKernelContext {
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
fn empty_legacy_bodies_keep_the_existing_context_free_document_path() {
    let raw = include_bytes!("fixtures/edge-flanges/owned-2-0.bin");
    let document = prepared_saved_manager_document(raw);
    let archive = owned_resolved_document::read_owned_resolved_document(&document, LIMITS);
    assert_eq!(archive.to_bytes().unwrap(), raw);
}

fn complete_document(raw: &[u8], context: usize) -> Document {
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

fn read_archive(
    document: &Document,
    kernel: bool,
) -> bessemer::Result<PartResolvedFeaturesArchive> {
    let source = read_source(document, kernel)?;
    PartResolvedFeaturesArchive::from_bytes(
        &document.read_stream(OWNED_RESOLVED_STREAM)?,
        source,
        &document.read_stream("Contents/CMgr")?,
        LIMITS,
    )
}

fn read_source(document: &Document, kernel: bool) -> bessemer::Result<PartConfigurationArchive> {
    let context = PartConfigurationContext::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        kernel.then(kernel_context),
        LIMITS,
    )?;
    PartConfigurationArchive::from_bytes(
        &document.read_stream("Contents/Config-0")?,
        context,
        LIMITS,
    )
}

fn edited_fields(source: &PartResolvedFeaturesArchive, edit: usize) -> Vec<ModelField> {
    let mut fields = source.fields().to_vec();
    for field in &mut fields {
        if edit == 1 {
            replace_identifier(field);
        }
        if edit == 2 && field.path.ends_with("body_presence_b40") {
            field.value = ModelFieldValue::I32(-1);
        }
    }
    fields
}

fn replace_identifier(field: &mut ModelField) {
    let ModelFieldValue::Kernel(frame) = &mut field.value else {
        return;
    };
    let mut contents = frame.contents().clone();
    let TransmitBody::Partition(list) = &mut contents.body else {
        panic!("partition")
    };
    list.records[0].fields[0] = FieldValues::Integers(vec![9]);
    **frame = frame.with_contents(contents).unwrap();
}

#[test]
fn complete_documents_preserve_all_frames_shared_slots_and_carrier_bytes() {
    for (context, cases) in CASES.iter().enumerate() {
        for raw in cases {
            let mut document = complete_document(raw, context);
            let original = document.to_bytes().unwrap();
            let archive = read_archive(&document, true).unwrap();
            assert_eq!(archive.to_bytes().unwrap(), *raw);
            assert_eq!(
                archive.object_slots(),
                (97, [101, 113, 121, 101, 111, 119][context])
            );
            assert_kernel_count(&archive, context);
            document
                .replace_stream(OWNED_RESOLVED_STREAM, archive.to_bytes().unwrap())
                .unwrap();
            assert_eq!(document.to_bytes().unwrap(), original);
        }
    }
}

fn assert_kernel_count(archive: &PartResolvedFeaturesArchive, context: usize) {
    let count = archive
        .fields()
        .iter()
        .filter(|field| matches!(field.value, ModelFieldValue::Kernel(_)))
        .count();
    assert_eq!(count, if context % 3 == 2 { 10 } else { 5 });
}

fn check_edit(context: usize, edit_index: usize) {
    let original = read_archive(&complete_document(CASES[context][0], context), true).unwrap();
    let edit = original
        .edit_fields(edited_fields(&original, edit_index))
        .unwrap();
    assert_expected_edit(edit.archive(), context, edit_index);
    check_history(&original, &edit);
    let fresh = read_archive(
        &complete_document(CASES[context][edit_index], context),
        true,
    )
    .unwrap();
    let reverse = fresh.edit_fields(original.fields().to_vec()).unwrap();
    assert_eq!(reverse.archive().to_bytes().unwrap(), CASES[context][0]);
    assert_eq!(original.to_bytes().unwrap(), CASES[context][0]);
}

fn assert_expected_edit(archive: &PartResolvedFeaturesArchive, context: usize, edit: usize) {
    let expected = read_archive(&complete_document(CASES[context][edit], context), true).unwrap();
    assert_eq!(archive.fields().len(), expected.fields().len());
    for (actual, oracle) in archive.fields().iter().zip(expected.fields()) {
        assert_eq!(actual.path, oracle.path);
        match (&actual.value, &oracle.value) {
            (ModelFieldValue::Kernel(left), ModelFieldValue::Kernel(right)) => {
                assert_eq!(left.contents(), right.contents())
            }
            _ => assert_eq!(actual.value, oracle.value),
        }
    }
    let bytes = archive.to_bytes().unwrap();
    let fresh = read_archive(&complete_document(&bytes, context), true).unwrap();
    assert_eq!(fresh.to_bytes().unwrap(), bytes);
    if edit == 2 {
        assert_eq!(bytes, CASES[context][edit]);
    }
}

fn check_history(
    original: &PartResolvedFeaturesArchive,
    edit: &bessemer::model_archives::PartResolvedFeaturesEdit,
) {
    assert_eq!(edit.completion(), ModelEditCompletion::Changed);
    assert_eq!(edit.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(edit.coverage().mode, ModelEditCheckMode::ExactStoredArchive);
    assert_eq!(edit.diagnostics(), original.diagnostics());
    for change in edit.changes() {
        assert_eq!(change.relation(), ModelFieldRelation::Derived);
        assert_eq!(change.parents().len(), 1);
        assert!(original.resolve_field(&change.parents()[0]).is_ok());
        assert!(
            edit.archive()
                .resolve_field(change.result().unwrap())
                .is_ok()
        );
    }
}

#[test]
fn twelve_independent_edits_and_fresh_reversals_preserve_exact_history() {
    for context in 0..CASES.len() {
        for edit in 1..=2 {
            check_edit(context, edit);
        }
    }
}

#[test]
fn missing_schema_and_inconsistent_presence_edits_leave_inputs_unchanged() {
    for (context, cases) in CASES.iter().enumerate() {
        let mut document = complete_document(cases[0], context);
        let original = document.to_bytes().unwrap();
        assert_eq!(
            read_archive(&document, false).unwrap_err().class,
            bessemer::FailureClass::Missing
        );
        let source = read_archive(&document, true).unwrap();
        reject_absent_with_retained_body(&source);
        assert_eq!(document.to_bytes().unwrap(), original);
        document
            .replace_stream(OWNED_RESOLVED_STREAM, vec![0])
            .unwrap();
        assert_eq!(source.to_bytes().unwrap(), cases[0]);
    }
}

fn reject_absent_with_retained_body(source: &PartResolvedFeaturesArchive) {
    let mut fields = source.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.ends_with("body_presence_b40"))
        .unwrap()
        .value = ModelFieldValue::I32(0);
    assert!(source.edit_fields(fields).is_err());
}

#[test]
fn immutable_archives_support_eight_concurrent_edit_and_reversal_workers() {
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    for context in 0..CASES.len() {
                        for edit in 1..=2 {
                            check_edit(context, edit);
                        }
                    }
                })
            })
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
}
