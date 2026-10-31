use super::load_edit_case::{LoadEditCase, read_edit_case};
use super::load_history::{LoadHistorySnapshot, LoadHistoryView, project_history, project_nested};
use super::*;

const HISTORY: &[u8] = include_bytes!("../fixtures/simulation-solid-body/history.bin");

fn snapshot(body: &SimulationSolidBodyProperty) -> LoadHistorySnapshot<'_> {
    LoadHistorySnapshot::new(
        body.to_bytes(),
        body.fields(),
        body.field_offsets(),
        |ordinal| body.field_id(ordinal).unwrap(),
    )
}

fn make_edit(case: &LoadEditCase<'_, ()>) -> SimulationSolidBodyEdit {
    let source =
        SimulationSolidBodyProperty::from_bytes(case.original, case.version, LIMITS).unwrap();
    let edit = source
        .edit_field(
            &source.field_id(0).unwrap(),
            SimulationSolidBodyValue::Signed(case.replacement),
        )
        .unwrap();
    assert_edit(&edit, case);
    edit
}

fn assert_edit(edit: &SimulationSolidBodyEdit, case: &LoadEditCase<'_, ()>) {
    assert_eq!(case.kind, ());
    assert_eq!(edit.source().to_bytes(), case.original);
    assert_eq!(edit.archive().to_bytes(), case.expected);
    assert_eq!(
        edit.coverage().completion,
        SimulationSolidBodyCompletion::Changed
    );
    assert_eq!(
        edit.coverage().check_mode,
        SimulationSolidBodyCheckMode::ExactSelectedProfilesAndPreservation
    );
    assert_eq!(edit.parent(), edit.source().field_id(0).unwrap());
    assert_eq!(edit.result(), edit.archive().field_id(0).unwrap());
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
fn every_owned_scalar_edit_matches_all_independent_outer_and_nested_parents() {
    project_packet(HISTORY, 240, project_case);
}

#[test]
#[ignore = "requires complete independent native edited-field and nested-history expectations"]
fn every_native_scalar_edit_matches_complete_history_with_optional_export() {
    let path = std::env::var_os("BESSEMER_SOLID_BODY_NATIVE_HISTORY_INPUT")
        .expect("set full native history packet");
    let mut packet = project_packet(HISTORY, 240, project_case);
    packet.extend(project_packet(
        &std::fs::read(path).unwrap(),
        691,
        project_case,
    ));
    if let Some(path) = std::env::var_os("BESSEMER_SOLID_BODY_HISTORY_OUTPUT") {
        std::fs::write(path, packet).unwrap();
    }
}
