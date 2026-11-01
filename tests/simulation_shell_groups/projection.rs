use super::*;

pub(super) fn framed_bytes<'a>(cursor: &mut &'a [u8]) -> &'a [u8] {
    let size = read_word(cursor) as usize;
    take_projection(cursor, size)
}

pub(super) fn consume_fields(cursor: &mut &[u8]) {
    for _ in 0..read_word(cursor) {
        framed_bytes(cursor);
        let tag = read_word(cursor);
        let start = read_word(cursor) as usize;
        let end = read_word(cursor) as usize;
        take_projection(cursor, end - start);
        if tag == 3 {
            read_word(cursor);
            framed_bytes(cursor);
        }
    }
}

fn value_tag(value: &SimulationShellGroupValue) -> usize {
    match value {
        SimulationShellGroupValue::Signed(_) => 0,
        SimulationShellGroupValue::DoubleBits(_) => 2,
        SimulationShellGroupValue::Text(_) => 3,
    }
}

fn project_value(output: &mut Vec<u8>, value: &SimulationShellGroupValue, wire: &[u8]) {
    match value {
        SimulationShellGroupValue::Signed(word) => output.extend_from_slice(&word.to_le_bytes()),
        SimulationShellGroupValue::DoubleBits(bits) => {
            output.extend_from_slice(&bits.to_le_bytes())
        }
        SimulationShellGroupValue::Text(text) => {
            output.extend_from_slice(wire);
            project_word(output, usize::from(text.encoding() == TextEncoding::Utf16));
            project_word(output, text.text().len());
            output.extend_from_slice(text.text().as_bytes());
        }
    }
}

pub(super) fn project_fields(output: &mut Vec<u8>, group: &SimulationShellGroup) {
    let raw = group.to_bytes();
    project_word(output, group.fields().len());
    for (field, &(start, end)) in group.fields().iter().zip(group.field_offsets()) {
        project_word(output, field.path.len());
        output.extend_from_slice(field.path.as_bytes());
        project_word(output, value_tag(&field.value));
        project_word(output, start);
        project_word(output, end);
        project_value(output, &field.value, &raw[start..end]);
    }
}

pub(super) fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let group = read_group(cursor);
    assert_constructed(&group);
    assert_coverage(&group);
    project_word(
        output,
        usize::from(group.wire_profile() == SimulationShellWireProfile::Writer),
    );
    project_word(output, group.version().value() as usize);
    let raw = group.to_bytes();
    project_word(output, raw.len());
    output.extend_from_slice(&raw);
    project_fields(output, &group);
    consume_fields(cursor);
}
