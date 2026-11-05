//! Complete document sweep-twist round trips and independently expected field edits (#211).
#[path = "support/sweep_twist_cases.rs"]
mod sweep_twist_cases;
use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::{
    ConfigurationId, ModelArchiveLimits, ModelFieldValue, ModelObjectRecord,
    PartResolvedFeaturesArchive, StoredDouble,
};
use sweep_twist_cases::{OwnedTwistCase, owned_twist_cases};

const STREAM: &str = "Contents/Config-0-ResolvedFeatures";
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

fn document_with(raw: &[u8]) -> Document {
    let mut document = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    document.replace_stream(STREAM, raw.to_vec()).unwrap();
    document
}

fn parsed(document: &Document) -> PartResolvedFeaturesArchive {
    PartResolvedFeaturesArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap()
}

fn assert_twist_targets(archive: &PartResolvedFeaturesArchive, case: &OwnedTwistCase) {
    let targets: Vec<_> = archive.fields().iter().filter_map(twist_target).collect();
    let expected = if case.label.contains("-shared-") {
        vec![case.object_slots[0]; case.payloads.len()]
    } else {
        case.object_slots.clone()
    };
    assert_eq!(targets, expected);
}

fn twist_target(field: &bessemer::model_archives::ModelField) -> Option<u32> {
    if !field.path.ends_with("data.twist_298") {
        return None;
    }
    let ModelFieldValue::ObjectTag(tag) = &field.value else {
        panic!("expected twist pointer")
    };
    match &tag.record {
        ModelObjectRecord::New { index, .. } | ModelObjectRecord::Reference { index, .. } => {
            Some(index.value())
        }
        ModelObjectRecord::Null => panic!("expected populated twist"),
    }
}

#[test]
fn all_owned_sweep_twists_reconstruct_documents_and_original_pointer_identities() {
    for case in owned_twist_cases() {
        let mut document = document_with(case.raw);
        let original = document.to_bytes().unwrap();
        let archive = parsed(&document);
        assert_eq!(archive.object_slots(), (97, case.slots));
        assert_eq!(archive.to_bytes().unwrap(), case.raw);
        assert_twist_targets(&archive, &case);
        document
            .replace_stream(STREAM, archive.to_bytes().unwrap())
            .unwrap();
        assert_eq!(document.to_bytes().unwrap(), original);
        let reopened = Document::from_bytes(&original).unwrap();
        assert_eq!(parsed(&reopened).to_bytes().unwrap(), case.raw);
    }
}

fn edited_first_twist(
    archive: &PartResolvedFeaturesArchive,
    offset: usize,
) -> Vec<bessemer::model_archives::ModelField> {
    let mut fields = archive.fields().to_vec();
    let suffix = ["raw_08", "raw_10", "raw_18"][offset];
    let path = format!("twist_298.{suffix}");
    let field = fields
        .iter_mut()
        .find(|field| field.path.ends_with(&path))
        .unwrap();
    field.value = twist_edit_value(offset);
    fields
}

fn twist_edit_value(offset: usize) -> ModelFieldValue {
    match offset {
        0 => ModelFieldValue::U8(254),
        1 => ModelFieldValue::F64(StoredDouble::from_bits(0x8000000000000001)),
        2 => ModelFieldValue::F64(StoredDouble::from_bits(0x7FF8000000001235)),
        _ => panic!("expected field index 0..3"),
    }
}

#[test]
fn sweep_twist_edits_match_independent_streams_and_fresh_reversals() {
    let cases = owned_twist_cases();
    for group in cases.as_chunks::<4>().0 {
        let original = parsed(&document_with(group[0].raw));
        for offset in 0..3 {
            let edit = original
                .edit_fields(edited_first_twist(&original, offset))
                .unwrap();
            assert_eq!(edit.archive().to_bytes().unwrap(), group[offset + 1].raw);
            assert_eq!(edit.changes().len(), 1);
            let fresh = parsed(&document_with(group[offset + 1].raw));
            let reverse = fresh.edit_fields(original.fields().to_vec()).unwrap();
            assert_eq!(reverse.archive().to_bytes().unwrap(), group[0].raw);
            assert_eq!(original.to_bytes().unwrap(), group[0].raw);
        }
    }
}

