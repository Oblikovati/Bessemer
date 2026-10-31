use super::edits::{assert_copied_nested, assert_other_fields, assert_outer_history, role_index};
use super::*;
use bessemer::simulation::{SimulationThermalCompletion, SimulationThermalValue};

const OWNED_EDITS: &[u8] =
    include_bytes!("../fixtures/simulation-binary-thermal/thermal-edits.bin");

pub(super) struct EditOracleCase<'a> {
    pub kind: SimulationThermalKind,
    pub version: SimulationPersistVersion,
    pub replacement: i32,
    pub changed: bool,
    pub original: &'a [u8],
    pub expected: &'a [u8],
}

pub(super) fn read_oracle<'a>(cursor: &mut &'a [u8]) -> EditOracleCase<'a> {
    let kind = KINDS[packet_word(cursor) as usize];
    let version = SimulationPersistVersion::new(packet_word(cursor) as i32).unwrap();
    let replacement = packet_word(cursor) as i32;
    let changed = packet_word(cursor) != 0;
    let size = packet_word(cursor) as usize;
    let (original, remaining) = cursor.split_at(size);
    let (expected, remaining) = remaining.split_at(size);
    *cursor = remaining;
    EditOracleCase {
        kind,
        version,
        replacement,
        changed,
        original,
        expected,
    }
}

pub(super) fn check_oracle(
    case: &EditOracleCase<'_>,
) -> bessemer::simulation::SimulationThermalEdit {
    let source =
        SimulationThermalCollection::from_bytes(case.original, case.version, case.kind, LIMITS)
            .unwrap();
    let role = if case.changed {
        "record0.base.integer"
    } else {
        "collection.count"
    };
    let ordinal = role_index(&source, role);
    let id = source.field_id(ordinal).unwrap();
    let edit = source
        .edit_field(&id, SimulationThermalValue::Signed(case.replacement))
        .unwrap();
    assert_eq!(edit.parent(), id);
    assert_oracle_result(&edit, case, ordinal);
    edit
}

fn assert_oracle_result(
    edit: &bessemer::simulation::SimulationThermalEdit,
    case: &EditOracleCase<'_>,
    ordinal: usize,
) {
    assert_eq!(edit.archive().to_bytes(), case.expected);
    assert_eq!(edit.source().to_bytes(), case.original);
    assert_outer_history(edit, ordinal, case.changed);
    assert_copied_nested(edit);
    assert_other_fields(edit, ordinal);
    let completion = if case.changed {
        SimulationThermalCompletion::Changed
    } else {
        SimulationThermalCompletion::Unchanged
    };
    assert_eq!(edit.coverage().completion, completion);
}

fn check_packet(mut cursor: &[u8]) -> usize {
    let mut count = 0;
    while !cursor.is_empty() {
        check_oracle(&read_oracle(&mut cursor));
        count += 1;
    }
    count
}

#[test]
fn complete_owned_edited_images_match_independent_actual_reader_expectations() {
    assert_eq!(check_packet(OWNED_EDITS), 295);
}

#[test]
#[ignore = "Requires explicitly extracted native thermal edit expectations"]
fn complete_native_edited_images_match_independent_actual_reader_expectations() {
    let path = std::env::var("BESSEMER_THERMAL_NATIVE_EDIT_INPUT")
        .expect("BESSEMER_THERMAL_NATIVE_EDIT_INPUT=<unset>; expected explicit native edit oracle packet path");
    let packet = std::fs::read(&path).unwrap_or_else(|error| {
        panic!("input {path:?}: {error}; expected readable native edit oracle packet")
    });
    assert_eq!(check_packet(&packet), 68);
}
