//! Full direct solid-body fields against unchanged actual C-738 readers (#152).
use bessemer::model_archives::ModelArchiveLimits;
use bessemer::simulation::*;
#[path = "simulation_component_persist_id/concurrency.rs"]
mod concurrency;
#[path = "simulation_component_persist_id/edits.rs"]
mod edits;
#[path = "simulation_component_persist_id/history.rs"]
mod history;
#[path = "support/binary_load_edit_case.rs"]
mod load_edit_case;
#[path = "support/binary_load_history.rs"]
mod load_history;
#[path = "support/binary_load_limits.rs"]
mod load_limits;
#[path = "support/binary_load_mutations.rs"]
mod load_mutations;
#[allow(
    dead_code,
    reason = "Shared field/history helpers have dedicated family corpus tests."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "support/binary_load_strings.rs"]
mod load_strings;
#[path = "simulation_component_persist_id/malformed.rs"]
mod malformed;
use load_projection::{
    consume_expected_fields, project_field, project_packet, project_word, read_word,
    take_projection,
};

const LIMITS: SimulationComponentPersistIdLimits = SimulationComponentPersistIdLimits {
    maximum_bytes: 2000000,
    maximum_fields: 250000,
    maximum_entries: 1000000,
    maximum_text_bytes: 1000000,
    embedded: ModelArchiveLimits {
        maximum_bytes: 1000000,
        maximum_fields: 100000,
        maximum_entries: 100000,
        maximum_depth: 128,
        maximum_objects: 100000,
    },
};
const INPUT: &[u8] = include_bytes!("fixtures/simulation-component-persist-id/owned.bin");
const EXPECTED: &[u8] = include_bytes!("fixtures/simulation-component-persist-id/projection.bin");

fn owned_property(version: i32, profile: usize) -> SimulationComponentPersistId {
    let mut cursor = EXPECTED;
    let mut selected = 0;
    while !cursor.is_empty() {
        let archive = read_original(&mut cursor);
        consume_expected_fields(&mut cursor);
        if archive.version().value() != version {
            continue;
        }
        if selected == profile {
            return archive;
        }
        selected += 1;
    }
    panic!("version {version}, profile {profile}; expected complete owned Component PersistID");
}

fn read_original(cursor: &mut &[u8]) -> SimulationComponentPersistId {
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    SimulationComponentPersistId::from_bytes(take_projection(cursor, size), version, LIMITS)
        .unwrap()
}

fn assert_constructed(body: &SimulationComponentPersistId) {
    let rebuilt =
        SimulationComponentPersistId::from_fields(body.fields().to_vec(), body.version(), LIMITS)
            .unwrap();
    assert_eq!(rebuilt.to_bytes(), body.to_bytes());
    assert_eq!(rebuilt.fields(), body.fields());
    assert_eq!(rebuilt.field_offsets(), body.field_offsets());
    assert_ne!(rebuilt.field_id(0).unwrap(), body.field_id(0).unwrap());
    let coverage = body.coverage();
    assert_eq!(
        coverage.completion,
        SimulationComponentPersistIdCompletion::CompletedBody
    );
    assert_eq!(coverage.body_count, 1);
    assert_eq!(coverage.checked_bytes, body.to_bytes().len());
    assert_eq!(coverage.checked_fields, body.fields().len());
    assert_eq!(coverage.requested_pattern_error_bits, 0);
}

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let body = read_original(cursor);
    assert_constructed(&body);
    project_word(output, body.version().value() as usize);
    let raw = body.to_bytes();
    project_word(output, raw.len());
    output.extend_from_slice(&raw);
    project_word(output, body.fields().len());
    for (field, span) in body.fields().iter().zip(body.field_offsets()) {
        project_field(output, field, *span);
    }
    consume_expected_fields(cursor);
}

fn assert_source_span(cursor: &mut &[u8], body: Option<&SimulationComponentPersistId>) {
    let start = read_word(cursor) as usize;
    let end = read_word(cursor) as usize;
    if let Some(body) = body {
        assert!(body.field_offsets().iter().any(|span| span.0 == start));
        assert!(body.field_offsets().iter().any(|span| span.1 == end));
    }
}

fn check_input_case(cursor: &mut &[u8]) -> bool {
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let expected = read_word(cursor) != 0;
    let size = read_word(cursor) as usize;
    let raw = take_projection(cursor, size);
    let body = SimulationComponentPersistId::from_bytes(raw, version, LIMITS);
    assert_eq!(body.is_ok(), expected, "version {version:?}, bytes {size}");
    if let Ok(body) = &body {
        assert_eq!(body.to_bytes(), raw);
    }
    for _ in 0..read_word(cursor) {
        assert_source_span(cursor, body.as_ref().ok());
    }
    expected
}

fn check_input(mut cursor: &[u8]) -> (usize, usize) {
    let (mut accepted, mut rejected) = (0, 0);
    while !cursor.is_empty() {
        let success = check_input_case(&mut cursor);
        accepted += usize::from(success);
        rejected += usize::from(!success);
    }
    (accepted, rejected)
}

#[test]
fn every_owned_gate_profile_matches_complete_actual_source_and_construction() {
    assert_eq!(check_input(INPUT), (20, 0));
    project_packet(EXPECTED, 20, project_case);
}

#[test]
#[ignore = "requires every freshly recovered original solid-body argument"]
fn all_native_arguments_preserve_every_accepted_and_failed_alternative() {
    let path = std::env::var_os("BESSEMER_COMPONENT_PERSIST_ID_NATIVE_INPUT")
        .expect("set complete native argument packet");
    assert_eq!(check_input(&std::fs::read(path).unwrap()), (427, 26));
}

#[test]
#[ignore = "requires full independent native solid-body typed fields"]
fn all_native_fields_match_full_independent_source_with_optional_export() {
    let path = std::env::var_os("BESSEMER_COMPONENT_PERSIST_ID_NATIVE_PROJECTION_INPUT")
        .expect("set complete native field expectations");
    let mut packet = project_packet(EXPECTED, 20, project_case);
    packet.extend(project_packet(
        &std::fs::read(path).unwrap(),
        427,
        project_case,
    ));
    if let Some(path) = std::env::var_os("BESSEMER_COMPONENT_PERSIST_ID_PROJECTION_OUTPUT") {
        std::fs::write(path, packet).unwrap();
    }
}
