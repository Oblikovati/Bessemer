use super::load_edit_case::{LoadEditCase, read_edit_case};
use super::load_history::{LoadHistorySnapshot, LoadHistoryView, project_history, project_nested};
use super::*;

const EXPECTED: &[u8] = include_bytes!("../fixtures/simulation-mesh-property/history.bin");

fn snapshot(property: &SimulationMeshProperty) -> LoadHistorySnapshot<'_> {
    LoadHistorySnapshot::new(
        property.to_bytes(),
        property.fields(),
        property.field_offsets(),
        |index| property.field_id(index).unwrap(),
    )
}

fn make_edit(case: &LoadEditCase<'_, ()>) -> SimulationMeshPropertyEdit {
    let source = SimulationMeshProperty::from_bytes(case.original, case.version, LIMITS).unwrap();
    assert_eq!(source.fields()[0].path.as_ref(), "mesh.word594");
    let parent = source.field_id(0).unwrap();
    let replacement = SimulationMeshPropertyValue::Signed(case.replacement);
    let edit = source.edit_field(&parent, replacement.clone()).unwrap();
    assert_eq!(edit.archive().to_bytes(), case.expected);
    assert_eq!(edit.source().to_bytes(), case.original);
    assert_eq!(edit.parent(), parent);
    assert_eq!(edit.replacement(), &replacement);
    assert_eq!(edit.result(), edit.archive().field_id(0).unwrap());
    assert_ne!(edit.result(), parent);
    assert_edit_coverage(&edit);
    assert_constructed(edit.archive());
    edit
}

fn assert_edit_coverage(edit: &SimulationMeshPropertyEdit) {
    assert_eq!(
        edit.coverage().completion,
        SimulationMeshPropertyCompletion::Changed
    );
    assert_eq!(
        edit.coverage().check_mode,
        SimulationMeshPropertyCheckMode::ExactSelectedProfilesAndPreservation
    );
}

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let case = read_edit_case(cursor, &[()]);
    let edit = make_edit(&case);
    let view = LoadHistoryView {
        source: snapshot(edit.source()),
        result: snapshot(edit.archive()),
        outer: edit.history(),
        nested: edit.nested_history(),
    };
    project_history(cursor, output, &case, 0, &view);
    project_nested(cursor, output, &view);
}

#[test]
fn every_owned_edit_matches_complete_fields_and_all_ordered_source_replacement_parents() {
    project_packet(EXPECTED, 588, project_case);
}

#[test]
#[ignore = "requires complete native Mesh Property edit/history expectations"]
fn every_native_edit_matches_complete_history_with_optional_export() {
    let native =
        std::fs::read(std::env::var_os("BESSEMER_MESH_PROPERTY_NATIVE_HISTORY_INPUT").unwrap())
            .unwrap();
    let mut output = project_packet(EXPECTED, 588, project_case);
    output.extend(project_packet(&native, 357, project_case));
    if let Some(path) = std::env::var_os("BESSEMER_MESH_PROPERTY_HISTORY_OUTPUT") {
        std::fs::write(path, output).unwrap();
    }
}
