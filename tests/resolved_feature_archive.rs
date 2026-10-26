//! Public complete resolved-feature reading and reconstruction against owned fixtures.
use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::{
    ConfigurationId, ModelArchiveDiagnostic, ModelArchiveLimits, ModelEditCheckMode,
    PartConfigurationArchive, PartResolvedFeaturesArchive,
};

const OWNED: &[u8] = include_bytes!("fixtures/blank-part.SLDPRT");
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

fn archive(document: &Document) -> PartResolvedFeaturesArchive {
    PartResolvedFeaturesArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap()
}

#[test]
fn public_resolved_archive_reconstructs_the_owned_complete_stream() {
    let document = Document::from_bytes(OWNED).unwrap();
    let parsed = archive(&document);
    let expected = document
        .read_stream("Contents/Config-0-ResolvedFeatures")
        .unwrap();
    assert_eq!(parsed.to_bytes().unwrap(), expected);
    assert_eq!(parsed.root_count(), 17);
    assert_eq!(parsed.object_slots(), (97, 161));
    assert_eq!(parsed.coverage().wire_bytes, 5558);
    assert_eq!(parsed.coverage().typed_fields, parsed.fields().len());
    assert_eq!(
        parsed.coverage().mode,
        ModelEditCheckMode::ExactStoredArchive
    );
    assert!(parsed.coverage().new_objects > 0 && parsed.coverage().object_aliases > 0);
    assert_eq!(
        parsed.diagnostics(),
        &[
            ModelArchiveDiagnostic::GeometryOutsideScope,
            ModelArchiveDiagnostic::ExternalDocumentsNotChecked
        ]
    );
    assert_eq!(document.to_bytes().unwrap(), OWNED);
}

#[test]
fn public_resolved_archive_owns_inputs_and_supports_concurrent_reconstruction() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<PartResolvedFeaturesArchive>();
    let document = Document::from_bytes(OWNED).unwrap();
    let source = PartConfigurationArchive::from_document(
        &document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap();
    let mut manager = document.read_stream("Contents/CMgr").unwrap();
    let mut features = document
        .read_stream("Contents/Config-0-ResolvedFeatures")
        .unwrap();
    let expected = features.clone();
    let parsed =
        PartResolvedFeaturesArchive::from_bytes(&features, source, &manager, LIMITS).unwrap();
    features.fill(0xa5);
    manager.fill(0x5a);
    let workers: Vec<_> = (0..4)
        .map(|_| {
            let copy = parsed.clone();
            std::thread::spawn(move || copy.to_bytes().unwrap())
        })
        .collect();
    for worker in workers {
        assert_eq!(worker.join().unwrap(), expected);
    }
    assert_eq!(parsed.to_bytes().unwrap(), expected);
}

