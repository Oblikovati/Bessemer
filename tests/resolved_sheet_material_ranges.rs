//! Containing sheet-metal material edits with independent expected streams (#214).
use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::{
    ConfigurationId, ModelArchiveLimits, ModelEditCheckMode, ModelEditCompletion,
    ModelEditFidelity, ModelField, ModelFieldRelation, ModelFieldValue,
    PartResolvedFeaturesArchive, PartResolvedFeaturesEdit,
};

const STREAM: &str = "Contents/Config-0-ResolvedFeatures";
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

struct MaterialCase {
    kind: u8,
    state: u8,
    edit: u8,
    raw: &'static [u8],
}

fn take_material<'a>(cursor: &mut &'a [u8], count: usize) -> &'a [u8] {
    let (value, rest) = cursor.split_at(count);
    *cursor = rest;
    value
}

fn material_case(cursor: &mut &'static [u8]) -> MaterialCase {
    let prefix = take_material(cursor, 3);
    let length = u32::from_le_bytes(take_material(cursor, 4).try_into().unwrap()) as usize;
    MaterialCase {
        kind: prefix[0],
        state: prefix[1],
        edit: prefix[2],
        raw: take_material(cursor, length),
    }
}

fn material_cases() -> Vec<MaterialCase> {
    let mut cursor = include_bytes!("fixtures/sheet-material-ranges/owned-cases.bin").as_slice();
    assert_eq!(take_material(&mut cursor, 4), b"BMR1");
    assert_eq!(
        u32::from_le_bytes(take_material(&mut cursor, 4).try_into().unwrap()),
        44
    );
    let cases = (0..44).map(|_| material_case(&mut cursor)).collect();
    assert!(cursor.is_empty());
    cases
}

fn material_document(case: &MaterialCase) -> Document {
    let mut document = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    document.replace_stream(STREAM, case.raw.to_vec()).unwrap();
    let manager: &[u8] = if case.state == 0 {
        include_bytes!("fixtures/sheet-metal/manager-0.bin")
    } else {
        include_bytes!("fixtures/sheet-metal/manager-1.bin")
    };
    document
        .replace_stream("Contents/CMgr", manager.to_vec())
        .unwrap();
    document
}

fn material_archive(document: &Document) -> PartResolvedFeaturesArchive {
    PartResolvedFeaturesArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap()
}

fn edited_material_fields(
    source: &PartResolvedFeaturesArchive,
    target: &PartResolvedFeaturesArchive,
) -> Vec<ModelField> {
    let mut fields = source.fields().to_vec();
    assert_eq!(fields.len(), target.fields().len());
    let mut changed = 0;
    for (field, expected) in fields.iter_mut().zip(target.fields()) {
        assert_eq!(field.path, expected.path);
        if field.value != expected.value {
            assert!(field.path.contains(".material_128."));
            field.value = expected.value.clone();
            changed += 1;
        }
    }
    assert_eq!(changed, 1);
    fields
}

