use super::edits::{assert_isolated, assert_outer, role};
use super::load_edit_case::{LoadEditCase, read_edit_case};
use super::*;
use bessemer::simulation::{SimulationInertialEdit, SimulationInertialRelation};

const EXPECTED: &[u8] = include_bytes!("../fixtures/simulation-inertial-loads/edits.bin");

pub(super) fn check_case(
    case: &LoadEditCase<'_, SimulationInertialKind>,
) -> SimulationInertialEdit {
    let source =
        SimulationInertialCollection::from_bytes(case.original, case.version, case.kind, LIMITS)
            .unwrap();
    let path = if case.changed {
        "record0.base.integer"
    } else {
        "collection.count"
    };
    let ordinal = role(&source, path);
    let id = source.field_id(ordinal).unwrap();
    let edit = source
        .edit_field(&id, SimulationInertialValue::Signed(case.replacement))
        .unwrap();
    assert_eq!(edit.parent(), id);
    assert_case_result(&edit, case, ordinal);
    edit
}

fn assert_copied_nested(edit: &SimulationInertialEdit) {
    assert_eq!(
        edit.nested_history().len(),
        edit.source().coverage().embedded_fields
    );
    assert!(edit.nested_history().iter().all(|row| row.relation == SimulationInertialRelation::Copied && row.parents.len() == 1));
}

fn check_packet(mut cursor: &[u8]) -> usize {
    let mut count = 0;
    while !cursor.is_empty() {
        check_case(&read_edit_case(&mut cursor, &KINDS));
        count += 1;
    }
    count
}

#[test]
fn every_complete_owned_edited_image_matches_independent_actual_reader_expectations() {
    assert_eq!(check_packet(EXPECTED), 135);
}

#[test]
#[ignore = "Requires explicitly extracted complete native inertial edit expectations"]
fn every_complete_native_edited_image_matches_independent_actual_reader_expectations() {
    let path = std::env::var("BESSEMER_INERTIAL_NATIVE_EDIT_INPUT")
        .expect("BESSEMER_INERTIAL_NATIVE_EDIT_INPUT=<unset>; expected explicit native edit oracle packet path");
    let packet = std::fs::read(&path).unwrap_or_else(|error| {
        panic!("input {path:?}: {error}; expected readable complete native edit expectations")
    });
    assert_eq!(check_packet(&packet), 18);
}

fn assert_case_result(
    edit: &SimulationInertialEdit,
    case: &LoadEditCase<'_, SimulationInertialKind>,
    ordinal: usize,
) {
    assert_eq!(edit.archive().to_bytes(), case.expected);
    assert_eq!(edit.source().to_bytes(), case.original);
    assert_outer(edit, ordinal, case.changed);
    assert_isolated(edit, ordinal);
    assert_copied_nested(edit);
    assert_eq!(
        edit.coverage().completion,
        if case.changed {
            SimulationInertialCompletion::Changed
        } else {
            SimulationInertialCompletion::Unchanged
        }
    );
}
