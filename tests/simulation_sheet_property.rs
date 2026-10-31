//! Full direct solid-body fields against unchanged actual C-737 readers (#151).
use bessemer::model_archives::{
    ModelArchiveLimits, SimulationEmbeddedArchive, SimulationEmbeddedProfile,
};
use bessemer::simulation::*;
#[path = "simulation_sheet_property/concurrency.rs"]
mod concurrency;
#[path = "simulation_sheet_property/edits.rs"]
mod edits;
#[path = "simulation_sheet_property/history.rs"]
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
#[path = "simulation_sheet_property/malformed.rs"]
mod malformed;
use load_projection::{
    consume_expected_fields, project_field, project_packet, project_word, read_word,
    take_projection,
};

const LIMITS: SimulationSheetLimits = SimulationSheetLimits {
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
const INPUT: &[u8] = include_bytes!("fixtures/simulation-sheet-property/owned.bin");
const EXPECTED: &[u8] = include_bytes!("fixtures/simulation-sheet-property/projection.bin");

fn owned_property(version: i32, profile: usize) -> SimulationSheetProperty {
    let mut cursor = EXPECTED;
    let mut selected = 0;
    while !cursor.is_empty() {
        let body = read_original(&mut cursor);
        consume_expected_fields(&mut cursor);
        if body.version().value() != version {
            continue;
        }
        if selected == profile {
            return body;
        }
        selected += 1;
    }
    panic!(
        "version {version}, profile {profile}; expected a complete owned inherited sheet Property"
    );
}

fn read_original(cursor: &mut &[u8]) -> SimulationSheetProperty {
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    SimulationSheetProperty::from_bytes(take_projection(cursor, size), version, LIMITS).unwrap()
}

fn assert_constructed(body: &SimulationSheetProperty) {
    let rebuilt =
        SimulationSheetProperty::from_fields(body.fields().to_vec(), body.version(), LIMITS)
            .unwrap();
    assert_eq!(rebuilt.to_bytes(), body.to_bytes());
    assert_eq!(rebuilt.fields(), body.fields());
    assert_eq!(rebuilt.field_offsets(), body.field_offsets());
    assert_ne!(rebuilt.field_id(0).unwrap(), body.field_id(0).unwrap());
    let coverage = body.coverage();
    assert_eq!(
        coverage.completion,
        SimulationSheetCompletion::CompletedBody
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

fn assert_source_span(cursor: &mut &[u8], body: Option<&SimulationSheetProperty>) {
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
    let body = SimulationSheetProperty::from_bytes(raw, version, LIMITS);
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
    assert_eq!(check_input(INPUT), (32, 0));
    project_packet(EXPECTED, 32, project_case);
}

#[test]
#[ignore = "requires every freshly recovered original solid-body argument"]
fn all_native_arguments_preserve_every_accepted_and_failed_alternative() {
    let path = std::env::var_os("BESSEMER_SHEET_PROPERTY_NATIVE_INPUT")
        .expect("set complete native argument packet");
    assert_eq!(check_input(&std::fs::read(path).unwrap()), (240, 3));
}

#[test]
#[ignore = "requires full independent native solid-body typed fields"]
fn all_native_fields_match_full_independent_source_with_optional_export() {
    let path = std::env::var_os("BESSEMER_SHEET_PROPERTY_NATIVE_PROJECTION_INPUT")
        .expect("set complete native field expectations");
    let mut packet = project_packet(EXPECTED, 32, project_case);
    packet.extend(project_packet(
        &std::fs::read(path).unwrap(),
        240,
        project_case,
    ));
    if let Some(path) = std::env::var_os("BESSEMER_SHEET_PROPERTY_PROJECTION_OUTPUT") {
        std::fs::write(path, packet).unwrap();
    }
}

#[test]
fn sheet_embedded_cases_preserve_explicit_versions_kernel_stops_and_unrelated_rejections() {
    let mut cursor: &[u8] = include_bytes!("fixtures/simulation-sheet-property/embedded.bin");
    let (mut accepted, mut rejected) = (0, 0);
    while !cursor.is_empty() {
        let expected = read_word(&mut cursor) != 0;
        let size = read_word(&mut cursor) as usize;
        let raw = take_projection(&mut cursor, size);
        let body = read_identifier(raw);
        assert_eq!(body.is_ok(), expected);
        if let Ok(body) = body {
            assert_identifier(&body, raw);
        }
        accepted += usize::from(expected);
        rejected += usize::from(!expected);
    }
    assert_eq!((accepted, rejected), (23, 38));
}

fn read_identifier(raw: &[u8]) -> bessemer::Result<SimulationEmbeddedArchive> {
    let mut cursor = raw;
    let profile = if read_word(&mut cursor) == 4281 {
        SimulationEmbeddedProfile::Legacy4281
    } else {
        SimulationEmbeddedProfile::Modern
    };
    SimulationEmbeddedArchive::from_bytes(raw, profile, LIMITS.embedded)
}

fn assert_identifier(archive: &SimulationEmbeddedArchive, raw: &[u8]) {
    assert_eq!(archive.to_bytes().unwrap(), raw);
    let rebuilt = read_identifier(&archive.to_bytes().unwrap()).unwrap();
    assert_eq!(archive.fields(), rebuilt.fields());
    assert_eq!(archive.object_map_slots(), rebuilt.object_map_slots());
    let name_length = u16::from_le_bytes(raw[8..10].try_into().unwrap()) as usize;
    if &raw[10..10 + name_length] == b"moCompRefSurface_c" {
        let last = archive.fields().last().unwrap();
        assert!(last.path.ends_with(".unknown_100"));
        assert_eq!(
            last.value,
            bessemer::model_archives::ModelFieldValue::I32(-100)
        );
    }
}
