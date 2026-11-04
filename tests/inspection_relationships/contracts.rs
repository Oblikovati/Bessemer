use super::*;

#[test]
fn absent_empty_and_nonmatching_dictionaries_are_complete_empty_results() {
    for package in [
        "",
        "<PackageOptions/>",
        "<PackageOptions><relationshipsDictionary/></PackageOptions>",
        "<PackageOptions><wrapper><RelationshipsDictionary><item/></RelationshipsDictionary></wrapper></PackageOptions>",
        "<PackageOptions><x:RelationshipsDictionary><item/></x:RelationshipsDictionary></PackageOptions>",
    ] {
        let project = package_project(package);
        for policy in [
            InspectionRelationshipPolicy::RetainedNodes,
            InspectionRelationshipPolicy::ElementsOnly,
        ] {
            let result = views_from(&project, policy);
            assert_completed_empty(&result, &project);
        }
    }
}

#[test]
fn decoded_references_and_empty_cdata_preserve_selected_dom_node_grouping() {
    let project = package_project(
        "<PackageOptions><RelationshipsDictionary>a<![CDATA[]]>b&amp;&#99;<![CDATA[x]]><![CDATA[y]]>z<!--c--><?p i?></RelationshipsDictionary></PackageOptions>",
    );
    let result = views_from(&project, InspectionRelationshipPolicy::RetainedNodes);
    use InspectionRelationshipNodeKind::*;
    let kinds: Vec<_> = result.attempts.iter().map(|attempt| attempt.kind).collect();
    assert_eq!(
        kinds,
        [Text, CData, CData, Text, Comment, ProcessingInstruction]
    );
    assert!(result.attempts.iter().all(|attempt| attempt.key.is_empty()
        && attempt.values.is_empty()
        && attempt.element_name.is_none()));
    assert_eq!(result.entries.len(), 1);
    assert!(result.writer_entries.is_empty());
}

#[test]
fn nested_text_and_numeric_line_references_keep_exact_values_before_qt_trim() {
    let project = package_project(
        "<PackageOptions><RelationshipsDictionary><item><key><string>&#x85; K &#x3000;</string></key><value><ArrayOfString><string>a&amp;&#98;<![CDATA[c]]>d&#x65;</string><Other>a<x>b<y>c</y>d</x>e<!--ignored--><?p ignored?>f</Other><string>a\r\nb\rc&#13;d&#10;e<![CDATA[f\r\ng\rh]]></string><string>&#x200b;x&#xfeff;</string></ArrayOfString></value></item></RelationshipsDictionary></PackageOptions>",
    );
    let result = views_from(&project, InspectionRelationshipPolicy::RetainedNodes);
    assert_eq!(result.entries[0].key.as_ref(), "K");
    let values: Vec<_> = result.entries[0]
        .values
        .iter()
        .map(|value| value.as_ref())
        .collect();
    assert_eq!(
        values,
        [
            "a&bcde",
            "abcdef",
            "a\nb\nc\rd\nef\ng\nh",
            "\u{200b}x\u{feff}"
        ]
    );
}

#[test]
fn wrong_record_roles_and_foreign_sheet_ids_fail_without_mutating_source() {
    let project = package_project("<DOCUMENT><DOCUMENT Uid='D'/></DOCUMENT><PackageOptions/>");
    let before = project.envelope().to_bytes().unwrap();
    let document = record_id(&project, InspectionRecordKind::Document);
    assert!(
        project
            .relationship_views(document, InspectionRelationshipPolicy::RetainedNodes)
            .is_err()
    );
    let foreign = package_project("<PackageOptions/>");
    let sheet = record_id(&foreign, InspectionRecordKind::Sheet);
    assert!(
        project
            .relationship_views(sheet, InspectionRelationshipPolicy::RetainedNodes)
            .is_err()
    );
    assert_eq!(project.envelope().to_bytes().unwrap(), before);
}

#[test]
fn completed_projection_reopens_both_cipher_profiles_without_changing_any_bytes() {
    for profile in [
        InspectionProfile::Current2020,
        InspectionProfile::Historical2015,
    ] {
        let xml = include_bytes!("../fixtures/inspection-relationships/ordered.xml");
        let source = project();
        let envelope = InspectionEnvelope::from_xml(profile, xml, LIMITS).unwrap();
        let project = InspectionRecords::from_envelope(&envelope).unwrap();
        let original = envelope.to_bytes().unwrap();
        let result = views_from(&project, InspectionRelationshipPolicy::RetainedNodes);
        assert_eq!(result.source.envelope().to_bytes().unwrap(), original);
        assert_eq!(result.source.envelope().xml_bytes(), xml);
        assert_eq!(result.attempts.len(), 9);
        assert_eq!(result.coverage, source.coverage());
        assert_reopened_values(&original, profile, xml, &result);
    }
}

#[test]
fn historical_tutorial_projects_all_48_writer_keys_and_120_reciprocal_links() {
    let project = project_from_xml(include_bytes!(
        "../fixtures/standalone-inspection/historical.xml"
    ));
    let result = views_from(&project, InspectionRelationshipPolicy::RetainedNodes);
    assert_eq!(result.writer_entries.len(), 48);
    assert_eq!(
        result
            .writer_entries
            .iter()
            .map(|entry| entry.values.len())
            .sum::<usize>(),
        120
    );
    assert_reciprocal(&result);
}

fn assert_completed_empty(result: &InspectionRelationshipViews, project: &InspectionRecords) {
    assert_eq!(
        result.completion,
        InspectionRelationshipCompletion::Completed
    );
    assert!(result.attempts.is_empty());
    assert!(result.entries.is_empty());
    assert!(result.writer_entries.is_empty());
    assert_eq!(result.coverage, project.coverage());
}

fn record_id(project: &InspectionRecords, kind: InspectionRecordKind) -> &InspectionRecordId {
    &project
        .records()
        .iter()
        .find(|record| record.kind == kind)
        .unwrap()
        .id
}

fn assert_reopened_values(
    original: &[u8],
    profile: InspectionProfile,
    xml: &[u8],
    result: &InspectionRelationshipViews,
) {
    let reopened = InspectionEnvelope::from_bytes(original, LIMITS).unwrap();
    assert_eq!(reopened.profile(), profile);
    let reopened = InspectionRecords::from_envelope(&reopened).unwrap();
    let reopened = views_from(&reopened, InspectionRelationshipPolicy::RetainedNodes);
    assert_eq!(reopened.source.envelope().xml_bytes(), xml);
    assert_eq!(reopened.entries[1].values, result.entries[1].values);
}

fn assert_reciprocal(result: &InspectionRelationshipViews) {
    for entry in result.writer_entries.iter() {
        for value in entry.values.iter() {
            let target = result
                .entries
                .iter()
                .find(|target| &target.key == value)
                .unwrap();
            assert!(target.values.contains(&entry.key));
        }
    }
}
