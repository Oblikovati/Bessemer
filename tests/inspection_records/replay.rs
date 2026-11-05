use super::*;

fn attribute_record<'a>(project: &'a InspectionRecords, local: &str) -> &'a InspectionRecord {
    project
        .records()
        .iter()
        .find(|record| record.attribute(local).is_some())
        .unwrap()
}

fn changed_attribute(source: &InspectionRecords, local: &str, value: &str) -> InspectionRecordEdit {
    source
        .edit_attribute(&attribute_record(source, local).id, local, value)
        .unwrap()
}

#[test]
fn numeric_reference_reversal_restores_original_lexeme_and_both_history_parents() {
    let source = project(OWNED, InspectionProfile::Current2020);
    assert!(
        std::str::from_utf8(OWNED)
            .unwrap()
            .contains("Extension=\"&#x3bb;\"")
    );
    let changed = changed_attribute(&source, "Extension", "changed");
    let reverse = changed_attribute(&changed.project, "Extension", "λ");
    assert_eq!(reverse.project.envelope().xml_bytes(), OWNED);
    check_replay_parents(&reverse);
    assert_eq!(reverse.checkpoint.envelope().xml_bytes(), OWNED);
    assert_eq!(source.envelope().xml_bytes(), OWNED);
}

fn check_replay_parents(reverse: &InspectionRecordEdit) {
    let relation = reverse
        .history
        .iter()
        .find(|relation| relation.relation == InspectionRecordRelation::Derived)
        .unwrap();
    assert_eq!(relation.parents.len(), 2);
    let current = reverse.source.record(&relation.parents[0]).unwrap();
    let original = reverse.checkpoint.record(&relation.parents[1]).unwrap();
    assert_eq!(current.attribute("Extension"), Some("changed"));
    assert_eq!(original.attribute("Extension"), Some("λ"));
}

#[test]
fn checkpoint_replay_preserves_unrelated_later_edits_and_original_ownership() {
    let source = project(OWNED, InspectionProfile::Current2020);
    let first = changed_attribute(&source, "Extension", "changed");
    let second = changed_attribute(&first.project, "Nominal", "later");
    let third = changed_attribute(&second.project, "Extension", "λ");
    let expected = std::str::from_utf8(OWNED).unwrap().replacen(
        "Nominal=\"3.503 / 3.496\"",
        "Nominal=\"later\"",
        1,
    );
    assert_eq!(third.project.envelope().xml_bytes(), expected.as_bytes());
    let reverse = changed_attribute(&third.project, "Nominal", "3.503 / 3.496");
    assert_eq!(reverse.project.envelope().xml_bytes(), OWNED);
    let expected_second = expected.replace("&#x3bb;", "changed");
    assert_eq!(
        second.project.envelope().xml_bytes(),
        expected_second.as_bytes()
    );
    assert_eq!(first.source.envelope().xml_bytes(), OWNED);
}

#[test]
fn single_quoted_attribute_replays_noncanonical_named_entities_and_spacing() {
    let raw = include_bytes!("../fixtures/inspection-records/single-quotes.xml");
    let source = project(raw, InspectionProfile::Current2020);
    let original = attribute_record(&source, "Title")
        .attribute("Title")
        .unwrap();
    let changed = changed_attribute(&source, "Title", "changed");
    let reverse = changed_attribute(&changed.project, "Title", original);
    assert_eq!(reverse.project.envelope().xml_bytes(), raw);
    assert_eq!(
        reverse.project.envelope().to_bytes().unwrap(),
        source.envelope().to_bytes().unwrap()
    );
}

#[test]
fn reopened_edited_envelope_has_its_own_observed_lexical_checkpoint() {
    let source = project(OWNED, InspectionProfile::Current2020);
    let changed = changed_attribute(&source, "Extension", "changed");
    let stored = changed.project.envelope().to_bytes().unwrap();
    let reopened = InspectionEnvelope::from_bytes(&stored, LIMITS).unwrap();
    let fresh = InspectionRecords::from_envelope(&reopened).unwrap();
    let edited = changed_attribute(&fresh, "Extension", "λ");
    let expected = std::str::from_utf8(OWNED).unwrap().replace("&#x3bb;", "λ");
    assert_eq!(edited.project.envelope().xml_bytes(), expected.as_bytes());
    let reverse = changed_attribute(&edited.project, "Extension", "changed");
    assert_eq!(reverse.project.envelope().to_bytes().unwrap(), stored);
}
