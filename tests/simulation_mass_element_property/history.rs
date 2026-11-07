use super::edits::{assert_outer_history, assert_preservation, ordinal};
use super::load_edit_case::read_edit_case;
use super::load_history::{LoadHistorySnapshot, LoadHistoryView, project_history, project_nested};
use super::*;

const HISTORY: &[u8] = include_bytes!("../fixtures/simulation-mass-element-property/history.bin");
const NESTED: &[u8] =
    include_bytes!("../fixtures/simulation-mass-element-property/nested-history.bin");

fn snapshot(property: &SimulationMassElementProperty) -> LoadHistorySnapshot<'_> {
    LoadHistorySnapshot::new(
        property.to_bytes(),
        property.fields(),
        property.field_offsets(),
        |index| property.field_id(index).unwrap(),
    )
}

fn history_view(edit: &SimulationMassElementEdit) -> LoadHistoryView<'_> {
    LoadHistoryView {
        source: snapshot(edit.source()),
        result: snapshot(edit.archive()),
        outer: edit.history(),
        nested: edit.nested_history(),
    }
}

fn project_outer_case(cursor: &mut &[u8], output: &mut Vec<u8>) -> SimulationMassElementEdit {
    let case = read_edit_case(cursor, &[0_usize]);
    let source =
        SimulationMassElementProperty::from_bytes(case.original, case.version, LIMITS).unwrap();
    assert!(case.changed, "expected an existing mandatory base integer");
    let index = ordinal(&source, "record0.base.integer");
    let id = source.field_id(index).unwrap();
    let edit = source
        .edit_field(&id, SimulationMassElementValue::Signed(case.replacement))
        .unwrap();
    assert_eq!(edit.parent(), id);
    assert_eq!(edit.source().to_bytes(), case.original);
    assert_eq!(edit.archive().to_bytes(), case.expected);
    assert_eq!(
        edit.coverage().completion,
        SimulationMassElementCompletion::Changed
    );
    assert_outer_history(&edit, index);
    assert_preservation(&edit, index);
    project_history(cursor, output, &case, case.kind, &history_view(&edit));
    edit
}

fn project_history_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    project_outer_case(cursor, output);
}

fn project_nested_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let edit = project_outer_case(cursor, output);
    project_nested(cursor, output, &history_view(&edit));
}

#[test]
fn every_owned_edited_field_and_outer_parent_matches_independent_complete_source() {
    assert_eq!(
        load_projection::project_packet(HISTORY, 196, project_history_case),
        HISTORY
    );
}

#[test]
fn every_owned_nested_copy_parent_matches_independently_observed_physical_occurrences() {
    assert_eq!(
        load_projection::project_packet(NESTED, 196, project_nested_case),
        NESTED
    );
}

fn native_projection(name: &str, owned: &[u8], project: fn(&mut &[u8], &mut Vec<u8>)) -> Vec<u8> {
    let path = std::env::var(name).unwrap_or_else(|error| {
        panic!("{name}=<unset>: {error}; expected explicit complete native history path")
    });
    let native = std::fs::read(&path).unwrap_or_else(|error| {
        panic!("input {path:?}: {error}; expected readable complete native history packet")
    });
    let mut output = load_projection::project_packet(owned, 196, project);
    output.extend(load_projection::project_packet(&native, 2, project));
    output
}

fn export_if_requested(name: &str, output: Vec<u8>) {
    if let Some(path) = std::env::var_os(name) {
        std::fs::write(&path, output).unwrap_or_else(|error| {
            panic!("output {path:?}: {error}; expected writable explicit full history export")
        });
    }
}

#[test]
#[ignore = "Requires explicitly extracted complete native mass Property edited-field/history expectations"]
fn all_native_edited_fields_and_outer_parents_match_source_with_optional_export() {
    let output = native_projection(
        "BESSEMER_MASS_ELEMENT_NATIVE_HISTORY_INPUT",
        HISTORY,
        project_history_case,
    );
    export_if_requested("BESSEMER_MASS_ELEMENT_HISTORY_OUTPUT", output);
}

#[test]
#[ignore = "Requires explicitly extracted complete native mass Property nested-history expectations"]
fn all_native_nested_copy_parents_match_source_with_optional_export() {
    let output = native_projection(
        "BESSEMER_MASS_ELEMENT_NATIVE_NESTED_HISTORY_INPUT",
        NESTED,
        project_nested_case,
    );
    export_if_requested("BESSEMER_MASS_ELEMENT_NESTED_HISTORY_OUTPUT", output);
}
