use super::*;

const BOUNDED_XML: &[u8] = b"<i:SOLIDWORKSINSPECTION xmlns:i='http://www.extensiblecad.com'><INSPECTION_SHEET><PackageOptions><RelationshipsDictionary><!--c--><?p x?><![CDATA[t]]>a&amp;<item key='v'/></RelationshipsDictionary></PackageOptions></INSPECTION_SHEET></i:SOLIDWORKSINSPECTION>";

fn exact_limits() -> XmlLimits {
    XmlLimits {
        maximum_bytes: (BOUNDED_XML.len() / 16 + 1) * 16,
        maximum_nodes: 15,
        maximum_attributes: 2,
        maximum_depth: 5,
    }
}

fn one_below_limits() -> [XmlLimits; 4] {
    let exact = exact_limits();
    [
        XmlLimits {
            maximum_bytes: exact.maximum_bytes - 1,
            ..exact
        },
        XmlLimits {
            maximum_nodes: exact.maximum_nodes - 1,
            ..exact
        },
        XmlLimits {
            maximum_attributes: exact.maximum_attributes - 1,
            ..exact
        },
        XmlLimits {
            maximum_depth: exact.maximum_depth - 1,
            ..exact
        },
    ]
}

#[test]
fn exact_lexical_limits_include_omitted_nodes_and_each_one_below_is_rejected() {
    for profile in [
        InspectionProfile::Current2020,
        InspectionProfile::Historical2015,
    ] {
        let envelope = InspectionEnvelope::from_xml(profile, BOUNDED_XML, exact_limits()).unwrap();
        let project = InspectionRecords::from_envelope(&envelope).unwrap();
        let retained = views_from(&project, InspectionRelationshipPolicy::RetainedNodes);
        let elements = views_from(&project, InspectionRelationshipPolicy::ElementsOnly);
        assert_eq!(retained.attempts.len(), 5);
        assert_eq!(elements.attempts.len(), 1);
        assert_eq!(retained.coverage.elements, 5);
        for limits in one_below_limits() {
            assert!(
                InspectionEnvelope::from_xml(profile, BOUNDED_XML, limits).is_err(),
                "accepted {limits:?}; expected exact lower bound rejection"
            );
        }
    }
}

#[test]
fn every_proper_prefix_and_appended_nul_is_rejected_without_source_mutation() {
    let source = project_from_xml(BOUNDED_XML);
    let original = source.envelope().to_bytes().unwrap();
    assert_every_prefix_rejected();
    let mut appended = BOUNDED_XML.to_vec();
    appended.push(0);
    assert!(
        InspectionEnvelope::from_xml(InspectionProfile::Current2020, &appended, LIMITS).is_err()
    );
    assert_eq!(source.envelope().to_bytes().unwrap(), original);
    assert_eq!(
        views_from(&source, InspectionRelationshipPolicy::ElementsOnly)
            .attempts
            .len(),
        1
    );
}

fn assert_every_prefix_rejected() {
    for end in 0..BOUNDED_XML.len() {
        assert!(
            InspectionEnvelope::from_xml(
                InspectionProfile::Current2020,
                &BOUNDED_XML[..end],
                LIMITS
            )
            .is_err(),
            "accepted prefix {end}; expected complete XML"
        );
    }
}
