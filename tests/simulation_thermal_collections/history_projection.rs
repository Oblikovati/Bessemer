use super::edit_oracles::{EditOracleCase, check_oracle, read_oracle};
use super::projection::{consume_expected_fields, project_field, project_word};
use super::*;
use bessemer::simulation::{
    SimulationThermalEdit, SimulationThermalHistoryParent, SimulationThermalRelation,
};

const EXPECTED: &[u8] = include_bytes!("../fixtures/simulation-binary-thermal/thermal-history.bin");

fn project_oracle(output: &mut Vec<u8>, case: &EditOracleCase<'_>, edit: &SimulationThermalEdit) {
    project_word(
        output,
        KINDS.iter().position(|kind| *kind == case.kind).unwrap(),
    );
    project_word(output, case.version.value() as usize);
    output.extend_from_slice(&case.replacement.to_le_bytes());
    project_word(output, usize::from(case.changed));
    project_word(output, case.original.len());
    output.extend_from_slice(&edit.source().to_bytes());
    output.extend_from_slice(&edit.archive().to_bytes());
    project_word(output, edit.archive().fields().len());
    for (field, span) in edit
        .archive()
        .fields()
        .iter()
        .zip(edit.archive().field_offsets())
    {
        project_field(output, field, *span);
    }
}

fn relation_tag(relation: SimulationThermalRelation) -> usize {
    match relation {
        SimulationThermalRelation::Copied => 0,
        SimulationThermalRelation::Derived => 1,
        SimulationThermalRelation::Created => 2,
        SimulationThermalRelation::Deleted => 3,
    }
}

fn project_parents(
    output: &mut Vec<u8>,
    edit: &SimulationThermalEdit,
    parents: &[SimulationThermalHistoryParent],
) {
    for parent in parents {
        let (tag, ordinal) = match parent {
            SimulationThermalHistoryParent::Source(id) => {
                assert_eq!(*id, edit.source().field_id(id.ordinal()).unwrap());
                (0, id.ordinal())
            }
            SimulationThermalHistoryParent::Replacement => (1, u32::MAX as usize),
        };
        project_word(output, tag);
        project_word(output, ordinal);
    }
}

fn project_outer_history(output: &mut Vec<u8>, edit: &SimulationThermalEdit) {
    project_word(output, edit.history().len());
    for row in edit.history() {
        assert_eq!(
            row.result,
            edit.archive().field_id(row.result.ordinal()).unwrap()
        );
        project_word(output, row.result.ordinal());
        project_word(output, relation_tag(row.relation));
        project_word(output, row.parents.len());
        project_parents(output, edit, &row.parents);
    }
}

fn consume_expected_history(cursor: &mut &[u8]) {
    for _ in 0..packet_word(cursor) {
        packet_word(cursor);
        packet_word(cursor);
        for _ in 0..packet_word(cursor) {
            packet_word(cursor);
            packet_word(cursor);
        }
    }
}

pub(super) fn project_history_case(
    cursor: &mut &[u8],
    output: &mut Vec<u8>,
) -> SimulationThermalEdit {
    let case = read_oracle(cursor);
    let edit = check_oracle(&case);
    project_oracle(output, &case, &edit);
    project_outer_history(output, &edit);
    consume_expected_fields(cursor);
    consume_expected_history(cursor);
    edit
}

fn project_history_packet(mut cursor: &[u8], expected_count: usize) -> Vec<u8> {
    let expected = cursor;
    let mut output = Vec::new();
    let mut count = 0;
    while !cursor.is_empty() {
        let start = output.len();
        let expected_start = expected.len() - cursor.len();
        project_history_case(&mut cursor, &mut output);
        assert_eq!(
            &output[start..],
            &expected[expected_start..expected.len() - cursor.len()],
            "case {count}"
        );
        count += 1;
    }
    assert_eq!(count, expected_count);
    output
}

#[test]
fn every_owned_edited_primitive_and_outer_history_parent_matches_independent_projection() {
    assert_eq!(project_history_packet(EXPECTED, 295), EXPECTED);
}

#[test]
#[ignore = "Requires explicitly extracted complete native edited-field and history expectations"]
fn complete_native_edited_field_and_history_projection_with_optional_export() {
    let path = std::env::var("BESSEMER_THERMAL_NATIVE_HISTORY_INPUT")
        .expect("BESSEMER_THERMAL_NATIVE_HISTORY_INPUT=<unset>; expected explicit native history packet path");
    let native = std::fs::read(&path).unwrap_or_else(|error| {
        panic!("input {path:?}: {error}; expected readable complete native history expectations")
    });
    let mut output = project_history_packet(EXPECTED, 295);
    output.extend(project_history_packet(&native, 68));
    if let Some(path) = std::env::var_os("BESSEMER_THERMAL_HISTORY_OUTPUT") {
        std::fs::write(&path, output).unwrap_or_else(|error| {
            panic!("output {path:?}: {error}; expected writable explicit history output")
        });
    }
}
