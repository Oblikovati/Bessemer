use super::load_edit_case::{LoadEditCase, read_edit_case};
use super::load_history::{LoadHistorySnapshot, LoadHistoryView, project_history, project_nested};
use super::*;

const EXPECTED_HISTORY: &[u8] = include_bytes!("../fixtures/simulation-shell-property/history.bin");

fn snapshot(body: &SimulationShellProperty) -> LoadHistorySnapshot<'_> {
    LoadHistorySnapshot::new(
        body.to_bytes(),
        body.fields(),
        body.field_offsets(),
        |index| body.field_id(index).unwrap(),
    )
}

fn target(body: &SimulationShellProperty) -> usize {
    body.fields()
        .iter()
        .position(|field| field.path.as_ref() == "property.base.integer")
        .unwrap()
}

fn make_edit(
    case: &LoadEditCase<'_, SimulationShellPropertyKind>,
    presence: SimulationShellLayerPresence,
) -> SimulationShellPropertyEdit {
    let context = SimulationShellPropertyContext {
        kind: case.kind,
        layer_presence: presence,
    };
    let source =
        SimulationShellProperty::from_bytes(case.original, case.version, context, LIMITS).unwrap();
    let id = source.field_id(target(&source)).unwrap();
    let replacement = SimulationShellPropertyValue::Signed(case.replacement);
    let edit = source.edit_field(&id, replacement.clone()).unwrap();
    assert_edit_result(case, &source, &edit, id);
    edit
}

fn assert_edit_result(
    case: &LoadEditCase<'_, SimulationShellPropertyKind>,
    source: &SimulationShellProperty,
    edit: &SimulationShellPropertyEdit,
    id: SimulationShellPropertyFieldId,
) {
    assert_eq!(edit.archive().to_bytes(), case.expected);
    assert_eq!(edit.source().to_bytes(), case.original);
    assert_eq!(
        edit.replacement(),
        &SimulationShellPropertyValue::Signed(case.replacement)
    );
    assert_eq!(edit.parent(), id);
    assert_eq!(
        edit.result(),
        edit.archive().field_id(target(edit.archive())).unwrap()
    );
    assert_eq!(edit.result() != id, case.changed);
    assert_eq!(
        edit.coverage().completion,
        SimulationShellPropertyCompletion::Changed
    );
    assert_eq!(edit.coverage().context, *source.context());
}

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let presence = read_presence(cursor);
    let case = read_edit_case(cursor, &KINDS);
    let edit = make_edit(&case, presence);
    project_presence(output, &edit.archive().context().layer_presence);
    let view = LoadHistoryView {
        source: snapshot(edit.source()),
        result: snapshot(edit.archive()),
        outer: edit.history(),
        nested: edit.nested_history(),
    };
    project_history(cursor, output, &case, context_kind(edit.archive()), &view);
    project_nested(cursor, output, &view);
}

#[test]
fn every_owned_edit_matches_all_independent_fields_and_ordered_outer_nested_parents() {
    project_packet(EXPECTED_HISTORY, 2284, project_case);
}

#[test]
#[ignore = "requires complete independent native shell Property edit/history expectations"]
fn all_native_accepted_edits_match_complete_history_with_optional_export() {
    let path = std::env::var_os("BESSEMER_SHELL_PROPERTY_NATIVE_HISTORY_INPUT")
        .expect("set native shell Property histories");
    let native = std::fs::read(path).unwrap();
    let mut output = project_packet(EXPECTED_HISTORY, 2284, project_case);
    output.extend(project_packet(&native, 251, project_case));
    if let Some(path) = std::env::var_os("BESSEMER_SHELL_PROPERTY_HISTORY_OUTPUT") {
        std::fs::write(path, output).unwrap();
    }
}
