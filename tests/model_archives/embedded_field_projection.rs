use crate::model_archives::{
    ModelArchiveLimits, ModelFieldValue, SimulationEmbeddedArchive, SimulationEmbeddedProfile,
    encode_fields,
};

const EXPECTED: &[u8] = include_bytes!("../fixtures/simulation-binary-thermal/embedded-fields.bin");
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 100000,
    maximum_fields: 50000,
    maximum_entries: 100000,
    maximum_depth: 256,
    maximum_objects: 20000,
};

fn word(cursor: &mut &[u8]) -> usize {
    let (raw, remaining) = cursor.split_at(4);
    *cursor = remaining;
    u32::from_le_bytes(raw.try_into().unwrap()) as usize
}

fn emit_word(output: &mut Vec<u8>, value: usize) {
    output.extend_from_slice(&u32::try_from(value).unwrap().to_le_bytes());
}

fn field_tag(value: &ModelFieldValue) -> usize {
    match value {
        ModelFieldValue::U8(_) => 0,
        ModelFieldValue::U16(_) => 1,
        ModelFieldValue::I16(_) => 2,
        ModelFieldValue::I32(_) => 3,
        ModelFieldValue::U32(_) => 4,
        ModelFieldValue::U64(_) => 5,
        ModelFieldValue::F32(_) => 6,
        ModelFieldValue::F64(_) => 7,
        ModelFieldValue::Text(_) => 8,
        ModelFieldValue::ObjectTag(_) => 9,
        ModelFieldValue::Transform(_) => 10,
        ModelFieldValue::Kernel(_) => 11,
    }
}

fn read_archive(cursor: &mut &[u8]) -> SimulationEmbeddedArchive {
    let length = word(cursor);
    let (raw, remaining) = cursor.split_at(length);
    *cursor = remaining;
    let profile = if raw[..4] == 4281_u32.to_le_bytes() {
        SimulationEmbeddedProfile::Legacy4281
    } else {
        SimulationEmbeddedProfile::Modern
    };
    SimulationEmbeddedArchive::from_bytes(raw, profile, LIMITS).unwrap()
}

fn consume_fields(cursor: &mut &[u8]) {
    for _ in 0..word(cursor) {
        let start = word(cursor);
        let end = word(cursor);
        word(cursor);
        *cursor = &cursor[end - start..];
    }
}

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let archive = read_archive(cursor);
    let raw = archive.to_bytes().unwrap();
    emit_word(output, raw.len());
    output.extend_from_slice(&raw);
    emit_word(output, archive.fields().len());
    let mut offset = 0;
    for field in archive.fields() {
        let encoded = encode_fields(std::slice::from_ref(field), LIMITS.maximum_bytes).unwrap();
        for value in [offset, offset + encoded.len(), field_tag(&field.value)] {
            emit_word(output, value);
        }
        output.extend_from_slice(&encoded);
        offset += encoded.len();
    }
    assert_eq!(offset, raw.len());
    consume_fields(cursor);
}

fn project_packet(mut cursor: &[u8], expected_count: usize) -> Vec<u8> {
    let expected = cursor;
    let mut output = Vec::new();
    let mut count = 0;
    while !cursor.is_empty() {
        let start = output.len();
        let expected_start = expected.len() - cursor.len();
        project_case(&mut cursor, &mut output);
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
fn every_owned_nested_type_offset_and_raw_value_matches_original_source_calls() {
    assert_eq!(project_packet(EXPECTED, 1595), EXPECTED);
}

#[test]
#[ignore = "Requires explicitly extracted complete native nested-field expectations"]
fn every_native_nested_type_offset_and_raw_value_matches_source_with_optional_export() {
    let path = std::env::var("BESSEMER_EMBEDDED_NATIVE_FIELD_INPUT").expect(
        "BESSEMER_EMBEDDED_NATIVE_FIELD_INPUT=<unset>; expected explicit nested-field packet path",
    );
    let native = std::fs::read(&path).unwrap_or_else(|error| {
        panic!("input {path:?}: {error}; expected readable complete native nested fields")
    });
    let mut output = project_packet(EXPECTED, 1595);
    output.extend(project_packet(&native, 242));
    if let Some(path) = std::env::var_os("BESSEMER_EMBEDDED_FIELD_OUTPUT") {
        std::fs::write(&path, output).unwrap_or_else(|error| {
            panic!("output {path:?}: {error}; expected writable explicit nested-field output")
        });
    }
}
