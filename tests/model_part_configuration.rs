//! Complete public configuration operations, ownership, diagnostics and revision contracts.
use bessemer::model_archives::{
    ArchiveString, ConfigurationId, ModelArchiveLimits, ModelEditCheckMode, ModelEditCompletion,
    ModelEditFidelity, ModelField, ModelFieldRelation, ModelFieldValue, ModelObjectId,
    PartConfigurationArchive, PartConfigurationContext, TextEncoding,
};
use bessemer::{
    Document, FailureClass,
    header::{ArchiveVersion, HeaderObject, HeaderReference},
};
use std::sync::Arc;

const DOCUMENT: &[u8] = include_bytes!("fixtures/blank-part.SLDPRT");
const EDITED: &[u8] = include_bytes!("fixtures/model-archives/native-part-name-edited.bin");
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

fn document() -> Document {
    Document::from_bytes(DOCUMENT).unwrap()
}

fn archive() -> PartConfigurationArchive {
    PartConfigurationArchive::from_document(
        &document(),
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap()
}

fn renamed_fields(source: &PartConfigurationArchive) -> Vec<ModelField> {
    let mut fields = source.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.node.name.name")
        .unwrap()
        .value =
        ModelFieldValue::Text(ArchiveString::new("Bessemer Ω", TextEncoding::Utf16).unwrap());
    fields
}

#[test]
fn canonical_configuration_paths_normalize_once_and_reject_ambiguous_aliases() {
    for number in [0, 9, u32::MAX] {
        let identity = ConfigurationId::new(number);
        assert_eq!(
            ConfigurationId::from_stream_name(&identity.stream_name()).unwrap(),
            identity
        );
        assert_eq!(identity.value(), number);
        assert_eq!(
            identity.header_name(),
            format!("Contents/Config-{number}-ModelHeader")
        );
    }
    for name in [
        "",
        "Config-0",
        "Contents/Config-01",
        "Contents/Config-+1",
        "Contents/Config--1",
        "Contents/Config-1-ModelHeader",
        "Contents/Config-4294967296",
        "Contents/Config- 1",
        "Contents/Config-1/",
    ] {
        let error = ConfigurationId::from_stream_name(name).unwrap_err();
        assert_eq!(error.entity, name);
        assert_eq!(error.class, FailureClass::Invalid);
    }
    assert!(ModelObjectId::new(0).is_err());
    assert!(ModelObjectId::new(0x80000000).is_err());
}

#[test]
fn public_owned_read_construct_and_edit_match_independent_bytes_and_history() {
    let source = archive();
    let original = source.to_bytes().unwrap();
    let rebuilt = PartConfigurationArchive::from_fields(
        source.fields().to_vec(),
        source.context().clone(),
        LIMITS,
    )
    .unwrap();
    assert_eq!(rebuilt.to_bytes().unwrap(), original);
    let edit = source.edit_fields(renamed_fields(&source)).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), EDITED);
    assert_eq!(edit.completion(), ModelEditCompletion::Changed);
    assert_eq!(edit.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(edit.coverage().mode, ModelEditCheckMode::ExactStoredArchive);
    assert_eq!(edit.coverage().wire_bytes, 24038);
    assert_eq!(edit.coverage().typed_fields, edit.archive().fields().len());
    assert_eq!(edit.coverage().imported_slots, 1);
    assert_eq!(edit.diagnostics().len(), 2);
    assert_eq!(edit.changes().len(), 2);
    for change in edit.changes() {
        assert_eq!(change.relation(), ModelFieldRelation::Derived);
        assert_eq!(change.parents().len(), 1);
        assert_eq!(
            source
                .resolve_field(&change.parents()[0])
                .unwrap()
                .path
                .as_ref(),
            change.path()
        );
        assert_eq!(
            edit.archive()
                .resolve_field(change.result().unwrap())
                .unwrap()
                .path
                .as_ref(),
            change.path()
        );
    }
    assert_eq!(
        edit.unchanged_fields() + edit.changes().len(),
        source.fields().len()
    );
    assert_eq!(source.to_bytes().unwrap(), original);
    assert_eq!(
        edit.archive()
            .edit_fields(source.fields().to_vec())
            .unwrap()
            .archive()
            .to_bytes()
            .unwrap(),
        original
    );
}

