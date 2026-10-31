use super::load_edit_case::{LoadEditCase, read_edit_case};
use super::*;

const HISTORY: &[u8] = include_bytes!("../fixtures/simulation-post-graph/history.bin");

fn make_edit(case: &LoadEditCase<'_, ()>) -> SimulationPostGraphEdit {
    let body =
        SimulationPostGraphParameters::from_bytes(case.original, case.version, LIMITS).unwrap();
    let edit = body
        .edit_field(
            &body.field_id(0).unwrap(),
            SimulationPostGraphValue::Signed(case.replacement),
        )
        .unwrap();
    assert_edit(&edit, case);
    edit
}

fn assert_edit(edit: &SimulationPostGraphEdit, case: &LoadEditCase<'_, ()>) {
    assert_eq!(edit.source().to_bytes(), case.original);
    assert_eq!(edit.archive().to_bytes(), case.expected);
    assert_eq!(
        edit.coverage().completion,
        SimulationPostGraphCompletion::Changed
    );
    assert_eq!(
        edit.coverage().check_mode,
        SimulationPostGraphCheckMode::ExactSelectedProfilesAndPreservation
    );
    assert_eq!(edit.parent(), edit.source().field_id(0).unwrap());
    assert_eq!(edit.result(), edit.archive().field_id(0).unwrap());
    assert_eq!(
        edit.replacement(),
        &SimulationPostGraphValue::Signed(case.replacement)
    );
    assert_constructed(edit.archive());
}

fn project_edit(output: &mut Vec<u8>, case: &LoadEditCase<'_, ()>, edit: &SimulationPostGraphEdit) {
    assert_eq!(case.kind, ());
    project_word(output, 0);
    project_word(output, case.version.value() as usize);
    output.extend_from_slice(&case.replacement.to_le_bytes());
    project_word(output, usize::from(case.changed));
    project_word(output, case.original.len());
    output.extend_from_slice(case.original);
    output.extend(edit.archive().to_bytes());
    project_graph_fields(output, edit.archive());
    let source: Vec<_> = (0..edit.source().fields().len())
        .map(|index| edit.source().field_id(index).unwrap())
        .collect();
    let result: Vec<_> = (0..edit.archive().fields().len())
        .map(|index| edit.archive().field_id(index).unwrap())
        .collect();
    project_outer_history(output, edit.history(), &source, &result);
    assert!(edit.nested_history().is_empty());
    project_word(output, 0);
}

fn consume_history(cursor: &mut &[u8]) {
    consume_expected_fields(cursor);
    for _ in 0..read_word(cursor) {
        read_word(cursor);
        read_word(cursor);
        for _ in 0..read_word(cursor) {
            read_word(cursor);
            read_word(cursor);
        }
    }
    assert_eq!(read_word(cursor), 0);
}

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let case = read_edit_case(cursor, &[()]);
    let edit = make_edit(&case);
    project_edit(output, &case, &edit);
    consume_history(cursor);
}

#[test]
fn every_owned_scalar_edit_matches_complete_fields_and_all_ordered_parents() {
    project_packet(HISTORY, 270, project_case);
}

#[test]
#[ignore = "requires fresh complete native graph edited-field and history expectations"]
fn every_native_scalar_edit_matches_complete_history_with_optional_export() {
    let path = std::env::var_os("BESSEMER_POST_GRAPH_NATIVE_HISTORY_INPUT")
        .expect("set full native graph history packet");
    let mut packet = project_packet(HISTORY, 270, project_case);
    packet.extend(project_packet(
        &std::fs::read(path).unwrap(),
        224,
        project_case,
    ));
    if let Some(path) = std::env::var_os("BESSEMER_POST_GRAPH_HISTORY_OUTPUT") {
        std::fs::write(path, packet).unwrap();
    }
}
