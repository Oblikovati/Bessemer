use super::load_edit_case::{LoadEditCase, read_edit_case};
use super::load_history::{LoadHistorySnapshot, LoadHistoryView, project_history, project_nested};
use super::*;

const EXPECTED_HISTORY: &[u8] =
    include_bytes!("../fixtures/simulation-shell-composite/history.bin");

fn snapshot(body: &SimulationShellComposite) -> LoadHistorySnapshot<'_> {
    LoadHistorySnapshot::new(
        body.to_bytes(),
        body.fields(),
        body.field_offsets(),
        |index| body.field_id(index).unwrap(),
    )
}

fn target(body: &SimulationShellComposite) -> usize {
    let flip = matches!(
        body.profile(),
        SimulationShellCompositeProfile::FlipTurnReader
            | SimulationShellCompositeProfile::FlipTurnWriter
    );
    usize::from(flip && body.coverage().declared_entries > 0)
}

fn make_edit(
    case: &LoadEditCase<'_, SimulationShellCompositeProfile>,
) -> SimulationShellCompositeEdit {
    let source =
        SimulationShellComposite::from_bytes(case.original, case.version, case.kind, LIMITS)
            .unwrap();
    let id = source.field_id(target(&source)).unwrap();
    let replacement = SimulationShellCompositeValue::Signed(case.replacement);
    let edit = source.edit_field(&id, replacement.clone()).unwrap();
    assert_eq!(edit.archive().to_bytes(), case.expected);
    assert_eq!(edit.source().to_bytes(), case.original);
    assert_eq!(edit.replacement(), &replacement);
    assert_eq!(edit.parent(), id);
    assert_eq!(
        edit.result(),
        edit.archive().field_id(target(edit.archive())).unwrap()
    );
    assert_eq!(edit.result() != id, case.changed);
    assert_edit_completion(case, &edit);
    edit
}

fn assert_edit_completion(
    case: &LoadEditCase<'_, SimulationShellCompositeProfile>,
    edit: &SimulationShellCompositeEdit,
) {
    let completion = if case.changed {
        SimulationShellCompositeCompletion::Changed
    } else {
        SimulationShellCompositeCompletion::Unchanged
    };
    assert_eq!(edit.coverage().completion, completion);
    assert_eq!(edit.coverage().profile, case.kind);
}

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let case = read_edit_case(cursor, &PROFILES);
    let edit = make_edit(&case);
    let view = LoadHistoryView {
        source: snapshot(edit.source()),
        result: snapshot(edit.archive()),
        outer: edit.history(),
        nested: edit.nested_history(),
    };
    project_history(cursor, output, &case, context(case.kind), &view);
    project_nested(cursor, output, &view);
}

#[test]
fn every_owned_edit_matches_complete_independent_fields_and_all_outer_nested_parents() {
    project_packet(EXPECTED_HISTORY, 270, project_case);
}

#[test]
#[ignore = "requires complete independent native composite edit/history expectations"]
fn every_native_accepted_edit_matches_all_history_with_optional_export() {
    let path = std::env::var_os("BESSEMER_SHELL_COMPOSITE_NATIVE_HISTORY_INPUT")
        .expect("set native composite histories");
    let native = std::fs::read(path).unwrap();
    let mut output = project_packet(EXPECTED_HISTORY, 270, project_case);
    output.extend(project_packet(&native, 30, project_case));
    if let Some(path) = std::env::var_os("BESSEMER_SHELL_COMPOSITE_HISTORY_OUTPUT") {
        std::fs::write(path, output).unwrap();
    }
}
