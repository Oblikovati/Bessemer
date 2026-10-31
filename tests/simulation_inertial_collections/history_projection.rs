use super::edit_oracles::check_case;
use super::load_edit_case::read_edit_case;
use super::load_history::{LoadHistorySnapshot, LoadHistoryView, project_history};
use super::*;
use bessemer::simulation::SimulationInertialEdit;

const EXPECTED: &[u8] = include_bytes!("../fixtures/simulation-inertial-loads/history.bin");

fn snapshot(collection: &SimulationInertialCollection) -> LoadHistorySnapshot<'_> {
    LoadHistorySnapshot::new(
        collection.to_bytes(),
        collection.fields(),
        collection.field_offsets(),
        |index| collection.field_id(index).unwrap(),
    )
}

pub(super) fn history_view(edit: &SimulationInertialEdit) -> LoadHistoryView<'_> {
    LoadHistoryView {
        source: snapshot(edit.source()),
        result: snapshot(edit.archive()),
        outer: edit.history(),
        nested: edit.nested_history(),
    }
}

pub(super) fn project_history_case(
    cursor: &mut &[u8],
    output: &mut Vec<u8>,
) -> SimulationInertialEdit {
    let case = read_edit_case(cursor, &KINDS);
    let edit = check_case(&case);
    let kind = KINDS.iter().position(|kind| *kind == case.kind).unwrap();
    project_history(cursor, output, &case, kind, &history_view(&edit));
    edit
}

fn project_history_packet(cursor: &[u8], expected_count: usize) -> Vec<u8> {
    super::load_projection::project_packet(cursor, expected_count, |cursor, output| {
        project_history_case(cursor, output);
    })
}

#[test]
fn every_owned_edited_primitive_and_outer_history_parent_matches_independent_projection() {
    assert_eq!(project_history_packet(EXPECTED, 135), EXPECTED);
}

#[test]
#[ignore = "Requires explicitly extracted complete native edited-field and history expectations"]
fn complete_native_edited_field_and_history_projection_with_optional_export() {
    let path = std::env::var("BESSEMER_INERTIAL_NATIVE_HISTORY_INPUT")
        .expect("BESSEMER_INERTIAL_NATIVE_HISTORY_INPUT=<unset>; expected explicit native history packet path");
    let native = std::fs::read(&path).unwrap_or_else(|error| {
        panic!("input {path:?}: {error}; expected readable complete native history expectations")
    });
    let mut output = project_history_packet(EXPECTED, 135);
    output.extend(project_history_packet(&native, 18));
    if let Some(path) = std::env::var_os("BESSEMER_INERTIAL_HISTORY_OUTPUT") {
        std::fs::write(&path, output).unwrap_or_else(|error| {
            panic!("output {path:?}: {error}; expected writable explicit history output")
        });
    }
}