#[test]
fn public_model_edit_reopens_whole_document_and_preserves_unrelated_streams() {
    let original = document();
    let source = archive();
    let edit = source.edit_fields(renamed_fields(&source)).unwrap();
    let mut changed = original.clone();
    changed
        .replace_stream("Contents/Config-0", edit.archive().to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&changed.to_bytes().unwrap()).unwrap();
    let model = PartConfigurationArchive::from_document(
        &reopened,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap();
    assert_eq!(model.to_bytes().unwrap(), EDITED);
    for stream in original
        .streams()
        .filter(|stream| stream.name != "Contents/Config-0")
    {
        assert_eq!(
            original.read_stream(stream.name).unwrap(),
            reopened.read_stream(stream.name).unwrap()
        );
    }
    changed
        .replace_stream("Contents/Config-0", source.to_bytes().unwrap())
        .unwrap();
    assert_eq!(changed.to_bytes().unwrap(), DOCUMENT);
}

#[test]
fn revision_bound_fields_and_imported_headers_retain_their_owners() {
    let source = archive();
    let handle = source.field_handle(0).unwrap();
    assert_eq!(handle.index(), 0);
    assert!(source.clone().resolve_field(&handle).is_ok());
    let unchanged = source.edit_fields(source.fields().to_vec()).unwrap();
    assert_eq!(unchanged.completion(), ModelEditCompletion::Unchanged);
    assert!(unchanged.changes().is_empty());
    assert_eq!(unchanged.unchanged_fields(), source.fields().len());
    assert!(unchanged.archive().resolve_field(&handle).is_ok());
    assert!(archive().resolve_field(&handle).is_err());
    assert!(
        source
            .edit_fields(renamed_fields(&source))
            .unwrap()
            .archive()
            .resolve_field(&handle)
            .is_err()
    );
    assert!(source.field_handle(source.fields().len()).is_err());
    let (HeaderReference::Object(identity), header) = source
        .header_origin(ModelObjectId::new(3).unwrap())
        .unwrap()
    else {
        panic!("expected imported object")
    };
    assert!(matches!(
        header.contents().objects[&identity].object,
        HeaderObject::External(_)
    ));
    assert!(
        source
            .header_origin(ModelObjectId::new(2).unwrap())
            .is_none()
    );
    assert_eq!(source.context().version(), ArchiveVersion::new(19000));
    assert_eq!(source.context().configuration(), ConfigurationId::new(0));
    assert_eq!(source.context().header(), header);
    assert!(format!("{source:?}").contains("PartConfigurationArchive"));
}

fn context_for_nine() -> PartConfigurationContext {
    let context = archive().context().clone();
    PartConfigurationContext::ordinary_external_xml(
        ConfigurationId::new(9),
        context.version(),
        Arc::new(context.header().clone()),
        None,
    )
    .unwrap()
}

#[test]
fn selected_configuration_errors_never_report_configuration_zero() {
    let context = context_for_nine();
    let parse = PartConfigurationArchive::from_bytes(&[], context.clone(), LIMITS).unwrap_err();
    let mut fields = archive().fields().to_vec();
    fields[0].path = "forged.root".into();
    let construct =
        PartConfigurationArchive::from_fields(fields, context.clone(), LIMITS).unwrap_err();
    let source =
        PartConfigurationArchive::from_fields(archive().fields().to_vec(), context.clone(), LIMITS)
            .unwrap();
    let mut invalid_fields = source.fields().to_vec();
    invalid_fields[0].path = "forged.root".into();
    let edit = source.edit_fields(invalid_fields).unwrap_err();
    let missing = PartConfigurationContext::from_document(
        &document(),
        ConfigurationId::new(9),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap_err();
    let mismatch = PartConfigurationContext::ordinary_external_xml(
        ConfigurationId::new(9),
        ArchiveVersion::new(18000),
        Arc::new(context.header().clone()),
        None,
    )
    .unwrap_err();
    for error in [parse, construct, edit, missing, mismatch] {
        assert_eq!(error.entity, "Contents/Config-9");
        assert!(!error.to_string().contains("Config-0"), "{error}");
    }
}

#[test]
fn failed_edits_and_source_overwrites_keep_published_results_isolated() {
    let source = archive();
    let original = source.to_bytes().unwrap();
    let mut changed = source.fields().to_vec();
    changed[0].path = "forged.root".into();
    assert!(source.edit_fields(changed).is_err());
    assert_eq!(source.to_bytes().unwrap(), original);
    let mut bytes = original.clone();
    let owned =
        PartConfigurationArchive::from_bytes(&bytes, source.context().clone(), LIMITS).unwrap();
    bytes.fill(0xa5);
    assert_eq!(owned.to_bytes().unwrap(), original);
    let tight = ModelArchiveLimits {
        maximum_bytes: original.len() - 1,
        ..LIMITS
    };
    assert!(
        PartConfigurationArchive::from_bytes(&original, source.context().clone(), tight).is_err()
    );
    assert_eq!(source.to_bytes().unwrap(), original);
}

#[test]
fn public_snapshot_types_are_send_sync_and_concurrent_edits_are_deterministic() {
    fn shared<T: Send + Sync>() {}
    shared::<PartConfigurationArchive>();
    shared::<PartConfigurationContext>();
    shared::<bessemer::model_archives::PartConfigurationEdit>();
    shared::<ModelField>();
    shared::<bessemer::model_archives::ModelFieldHandle>();
    shared::<bessemer::model_archives::KernelFrame>();
    shared::<(
        ConfigurationId,
        ModelArchiveLimits,
        ModelFieldValue,
        ModelObjectId,
        bessemer::model_archives::ModelClass,
        bessemer::model_archives::ModelObjectTag,
        bessemer::model_archives::ModelObjectFraming,
        bessemer::model_archives::ModelObjectRecord,
        bessemer::model_archives::ModelArchiveCoverage,
        bessemer::model_archives::ModelArchiveDiagnostic,
        bessemer::model_archives::ModelFieldChange,
        bessemer::model_archives::CompressedKernelContext,
        ModelEditCompletion,
        ModelEditCheckMode,
        ModelEditFidelity,
        ModelFieldRelation,
    )>();
    let source = archive();
    std::thread::scope(|scope| {
        let edits: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| source.edit_fields(renamed_fields(&source)).unwrap()))
            .collect();
        for edit in edits {
            assert_eq!(edit.join().unwrap().archive().to_bytes().unwrap(), EDITED);
        }
    });
}

