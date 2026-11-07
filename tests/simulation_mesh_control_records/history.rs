use super::load_edit_case::{LoadEditCase, read_edit_case};
use super::load_history::{LoadHistorySnapshot, LoadHistoryView, project_history, project_nested};
use super::*;

const EXPECTED: &[u8] = include_bytes!("../fixtures/simulation-mesh-control/records-history.bin");

fn snapshot(body: &SimulationMeshControlRecords) -> LoadHistorySnapshot<'_> {
    LoadHistorySnapshot::new(
        body.to_bytes(),
        body.fields(),
        body.field_offsets(),
        |index| body.field_id(index).unwrap(),
    )
}

fn edit_target(body: &SimulationMeshControlRecords) -> usize {
    if body.record_spans().is_empty() {
        return 0;
    }
    body.fields()
        .iter()
        .position(|field| field.path.as_ref() == "record0.base.integer")
        .unwrap()
}

fn make_edit(case: &LoadEditCase<'_, ()>) -> SimulationMeshControlEdit {
    let source =
        SimulationMeshControlRecords::from_bytes(case.original, case.version, LIMITS).unwrap();
    let id = source.field_id(edit_target(&source)).unwrap();
    let replacement = SimulationMeshControlValue::Signed(case.replacement);
    let edit = source.edit_field(&id, replacement.clone()).unwrap();
    assert_eq!(edit.archive().to_bytes(), case.expected);
    assert_eq!(edit.source().to_bytes(), case.original);
    assert_eq!(edit.parent(), id);
    assert_eq!(edit.replacement(), &replacement);
    assert_eq!(
        edit.result(),
        edit.archive()
            .field_id(edit_target(edit.archive()))
            .unwrap()
    );
    assert_eq!(edit.result() != id, case.changed);
    assert_edit_scope(&edit, case.changed);
    edit
}

fn assert_edit_scope(edit: &SimulationMeshControlEdit, changed: bool) {
    let completion = if changed {
        SimulationMeshControlCompletion::Changed
    } else {
        SimulationMeshControlCompletion::Unchanged
    };
    assert_eq!(edit.coverage().completion, completion);
    assert_eq!(
        edit.coverage().check_mode,
        SimulationMeshControlCheckMode::ExactSelectedProfilesAndPreservation
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
fn every_owned_record_edit_matches_all_fields_and_ordered_outer_and_nested_source_parents() {
    project_packet(EXPECTED, 322, project_case);
}

#[test]
#[ignore = "requires complete independently generated native MeshControl record edit/history expectations"]
fn every_native_record_edit_matches_complete_source_history_with_optional_export() {
    let path = std::env::var_os("BESSEMER_MESH_RECORDS_NATIVE_HISTORY_INPUT")
        .expect("set native record history input");
    let native = std::fs::read(path).unwrap();
    let mut output = project_packet(EXPECTED, 322, project_case);
    output.extend(project_packet(&native, 12, project_case));
    if let Some(path) = std::env::var_os("BESSEMER_MESH_RECORDS_HISTORY_OUTPUT") {
        std::fs::write(path, output).unwrap();
    }
}
