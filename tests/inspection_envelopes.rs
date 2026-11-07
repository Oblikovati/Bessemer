//! Exact standalone Inspection envelopes, immutable XML replacement and native Framework oracles.
use bessemer::inspection::{InspectionEnvelope, InspectionProfile};
use bessemer::metadata::XmlLimits;

const LIMITS: XmlLimits = XmlLimits {
    maximum_bytes: 1024 * 1024,
    maximum_nodes: 100_000,
    maximum_attributes: 200_000,
    maximum_depth: 64,
};
const MINIMAL: &[u8] = br#"<iex:SOLIDWORKSINSPECTION xmlns:iex="http://www.extensiblecad.com"><INSPECTION_SHEET CapturesVisible="false"/></iex:SOLIDWORKSINSPECTION>"#;
type EnvelopeCase = (
    &'static [u8],
    &'static [u8],
    &'static [u8],
    &'static [u8],
    InspectionProfile,
);
const CASES: [EnvelopeCase; 3] = [
    (
        include_bytes!("fixtures/standalone-inspection/current-0.bin"),
        include_bytes!("fixtures/standalone-inspection/current-0.xml"),
        include_bytes!("fixtures/standalone-inspection/current-0-edited.bin"),
        include_bytes!("fixtures/standalone-inspection/current-0-edited.xml"),
        InspectionProfile::Current2020,
    ),
    (
        include_bytes!("fixtures/standalone-inspection/current-1.bin"),
        include_bytes!("fixtures/standalone-inspection/current-1.xml"),
        include_bytes!("fixtures/standalone-inspection/current-1-edited.bin"),
        include_bytes!("fixtures/standalone-inspection/current-1-edited.xml"),
        InspectionProfile::Current2020,
    ),
    (
        include_bytes!("fixtures/standalone-inspection/historical.bin"),
        include_bytes!("fixtures/standalone-inspection/historical.xml"),
        include_bytes!("fixtures/standalone-inspection/historical-edited.bin"),
        include_bytes!("fixtures/standalone-inspection/historical-edited.xml"),
        InspectionProfile::Historical2015,
    ),
];

