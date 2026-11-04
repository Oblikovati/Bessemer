use bessemer::inspection::records::*;
use bessemer::inspection::{InspectionEnvelope, InspectionProfile};
use bessemer::metadata::XmlLimits;

#[path = "inspection_records/limits.rs"]
mod limits;
#[path = "inspection_records/load.rs"]
mod load;
#[path = "inspection_records/templates.rs"]
mod templates;

const OWNED: &[u8] = include_bytes!("fixtures/inspection-records/ordered.xml");
const LIMITS: XmlLimits = XmlLimits {
    maximum_bytes: 1_000_000,
    maximum_nodes: 100_000,
    maximum_attributes: 100_000,
    maximum_depth: 100,
};

fn project(raw: &[u8], profile: InspectionProfile) -> InspectionRecords {
    InspectionRecords::from_envelope(&InspectionEnvelope::from_xml(profile, raw, LIMITS).unwrap())
        .unwrap()
}

fn measurement(project: &InspectionRecords) -> &InspectionRecord {
    project
        .records()
        .iter()
        .find(|record| record.kind == InspectionRecordKind::Measurement)
        .unwrap()
}

#[test]
fn literal_roles_preserve_scope_duplicates_attributes_and_owners() {
    let project = project(OWNED, InspectionProfile::Current2020);
    assert_eq!(project.records().len(), 13);
    let selected = measurement(&project);
    assert_eq!(selected.attribute("Value"), Some(""));
    let owner = project.record(selected.owner.as_ref().unwrap()).unwrap();
    assert_eq!(owner.kind, InspectionRecordKind::Characteristic);
    assert_eq!(owner.attribute("Nominal"), Some("3.503 / 3.496"));
    assert_eq!(owner.attribute("Extension"), Some("λ"));
    let values: Vec<_> = project
        .records()
        .iter()
        .filter(|r| r.kind == InspectionRecordKind::CustomProperty)
        .map(|r| r.attribute("Value"))
        .collect();
    assert_eq!(values, [Some("1"), Some(""), None]);
    check_coverage(&project);
}

fn check_coverage(project: &InspectionRecords) {
    assert_eq!(project.coverage().records, 13);
    assert_eq!(project.coverage().xml_bytes, OWNED.len());
    assert_eq!(
        project.coverage().mode,
        InspectionRecordCheckMode::ExactLexicalXml
    );
    assert_eq!(project.diagnostics().len(), 3);
}

#[test]
fn complete_measurement_edit_encrypts_reopens_and_reverses_both_profiles() {
    for profile in [
        InspectionProfile::Current2020,
        InspectionProfile::Historical2015,
    ] {
        let source = project(OWNED, profile);
        let original_cipher = source.envelope().to_bytes().unwrap();
        let edit = source
            .edit_attribute(&measurement(&source).id, "Value", "Pass")
            .unwrap();
        let expected = std::str::from_utf8(OWNED).unwrap().replacen(
            "<Measurement Name=\"same\" Value=\"\"/>",
            "<Measurement Name=\"same\" Value=\"Pass\"/>",
            1,
        );
        assert_eq!(edit.project.envelope().xml_bytes(), expected.as_bytes());
        assert_eq!(edit.completion, InspectionRecordCompletion::Changed);
        assert_eq!(source.envelope().xml_bytes(), OWNED);
        check_encrypted_reverse(&edit.project, &original_cipher);
    }
}

fn check_encrypted_reverse(edited: &InspectionRecords, original_cipher: &[u8]) {
    let cipher = edited.envelope().to_bytes().unwrap();
    let reopened = InspectionEnvelope::from_bytes(&cipher, LIMITS).unwrap();
    let project = InspectionRecords::from_envelope(&reopened).unwrap();
    let reverse = project
        .edit_attribute(&measurement(&project).id, "Value", "")
        .unwrap();
    assert_eq!(reverse.project.envelope().xml_bytes(), OWNED);
    assert_eq!(
        reverse.project.envelope().to_bytes().unwrap(),
        original_cipher
    );
}

