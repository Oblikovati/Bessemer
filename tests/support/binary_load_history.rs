use super::load_edit_case::LoadEditCase;
use super::load_projection::{consume_expected_fields, project_field, project_word, read_word};
use bessemer::model_archives::SimulationEmbeddedArchive;
use bessemer::simulation::{
    SimulationThermalField, SimulationThermalFieldId, SimulationThermalHistory,
    SimulationThermalHistoryParent, SimulationThermalNestedHistory, SimulationThermalNestedParent,
    SimulationThermalRelation, SimulationThermalValue,
};

pub(super) struct LoadHistorySnapshot<'a> {
    bytes: Vec<u8>,
    fields: &'a [SimulationThermalField],
    offsets: &'a [(usize, usize)],
    ids: Vec<SimulationThermalFieldId>,
}

impl<'a> LoadHistorySnapshot<'a> {
    pub(super) fn new(
        bytes: Vec<u8>,
        fields: &'a [SimulationThermalField],
        offsets: &'a [(usize, usize)],
        field_id: impl Fn(usize) -> SimulationThermalFieldId,
    ) -> Self {
        Self {
            bytes,
            fields,
            offsets,
            ids: (0..fields.len()).map(field_id).collect(),
        }
    }
}

pub(super) struct LoadHistoryView<'a> {
    pub source: LoadHistorySnapshot<'a>,
    pub result: LoadHistorySnapshot<'a>,
    pub outer: &'a [SimulationThermalHistory],
    pub nested: &'a [SimulationThermalNestedHistory],
}

fn project_oracle<K>(
    output: &mut Vec<u8>,
    case: &LoadEditCase<'_, K>,
    kind: usize,
    view: &LoadHistoryView<'_>,
) {
    project_word(output, kind);
    project_word(output, case.version.value() as usize);
    output.extend_from_slice(&case.replacement.to_le_bytes());
    project_word(output, usize::from(case.changed));
    project_word(output, case.original.len());
    output.extend_from_slice(&view.source.bytes);
    output.extend_from_slice(&view.result.bytes);
    project_word(output, view.result.fields.len());
    for (field, span) in view.result.fields.iter().zip(view.result.offsets) {
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
    source: &LoadHistorySnapshot<'_>,
    parents: &[SimulationThermalHistoryParent],
) {
    for parent in parents {
        let (tag, ordinal) = match parent {
            SimulationThermalHistoryParent::Source(id) => {
                assert_eq!(*id, source.ids[id.ordinal()]);
                (0, id.ordinal())
            }
            SimulationThermalHistoryParent::Replacement => (1, u32::MAX as usize),
        };
        project_word(output, tag);
        project_word(output, ordinal);
    }
}

fn project_outer(output: &mut Vec<u8>, view: &LoadHistoryView<'_>) {
    project_word(output, view.outer.len());
    for row in view.outer {
        assert_eq!(row.result, view.result.ids[row.result.ordinal()]);
        project_word(output, row.result.ordinal());
        project_word(output, relation_tag(row.relation));
        project_word(output, row.parents.len());
        project_parents(output, &view.source, &row.parents);
    }
}

fn consume_history(cursor: &mut &[u8]) {
    for _ in 0..read_word(cursor) {
        read_word(cursor);
        read_word(cursor);
        for _ in 0..read_word(cursor) {
            read_word(cursor);
            read_word(cursor);
        }
    }
}

pub(super) fn project_history<K>(
    cursor: &mut &[u8],
    output: &mut Vec<u8>,
    case: &LoadEditCase<'_, K>,
    kind: usize,
    view: &LoadHistoryView<'_>,
) {
    project_oracle(output, case, kind, view);
    project_outer(output, view);
    consume_expected_fields(cursor);
    consume_history(cursor);
}

fn embedded<'a>(
    snapshot: &LoadHistorySnapshot<'a>,
    ordinal: usize,
) -> &'a SimulationEmbeddedArchive {
    let SimulationThermalValue::Embedded(archive) = &snapshot.fields[ordinal].value else {
        panic!("ordinal {ordinal}; expected embedded value");
    };
    archive
}

fn checked_nested_indices(view: &LoadHistoryView<'_>, ordinal: usize) -> (usize, usize) {
    let row = &view.nested[ordinal];
    assert_eq!(
        row.outer_parent,
        view.source.ids[row.outer_parent.ordinal()]
    );
    assert_eq!(
        row.outer_result,
        view.result.ids[row.outer_result.ordinal()]
    );
    let before = embedded(&view.source, row.outer_parent.ordinal());
    let after = embedded(&view.result, row.outer_result.ordinal());
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

pub(super) fn project_nested(cursor: &mut &[u8], output: &mut Vec<u8>, view: &LoadHistoryView<'_>) {
    project_word(output, view.nested.len());
    for ordinal in 0..view.nested.len() {
        let row = &view.nested[ordinal];
        let (parent, result) = checked_nested_indices(view, ordinal);
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
    let count = read_word(cursor) as usize;
    *cursor = &cursor[count * 28..];
}
