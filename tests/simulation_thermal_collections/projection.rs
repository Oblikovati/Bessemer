use super::*;
use bessemer::simulation::{SimulationThermalField, SimulationThermalValue};

const EXPECTED: &[u8] =
    include_bytes!("../fixtures/simulation-binary-thermal/thermal-projection.bin");

pub(super) fn project_word(output: &mut Vec<u8>, value: usize) {
    output.extend_from_slice(&u32::try_from(value).unwrap().to_le_bytes());
}

fn take_projection<'a>(cursor: &mut &'a [u8], size: usize) -> &'a [u8] {
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
    for _ in 0..packet_word(cursor) {
        let length = packet_word(cursor) as usize;
        take_projection(cursor, length);
        packet_word(cursor);
        let start = packet_word(cursor) as usize;
        let end = packet_word(cursor) as usize;
        take_projection(cursor, end - start);
    }
}

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let kind = packet_word(cursor) as usize;
    let version = packet_word(cursor) as usize;
    let size = packet_word(cursor) as usize;
    let raw = take_projection(cursor, size);
    let collection = SimulationThermalCollection::from_bytes(
        raw,
        SimulationPersistVersion::new(version as i32).unwrap(),
        KINDS[kind],
        LIMITS,
    )
    .unwrap();
    for value in [kind, version, size] {
        project_word(output, value);
    }
    output.extend_from_slice(&collection.to_bytes());
    project_word(output, collection.fields().len());
    for (field, span) in collection.fields().iter().zip(collection.field_offsets()) {
        project_field(output, field, *span);
    }
    consume_expected_fields(cursor);
}

fn project_packet(mut cursor: &[u8], expected_count: usize) -> Vec<u8> {
    let expected = cursor;
    let mut output = Vec::new();
    let mut count = 0;
    while !cursor.is_empty() {
        let start = output.len();
        let expected_start = expected.len() - cursor.len();
        project_case(&mut cursor, &mut output);
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

#[test]
fn every_owned_primitive_path_type_pattern_and_offset_matches_independent_source_projection() {
    assert_eq!(project_packet(EXPECTED, 295), EXPECTED);
}

#[test]
#[ignore = "Requires explicitly extracted complete native primitive projections"]
fn complete_native_primitive_projection_and_optional_fresh_process_export() {
    let path = std::env::var("BESSEMER_THERMAL_NATIVE_PROJECTION_INPUT")
        .expect("BESSEMER_THERMAL_NATIVE_PROJECTION_INPUT=<unset>; expected explicit projection packet path");
    let native = std::fs::read(&path).unwrap_or_else(|error| {
        panic!("input {path:?}: {error}; expected readable complete native projections")
    });
    let mut output = project_packet(EXPECTED, 295);
    output.extend(project_packet(&native, 68));
    if let Some(path) = std::env::var_os("BESSEMER_THERMAL_PROJECTION_OUTPUT") {
        std::fs::write(&path, output).unwrap_or_else(|error| {
            panic!("output {path:?}: {error}; expected writable explicit projection output")
        });
    }
}
