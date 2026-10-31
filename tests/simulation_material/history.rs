use super::load_edit_case::read_edit_case;
use super::load_projection::{
    consume_expected_fields, project_field, project_outer_history, project_packet, project_word,
};
use super::*;

const EXPECTED: &[u8] = include_bytes!("../fixtures/simulation-material/history.bin");

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let case = read_edit_case(cursor, &[0_usize]);
    let source = SimulationStudyMaterial::from_bytes(case.original, case.version, LIMITS).unwrap();
    let requested = source
        .fields()
        .iter()
        .position(|field| field.path.as_ref() == "word68")
        .unwrap();
    let id = source.field_id(requested).unwrap();
    let edit = source
        .edit_field(&id, SimulationMaterialValue::Signed(case.replacement))
        .unwrap();
    assert_eq!(edit.source().to_bytes(), case.original);
    assert_eq!(edit.archive().to_bytes(), case.expected);
    assert!(case.changed);
    assert_eq!(edit.parent(), id);
    project_header(output, case.kind, case.version, case.replacement, &edit);
    project_result_fields(output, &edit);
    project_history(output, &edit);
    consume_expected_fields(cursor);
    consume_history(cursor);
}

fn project_header(
    output: &mut Vec<u8>,
    kind: usize,
    version: SimulationPersistVersion,
    replacement: i32,
    edit: &SimulationMaterialEdit,
) {
    project_word(output, kind);
    project_word(output, version.value() as usize);
    output.extend_from_slice(&replacement.to_le_bytes());
    project_word(
        output,
        usize::from(edit.coverage().completion == SimulationMaterialCompletion::Changed),
    );
    project_word(output, edit.source().to_bytes().len());
    output.extend_from_slice(&edit.source().to_bytes());
    output.extend_from_slice(&edit.archive().to_bytes());
}

fn project_result_fields(output: &mut Vec<u8>, edit: &SimulationMaterialEdit) {
    let body = edit.archive();
    project_word(output, body.fields().len());
    for (field, span) in body.fields().iter().zip(body.field_offsets()) {
        project_field(output, field, *span);
    }
}

fn project_history(output: &mut Vec<u8>, edit: &SimulationMaterialEdit) {
    let source_ids: Vec<SimulationMaterialFieldId> = (0..edit.source().fields().len())
        .map(|index| edit.source().field_id(index).unwrap())
        .collect();
    let result_ids: Vec<SimulationMaterialFieldId> = (0..edit.archive().fields().len())
        .map(|index| edit.archive().field_id(index).unwrap())
        .collect();
    project_outer_history(output, edit.history(), &source_ids, &result_ids);
}

fn consume_history(cursor: &mut &[u8]) {
    for _ in 0..word(cursor) {
        word(cursor);
        word(cursor);
        for _ in 0..word(cursor) {
            word(cursor);
            word(cursor);
        }
    }
}

#[test]
fn every_owned_edited_image_field_and_parent_matches_complete_independent_source() {
    assert_eq!(project_packet(EXPECTED, 300, project_case), EXPECTED);
}

#[test]
#[ignore = "Requires explicitly recovered complete native Material edited-field/history expectations"]
fn all_native_edited_fields_and_parents_match_source_with_optional_export() {
    let path = std::env::var("BESSEMER_MATERIAL_NATIVE_HISTORY_INPUT")
        .expect("BESSEMER_MATERIAL_NATIVE_HISTORY_INPUT=<unset>; expected explicit complete native history path");
    let native = std::fs::read(&path).unwrap_or_else(|error| {
        panic!("input {path:?}: {error}; expected readable complete native history packet")
    });
    let mut output = project_packet(EXPECTED, 300, project_case);
    output.extend(project_packet(&native, 1230, project_case));
    if let Some(path) = std::env::var_os("BESSEMER_MATERIAL_HISTORY_OUTPUT") {
        std::fs::write(&path, output).unwrap_or_else(|error| {
            panic!("output {path:?}: {error}; expected writable explicit full history export")
        });
    }
}
