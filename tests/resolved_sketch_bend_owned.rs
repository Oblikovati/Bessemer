//! Canonical kind-7 profile persists through complete documents and edit revalidation (#219).
#[path = "support/owned_resolved_document.rs"]
mod owned_resolved_document;
#[path = "support/owned_saved_manager_document.rs"]
mod owned_saved_manager_document;
use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::{
    ConfigurationId, PartConfigurationArchive, PartConfigurationContext, SketchBendOwnedProfile,
};
use bessemer::model_archives::{
    ModelArchiveLimits, ModelEditCheckMode, ModelEditCompletion, ModelEditFidelity, ModelField,
    ModelFieldRelation, ModelFieldValue, PartResolvedFeaturesArchive, PartResolvedFeaturesEdit,
    StoredDouble,
};
use bessemer::parasolid::{BaseSchema, TransmitLimits};
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
        include_bytes!("fixtures/sketch-bend-owned/owned-empty-0.bin"),
        include_bytes!("fixtures/sketch-bend-owned/owned-empty-1.bin"),
        include_bytes!("fixtures/sketch-bend-owned/owned-empty-2.bin"),
    ],
    [
        include_bytes!("fixtures/sketch-bend-owned/owned-rich-0.bin"),
        include_bytes!("fixtures/sketch-bend-owned/owned-rich-1.bin"),
        include_bytes!("fixtures/sketch-bend-owned/owned-rich-2.bin"),
    ],
    [
        include_bytes!("fixtures/sketch-bend-owned/owned-aliases-0.bin"),
        include_bytes!("fixtures/sketch-bend-owned/owned-aliases-1.bin"),
        include_bytes!("fixtures/sketch-bend-owned/owned-aliases-2.bin"),
    ],
    [
        include_bytes!("fixtures/sketch-bend-owned/owned-helper-empty-0.bin"),
        include_bytes!("fixtures/sketch-bend-owned/owned-helper-empty-1.bin"),
        include_bytes!("fixtures/sketch-bend-owned/owned-helper-empty-2.bin"),
    ],
    [
        include_bytes!("fixtures/sketch-bend-owned/owned-helper-rich-0.bin"),
        include_bytes!("fixtures/sketch-bend-owned/owned-helper-rich-1.bin"),
        include_bytes!("fixtures/sketch-bend-owned/owned-helper-rich-2.bin"),
    ],
    [
        include_bytes!("fixtures/sketch-bend-owned/owned-helper-aliases-0.bin"),
        include_bytes!("fixtures/sketch-bend-owned/owned-helper-aliases-1.bin"),
        include_bytes!("fixtures/sketch-bend-owned/owned-helper-aliases-2.bin"),
    ],
];

#[test]
fn legacy_one_bend_documents_keep_the_default_unselected_profile() {
    let raw = include_bytes!("fixtures/edge-flanges/owned-2-0.bin");
    let document = prepared_saved_manager_document(raw);
    let archive = owned_resolved_document::read_owned_resolved_document(&document, LIMITS);
    assert_eq!(archive.to_bytes().unwrap(), raw);
    assert_eq!(archive.object_slots(), (97, 101));
}

fn owned_document(raw: &[u8], context: usize) -> Document {
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

fn owned_kernel() -> bessemer::model_archives::CompressedKernelContext {
    bessemer::model_archives::CompressedKernelContext {
        base: BaseSchema::from_bytes(include_bytes!("fixtures/parasolid/base-schema.s_t")).unwrap(),
        maximum_expanded: 1024 * 1024,
        transmit_limits: TransmitLimits {
            maximum_records: 1000,
            maximum_field_elements: 10000,
        },
    }
}

fn owned_source(document: &Document, profile: bool, kernel: bool) -> PartConfigurationArchive {
    let mut context = PartConfigurationContext::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        kernel.then(owned_kernel),
        LIMITS,
    )
    .unwrap();
    if profile {
        context =
            context.with_sketch_bend_owned_profile(SketchBendOwnedProfile::CanonicalRuledGrid);
    }
    let raw = document.read_stream("Contents/Config-0").unwrap();
    PartConfigurationArchive::from_bytes(&raw, context, LIMITS).unwrap()
}

fn owned_archive(document: &Document) -> PartResolvedFeaturesArchive {
    let source = owned_source(document, true, true);
    owned_from_source(document, source).unwrap()
}

fn owned_from_source(
    document: &Document,
    source: PartConfigurationArchive,
) -> bessemer::Result<PartResolvedFeaturesArchive> {
    let bytes = document.read_stream(OWNED_RESOLVED_STREAM).unwrap();
    let manager = document.read_stream("Contents/CMgr").unwrap();
    PartResolvedFeaturesArchive::from_bytes(&bytes, source, &manager, LIMITS)
}

#[test]
fn profile_and_kernel_selection_are_independent_required_contexts() {
    let document = owned_document(CASES[0][0], 0);
    for (profile, kernel, expected) in [
        (false, true, bessemer::FailureClass::Unsupported),
        (true, false, bessemer::FailureClass::Missing),
    ] {
        let source = owned_source(&document, profile, kernel);
        let error = owned_from_source(&document, source).unwrap_err();
        assert_eq!(error.class, expected);
    }
}

