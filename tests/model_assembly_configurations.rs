//! Complete owned assembly configurations through the public archive APIs (#99).
use bessemer::model_archives::{
    ArchiveString, AssemblyConfigurationArchive, AssemblyRootKind, ConfigurationId,
    ModelArchiveLimits, ModelEditCompletion, ModelFieldValue, TextEncoding,
};
use bessemer::{Document, header::ArchiveVersion};

const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

fn document() -> Document {
    Document::from_bytes(include_bytes!("fixtures/blank-assembly.SLDASM")).unwrap()
}

fn renamed(
    archive: &AssemblyConfigurationArchive,
    path: &str,
    value: &str,
) -> Vec<bessemer::model_archives::ModelField> {
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value = ModelFieldValue::Text(ArchiveString::new(value, TextEncoding::Utf16).unwrap());
    fields
}

#[test]
fn assembly_name_edit_matches_independent_reference_reopens_and_reverts() {
    let document = document();
    let source = archive(&document);
    let edit = source
        .edit_fields(renamed(&source, "root.node.name.name", "Bessemer Ω"))
        .unwrap();
    let expected =
        include_bytes!("fixtures/assembly-configurations/owned-assembly-name-edited.bin");
    assert_eq!(edit.archive().to_bytes().unwrap(), expected);
    assert_eq!(edit.changes().len(), 2);
    for change in edit.changes() {
        assert_eq!(change.parents().len(), 1);
        assert!(source.resolve_field(&change.parents()[0]).is_ok());
        assert!(
            edit.archive()
                .resolve_field(change.result().unwrap())
                .is_ok()
        );
        assert!(source.resolve_field(change.result().unwrap()).is_err());
    }
    assert_eq!(edit.completion(), ModelEditCompletion::Changed);
    assert_eq!(edit.coverage(), edit.archive().coverage());
    assert_eq!(edit.diagnostics(), source.diagnostics());
    let mut changed = document.clone();
    changed
        .replace_stream("Contents/Config-0", edit.archive().to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&changed.to_bytes().unwrap()).unwrap();
    assert_eq!(archive(&reopened).to_bytes().unwrap(), expected);
    for stream in document
        .streams()
        .filter(|stream| stream.name != "Contents/Config-0")
    {
        assert_eq!(
            reopened.read_stream(stream.name).unwrap(),
            document.read_stream(stream.name).unwrap()
        );
    }
    assert_eq!(
        edit.archive()
            .edit_fields(source.fields().to_vec())
            .unwrap()
            .archive()
            .to_bytes()
            .unwrap(),
        source.to_bytes().unwrap()
    );
    assert_eq!(
        source.to_bytes().unwrap(),
        document.read_stream("Contents/Config-0").unwrap()
    );
}

#[test]
fn routing_trailer_edit_preserves_both_section_sizes_and_matches_its_independent_golden() {
    let mut document = document();
    let original = include_bytes!("fixtures/assembly-configurations/owned-routing.bin");
    document
        .replace_stream("Contents/Config-0", original.to_vec())
        .unwrap();
    let source = AssemblyConfigurationArchive::from_document(
        &document,
        ConfigurationId::new(0),
        AssemblyRootKind::RoutingAssembly,
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap();
    assert_eq!(source.to_bytes().unwrap(), original);
    let edit = source
        .edit_fields(renamed(
            &source,
            "root.routing_trailer.raw_1aa0",
            "Bessemer routing Ω",
        ))
        .unwrap();
    assert_eq!(
        edit.archive().to_bytes().unwrap(),
        include_bytes!("fixtures/assembly-configurations/owned-routing-edited.bin")
    );
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(edit.changes()[0].path(), "root.routing_trailer.raw_1aa0");
    assert_eq!(source.to_bytes().unwrap(), original);
}

#[test]
fn unchanged_and_failed_edits_preserve_revision_bound_history_and_parallel_input_isolation() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<AssemblyConfigurationArchive>();
    send_sync::<AssemblyRootKind>();
    send_sync::<bessemer::model_archives::AssemblyConfigurationContext>();
    send_sync::<bessemer::model_archives::AssemblyConfigurationEdit>();
    let document = document();
    let source = archive(&document);
    let handle = source.field_handle(0).unwrap();
    let edit = source.edit_fields(source.fields().to_vec()).unwrap();
    assert_eq!(edit.completion(), ModelEditCompletion::Unchanged);
    assert!(edit.changes().is_empty());
    assert_eq!(edit.unchanged_fields(), source.fields().len());
    assert_eq!(
        edit.archive().resolve_field(&handle).unwrap(),
        source.resolve_field(&handle).unwrap()
    );
    assert!(source.field_handle(source.fields().len()).is_err());
    let mut invalid = source.fields().to_vec();
    invalid[0].value = ModelFieldValue::U8(0);
    assert!(source.edit_fields(invalid).is_err());
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..6)
            .map(|_| scope.spawn(|| archive(&document).to_bytes().unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), source.to_bytes().unwrap());
        }
    });
}

