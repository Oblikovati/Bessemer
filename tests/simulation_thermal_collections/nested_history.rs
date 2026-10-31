use super::history_projection::project_history_case;
use super::projection::project_word;
use super::*;
use bessemer::model_archives::SimulationEmbeddedArchive;
use bessemer::simulation::{
    SimulationThermalEdit, SimulationThermalNestedParent, SimulationThermalRelation,
    SimulationThermalValue,
};

const EXPECTED: &[u8] =
    include_bytes!("../fixtures/simulation-binary-thermal/thermal-nested-history.bin");

fn embedded(
    collection: &SimulationThermalCollection,
    ordinal: usize,
) -> &SimulationEmbeddedArchive {
    let SimulationThermalValue::Embedded(archive) = &collection.fields()[ordinal].value else {
        panic!("ordinal {ordinal}; expected embedded value");
    };
    archive
}

fn assert_outer_identities(
    edit: &SimulationThermalEdit,
    row: &bessemer::simulation::SimulationThermalNestedHistory,
) {
    assert_eq!(
        row.outer_parent,
        edit.source().field_id(row.outer_parent.ordinal()).unwrap()
    );
    assert_eq!(
        row.outer_result,
        edit.archive().field_id(row.outer_result.ordinal()).unwrap()
    );
}

fn checked_nested_indices(edit: &SimulationThermalEdit, ordinal: usize) -> (usize, usize) {
    let row = &edit.nested_history()[ordinal];
    assert_outer_identities(edit, row);
    let before = embedded(edit.source(), row.outer_parent.ordinal());
    let after = embedded(edit.archive(), row.outer_result.ordinal());
    let [SimulationThermalNestedParent::Source(parent)] = row.parents.as_ref() else {
        panic!("row {ordinal}; expected exactly one source parent");
    };
    let result = row.result.as_ref().unwrap();
    assert_eq!(*parent, before.field_handle(parent.index()).unwrap());
    assert_eq!(*result, after.field_handle(result.index()).unwrap());
    assert_eq!(row.path, before.fields()[parent.index()].path);
    assert_eq!(row.path, after.fields()[result.index()].path);
    (parent.index(), result.index())
}

fn project_nested_row(output: &mut Vec<u8>, edit: &SimulationThermalEdit, ordinal: usize) {
    let row = &edit.nested_history()[ordinal];
    let (parent, result) = checked_nested_indices(edit, ordinal);
    assert_eq!(row.relation, SimulationThermalRelation::Copied);
    for value in [
        row.outer_parent.ordinal(),
        row.outer_result.ordinal(),
        result,
        0,
        1,
        0,
        parent,
    ] {
        project_word(output, value);
    }
}

fn project_nested_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let edit = project_history_case(cursor, output);
    project_word(output, edit.nested_history().len());
    for ordinal in 0..edit.nested_history().len() {
        project_nested_row(output, &edit, ordinal);
    }
    let count = packet_word(cursor) as usize;
    *cursor = &cursor[count * 28..];
}

fn project_nested_packet(mut cursor: &[u8], expected_count: usize) -> Vec<u8> {
    let expected = cursor;
    let mut output = Vec::new();
    let mut count = 0;
    while !cursor.is_empty() {
        let start = output.len();
        let expected_start = expected.len() - cursor.len();
        project_nested_case(&mut cursor, &mut output);
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
fn all_owned_nested_copy_relations_match_every_independent_physical_source_occurrence() {
    assert_eq!(project_nested_packet(EXPECTED, 295), EXPECTED);
}

#[test]
#[ignore = "Requires explicitly extracted complete native nested-history expectations"]
fn all_native_nested_copy_relations_match_source_with_optional_export() {
    let path = std::env::var("BESSEMER_THERMAL_NATIVE_NESTED_HISTORY_INPUT")
        .expect("BESSEMER_THERMAL_NATIVE_NESTED_HISTORY_INPUT=<unset>; expected explicit nested-history packet path");
    let native = std::fs::read(&path).unwrap_or_else(|error| {
        panic!("input {path:?}: {error}; expected readable complete native nested history")
    });
    let mut output = project_nested_packet(EXPECTED, 295);
    output.extend(project_nested_packet(&native, 68));
    if let Some(path) = std::env::var_os("BESSEMER_THERMAL_NESTED_HISTORY_OUTPUT") {
        std::fs::write(&path, output).unwrap_or_else(|error| {
            panic!("output {path:?}: {error}; expected writable explicit nested-history output")
        });
    }
}
