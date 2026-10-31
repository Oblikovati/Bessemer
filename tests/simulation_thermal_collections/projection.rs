use super::load_projection::take_projection;
pub(super) use super::load_projection::{consume_expected_fields, project_field, project_word};
use super::*;

const EXPECTED: &[u8] =
    include_bytes!("../fixtures/simulation-binary-thermal/thermal-projection.bin");

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

fn project_packet(cursor: &[u8], expected_count: usize) -> Vec<u8> {
    super::load_projection::project_packet(cursor, expected_count, project_case)
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