fn assert_material_contract(source: &PartResolvedFeaturesArchive, edit: &PartResolvedFeaturesEdit) {
    assert_eq!(edit.completion(), ModelEditCompletion::Changed);
    assert_eq!(edit.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(edit.coverage().mode, ModelEditCheckMode::ExactStoredArchive);
    assert_eq!(edit.coverage(), edit.archive().coverage());
    assert_eq!(edit.coverage().typed_fields, edit.archive().fields().len());
    assert_eq!(edit.diagnostics(), source.diagnostics());
    assert_eq!(edit.unchanged_fields(), source.fields().len() - 1);
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
fn complete_material_documents_reconstruct_saved_states_and_object_identity() {
    for case in material_cases() {
        assert!(case.kind < 2 && case.edit <= 10);
        let mut document = material_document(&case);
        let original = document.to_bytes().unwrap();
        let archive = material_archive(&document);
        assert_eq!(archive.object_slots(), (97, 103));
        assert_eq!(archive.to_bytes().unwrap(), case.raw);
        document
            .replace_stream(STREAM, archive.to_bytes().unwrap())
            .unwrap();
        assert_eq!(document.to_bytes().unwrap(), original);
        let reopened = Document::from_bytes(&original).unwrap();
        assert_eq!(material_archive(&reopened).to_bytes().unwrap(), case.raw);
    }
}

fn check_material_edit(source: &PartResolvedFeaturesArchive, case: &MaterialCase) {
    let target = material_archive(&material_document(case));
    let edit = source
        .edit_fields(edited_material_fields(source, &target))
        .unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), case.raw);
    assert_eq!(edit.changes().len(), 1);
    assert_material_contract(source, &edit);
    let fresh = material_archive(&material_document(case));
    let reverse = fresh
        .edit_fields(edited_material_fields(&fresh, source))
        .unwrap();
    assert_eq!(
        reverse.archive().to_bytes().unwrap(),
        source.to_bytes().unwrap()
    );
    assert_material_contract(&fresh, &reverse);
}

#[test]
fn forty_material_edits_and_fresh_reversals_preserve_history_contracts_and_inputs() {
    for cases in material_cases().as_chunks::<11>().0 {
        let document = material_document(&cases[0]);
        let original = document.to_bytes().unwrap();
        let source = material_archive(&document);
        for case in &cases[1..] {
            check_material_edit(&source, case);
        }
        assert_eq!(source.to_bytes().unwrap(), cases[0].raw);
        assert_eq!(document.to_bytes().unwrap(), original);
    }
}

#[test]
fn rejected_material_type_edits_preserve_source_and_document() {
    for cases in material_cases().as_chunks::<11>().0 {
        let document = material_document(&cases[0]);
        let original = document.to_bytes().unwrap();
        let source = material_archive(&document);
        let mut fields = source.fields().to_vec();
        fields
            .iter_mut()
            .find(|field| field.path.ends_with(".material_128.ranges[0].raw_2c"))
            .unwrap()
            .value = ModelFieldValue::I32(-1);
        assert!(source.edit_fields(fields).is_err());
        assert_eq!(source.to_bytes().unwrap(), cases[0].raw);
        assert_eq!(document.to_bytes().unwrap(), original);
    }
}

fn assert_material_send_sync<T: Send + Sync>() {
    let size = std::mem::size_of::<T>();
    assert!(size > 0);
}

fn repeat_material_edits(sources: &[PartResolvedFeaturesArchive], cases: &[MaterialCase]) {
    for (source, group) in sources.iter().zip(cases.as_chunks::<11>().0) {
        for case in &group[1..] {
            check_material_edit(source, case);
        }
        assert_eq!(source.to_bytes().unwrap(), group[0].raw);
    }
}

#[test]
fn published_material_archives_support_concurrent_isolated_edits() {
    assert_material_send_sync::<PartResolvedFeaturesArchive>();
    assert_material_send_sync::<PartResolvedFeaturesEdit>();
    assert_material_send_sync::<ModelField>();
    let cases = material_cases();
    let sources: Vec<_> = cases
        .as_chunks::<11>()
        .0
        .iter()
        .map(|group| material_archive(&material_document(&group[0])))
        .collect();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| repeat_material_edits(&sources, &cases)))
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
}

#[test]
fn changing_source_document_after_publication_does_not_change_material_archive() {
    for group in material_cases().as_chunks::<11>().0 {
        let mut document = material_document(&group[0]);
        let archive = material_archive(&document);
        document.replace_stream(STREAM, vec![0]).unwrap();
        document.replace_stream("Contents/CMgr", vec![0]).unwrap();
        check_material_edit(&archive, &group[1]);
        assert_eq!(archive.to_bytes().unwrap(), group[0].raw);
        assert_eq!(document.read_stream(STREAM).unwrap(), &[0]);
    }
}
