use super::load_edit_case::{LoadEditCase, read_edit_case};
use super::load_history::{LoadHistorySnapshot, LoadHistoryView, project_history, project_nested};
use super::*;

const EXPECTED_HISTORY: &[u8] = include_bytes!("../fixtures/simulation-beam-groups/history.bin");

fn history_snapshot(body: &SimulationBeamGroupBody) -> LoadHistorySnapshot<'_> {
    LoadHistorySnapshot::new(
        body.to_bytes(),
        body.fields(),
        body.field_offsets(),
        |index| body.field_id(index).unwrap(),
    )
}

fn make_edit(case: &LoadEditCase<'_, usize>, expected: i32, path: &str) -> SimulationBeamGroupEdit {
    let kind = kind(case.kind as u32, expected);
    let body =
        SimulationBeamGroupBody::from_bytes(case.original, case.version, kind, LIMITS).unwrap();
    let index = body
        .fields()
        .iter()
        .position(|field| field.path.as_ref() == path)
        .unwrap();
    let edit = body
        .edit_field(
            &body.field_id(index).unwrap(),
            SimulationBeamGroupValue::Signed(case.replacement),
        )
        .unwrap();
    assert_edit_case(&edit, case);
    edit
}

fn project_history_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let expected = read_word(cursor) as i32;
    let length = read_word(cursor) as usize;
    let name = take_projection(cursor, length);
    let path = std::str::from_utf8(name).unwrap();
    let case = read_edit_case(cursor, &[0_usize, 1, 2, 3]);
    let edit = make_edit(&case, expected, path);
    output.extend_from_slice(&expected.to_le_bytes());
    project_word(output, name.len());
    output.extend_from_slice(name);
    let view = LoadHistoryView {
        source: history_snapshot(edit.source()),
        result: history_snapshot(edit.archive()),
        outer: edit.history(),
        nested: edit.nested_history(),
    };
    project_history(cursor, output, &case, case.kind, &view);
    project_nested(cursor, output, &view);
}

#[test]
fn all_owned_edits_and_complete_outer_nested_parents_match_independent_source() {
    project_packet(EXPECTED_HISTORY, 560, project_history_case);
}

#[test]
#[ignore = "requires explicit complete native edited-field/outer/nested history expectations"]
fn all_native_edits_and_every_parent_match_independent_source_with_optional_export() {
    let path = std::env::var_os("BESSEMER_BEAM_GROUPS_NATIVE_HISTORY_INPUT")
        .expect("set complete native history expectations");
    let native = std::fs::read(path).unwrap();
    let mut packet = project_packet(EXPECTED_HISTORY, 560, project_history_case);
    packet.extend(project_packet(&native, 414, project_history_case));
    if let Some(path) = std::env::var_os("BESSEMER_BEAM_GROUPS_HISTORY_OUTPUT") {
        std::fs::write(path, packet).unwrap();
    }
}

fn assert_edit_case(edit: &SimulationBeamGroupEdit, case: &LoadEditCase<'_, usize>) {
    assert_eq!(edit.archive().to_bytes(), case.expected);
    assert_eq!(
        edit.coverage().completion,
        if case.changed {
            SimulationBeamGroupCompletion::Changed
        } else {
            SimulationBeamGroupCompletion::Unchanged
        }
    );
    assert_eq!(edit.source().to_bytes(), case.original);
    if !case.changed {
        assert_eq!(edit.parent(), edit.result());
    }
}
