//! Independent whole-source identity, exact metadata and immutable provenance contracts.
use bessemer::inspection::records::*;
use bessemer::inspection::{InspectionEnvelope, InspectionProfile};
use bessemer::metadata::XmlLimits;

#[path = "inspection_source_identity/contracts.rs"]
mod contracts;
#[path = "inspection_source_identity/fixtures.rs"]
mod fixtures;
#[path = "inspection_source_identity/native.rs"]
mod native;

const LIMITS: XmlLimits = XmlLimits {
    maximum_bytes: 1_000_000,
    maximum_nodes: 100_000,
    maximum_attributes: 100_000,
    maximum_depth: 100,
};
const EXPECTED_SOURCE_DIAGNOSTICS: [InspectionSourceDiagnostic; 3] = [
    InspectionSourceDiagnostic::ChecksumDoesNotAuthenticate,
    InspectionSourceDiagnostic::StoredPathsNotOpened,
    InspectionSourceDiagnostic::ApplicationCompatibilityNotChecked,
];

fn project(xml: &[u8]) -> InspectionRecords {
    InspectionRecords::from_envelope(
        &InspectionEnvelope::from_xml(InspectionProfile::Current2020, xml, LIMITS).unwrap(),
    )
    .unwrap()
}

fn document(project: &InspectionRecords) -> &InspectionRecord {
    project
        .records()
        .iter()
        .find(|record| record.kind == InspectionRecordKind::Document)
        .unwrap()
}

fn canonical_project(length: usize, md5: &str) -> InspectionRecords {
    let xml = format!(
        "<i:SOLIDWORKSINSPECTION xmlns:i='http://www.extensiblecad.com'><INSPECTION_SHEET><DOCUMENT><DOCUMENT Uid='D' SourcePath='Z:/never-open/from-stored-path.pdf' ModelSize='{length}' ModelMD5HashData='{md5}'/></DOCUMENT></INSPECTION_SHEET></i:SOLIDWORKSINSPECTION>"
    );
    project(xml.as_bytes())
}

#[test]
fn all_172_independent_vectors_match_complete_checksums_sizes_and_provenance() {
    for case in fixtures::checksum_cases() {
        let source = canonical_project(case.source.len(), &case.md5);
        let result = source
            .compare_document_source(&document(&source).id, case.source)
            .unwrap();
        assert_eq!(
            result.values.actual_md5.as_ref(),
            case.md5,
            "{}",
            case.label
        );
        assert_eq!(result.values.actual_size, case.source.len());
        assert!(result.values.size_matches && result.values.md5_matches);
        assert_eq!(
            result.values.stored_size.as_deref(),
            Some(case.source.len().to_string().as_str())
        );
        assert_eq!(result.values.stored_md5.as_deref(), Some(case.md5.as_str()));
        check_comparison_contract(&source, &result, case.source.len());
    }
}

fn check_comparison_contract(
    source: &InspectionRecords,
    result: &InspectionSourceComparison,
    length: usize,
) {
    let id = &document(source).id;
    assert_eq!(result.document, *id);
    assert_eq!(result.completion, InspectionSourceCompletion::Completed);
    assert_eq!(
        result.coverage.mode,
        InspectionSourceCheckMode::ExactCallerBytesAndStoredText
    );
    assert_eq!(result.coverage.source_bytes, length);
    assert_eq!(result.coverage.unread_source_bytes, 0);
    assert_eq!(result.history[0].parents.as_ref(), std::slice::from_ref(id));
    assert_eq!(result.history[0].result, *id);
    assert_eq!(result.history[0].relation, InspectionRecordRelation::Copied);
    assert_eq!(
        result.source.envelope().xml_bytes(),
        source.envelope().xml_bytes()
    );
    assert_eq!(result.diagnostics, EXPECTED_SOURCE_DIAGNOSTICS);
    assert!(source.record(&result.document).is_ok());
}

#[test]
fn all_22_source_metadata_cases_complete_without_normalizing_stored_text() {
    for case in fixtures::metadata_cases() {
        let source = project(case.xml);
        let result = source
            .compare_document_source(&document(&source).id, b"abc")
            .unwrap();
        assert_eq!(
            result.values.size_matches, case.size_matches,
            "{}",
            case.label
        );
        assert_eq!(
            result.values.md5_matches, case.md5_matches,
            "{}",
            case.label
        );
        check_raw_metadata(&source, &result);
        check_comparison_contract(&source, &result, 3);
    }
}

fn check_raw_metadata(source: &InspectionRecords, result: &InspectionSourceComparison) {
    let original = document(source);
    assert_eq!(
        result.values.stored_size.as_deref(),
        original.attribute("ModelSize")
    );
    assert_eq!(
        result.values.stored_md5.as_deref(),
        original.attribute("ModelMD5HashData")
    );
    assert_eq!(
        result.values.actual_md5.as_ref(),
        "900150983cd24fb0d6963f7d28e17f72"
    );
}

#[test]
fn wrong_role_foreign_and_stale_document_ids_reject_before_comparison() {
    let source = canonical_project(3, "900150983cd24fb0d6963f7d28e17f72");
    let wrong = source
        .compare_document_source(&source.records()[0].id, b"abc")
        .unwrap_err();
    assert_eq!(wrong.operation, "Inspection source identity");
    assert!(wrong.cause.contains("expected Document"));
    let foreign = canonical_project(3, "900150983cd24fb0d6963f7d28e17f72");
    check_rejected_source_id(&source, &document(&foreign).id);
    let changed = source
        .edit_attribute(&document(&source).id, "ModelSize", "4")
        .unwrap();
    check_rejected_source_id(&changed.project, &document(&source).id);
    assert!(source.record(&document(&source).id).is_ok());
}

fn check_rejected_source_id(source: &InspectionRecords, id: &InspectionRecordId) {
    let error = source.compare_document_source(id, b"abc").unwrap_err();
    assert_eq!(error.operation, "Inspection source identity");
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert!(error.cause.contains("expected"));
}

#[test]
fn published_source_comparison_types_are_send_and_sync() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<InspectionSourceComparison>();
    send_sync::<InspectionSourceIdentityValues>();
    send_sync::<InspectionSourceCompletion>();
    send_sync::<InspectionSourceCoverage>();
    send_sync::<InspectionSourceCheckMode>();
    send_sync::<InspectionSourceDiagnostic>();
}