fn assert_twist_send_sync<T: Send + Sync>() {
    assert!(std::mem::size_of::<T>() > 0);
}

fn concurrent_twist_edits(archives: &[PartResolvedFeaturesArchive]) {
    for archive in archives {
        let original = archive.to_bytes().unwrap();
        let edit = archive.edit_fields(edited_first_twist(archive, 1)).unwrap();
        assert_ne!(edit.archive().to_bytes().unwrap(), original);
        assert_eq!(archive.to_bytes().unwrap(), original);
    }
}

#[test]
fn published_sweep_twists_support_concurrent_isolated_edits() {
    assert_twist_send_sync::<PartResolvedFeaturesArchive>();
    let cases = owned_twist_cases();
    let archives: Vec<_> = cases
        .as_chunks::<4>()
        .0
        .iter()
        .map(|group| parsed(&document_with(group[0].raw)))
        .collect();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| concurrent_twist_edits(&archives)))
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
    concurrent_twist_edits(&archives);
}

#[test]
fn rejected_sweep_twist_field_types_leave_source_and_document_unchanged() {
    let case = owned_twist_cases().remove(0);
    let document = document_with(case.raw);
    let original = document.to_bytes().unwrap();
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.ends_with("twist_298.raw_08"))
        .unwrap()
        .value = ModelFieldValue::I32(-1);
    assert!(archive.edit_fields(fields).is_err());
    assert_eq!(archive.to_bytes().unwrap(), case.raw);
    assert_eq!(document.to_bytes().unwrap(), original);
}

fn assert_twist_history(source: &PartResolvedFeaturesArchive, offset: usize) {
    use bessemer::model_archives::ModelFieldRelation;
    let edit = source
        .edit_fields(edited_first_twist(source, offset))
        .unwrap();
    assert_twist_result_contract(source, &edit);
    let change = &edit.changes()[0];
    assert_eq!(change.relation(), ModelFieldRelation::Derived);
    assert_eq!(change.parents().len(), 1);
    let parent = source.resolve_field(&change.parents()[0]).unwrap();
    let result = edit
        .archive()
        .resolve_field(change.result().unwrap())
        .unwrap();
    assert_eq!(change.path(), parent.path.as_ref());
    assert_eq!(parent.path, result.path);
    assert_ne!(parent.value, result.value);
    assert!(source.resolve_field(change.result().unwrap()).is_err());
    assert!(edit.archive().resolve_field(&change.parents()[0]).is_err());
    assert_eq!(edit.unchanged_fields(), source.fields().len() - 1);
}

#[test]
fn all_twist_field_edits_return_complete_revision_bound_derived_history() {
    for group in owned_twist_cases().as_chunks::<4>().0 {
        let source = parsed(&document_with(group[0].raw));
        for offset in 0..3 {
            assert_twist_history(&source, offset);
        }
    }
}

fn assert_twist_result_contract(
    source: &PartResolvedFeaturesArchive,
    edit: &bessemer::model_archives::PartResolvedFeaturesEdit,
) {
    use bessemer::model_archives::{ModelEditCheckMode, ModelEditCompletion, ModelEditFidelity};
    assert_eq!(edit.completion(), ModelEditCompletion::Changed);
    assert_eq!(edit.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(edit.coverage().mode, ModelEditCheckMode::ExactStoredArchive);
    assert_eq!(edit.coverage().wire_bytes, source.to_bytes().unwrap().len());
    assert_eq!(edit.coverage().typed_fields, edit.archive().fields().len());
    assert_eq!(edit.coverage(), edit.archive().coverage());
    assert_eq!(edit.diagnostics(), source.diagnostics());
}
