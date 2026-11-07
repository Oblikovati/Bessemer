use super::projection::project_word;
use super::*;
use bessemer::model_archives::{
    ModelFieldValue, ModelObjectRecord, ModelObjectTag, SimulationEmbeddedArchive,
    SimulationEmbeddedProfile,
};

const EXPECTED: &[u8] = include_bytes!("../fixtures/simulation-binary-thermal/embedded-maps.bin");

fn project_tag(output: &mut Vec<u8>, tag: &ModelObjectTag) {
    let (meaning, index, schema, name) = match &tag.record {
        ModelObjectRecord::Null => (0, 0, 0, ""),
        ModelObjectRecord::New { index, class } => {
            (1, index.value(), class.schema, class.name.as_str())
        }
        ModelObjectRecord::Reference { index, class } => {
            (2, index.value(), class.schema, class.name.as_str())
        }
    };
    for value in [
        tag.framing.tag as usize,
        tag.framing.extended.unwrap_or(u32::MAX) as usize,
        meaning,
        index as usize,
        schema as usize,
        name.len(),
    ] {
        project_word(output, value);
    }
    output.extend_from_slice(name.as_bytes());
}

fn read_map_archive(cursor: &mut &[u8]) -> SimulationEmbeddedArchive {
    let size = packet_word(cursor) as usize;
    let (raw, remaining) = cursor.split_at(size);
    *cursor = remaining;
    let profile = if raw[..4] == 4281_u32.to_le_bytes() {
        SimulationEmbeddedProfile::Legacy4281
    } else {
        SimulationEmbeddedProfile::Modern
    };
    SimulationEmbeddedArchive::from_bytes(raw, profile, LIMITS.embedded).unwrap()
}

fn consume_map_expectations(cursor: &mut &[u8]) {
    packet_word(cursor);
    for _ in 0..packet_word(cursor) {
        for _ in 0..5 {
            packet_word(cursor);
        }
        let length = packet_word(cursor) as usize;
        *cursor = &cursor[length..];
    }
}

fn project_map_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let archive = read_map_archive(cursor);
    let raw = archive.to_bytes().unwrap();
    project_word(output, raw.len());
    output.extend_from_slice(&raw);
    project_word(output, archive.object_map_slots());
    let tags: Vec<_> = archive
        .fields()
        .iter()
        .filter_map(|field| match &field.value {
            ModelFieldValue::ObjectTag(tag) => Some(tag.as_ref()),
            _ => None,
        })
        .collect();
    project_word(output, tags.len());
    for tag in tags {
        project_tag(output, tag);
    }
    consume_map_expectations(cursor);
}

fn project_map_packet(mut cursor: &[u8], expected_count: usize) -> Vec<u8> {
    let expected = cursor;
    let mut output = Vec::new();
    let mut count = 0;
    while !cursor.is_empty() {
        let start = output.len();
        let expected_start = expected.len() - cursor.len();
        project_map_case(&mut cursor, &mut output);
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
fn every_owned_pointer_schema_map_identity_and_alias_matches_unmodified_source_calls() {
    assert_eq!(project_map_packet(EXPECTED, 1595), EXPECTED);
}

#[test]
#[ignore = "Requires explicitly extracted complete native object-map expectations"]
fn every_native_pointer_and_map_identity_matches_source_with_optional_export() {
    let path = std::env::var("BESSEMER_EMBEDDED_NATIVE_MAP_INPUT")
        .expect("BESSEMER_EMBEDDED_NATIVE_MAP_INPUT=<unset>; expected explicit native object-map packet path");
    let native = std::fs::read(&path).unwrap_or_else(|error| {
        panic!("input {path:?}: {error}; expected readable complete native map expectations")
    });
    let mut output = project_map_packet(EXPECTED, 1595);
    output.extend(project_map_packet(&native, 242));
    if let Some(path) = std::env::var_os("BESSEMER_EMBEDDED_MAP_OUTPUT") {
        std::fs::write(&path, output).unwrap_or_else(|error| {
            panic!("output {path:?}: {error}; expected writable explicit map output")
        });
    }
}
