use super::load_projection::{
    consume_expected_fields, project_field, project_packet, project_word,
};
use super::*;

const EXPECTED: &[u8] = include_bytes!("../fixtures/simulation-material/projection.bin");

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let version = SimulationPersistVersion::new(word(cursor) as i32).unwrap();
    let size = word(cursor);
    let raw = take(cursor, size);
    let body = SimulationStudyMaterial::from_bytes(raw, version, LIMITS).unwrap();
    assert_complete(&body, raw);
    project_word(output, version.value() as usize);
    project_word(output, size);
    output.extend_from_slice(&body.to_bytes());
    project_word(output, body.fields().len());
    for (field, span) in body.fields().iter().zip(body.field_offsets()) {
        project_field(output, field, *span);
    }
    consume_expected_fields(cursor);
}

#[test]
fn all_owned_types_paths_raw_values_and_offsets_match_independent_complete_source() {
    assert_eq!(project_packet(EXPECTED, 300, project_case), EXPECTED);
}

#[test]
#[ignore = "Requires explicitly recovered complete native Material field projections"]
fn every_accepted_native_argument_matches_full_source_fields_with_optional_export() {
    let path = std::env::var("BESSEMER_MATERIAL_NATIVE_PROJECTION_INPUT")
        .expect("BESSEMER_MATERIAL_NATIVE_PROJECTION_INPUT=<unset>; expected explicit complete native projection path");
    let native = std::fs::read(&path).unwrap_or_else(|error| {
        panic!("input {path:?}: {error}; expected readable complete native field projections")
    });
    let mut output = project_packet(EXPECTED, 300, project_case);
    output.extend(project_packet(&native, 1230, project_case));
    if let Some(path) = std::env::var_os("BESSEMER_MATERIAL_PROJECTION_OUTPUT") {
        std::fs::write(&path, output).unwrap_or_else(|error| {
            panic!("output {path:?}: {error}; expected writable explicit full field projection")
        });
    }
}