#[test]
fn native_current_templates_and_historical_tutorial_match_framework_plaintext_and_ciphertext() {
    for (stored, plain, _, _, profile) in CASES {
        let archive = InspectionEnvelope::from_bytes(stored, LIMITS).unwrap();
        assert_eq!(archive.profile(), profile);
        assert_eq!(archive.xml_bytes(), plain);
        assert_eq!(archive.to_bytes().unwrap(), stored);
        assert_eq!(
            InspectionEnvelope::from_xml(profile, plain, LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            stored
        );
    }
}

#[test]
fn xml_replacement_matches_independent_framework_goldens_without_mutating_inputs() {
    for (stored, plain, expected, edited_xml, profile) in CASES {
        let archive = InspectionEnvelope::from_bytes(stored, LIMITS).unwrap();
        let edited = archive.with_xml(edited_xml).unwrap();
        assert_eq!(edited.profile(), profile);
        assert_eq!(edited.to_bytes().unwrap(), expected);
        assert_eq!(
            InspectionEnvelope::from_bytes(expected, LIMITS)
                .unwrap()
                .xml_bytes(),
            edited_xml
        );
        assert_eq!(archive.xml_bytes(), plain);
        assert_eq!(archive.to_bytes().unwrap(), stored);
    }
}

#[test]
fn owned_xml_and_ciphertext_are_isolated_from_later_source_mutation() {
    for profile in [
        InspectionProfile::Current2020,
        InspectionProfile::Historical2015,
    ] {
        let mut xml = MINIMAL.to_vec();
        let archive = InspectionEnvelope::from_xml(profile, &xml, LIMITS).unwrap();
        xml.fill(b'X');
        assert_eq!(archive.xml_bytes(), MINIMAL);
        let mut stored = archive.to_bytes().unwrap();
        let parsed = InspectionEnvelope::from_bytes(&stored, LIMITS).unwrap();
        stored.fill(0);
        assert_eq!(parsed.xml_bytes(), MINIMAL);
        assert!(parsed.with_xml(b"<wrong/>").is_err());
        assert_eq!(parsed.xml_bytes(), MINIMAL);
    }
}

#[test]
fn every_small_envelope_prefix_and_extra_byte_rejects_incomplete_results() {
    for profile in [
        InspectionProfile::Current2020,
        InspectionProfile::Historical2015,
    ] {
        let stored = InspectionEnvelope::from_xml(profile, MINIMAL, LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap();
        for end in 0..stored.len() {
            assert!(
                InspectionEnvelope::from_bytes(&stored[..end], LIMITS).is_err(),
                "{profile:?}/{end}"
            );
        }
        let mut extra = stored;
        extra.push(0);
        assert!(InspectionEnvelope::from_bytes(&extra, LIMITS).is_err());
    }
}

#[test]
fn invalid_xml_namespaces_entities_utf8_and_all_resource_budgets_are_rejected() {
    for invalid in [
        b"<SOLIDWORKSINSPECTION/>".as_slice(),
        b"<x:SOLIDWORKSINSPECTION xmlns:x='wrong'/>",
        b"<!DOCTYPE p [<!ENTITY x 'v'>]><p>&x;</p>",
        b"\xff",
        b"<",
        b"",
    ] {
        assert!(
            InspectionEnvelope::from_xml(InspectionProfile::Current2020, invalid, LIMITS).is_err()
        );
    }
    let stored = InspectionEnvelope::from_xml(InspectionProfile::Current2020, MINIMAL, LIMITS)
        .unwrap()
        .to_bytes()
        .unwrap();
    for limits in [
        XmlLimits {
            maximum_bytes: stored.len() - 1,
            ..LIMITS
        },
        XmlLimits {
            maximum_nodes: 0,
            ..LIMITS
        },
        XmlLimits {
            maximum_attributes: 0,
            ..LIMITS
        },
        XmlLimits {
            maximum_depth: 0,
            ..LIMITS
        },
    ] {
        assert!(InspectionEnvelope::from_bytes(&stored, limits).is_err());
    }
}

#[test]
fn namespace_prefix_aliases_bom_and_unqualified_children_preserve_exact_xml() {
    let mut xml = vec![0xef, 0xbb, 0xbf];
    xml.extend_from_slice(r#"<?xml version="1.0" encoding="UTF-8"?><alias:SOLIDWORKSINSPECTION xmlns:alias="http://www.extensiblecad.com"><INSPECTION_SHEET Name="鋼"/></alias:SOLIDWORKSINSPECTION>"#.as_bytes());
    let archive =
        InspectionEnvelope::from_xml(InspectionProfile::Current2020, &xml, LIMITS).unwrap();
    let stored = archive.to_bytes().unwrap();
    assert_eq!(
        InspectionEnvelope::from_bytes(&stored, LIMITS)
            .unwrap()
            .xml_bytes(),
        xml
    );
}

#[test]
fn published_envelopes_are_send_sync_and_parallel_writes_are_identical() {
    fn require_bounds<T: Send + Sync>() {}
    require_bounds::<InspectionEnvelope>();
    require_bounds::<InspectionProfile>();
    let archive = InspectionEnvelope::from_bytes(CASES[0].0, LIMITS).unwrap();
    let outputs: Vec<_> = (0..8)
        .map(|_| {
            let snapshot = archive.clone();
            std::thread::spawn(move || snapshot.to_bytes().unwrap())
        })
        .collect();
    for output in outputs {
        assert_eq!(output.join().unwrap(), CASES[0].0);
    }
}

#[test]
fn all_forty_two_installed_template_entries_match_their_owned_envelopes() {
    let rows: Vec<_> = include_str!("fixtures/standalone-inspection/templates.tsv")
        .lines()
        .collect();
    assert_eq!(rows.len(), 42);
    let mut variants = [0_usize; 2];
    for row in rows {
        let columns: Vec<_> = row.split('\t').collect();
        assert_eq!(columns.len(), 4);
        let index = match columns[1] {
            "current-0" => 0,
            "current-1" => 1,
            other => panic!("unexpected template variant {other}"),
        };
        variants[index] += 1;
        let archive = InspectionEnvelope::from_bytes(CASES[index].0, LIMITS).unwrap();
        assert_eq!(archive.xml_bytes(), CASES[index].1);
        assert_eq!(archive.to_bytes().unwrap(), CASES[index].0);
    }
    assert_eq!(variants, [28, 14]);
}

#[test]
fn dtd_is_rejected_even_with_the_expected_standalone_root() {
    let xml=br#"<!DOCTYPE iex:SOLIDWORKSINSPECTION><iex:SOLIDWORKSINSPECTION xmlns:iex="http://www.extensiblecad.com"/>"#;
    assert!(InspectionEnvelope::from_xml(InspectionProfile::Current2020, xml, LIMITS).is_err());
}
