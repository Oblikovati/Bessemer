//! Explicit-policy relationship projection and immutable source contracts.
use bessemer::inspection::records::relationships::*;
use bessemer::inspection::records::*;
use bessemer::inspection::{InspectionEnvelope, InspectionProfile};
use bessemer::metadata::XmlLimits;

const LIMITS: XmlLimits = XmlLimits {
    maximum_bytes: 1_000_000,
    maximum_nodes: 100_000,
    maximum_attributes: 100_000,
    maximum_depth: 100,
};

fn project() -> InspectionRecords {
    project_from_xml(include_bytes!(
        "fixtures/inspection-relationships/ordered.xml"
    ))
}

fn project_from_xml(xml: &[u8]) -> InspectionRecords {
    InspectionRecords::from_envelope(
        &InspectionEnvelope::from_xml(InspectionProfile::Current2020, xml, LIMITS).unwrap(),
    )
    .unwrap()
}

fn views(policy: InspectionRelationshipPolicy) -> InspectionRelationshipViews {
    let project = project();
    let sheet = &project
        .records()
        .iter()
        .find(|record| record.kind == InspectionRecordKind::Sheet)
        .unwrap()
        .id;
    project.relationship_views(sheet, policy).unwrap()
}

#[test]
fn retained_dom_attempts_and_loaded_values_match_literal_oracle() {
    let result = views(InspectionRelationshipPolicy::RetainedNodes);
    assert_eq!(result.attempts.len(), 9);
    assert_eq!(result.entries.len(), 4);
    assert_eq!(result.writer_entries.len(), 3);
    assert_eq!(result.entries[0].key.as_ref(), "");
    assert!(result.entries[0].values.is_empty());
    assert_eq!(result.entries[1].key.as_ref(), "final");
    let actual: Vec<_> = result.entries[1]
        .values
        .iter()
        .map(|value| value.as_ref())
        .collect();
    assert_eq!(actual, ["abc", "x", ""]);
    assert_eq!(result.entries[3].key.as_ref(), "ns-key");
    assert_eq!(result.entries[3].values[0].as_ref(), "ns-value");
    assert_eq!(
        result.source.envelope().xml_bytes(),
        project().envelope().xml_bytes()
    );
    assert_eq!(result.history[0].result, result.sheet);
}

#[test]
fn element_policy_has_a_different_first_empty_key_and_physical_ids_resolve() {
    let result = views(InspectionRelationshipPolicy::ElementsOnly);
    assert_eq!(result.attempts.len(), 6);
    assert_eq!(result.entries[2].key.as_ref(), "");
    assert_eq!(result.entries[2].values[0].as_ref(), "later-empty");
    for attempt in result.attempts.iter() {
        assert_eq!(result.attempt(&attempt.id), Some(attempt));
    }
    let foreign = views(InspectionRelationshipPolicy::ElementsOnly);
    assert!(result.attempt(&foreign.attempts[0].id).is_none());
}

#[test]
fn projection_and_every_published_relationship_type_are_send_sync() {
    fn require<T: Send + Sync>() {}
    require::<InspectionRelationshipPolicy>();
    require::<InspectionRelationshipNodeId>();
    require::<InspectionRelationshipNodeKind>();
    require::<InspectionRelationshipAttempt>();
    require::<InspectionRelationshipCompletion>();
    require::<InspectionRelationshipDiagnostic>();
    require::<InspectionRelationshipViews>();
}

#[path = "inspection_relationships/contracts.rs"]
mod contracts;

fn views_from(
    project: &InspectionRecords,
    policy: InspectionRelationshipPolicy,
) -> InspectionRelationshipViews {
    let sheet = &project
        .records()
        .iter()
        .find(|record| record.kind == InspectionRecordKind::Sheet)
        .unwrap()
        .id;
    project.relationship_views(sheet, policy).unwrap()
}

fn package_project(package: &str) -> InspectionRecords {
    let xml = format!(
        "<i:SOLIDWORKSINSPECTION xmlns:i='http://www.extensiblecad.com' xmlns:x='urn:x'><INSPECTION_SHEET>{package}</INSPECTION_SHEET></i:SOLIDWORKSINSPECTION>"
    );
    project_from_xml(xml.as_bytes())
}

#[path = "inspection_relationships/exports.rs"]
mod exports;

#[path = "inspection_relationships/concurrency.rs"]
mod concurrency;
#[path = "inspection_relationships/limits.rs"]
mod limits;
