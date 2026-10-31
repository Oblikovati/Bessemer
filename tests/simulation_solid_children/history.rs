use super::load_edit_case::{LoadEditCase, read_edit_case};
use super::load_history::{LoadHistorySnapshot, LoadHistoryView, project_history, project_nested};
use super::*;

const EXPECTED_HISTORY: &[u8] =
    include_bytes!("../fixtures/simulation-solid-children/component-history.bin");

fn snapshot(header: &SimulationSolidChildProperty) -> LoadHistorySnapshot<'_> {
    LoadHistorySnapshot::new(
        header.to_bytes(),
        header.fields(),
        header.field_offsets(),
        |index| header.field_id(index).unwrap(),
    )
}

fn make_edit(case: &LoadEditCase<'_, SimulationSolidChildKind>) -> SimulationSolidChildEdit {
    let source =
        SimulationSolidChildProperty::from_bytes(case.original, case.version, case.kind, LIMITS)
            .unwrap();
    let edit = source
        .edit_field(
            &source
                .field_id(if case.kind == COMPONENT { 1 } else { 0 })
                .unwrap(),
            SimulationSolidChildValue::Signed(case.replacement),
        )
        .unwrap();
    assert_edit(&edit, case);
    edit
}

fn assert_edit(edit: &SimulationSolidChildEdit, case: &LoadEditCase<'_, SimulationSolidChildKind>) {
    assert_eq!(edit.archive().to_bytes(), case.expected);
    assert_eq!(
        edit.coverage().completion,
        SimulationSolidChildCompletion::Changed
    );
    assert_eq!(
        edit.coverage().check_mode,
        SimulationSolidChildCheckMode::ExactBodyAndPreservation
    );
    assert_eq!(edit.source().to_bytes(), case.original);
}

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let case = read_edit_case(cursor, &[COMPONENT, FOLDER]);
    let edit = make_edit(&case);
    let view = LoadHistoryView {
        source: snapshot(edit.source()),
        result: snapshot(edit.archive()),
        outer: edit.history(),
        nested: &[],
    };
    project_history(
        cursor,
        output,
        &case,
        usize::from(case.kind == FOLDER),
        &view,
    );
    project_nested(cursor, output, &view);
}

#[test]
fn every_owned_edit_matches_all_independent_fields_ordered_parents_and_empty_nested_scope() {
    project_packet(EXPECTED_HISTORY, 48, project_case);
    project_packet(
        include_bytes!("../fixtures/simulation-solid-children/folder-history.bin"),
        48,
        project_case,
    );
}

#[test]
#[ignore = "requires full independently generated native SolidChild edited-field/history expectations"]
fn every_native_edit_matches_complete_independent_history_with_optional_export() {
    let path = std::env::var_os("BESSEMER_SOLID_COMPONENT_NATIVE_HISTORY_INPUT")
        .expect("set complete native history expectations");
    let native = std::fs::read(path).unwrap();
    let mut packet = project_packet(EXPECTED_HISTORY, 48, project_case);
    project_packet(
        include_bytes!("../fixtures/simulation-solid-children/folder-history.bin"),
        48,
        project_case,
    );
    packet.extend(project_packet(&native, 699, project_case));
    if let Some(path) = std::env::var_os("BESSEMER_SOLID_CHILD_HISTORY_OUTPUT") {
        std::fs::write(path, packet).unwrap();
    }
}
