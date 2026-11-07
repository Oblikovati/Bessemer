use super::*;
use std::path::Path;

const CURRENT_CASES: [(&[u8], &[u8]); 2] = [
    (
        include_bytes!("../fixtures/standalone-inspection/current-0.bin"),
        include_bytes!("../fixtures/standalone-inspection/current-0.xml"),
    ),
    (
        include_bytes!("../fixtures/standalone-inspection/current-1.bin"),
        include_bytes!("../fixtures/standalone-inspection/current-1.xml"),
    ),
];

fn document(project: &InspectionRecords) -> &InspectionRecord {
    project
        .records()
        .iter()
        .find(|record| record.kind == InspectionRecordKind::Document)
        .unwrap()
}

fn check_template(stored: &[u8], xml: &[u8]) {
    let envelope = InspectionEnvelope::from_bytes(stored, LIMITS).unwrap();
    assert_eq!(envelope.xml_bytes(), xml);
    let source = InspectionRecords::from_envelope(&envelope).unwrap();
    assert_eq!(source.records().len(), 2);
    assert_eq!(source.records()[0].kind, InspectionRecordKind::Sheet);
    assert_eq!(
        source.records()[1].owner,
        Some(source.records()[0].id.clone())
    );
    let title = document(&source).attribute("Title").unwrap();
    let edit = source
        .edit_attribute(&document(&source).id, "Title", "Bessemer fixture Ω")
        .unwrap();
    check_template_title_bytes(xml, edit.project.envelope().xml_bytes(), title);
    check_template_reopen_reverse(&edit.project, title, stored);
    assert_eq!(source.envelope().to_bytes().unwrap(), stored);
}

fn check_template_title_bytes(original: &[u8], actual: &[u8], title: &str) {
    let original = std::str::from_utf8(original).unwrap();
    let needle = format!("Title=\"{title}\"");
    assert_eq!(original.matches(&needle).count(), 1);
    let expected = original.replacen(&needle, "Title=\"Bessemer fixture Ω\"", 1);
    assert_eq!(actual, expected.as_bytes());
}

fn check_template_reopen_reverse(edited: &InspectionRecords, title: &str, original: &[u8]) {
    let stored = edited.envelope().to_bytes().unwrap();
    let envelope = InspectionEnvelope::from_bytes(&stored, LIMITS).unwrap();
    let reopened = InspectionRecords::from_envelope(&envelope).unwrap();
    let reverse = reopened
        .edit_attribute(&document(&reopened).id, "Title", title)
        .unwrap();
    assert_eq!(reverse.project.envelope().to_bytes().unwrap(), original);
}

#[test]
fn both_owned_current_templates_keep_empty_characteristic_collections_and_exact_title_edits() {
    for (stored, xml) in CURRENT_CASES {
        check_template(stored, xml);
    }
}

#[test]
#[ignore = "requires explicit BESSEMER_INSPECTION_TEMPLATE_ROOT installed corpus"]
fn all_42_original_installed_templates_reopen_and_reverse_record_edits() {
    let root = std::env::var_os("BESSEMER_INSPECTION_TEMPLATE_ROOT")
        .expect("expected explicit template root");
    let rows: Vec<_> = include_str!("../fixtures/standalone-inspection/templates.tsv")
        .lines()
        .collect();
    assert_eq!(rows.len(), 42);
    let mut variants = [0; 2];
    for row in rows {
        let columns: Vec<_> = row.split('\t').collect();
        let variant = match columns[1] {
            "current-0" => 0,
            "current-1" => 1,
            other => panic!("variant {other}; expected current-0/current-1"),
        };
        let stored = std::fs::read(Path::new(&root).join(columns[0])).unwrap();
        assert_eq!(stored, CURRENT_CASES[variant].0);
        check_template(&stored, CURRENT_CASES[variant].1);
        variants[variant] += 1;
    }
    assert_eq!(variants, [28, 14]);
}
