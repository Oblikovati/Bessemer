use bessemer::simulation::{
    SimulationThermalField, SimulationThermalFieldId, SimulationThermalHistory,
    SimulationThermalHistoryParent, SimulationThermalRelation, SimulationThermalValue,
};

pub(super) fn read_word(cursor: &mut &[u8]) -> u32 {
    let (raw, remaining) = cursor.split_at(4);
    *cursor = remaining;
    u32::from_le_bytes(raw.try_into().unwrap())
}

pub(super) fn project_word(output: &mut Vec<u8>, value: usize) {
    output.extend_from_slice(&u32::try_from(value).unwrap().to_le_bytes());
}

pub(super) fn take_projection<'a>(cursor: &mut &'a [u8], size: usize) -> &'a [u8] {
    let (selected, remaining) = cursor.split_at(size);
    *cursor = remaining;
    selected
}

fn encoded_primitive(value: &SimulationThermalValue) -> (usize, Vec<u8>) {
    match value {
        SimulationThermalValue::Signed(value) => (0, value.to_le_bytes().to_vec()),
        SimulationThermalValue::Unsigned(value) => (1, value.to_le_bytes().to_vec()),
        SimulationThermalValue::DoubleBits(value) => (2, value.to_le_bytes().to_vec()),
        SimulationThermalValue::Text(text) => {
            let mut bytes = text.header().to_vec();
            bytes.extend_from_slice(text.payload());
            (3, bytes)
        }
        SimulationThermalValue::Bytes(bytes) => (4, bytes.to_vec()),
        SimulationThermalValue::Embedded(archive) => (5, archive.to_bytes().unwrap()),
    }
}

pub(super) fn project_field(
    output: &mut Vec<u8>,
    field: &SimulationThermalField,
    span: (usize, usize),
) {
    let (tag, value) = encoded_primitive(&field.value);
    project_word(output, field.path.len());
    output.extend_from_slice(field.path.as_bytes());
    project_word(output, tag);
    project_word(output, span.0);
    project_word(output, span.1);
    assert_eq!(span.1 - span.0, value.len());
    output.extend_from_slice(&value);
}

pub(super) fn consume_expected_fields(cursor: &mut &[u8]) {
    for _ in 0..read_word(cursor) {
        let length = read_word(cursor) as usize;
        take_projection(cursor, length);
        read_word(cursor);
        let start = read_word(cursor) as usize;
        let end = read_word(cursor) as usize;
        take_projection(cursor, end - start);
    }
}

pub(super) fn project_packet(
    mut cursor: &[u8],
    expected_count: usize,
    project: fn(&mut &[u8], &mut Vec<u8>),
) -> Vec<u8> {
    let expected = cursor;
    let mut output = Vec::new();
    let mut count = 0;
    while !cursor.is_empty() {
        let start = output.len();
        let expected_start = expected.len() - cursor.len();
        project(&mut cursor, &mut output);
        let expected_end = expected.len() - cursor.len();
        assert_eq!(
            &output[start..],
            &expected[expected_start..expected_end],
            "case {count}"
        );
        count += 1;
    }
    assert_eq!(count, expected_count);
    output
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
    source_ids: &[SimulationThermalFieldId],
    parents: &[SimulationThermalHistoryParent],
) {
    for parent in parents {
        let (tag, ordinal) = match parent {
            SimulationThermalHistoryParent::Source(id) => {
                assert_eq!(*id, source_ids[id.ordinal()]);
                (0, id.ordinal())
            }
            SimulationThermalHistoryParent::Replacement => (1, u32::MAX as usize),
        };
        project_word(output, tag);
        project_word(output, ordinal);
    }
}

pub(super) fn project_outer_history(
    output: &mut Vec<u8>,
    rows: &[SimulationThermalHistory],
    source_ids: &[SimulationThermalFieldId],
    result_ids: &[SimulationThermalFieldId],
) {
    project_word(output, rows.len());
    for row in rows {
        assert_eq!(row.result, result_ids[row.result.ordinal()]);
        project_word(output, row.result.ordinal());
        project_word(output, relation_tag(row.relation));
        project_word(output, row.parents.len());
        project_parents(output, source_ids, &row.parents);
    }
}
