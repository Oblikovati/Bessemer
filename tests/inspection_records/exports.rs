use super::*;
use std::io::{BufWriter, Write};

const REPLACEMENT: &str = "Bessemer Ω<&\"'\n\t\r";
const EXPORT_CASES: [(&[u8], InspectionProfile); 6] = [
    (OWNED, InspectionProfile::Current2020),
    (
        include_bytes!("../fixtures/inspection-records/load-projections.xml"),
        InspectionProfile::Current2020,
    ),
    (
        include_bytes!("../fixtures/standalone-inspection/historical.xml"),
        InspectionProfile::Historical2015,
    ),
    (
        include_bytes!("../fixtures/standalone-inspection/current-0.xml"),
        InspectionProfile::Current2020,
    ),
    (
        include_bytes!("../fixtures/standalone-inspection/current-1.xml"),
        InspectionProfile::Current2020,
    ),
    (
        include_bytes!("../fixtures/inspection-records/single-quotes.xml"),
        InspectionProfile::Current2020,
    ),
];

#[test]
#[ignore = "requires explicit BESSEMER_INSPECTION_RECORD_EXPORT fresh-process output path"]
fn every_unqualified_record_attribute_exports_a_reversible_isolated_edit() {
    let path = std::env::var_os("BESSEMER_INSPECTION_RECORD_EXPORT")
        .expect("expected explicit export output path");
    let projects: Vec<_> = EXPORT_CASES
        .iter()
        .map(|(raw, profile)| project(raw, *profile))
        .collect();
    let counts: Vec<_> = projects.iter().map(attribute_count).collect();
    assert_eq!(counts, [24, 3, 1288, 16, 16, 2]);
    let mut output = BufWriter::new(std::fs::File::create(path).unwrap());
    output.write_all(b"IRX1").unwrap();
    export_word(&mut output, counts.iter().sum());
    for (case, project) in projects.iter().enumerate() {
        export_project(&mut output, project, case);
        assert_eq!(project.envelope().xml_bytes(), EXPORT_CASES[case].0);
    }
    output.flush().unwrap();
}

fn attribute_count(project: &InspectionRecords) -> usize {
    project
        .records()
        .iter()
        .flat_map(|record| record.attributes.iter())
        .filter(|attribute| attribute.name.namespace.is_none())
        .count()
}

fn export_project(output: &mut impl Write, source: &InspectionRecords, case: usize) {
    for (record_index, record) in source.records().iter().enumerate() {
        for (attribute_index, attribute) in record.attributes.iter().enumerate() {
            if attribute.name.namespace.is_some() {
                continue;
            }
            let edit = source
                .edit_attribute(&record.id, &attribute.name.local, REPLACEMENT)
                .unwrap();
            check_export_edit(source, &edit, record_index, attribute_index);
            let xml = edit.project.envelope().xml_bytes();
            for word in [case, record_index, attribute_index, xml.len()] {
                export_word(output, word);
            }
            output.write_all(xml).unwrap();
        }
    }
}

fn check_export_edit(
    source: &InspectionRecords,
    edit: &InspectionRecordEdit,
    record_index: usize,
    attribute_index: usize,
) {
    let before = &source.records()[record_index];
    let attribute = &before.attributes[attribute_index];
    assert_ne!(attribute.value, REPLACEMENT);
    assert_eq!(edit.completion, InspectionRecordCompletion::Changed);
    check_record_history(source, edit, &before.id);
    let result = &edit.project.records()[record_index];
    assert_eq!(result.attribute(&attribute.name.local), Some(REPLACEMENT));
    let reverse = edit
        .project
        .edit_attribute(&result.id, &attribute.name.local, &attribute.value)
        .unwrap();
    assert_eq!(
        reverse.project.envelope().xml_bytes(),
        source.envelope().xml_bytes()
    );
    assert_eq!(
        edit.source.envelope().xml_bytes(),
        source.envelope().xml_bytes()
    );
}

fn export_word(output: &mut impl Write, value: usize) {
    output
        .write_all(&u32::try_from(value).unwrap().to_le_bytes())
        .unwrap();
}
