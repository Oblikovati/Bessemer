use super::load_projection::{project_packet, project_word};
use super::*;
use bessemer::model_archives::SimulationEmbeddedArchive;

const EXPECTED: &[u8] = include_bytes!("../fixtures/simulation-beam-property/nested-history.bin");

fn embedded(property: &SimulationBeamProperty, index: usize) -> &SimulationEmbeddedArchive {
    let SimulationBeamValue::Shared(SimulationThermalValue::Embedded(archive)) =
        &property.fields()[index].value
    else {
        panic!("field {index}; expected complete embedded archive");
    };
    archive
}

fn checked_row(edit: &SimulationBeamEdit, row: &SimulationBeamNestedHistory) -> [usize; 7] {
    let outer = row.outer_parent.ordinal();
    let result_outer = row.outer_result.ordinal();
    assert_eq!(row.outer_parent, edit.source().field_id(outer).unwrap());
    assert_eq!(
        row.outer_result,
        edit.archive().field_id(result_outer).unwrap()
    );
    assert_eq!(row.relation, SimulationBeamRelation::Copied);
    let [SimulationBeamNestedParent::Source(parent)] = row.parents.as_ref() else {
        panic!("expected one original source parent");
    };
    let result = row.result.as_ref().unwrap();
    let before = embedded(edit.source(), outer);
    let after = embedded(edit.archive(), result_outer);
    assert_eq!(*parent, before.field_handle(parent.index()).unwrap());
    assert_eq!(*result, after.field_handle(result.index()).unwrap());
    assert_eq!(row.path, before.fields()[parent.index()].path);
    assert_eq!(row.path, after.fields()[result.index()].path);
    [outer, result_outer, result.index(), 0, 1, 0, parent.index()]
}

fn project_nested_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let edit = super::projection::read_project_history(cursor, output);
    project_word(output, edit.nested_history().len());
    for row in edit.nested_history() {
        for value in checked_row(&edit, row) {
            project_word(output, value);
        }
    }
    let count = word(cursor) as usize;
    take(cursor, count * 28);
}

#[test]
fn every_owned_nested_copy_parent_matches_complete_independent_physical_occurrences() {
    project_packet(EXPECTED, 104, project_nested_case);
}

#[test]
#[ignore = "requires explicit complete native nested-history expectations"]
fn every_native_nested_copy_parent_matches_independent_source_with_optional_export() {
    super::projection::export_packet(
        EXPECTED,
        "BESSEMER_BEAM_NATIVE_NESTED_HISTORY_INPUT",
        "BESSEMER_BEAM_NESTED_HISTORY_OUTPUT",
        project_nested_case,
    );
}
