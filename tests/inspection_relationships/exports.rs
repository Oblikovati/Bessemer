use super::*;

#[test]
#[ignore = "manual fresh-process complete relationship packet"]
fn export_complete_relationship_results() {
    let destination = std::env::var_os("BESSEMER_RELATIONSHIP_EXPORT").unwrap();
    let cases = export_cases();
    let mut packet = b"IRL1".to_vec();
    put_number(&mut packet, cases.len());
    for xml in cases {
        put_bytes(&mut packet, &xml);
        let project = project_from_xml(&xml);
        for policy in [
            InspectionRelationshipPolicy::RetainedNodes,
            InspectionRelationshipPolicy::ElementsOnly,
        ] {
            let result = views_from(&project, policy);
            assert_provenance(&project, &result);
            put_result(&mut packet, &result);
        }
    }
    std::fs::write(destination, packet).unwrap();
}

fn export_cases() -> Vec<Vec<u8>> {
    let mut cases = vec![
        include_bytes!("../fixtures/inspection-relationships/ordered.xml").to_vec(),
        include_bytes!("../fixtures/standalone-inspection/historical.xml").to_vec(),
    ];
    for package in EDGE_PACKAGES {
        let project = package_project(package);
        cases.push(project.envelope().xml_bytes().to_vec());
    }
    cases
}

const EDGE_PACKAGES: &[&str] = &[
    "",
    "<PackageOptions/>",
    "<PackageOptions><relationshipsDictionary/></PackageOptions>",
    "<PackageOptions><wrapper><RelationshipsDictionary><item/></RelationshipsDictionary></wrapper></PackageOptions>",
    "<PackageOptions><x:RelationshipsDictionary><item/></x:RelationshipsDictionary></PackageOptions>",
    "<PackageOptions><RelationshipsDictionary>a<![CDATA[]]>b&amp;&#99;<![CDATA[x]]><![CDATA[y]]>z<!--c--><?p i?></RelationshipsDictionary></PackageOptions>",
    "<PackageOptions><RelationshipsDictionary><item><key><string>&#x85; K &#x3000;</string></key><value><ArrayOfString><string>a&amp;&#98;<![CDATA[c]]>d&#x65;</string><Other>a<x>b<y>c</y>d</x>e<!--ignored--><?p ignored?>f</Other><string>a\r\nb\rc&#13;d&#10;e<![CDATA[f\r\ng\rh]]></string><string>&#x200b;x&#xfeff;</string></ArrayOfString></value></item></RelationshipsDictionary></PackageOptions>",
    "<PackageOptions><RelationshipsDictionary><item><key><string>First</string></key><key/></item><item><key><string/></key><value><ArrayOfString><string>ignored</string></ArrayOfString></value></item></RelationshipsDictionary></PackageOptions>",
    "<PackageOptions><RelationshipsDictionary><item><key><string>A</string><string>ignored</string></key><value><ArrayOfString><string>x</string></ArrayOfString><ArrayOfString><string>ignored</string></ArrayOfString></value></item></RelationshipsDictionary><RelationshipsDictionary><item/></RelationshipsDictionary></PackageOptions>",
    "<PackageOptions><RelationshipsDictionary xmlns='urn:default'><item><key><string>😀</string></key><value><ArrayOfString><x:Any>é</x:Any></ArrayOfString></value></item><item><key><string>😀</string></key><value><ArrayOfString><string>discarded</string></ArrayOfString></value></item><item><key><string>é</string></key></item><item><key><string>é</string></key></item></RelationshipsDictionary></PackageOptions>",
];

fn assert_provenance(project: &InspectionRecords, result: &InspectionRelationshipViews) {
    assert_eq!(
        result.completion,
        InspectionRelationshipCompletion::Completed
    );
    assert_eq!(
        result.source.envelope().xml_bytes(),
        project.envelope().xml_bytes()
    );
    assert_eq!(result.coverage, project.coverage());
    assert_eq!(result.coverage.achieved_value_error, 0);
    assert_sheet_history(result);
    for attempt in result.attempts.iter() {
        assert_eq!(result.attempt(&attempt.id), Some(attempt));
    }
    assert_diagnostics(result);
}

fn put_result(packet: &mut Vec<u8>, result: &InspectionRelationshipViews) {
    packet.push(u8::from(
        result.policy == InspectionRelationshipPolicy::ElementsOnly,
    ));
    put_number(packet, result.coverage.xml_bytes);
    put_number(packet, result.coverage.elements);
    put_number(packet, result.coverage.records);
    put_number(packet, result.coverage.achieved_value_error);
    put_number(packet, result.attempts.len());
    for attempt in result.attempts.iter() {
        put_attempt(packet, attempt);
    }
    put_selected(packet, &result.entries, &result.attempts);
    put_selected(packet, &result.writer_entries, &result.attempts);
}

fn put_attempt(packet: &mut Vec<u8>, attempt: &InspectionRelationshipAttempt) {
    packet.push(node_number(attempt.kind));
    packet.push(u8::from(attempt.element_name.is_some()));
    if let Some(name) = &attempt.element_name {
        put_bytes(packet, name.as_bytes());
    }
    put_bytes(packet, attempt.key.as_bytes());
    put_number(packet, attempt.values.len());
    for value in attempt.values.iter() {
        put_bytes(packet, value.as_bytes());
    }
}

fn node_number(kind: InspectionRelationshipNodeKind) -> u8 {
    use InspectionRelationshipNodeKind::*;
    match kind {
        Element => 1,
        Text => 3,
        CData => 4,
        Comment => 8,
        ProcessingInstruction => 7,
    }
}

fn put_selected(
    packet: &mut Vec<u8>,
    selected: &[InspectionRelationshipAttempt],
    attempts: &[InspectionRelationshipAttempt],
) {
    put_number(packet, selected.len());
    for entry in selected {
        let index = attempts
            .iter()
            .position(|attempt| attempt.id == entry.id)
            .unwrap();
        assert_eq!(&attempts[index], entry);
        put_number(packet, index);
    }
}

fn put_number(packet: &mut Vec<u8>, value: usize) {
    packet.extend_from_slice(&u64::try_from(value).unwrap().to_le_bytes());
}

fn put_bytes(packet: &mut Vec<u8>, value: &[u8]) {
    put_number(packet, value.len());
    packet.extend_from_slice(value);
}

fn assert_sheet_history(result: &InspectionRelationshipViews) {
    assert_eq!(result.history[0].parents.as_ref(), &[result.sheet.clone()]);
    assert_eq!(result.history[0].result, result.sheet);
    assert_eq!(result.history[0].relation, InspectionRecordRelation::Copied);
    assert!(result.source.record(&result.sheet).is_ok());
}

fn assert_diagnostics(result: &InspectionRelationshipViews) {
    assert_eq!(
        result.diagnostics,
        [
            InspectionRelationshipDiagnostic::NativeDomPolicyUnknown,
            InspectionRelationshipDiagnostic::NativeMapOrderUnknown,
            InspectionRelationshipDiagnostic::GraphAndApplicationSemanticsNotChecked
        ]
    );
}