#[test]
fn source_field_reconstruction_retains_the_explicit_profile_for_dependent_features() {
    let document = owned_document(CASES[0][0], 0);
    let source = owned_source(&document, true, true);
    let rebuilt = PartConfigurationArchive::from_fields(
        source.fields().to_vec(),
        source.context().clone(),
        LIMITS,
    )
    .unwrap();
    assert_eq!(rebuilt.to_bytes().unwrap(), source.to_bytes().unwrap());
    let archive = owned_from_source(&document, rebuilt).unwrap();
    assert_eq!(archive.to_bytes().unwrap(), CASES[0][0]);
    check_owned_edit(&archive, 0, 1);
}

#[test]
fn canonical_profile_is_send_and_sync_without_runtime_algorithm_state() {
    let profile = SketchBendOwnedProfile::CanonicalRuledGrid;
    std::thread::scope(|scope| {
        let shared = &profile;
        let worker = scope.spawn(move || *shared);
        assert_eq!(worker.join().unwrap(), profile);
    });
}

fn owned_replacement(context: usize, edit: usize) -> (String, ModelFieldValue) {
    let root = if context % 3 == 2 {
        "root.features[0].items[0]"
    } else {
        "root.features[0]"
    };
    let (suffix, value) = match edit {
        1 => (
            "owned_be8.grid_58.raw_e8",
            ModelFieldValue::F64(StoredDouble::from_bits(0x7ff0000000000001)),
        ),
        2 => ("owned_be8.raw_138", ModelFieldValue::I32(12345)),
        _ => panic!("edit {edit}; expected grid scalar1 or face word2"),
    };
    (format!("{root}.{suffix}"), value)
}

fn owned_fields(
    source: &PartResolvedFeaturesArchive,
    context: usize,
    edit: usize,
) -> Vec<ModelField> {
    let (path, value) = owned_replacement(context, edit);
    let mut fields = source.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value = value;
    fields
}

fn assert_owned_contract(source: &PartResolvedFeaturesArchive, edit: &PartResolvedFeaturesEdit) {
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
fn all_owned_documents_reconstruct_whole_carriers_and_shared_maps() {
    for (context, cases) in CASES.iter().enumerate() {
        for raw in cases {
            let mut document = owned_document(raw, context);
            let before = document.to_bytes().unwrap();
            let archive = owned_archive(&document);
            assert_eq!(archive.to_bytes().unwrap(), *raw);
            assert_eq!(
                archive.object_slots(),
                (97, [101, 119, 127, 101, 117, 125][context])
            );
            document
                .replace_stream(OWNED_RESOLVED_STREAM, archive.to_bytes().unwrap())
                .unwrap();
            assert_eq!(document.to_bytes().unwrap(), before);
        }
    }
}

fn check_owned_edit(source: &PartResolvedFeaturesArchive, context: usize, edit_index: usize) {
    let edit = source
        .edit_fields(owned_fields(source, context, edit_index))
        .unwrap();
    assert_eq!(
        edit.archive().to_bytes().unwrap(),
        CASES[context][edit_index]
    );
    assert_owned_contract(source, &edit);
    let fresh = owned_archive(&owned_document(CASES[context][edit_index], context));
    let reverse = fresh.edit_fields(source.fields().to_vec()).unwrap();
    assert_eq!(reverse.archive().to_bytes().unwrap(), CASES[context][0]);
    assert_owned_contract(&fresh, &reverse);
    assert_eq!(source.to_bytes().unwrap(), CASES[context][0]);
}

#[test]
fn twelve_owned_edits_and_fresh_reversals_preserve_history_and_inputs() {
    for (context, cases) in CASES.iter().enumerate() {
        let document = owned_document(cases[0], context);
        let before = document.to_bytes().unwrap();
        let source = owned_archive(&document);
        for edit in 1..=2 {
            check_owned_edit(&source, context, edit);
        }
        assert_eq!(document.to_bytes().unwrap(), before);
    }
}

fn repeat_owned_edits(sources: &[PartResolvedFeaturesArchive]) {
    for (context, source) in sources.iter().enumerate() {
        for edit in 1..=2 {
            check_owned_edit(source, context, edit);
        }
    }
}

#[test]
fn owned_snapshots_isolate_eight_concurrent_workers() {
    let sources: Vec<_> = CASES
        .iter()
        .enumerate()
        .map(|(index, cases)| owned_archive(&owned_document(cases[0], index)))
        .collect();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| repeat_owned_edits(&sources)))
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
    repeat_owned_edits(&sources);
}

#[test]
fn rejected_owned_edits_and_source_mutation_do_not_change_published_archives() {
    for (context, cases) in CASES.iter().enumerate() {
        let mut document = owned_document(cases[0], context);
        let before = document.to_bytes().unwrap();
        let source = owned_archive(&document);
        let mut fields = owned_fields(&source, context, 1);
        fields
            .iter_mut()
            .find(|field| field.path.ends_with("owned_be8.grid_58.raw_e8"))
            .unwrap()
            .value = ModelFieldValue::I32(0);
        assert!(source.edit_fields(fields).is_err());
        assert_eq!(document.to_bytes().unwrap(), before);
        document
            .replace_stream(OWNED_RESOLVED_STREAM, vec![0])
            .unwrap();
        check_owned_edit(&source, context, 1);
        assert_eq!(document.read_stream(OWNED_RESOLVED_STREAM).unwrap(), &[0]);
    }
}
