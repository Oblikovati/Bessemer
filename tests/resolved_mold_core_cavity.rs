//! Complete core/cavity documents with owner cycles, aliases and literal edits (#215).
#[path = "support/owned_resolved_document.rs"]
mod owned_resolved_document;
#[path = "support/owned_saved_manager_document.rs"]
mod owned_saved_manager_document;
use bessemer::model_archives::{
    ModelArchiveLimits, ModelEditCheckMode, ModelEditCompletion, ModelEditFidelity, ModelField,
    ModelFieldRelation, ModelFieldValue, PartResolvedFeaturesArchive, PartResolvedFeaturesEdit,
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
const SETBACK: &str = "root.features[0].data_800.raw_34";
type MoldCase = (&'static [u8], &'static [u8], usize);

fn mold_cases() -> [MoldCase; 2] {
    [
        (
            include_bytes!("fixtures/mold-core-cavity/owned-0.bin"),
            include_bytes!("fixtures/mold-core-cavity/owned-0-edited.bin"),
            103,
        ),
        (
            include_bytes!("fixtures/mold-core-cavity/owned-1.bin"),
            include_bytes!("fixtures/mold-core-cavity/owned-1-edited.bin"),
            115,
        ),
    ]
}

fn read_mold(raw: &[u8]) -> PartResolvedFeaturesArchive {
    let document = prepared_saved_manager_document(raw);
    let archive = read_owned_resolved_document(&document, LIMITS);
    assert_eq!(archive.to_bytes().unwrap(), raw);
    archive
}

fn mold_changed_fields(
    source: &PartResolvedFeaturesArchive,
    value: ModelFieldValue,
) -> Vec<ModelField> {
    let mut fields = source.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == SETBACK)
        .unwrap()
        .value = value;
    fields
}

fn mold_setback_edit(source: &PartResolvedFeaturesArchive, value: i32) -> PartResolvedFeaturesEdit {
    let fields = mold_changed_fields(source, ModelFieldValue::I32(value));
    source.edit_fields(fields).unwrap()
}

fn assert_mold_history(source: &PartResolvedFeaturesArchive, edit: &PartResolvedFeaturesEdit) {
    assert_eq!(edit.changes().len(), 1);
    let change = &edit.changes()[0];
    assert_eq!(change.relation(), ModelFieldRelation::Derived);
    assert_eq!(change.path(), SETBACK);
    assert_eq!(change.parents().len(), 1);
    let parent = source.resolve_field(&change.parents()[0]).unwrap();
    let result = edit.archive();
    let child = result.resolve_field(change.result().unwrap()).unwrap();
    assert_eq!(parent.path, child.path);
    assert_ne!(parent.value, child.value);
    assert!(source.resolve_field(change.result().unwrap()).is_err());
    assert!(result.resolve_field(&change.parents()[0]).is_err());
}

fn assert_mold_edit(
    source: &PartResolvedFeaturesArchive,
    edit: &PartResolvedFeaturesEdit,
    expected: &[u8],
) {
    assert_eq!(edit.archive().to_bytes().unwrap(), expected);
    assert_eq!(edit.completion(), ModelEditCompletion::Changed);
    assert_eq!(edit.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(edit.coverage().mode, ModelEditCheckMode::ExactStoredArchive);
    assert_eq!(edit.coverage(), edit.archive().coverage());
    assert_eq!(edit.coverage().wire_bytes, expected.len());
    assert_eq!(edit.coverage().typed_fields, source.fields().len());
    assert_eq!(edit.unchanged_fields(), source.fields().len() - 1);
    assert_eq!(edit.diagnostics(), source.diagnostics());
    assert_mold_history(source, edit);
}

#[test]
fn complete_mold_documents_preserve_whole_carriers_and_imported_object_maps() {
    for (raw, edited, slots) in mold_cases() {
        for bytes in [raw, edited] {
            let mut document = prepared_saved_manager_document(bytes);
            let original = document.to_bytes().unwrap();
            let archive = read_owned_resolved_document(&document, LIMITS);
            assert_eq!(archive.object_slots(), (97, slots));
            document
                .replace_stream(OWNED_RESOLVED_STREAM, archive.to_bytes().unwrap())
                .unwrap();
            assert_eq!(document.to_bytes().unwrap(), original);
        }
    }
}

#[test]
fn mold_setback_edits_and_fresh_reversals_return_complete_revision_bound_history() {
    for (raw, edited, _) in mold_cases() {
        let document = prepared_saved_manager_document(raw);
        let before = document.to_bytes().unwrap();
        let source = read_owned_resolved_document(&document, LIMITS);
        assert_mold_edit(&source, &mold_setback_edit(&source, 12345), edited);
        let fresh = read_mold(edited);
        assert_mold_edit(&fresh, &mold_setback_edit(&fresh, i32::MIN), raw);
        assert_eq!(source.to_bytes().unwrap(), raw);
        assert_eq!(document.to_bytes().unwrap(), before);
    }
}

fn repeat_mold_edits(sources: &[PartResolvedFeaturesArchive], cases: &[MoldCase]) {
    for (source, (raw, edited, _)) in sources.iter().zip(cases) {
        assert_mold_edit(source, &mold_setback_edit(source, 12345), edited);
        assert_eq!(source.to_bytes().unwrap(), *raw);
    }
}

fn assert_mold_send_sync<T: Send + Sync>() {
    let size = std::mem::size_of::<T>();
    assert!(size > 0);
}

#[test]
fn published_mold_archives_support_eight_concurrent_isolated_workers() {
    assert_mold_send_sync::<PartResolvedFeaturesArchive>();
    assert_mold_send_sync::<PartResolvedFeaturesEdit>();
    let cases = mold_cases();
    let sources: Vec<_> = cases.iter().map(|(raw, _, _)| read_mold(raw)).collect();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| repeat_mold_edits(&sources, &cases)))
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
    repeat_mold_edits(&sources, &cases);
}

#[test]
fn failed_mold_edits_and_later_source_mutation_leave_published_archives_isolated() {
    for (raw, edited, _) in mold_cases() {
        let mut document = prepared_saved_manager_document(raw);
        let before = document.to_bytes().unwrap();
        let source = read_owned_resolved_document(&document, LIMITS);
        let fields = mold_changed_fields(&source, ModelFieldValue::U8(0));
        assert!(source.edit_fields(fields).is_err());
        assert_eq!(document.to_bytes().unwrap(), before);
        document
            .replace_stream(OWNED_RESOLVED_STREAM, vec![0])
            .unwrap();
        assert_mold_edit(&source, &mold_setback_edit(&source, 12345), edited);
        assert_eq!(source.to_bytes().unwrap(), raw);
        assert_eq!(document.read_stream(OWNED_RESOLVED_STREAM).unwrap(), &[0]);
    }
}
