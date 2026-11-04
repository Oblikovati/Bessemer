use super::*;
use relationships::InspectionRelationshipPolicy;

#[test]
fn ordinary_record_index_and_published_source_never_retain_operation_local_capture() {
    let xml = include_bytes!("fixtures/inspection-relationships/ordered.xml");
    let project = indexed_project(xml);
    assert!(project.xml.ordered_view().is_none());
    let sheet = project
        .records()
        .iter()
        .find(|record| record.kind == InspectionRecordKind::Sheet)
        .unwrap();
    let result = project
        .relationship_views(&sheet.id, InspectionRelationshipPolicy::RetainedNodes)
        .unwrap();
    assert_eq!(result.attempts.len(), 9);
    assert!(project.xml.ordered_view().is_none());
    assert!(result.source.xml.ordered_view().is_none());
    assert_eq!(result.source.envelope().xml_bytes(), xml);
}

fn indexed_project(xml: &[u8]) -> InspectionRecords {
    let limits = crate::metadata::XmlLimits {
        maximum_bytes: 1_000_000,
        maximum_nodes: 100_000,
        maximum_attributes: 100_000,
        maximum_depth: 100,
    };
    let envelope =
        InspectionEnvelope::from_xml(super::super::InspectionProfile::Current2020, xml, limits)
            .unwrap();
    InspectionRecords::from_envelope(&envelope).unwrap()
}
