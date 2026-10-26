//! Resolved-feature failures identify their own selected stream without losing causes (#64).
use bessemer::header::{ArchiveVersion, DocumentHeader};
use bessemer::model_archives::{
    ConfigurationId, ModelArchiveLimits, ModelFieldValue, PartConfigurationArchive,
    PartConfigurationContext, PartResolvedFeaturesArchive,
};
use bessemer::{BessemerError, Document, FailureClass};
use std::sync::Arc;

const VERSION: ArchiveVersion = ArchiveVersion::new(19000);
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

fn document() -> Document {
    Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap()
}

fn source_for(number: u32) -> PartConfigurationArchive {
    let document = document();
    let header = DocumentHeader::from_bytes(
        &document
            .read_stream("Contents/Config-0-ModelHeader")
            .unwrap(),
        VERSION,
        LIMITS.maximum_objects,
    )
    .unwrap();
    let context = PartConfigurationContext::ordinary_external_xml(
        ConfigurationId::new(number),
        VERSION,
        Arc::new(header),
        None,
    )
    .unwrap();
    PartConfigurationArchive::from_bytes(
        &document.read_stream("Contents/Config-0").unwrap(),
        context,
        LIMITS,
    )
    .unwrap()
}

fn archive() -> PartResolvedFeaturesArchive {
    PartResolvedFeaturesArchive::from_document(
        &document(),
        ConfigurationId::new(0),
        VERSION,
        None,
        LIMITS,
    )
    .unwrap()
}

fn assert_feature_context(error: &BessemerError, number: u32) {
    assert_eq!(error.operation, "part resolved features");
    assert_eq!(
        error.entity,
        format!("Contents/Config-{number}-ResolvedFeatures")
    );
    assert!(!error.cause.is_empty());
}

#[test]
fn truncated_feature_reads_identify_the_normalized_selected_feature_stream() {
    let manager = document().read_stream("Contents/CMgr").unwrap();
    for number in [0, 7] {
        let error =
            PartResolvedFeaturesArchive::from_bytes(&[], source_for(number), &manager, LIMITS)
                .unwrap_err();
        assert_feature_context(&error, number);
        assert_eq!(error.class, FailureClass::Invalid);
        assert!(error.cause.contains("model stream"));
    }
}

#[test]
fn invalid_feature_edits_preserve_the_bad_field_location_and_source() {
    let archive = archive();
    let original = archive.to_bytes().unwrap();
    let mut fields = archive.fields().to_vec();
    fields[0].value = ModelFieldValue::U32(7);
    let error = archive.edit_fields(fields).unwrap_err();
    assert_feature_context(&error, 0);
    assert_eq!(error.class, FailureClass::Invalid);
    assert!(error.cause.contains("raw_prefix"));
    assert_eq!(archive.to_bytes().unwrap(), original);
}

#[test]
fn feature_lookup_errors_retain_index_and_revision_details() {
    let archive = archive();
    let index_error = archive.field_handle(usize::MAX).unwrap_err();
    assert_feature_context(&index_error, 0);
    assert!(index_error.cause.contains(&usize::MAX.to_string()));
    let foreign = self::archive().field_handle(0).unwrap();
    let revision_error = archive.resolve_field(&foreign).unwrap_err();
    assert_feature_context(&revision_error, 0);
    assert!(revision_error.cause.contains("another revision"));
}

#[test]
fn configuration_failures_keep_their_existing_operation_and_entity() {
    let source = source_for(0);
    let error =
        PartConfigurationArchive::from_bytes(&[], source.context().clone(), LIMITS).unwrap_err();
    assert_eq!(error.operation, "part configuration");
    assert_eq!(error.entity, "Contents/Config-0");
    let lookup = source.field_handle(usize::MAX).unwrap_err();
    assert_eq!(lookup.operation, "model field lookup");
    assert_eq!(lookup.entity, "Contents/Config-0");
}

#[test]
fn document_feature_failure_preserves_unsupported_class_and_dependency_location() {
    let mut document = document();
    let name = b'X';
    let unsupported = [0, 0, 0, 0, 1, 0, 255, 255, 1, 0, 1, 0, name];
    document
        .replace_stream("Contents/Config-0-ResolvedFeatures", unsupported.to_vec())
        .unwrap();
    let error = PartResolvedFeaturesArchive::from_document(
        &document,
        ConfigurationId::new(0),
        VERSION,
        None,
        LIMITS,
    )
    .unwrap_err();
    assert_feature_context(&error, 0);
    assert_eq!(error.class, FailureClass::Unsupported);
    assert!(error.cause.contains("runtime class \"X\""));
    assert!(error.cause.contains("model stream"));
}

#[test]
fn manager_dependency_failure_preserves_its_actual_stream_and_failure_class() {
    let document = document();
    let features = document
        .read_stream("Contents/Config-0-ResolvedFeatures")
        .unwrap();
    let error =
        PartResolvedFeaturesArchive::from_bytes(&features, source_for(0), &[], LIMITS).unwrap_err();
    assert_feature_context(&error, 0);
    assert_eq!(error.class, FailureClass::Invalid);
    assert!(error.cause.contains("Contents/CMgr"));
}
