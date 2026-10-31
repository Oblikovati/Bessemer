use super::*;

pub(super) fn assert_outer(
    source: &SimulationShellProperty,
    edit: &SimulationShellPropertyEdit,
    changed: usize,
) {
    typed_load_history::assert_outer(
        source.fields().len(),
        |index| source.field_id(index).unwrap(),
        |index| edit.archive().field_id(index).unwrap(),
        edit.history(),
        changed,
    );
}

pub(super) fn assert_nested(
    source: &SimulationShellProperty,
    edit: &SimulationShellPropertyEdit,
    changed: usize,
) {
    typed_load_history::assert_nested(
        source.fields(),
        edit.archive().fields(),
        |index| source.field_id(index).unwrap(),
        |index| edit.archive().field_id(index).unwrap(),
        edit.replacement(),
        edit.nested_history(),
        changed,
    );
}