#[test]
fn all_public_archive_limits_reject_before_publishing_a_result() {
    let source = archive();
    let bytes = source.to_bytes().unwrap();
    let limits = [
        ModelArchiveLimits {
            maximum_bytes: bytes.len() - 1,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_fields: source.fields().len() - 1,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_entries: 0,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_depth: 0,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_objects: 0,
            ..LIMITS
        },
    ];
    for budget in limits {
        let error = PartConfigurationArchive::from_bytes(&bytes, source.context().clone(), budget)
            .unwrap_err();
        assert_eq!(error.entity, "Contents/Config-0");
        assert_eq!(error.class, FailureClass::Invalid);
    }
    assert_eq!(source.to_bytes().unwrap(), bytes);
}

#[test]
fn construction_errors_identify_the_mismatched_field_shape() {
    let source = archive();
    let mut fields = source.fields().to_vec();
    fields[0].path = "forged.root".into();
    let error = source.edit_fields(fields).unwrap_err();
    assert!(error.cause.contains("forged.root"), "{error}");
    assert!(
        error.cause.contains(source.fields()[0].path.as_ref()),
        "{error}"
    );
    let mut fields = source.fields().to_vec();
    let index = fields
        .iter()
        .position(|field| matches!(field.value, ModelFieldValue::U32(_)))
        .unwrap();
    let ModelFieldValue::U32(value) = fields[index].value else {
        panic!("expected unsigned field")
    };
    fields[index].value = ModelFieldValue::I32(value as i32);
    let error = source.edit_fields(fields).unwrap_err();
    assert!(
        error.cause.contains("i32") && error.cause.contains("u32"),
        "{error}"
    );
}

#[test]
fn construction_diagnostics_bound_metadata_and_report_object_identity_mismatches() {
    use bessemer::model_archives::ModelObjectRecord;
    let source = archive();
    let mut fields = source.fields().to_vec();
    fields[0].path = "x".repeat(100000).into();
    let error = source.edit_fields(fields).unwrap_err();
    assert!(error.cause.len() < 1000);
    assert!(error.cause.contains("100000 bytes"));
    let mut fields = source.fields().to_vec();
    let ModelFieldValue::ObjectTag(tag) = &mut fields[0].value else {
        panic!("expected root tag")
    };
    let ModelObjectRecord::New { index, .. } = &mut tag.record else {
        panic!("expected new root")
    };
    *index = ModelObjectId::new(9).unwrap();
    let error = source.edit_fields(fields).unwrap_err();
    assert!(error.cause.contains("map index 9"), "{error}");
    assert!(error.cause.contains("map index 2"), "{error}");
}