#[test]
fn selected_roots_section_sizes_and_physical_eof_are_strict() {
    let document = document();
    let source = archive(&document);
    let bytes = source.to_bytes().unwrap();
    for end in [0, 1, 6, 18, 21, 22, 100, 2305, 2306, 2309, bytes.len() - 1] {
        assert!(
            AssemblyConfigurationArchive::from_bytes(
                &bytes[..end],
                source.context().clone(),
                LIMITS
            )
            .is_err(),
            "prefix {end}"
        );
    }
    let mut extended = bytes.clone();
    extended.push(0);
    assert!(
        AssemblyConfigurationArchive::from_bytes(&extended, source.context().clone(), LIMITS)
            .is_err()
    );
    for offset in [18, 2306] {
        for size in [-1i32, 0, i32::MAX] {
            let mut changed = bytes.clone();
            changed[offset..offset + 4].copy_from_slice(&size.to_le_bytes());
            assert!(
                AssemblyConfigurationArchive::from_bytes(
                    &changed,
                    source.context().clone(),
                    LIMITS
                )
                .is_err()
            );
        }
    }
    assert!(
        AssemblyConfigurationArchive::from_document(
            &document,
            ConfigurationId::new(0),
            AssemblyRootKind::RoutingAssembly,
            ArchiveVersion::new(19000),
            None,
            LIMITS
        )
        .is_err()
    );
}

fn archive(document: &Document) -> AssemblyConfigurationArchive {
    AssemblyConfigurationArchive::from_document(
        document,
        ConfigurationId::new(0),
        AssemblyRootKind::Assembly,
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap()
}

#[test]
fn complete_owned_assembly_reconstructs_both_sections_through_physical_eof() {
    let document = document();
    let archive = archive(&document);
    let bytes = document.read_stream("Contents/Config-0").unwrap();
    assert_eq!(bytes.len(), 23359);
    assert_eq!(archive.to_bytes().unwrap(), bytes);
    let rebuilt = AssemblyConfigurationArchive::from_fields(
        archive.fields().to_vec(),
        archive.context().clone(),
        LIMITS,
    )
    .unwrap();
    assert_eq!(rebuilt.to_bytes().unwrap(), bytes);
    let mut unchanged = document.clone();
    unchanged
        .replace_stream("Contents/Config-0", rebuilt.to_bytes().unwrap())
        .unwrap();
    assert_eq!(unchanged.to_bytes().unwrap(), document.to_bytes().unwrap());
    assert_eq!(archive.coverage().wire_bytes, bytes.len());
    assert_eq!(archive.context().root_kind(), AssemblyRootKind::Assembly);
    assert_eq!(archive.context().configuration(), ConfigurationId::new(0));
    assert_eq!(archive.context().version(), ArchiveVersion::new(19000));
    let origins = (1..128)
        .filter_map(|index| {
            archive.header_origin(bessemer::model_archives::ModelObjectId::new(index).unwrap())
        })
        .count();
    assert_eq!(origins, archive.coverage().imported_slots);
    assert!(origins > 0);
}

#[test]
fn context_failures_keep_typed_assembly_operation_configuration_and_cause() {
    let document = document();
    for version in [10000, 0x27cb] {
        let error = AssemblyConfigurationArchive::from_document(
            &document,
            ConfigurationId::new(0),
            AssemblyRootKind::Assembly,
            ArchiveVersion::new(version),
            None,
            LIMITS,
        )
        .unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Unsupported);
        assert_eq!(error.operation, "assembly configuration");
        assert_eq!(error.entity, "Contents/Config-0");
        assert!(error.cause.contains(&version.to_string()));
    }
    let error = AssemblyConfigurationArchive::from_document(
        &document,
        ConfigurationId::new(999),
        AssemblyRootKind::Assembly,
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap_err();
    assert_eq!(error.operation, "assembly configuration");
    assert_eq!(error.entity, "Contents/Config-999");
    let source = archive(&document);
    let context = bessemer::model_archives::AssemblyConfigurationContext::ordinary_external_xml(
        ConfigurationId::new(0),
        AssemblyRootKind::Assembly,
        ArchiveVersion::new(19001),
        std::sync::Arc::new(source.context().header().clone()),
        None,
    );
    assert!(context.is_err());
}
