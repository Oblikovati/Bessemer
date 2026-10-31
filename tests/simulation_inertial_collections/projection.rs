use super::load_projection::{
    consume_expected_fields, project_field, project_word, take_projection,
};
use super::*;

const EXPECTED: &[u8] = include_bytes!("../fixtures/simulation-inertial-loads/projection.bin");

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let kind = word(cursor);
    let version = word(cursor);
    let size = word(cursor);
    let raw = take_projection(cursor, size);
    let collection = SimulationInertialCollection::from_bytes(
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
fn every_owned_field_path_type_offset_and_raw_value_matches_independent_source() {
    assert_eq!(project_packet(EXPECTED, 135), EXPECTED);
}

#[test]
#[ignore = "Requires explicitly extracted complete native inertial/excitation field projections"]
fn every_native_field_matches_independent_source_with_optional_full_export() {
    let path = std::env::var("BESSEMER_INERTIAL_NATIVE_PROJECTION_INPUT")
        .expect("BESSEMER_INERTIAL_NATIVE_PROJECTION_INPUT=<unset>; expected explicit native projection packet path");
    let native = std::fs::read(&path).unwrap_or_else(|error| {
        panic!("input {path:?}: {error}; expected readable complete native projections")
    });
    let mut output = project_packet(EXPECTED, 135);
    output.extend(project_packet(&native, 18));
    if let Some(path) = std::env::var_os("BESSEMER_INERTIAL_PROJECTION_OUTPUT") {
        std::fs::write(&path, output).unwrap_or_else(|error| {
            panic!("output {path:?}: {error}; expected writable explicit full projection output")
        });
    }
}