#[test]
fn public_resolved_archive_rejects_incomplete_context_and_streams() {
    let document = Document::from_bytes(OWNED).unwrap();
    let source = PartConfigurationArchive::from_document(
        &document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap();
    let manager = document.read_stream("Contents/CMgr").unwrap();
    let features = document
        .read_stream("Contents/Config-0-ResolvedFeatures")
        .unwrap();
    for cut in [0, 1, 3, features.len() / 2, features.len() - 1] {
        let error = PartResolvedFeaturesArchive::from_bytes(
            &features[..cut],
            source.clone(),
            &manager,
            LIMITS,
        )
        .unwrap_err();
        assert_eq!(error.entity, "Contents/Config-0-ResolvedFeatures");
    }
    assert!(
        PartResolvedFeaturesArchive::from_bytes(
            &features,
            source.clone(),
            &manager[..manager.len() - 1],
            LIMITS
        )
        .is_err()
    );
    assert!(
        PartResolvedFeaturesArchive::from_bytes(
            &[features.as_slice(), &[0]].concat(),
            source,
            &manager,
            LIMITS
        )
        .is_err()
    );
    assert_eq!(archive(&document).to_bytes().unwrap(), features);
}

#[test]
fn public_resolved_archive_distinguishes_completed_empty_from_incomplete() {
    let document = Document::from_bytes(OWNED).unwrap();
    let source = PartConfigurationArchive::from_document(
        &document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap();
    let manager = document.read_stream("Contents/CMgr").unwrap();
    // Active saved-helper state requires the final empty MBSM map after the empty root list.
    let empty = [0_u8; 10];
    let completed =
        PartResolvedFeaturesArchive::from_bytes(&empty, source.clone(), &manager, LIMITS).unwrap();
    assert_eq!(completed.root_count(), 0);
    assert_eq!(completed.object_slots(), (97, 97));
    assert_eq!(completed.to_bytes().unwrap(), empty);
    assert!(
        PartResolvedFeaturesArchive::from_bytes(&empty[..6], source, &manager, LIMITS).is_err()
    );
}

fn renamed_fields(
    source: &PartResolvedFeaturesArchive,
) -> Vec<bessemer::model_archives::ModelField> {
    use bessemer::model_archives::{ArchiveString, ModelFieldValue, TextEncoding};
    let mut fields = source.fields().to_vec();
    let field = fields
        .iter_mut()
        .find(|field| {
            field.path.as_ref() == "root.features[13].model_feature.feature.node.name.name"
        })
        .unwrap();
    field.value =
        ModelFieldValue::Text(ArchiveString::new("Bessemer Plane Ω", TextEncoding::Utf16).unwrap());
    fields
}

#[test]
fn resolved_edit_matches_independent_bytes_and_revision_bound_history() {
    use bessemer::model_archives::{
        ModelEditCompletion, ModelEditFidelity, ModelFieldRelation, PartResolvedFeaturesEdit,
    };
    fn send_sync<T: Send + Sync>() {}
    send_sync::<PartResolvedFeaturesEdit>();
    let source = archive(&Document::from_bytes(OWNED).unwrap());
    let edit = source.edit_fields(renamed_fields(&source)).unwrap();
    assert_eq!(
        edit.archive().to_bytes().unwrap(),
        include_bytes!("fixtures/resolved-features/owned-features-plane-edited.bin")
    );
    assert_eq!(edit.completion(), ModelEditCompletion::Changed);
    assert_eq!(edit.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(edit.coverage(), edit.archive().coverage());
    assert_eq!(edit.diagnostics(), source.diagnostics());
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(edit.unchanged_fields(), source.fields().len() - 1);
    let change = &edit.changes()[0];
    assert_eq!(change.relation(), ModelFieldRelation::Derived);
    assert_eq!(
        change.path(),
        "root.features[13].model_feature.feature.node.name.name"
    );
    assert_eq!(change.parents().len(), 1);
    assert!(source.resolve_field(&change.parents()[0]).is_ok());
    assert!(
        edit.archive()
            .resolve_field(change.result().unwrap())
            .is_ok()
    );
    assert!(source.resolve_field(change.result().unwrap()).is_err());
    assert!(edit.archive().resolve_field(&change.parents()[0]).is_err());
}

#[test]
fn resolved_unchanged_edits_preserve_handles_and_failed_edits_leave_source_isolated() {
    use bessemer::model_archives::{ModelEditCompletion, ModelFieldValue};
    let document = Document::from_bytes(OWNED).unwrap();
    let source = archive(&document);
    let original = source.to_bytes().unwrap();
    let handle = source.field_handle(0).unwrap();
    let unchanged = source.edit_fields(source.fields().to_vec()).unwrap();
    assert_eq!(unchanged.completion(), ModelEditCompletion::Unchanged);
    assert!(unchanged.changes().is_empty());
    assert_eq!(unchanged.unchanged_fields(), source.fields().len());
    assert_eq!(
        unchanged.archive().resolve_field(&handle).unwrap(),
        &source.fields()[0]
    );
    assert!(archive(&document).resolve_field(&handle).is_err());
    assert!(source.field_handle(usize::MAX).is_err());
    let mut broken = renamed_fields(&source);
    broken[0].value = ModelFieldValue::U8(7);
    assert!(source.edit_fields(broken).is_err());
    assert_eq!(source.to_bytes().unwrap(), original);
    assert!(source.resolve_field(&handle).is_ok());
}

#[test]
fn resolved_edit_reopens_preserves_other_streams_and_reverts_complete_document() {
    let document = Document::from_bytes(OWNED).unwrap();
    let source = archive(&document);
    let edit = source.edit_fields(renamed_fields(&source)).unwrap();
    let stream = "Contents/Config-0-ResolvedFeatures";
    let mut changed = document.clone();
    changed
        .replace_stream(stream, edit.archive().to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&changed.to_bytes().unwrap()).unwrap();
    assert_eq!(archive(&reopened).fields(), edit.archive().fields());
    for entry in document.streams().filter(|entry| entry.name != stream) {
        assert_eq!(
            reopened.read_stream(entry.name).unwrap(),
            document.read_stream(entry.name).unwrap()
        );
    }
    let reverted = edit
        .archive()
        .edit_fields(source.fields().to_vec())
        .unwrap();
    changed
        .replace_stream(stream, reverted.archive().to_bytes().unwrap())
        .unwrap();
    assert_eq!(changed.to_bytes().unwrap(), OWNED);
    assert_eq!(document.to_bytes().unwrap(), OWNED);
}

#[test]
fn concurrent_resolved_edits_are_deterministic_and_keep_the_source_immutable() {
    let source = archive(&Document::from_bytes(OWNED).unwrap());
    let original = source.to_bytes().unwrap();
    let workers: Vec<_> = (0..4)
        .map(|_| {
            let copy = source.clone();
            std::thread::spawn(move || {
                copy.edit_fields(renamed_fields(&copy))
                    .unwrap()
                    .archive()
                    .to_bytes()
                    .unwrap()
            })
        })
        .collect();
    for worker in workers {
        assert_eq!(
            worker.join().unwrap(),
            include_bytes!("fixtures/resolved-features/owned-features-plane-edited.bin")
        );
    }
    assert_eq!(source.to_bytes().unwrap(), original);
}

#[test]
fn resolved_history_records_deleted_and_created_field_parents() {
    use bessemer::model_archives::ModelFieldRelation;
    let document = Document::from_bytes(OWNED).unwrap();
    let source = archive(&document);
    let configuration = PartConfigurationArchive::from_document(
        &document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap();
    let manager = document.read_stream("Contents/CMgr").unwrap();
    let empty =
        PartResolvedFeaturesArchive::from_bytes(&[0; 10], configuration, &manager, LIMITS).unwrap();
    let removed = source.edit_fields(empty.fields().to_vec()).unwrap();
    assert_eq!(removed.archive().root_count(), 0);
    let deleted: Vec<_> = removed
        .changes()
        .iter()
        .filter(|change| change.relation() == ModelFieldRelation::Deleted)
        .collect();
    assert!(!deleted.is_empty());
    for change in deleted {
        assert_eq!(change.parents().len(), 1);
        assert!(source.resolve_field(&change.parents()[0]).is_ok());
        assert!(change.result().is_none());
    }
    let restored = removed
        .archive()
        .edit_fields(source.fields().to_vec())
        .unwrap();
    let created: Vec<_> = restored
        .changes()
        .iter()
        .filter(|change| change.relation() == ModelFieldRelation::Created)
        .collect();
    assert!(!created.is_empty());
    for change in created {
        assert!(change.parents().is_empty());
        assert!(
            restored
                .archive()
                .resolve_field(change.result().unwrap())
                .is_ok()
        );
    }
    assert_eq!(
        restored.archive().to_bytes().unwrap(),
        source.to_bytes().unwrap()
    );
}