#[test]
fn lexical_escaping_preserves_every_unselected_byte_and_full_record_history() {
    let source = project(OWNED, InspectionProfile::Current2020);
    let selected = measurement(&source);
    let edit = source
        .edit_attribute(&selected.id, "Value", "a<&\"'\n\t\rΩ")
        .unwrap();
    let expected = std::str::from_utf8(OWNED).unwrap().replacen(
        "<Measurement Name=\"same\" Value=\"\"/>",
        "<Measurement Name=\"same\" Value=\"a&lt;&amp;&quot;'&#10;&#9;&#13;Ω\"/>",
        1,
    );
    assert_eq!(edit.project.envelope().xml_bytes(), expected.as_bytes());
    check_record_history(&source, &edit, &selected.id);
    assert_eq!(edit.coverage.achieved_value_error, 0);
    assert_eq!(edit.diagnostics.as_ref(), source.diagnostics());
    assert_eq!(edit.source.envelope().xml_bytes(), OWNED);
}

fn check_record_history(
    source: &InspectionRecords,
    edit: &InspectionRecordEdit,
    selected: &InspectionRecordId,
) {
    assert_eq!(edit.history.len(), source.records().len());
    for ((before, after), relation) in source
        .records()
        .iter()
        .zip(edit.project.records())
        .zip(edit.history.iter())
    {
        assert_eq!(relation.parents.as_ref(), std::slice::from_ref(&before.id));
        assert_eq!(relation.result, after.id);
        assert_eq!(
            relation.relation == InspectionRecordRelation::Derived,
            before.id == *selected
        );
    }
}

#[test]
fn noops_foreign_stale_absent_and_invalid_values_isolate_inputs() {
    let source = project(OWNED, InspectionProfile::Current2020);
    let selected = &measurement(&source).id;
    let noop = source.edit_attribute(selected, "Value", "").unwrap();
    assert_eq!(noop.completion, InspectionRecordCompletion::Unchanged);
    assert!(noop.project.record(selected).is_ok());
    let changed = source.edit_attribute(selected, "Value", "Pass").unwrap();
    assert!(changed.project.record(selected).is_err());
    assert!(source.record(&measurement(&changed.project).id).is_err());
    assert!(source.edit_attribute(selected, "Absent", "x").is_err());
    assert!(source.edit_attribute(selected, "Value", "\0").is_err());
    let foreign = project(OWNED, InspectionProfile::Current2020);
    assert!(
        source
            .edit_attribute(&measurement(&foreign).id, "Value", "Pass")
            .is_err()
    );
    assert_eq!(source.envelope().xml_bytes(), OWNED);
}

#[test]
fn native_plain_projects_keep_complete_record_counts_and_cipher_profile() {
    let cases: &[(&[u8], InspectionProfile, usize)] = &[(
        include_bytes!("fixtures/standalone-inspection/historical.xml"),
        InspectionProfile::Historical2015,
        158,
    )];
    for (raw, profile, count) in cases {
        let source = project(raw, *profile);
        assert_eq!(source.records().len(), *count);
        let edit = source
            .edit_attribute(&measurement(&source).id, "Value", "Pass")
            .unwrap();
        assert_eq!(edit.project.envelope().profile(), *profile);
        let reverse = edit
            .project
            .edit_attribute(&measurement(&edit.project).id, "Value", "")
            .unwrap();
        assert_eq!(reverse.project.envelope().xml_bytes(), *raw);
    }
}

#[test]
fn empty_complete_projects_and_qualified_descendant_decoys_stay_empty() {
    for raw in [b"<i:SOLIDWORKSINSPECTION xmlns:i='http://www.extensiblecad.com'/>".as_slice(), b"<SOLIDWORKSINSPECTION xmlns='http://www.extensiblecad.com'><INSPECTION_SHEET/></SOLIDWORKSINSPECTION>".as_slice()] {
        let source = project(raw, InspectionProfile::Current2020);
        assert!(source.records().is_empty());
        assert_eq!(source.envelope().xml_bytes(), raw);
        assert_eq!(source.coverage().records, 0);
    }
}

#[test]
fn shared_snapshots_have_same_serial_and_eight_worker_edit_outputs() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<InspectionRecords>();
    send_sync::<InspectionRecord>();
    send_sync::<InspectionRecordId>();
    send_sync::<InspectionRecordEdit>();
    let source = project(OWNED, InspectionProfile::Current2020);
    let expected = edited_cipher(&source);
    std::thread::scope(|scope| {
        let jobs: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| edited_cipher(&source)))
            .collect();
        for job in jobs {
            assert_eq!(job.join().unwrap(), expected);
        }
    });
    assert_eq!(source.envelope().xml_bytes(), OWNED);
}

fn edited_cipher(source: &InspectionRecords) -> Vec<u8> {
    source
        .edit_attribute(&measurement(source).id, "Value", "Pass")
        .unwrap()
        .project
        .envelope()
        .to_bytes()
        .unwrap()
}
