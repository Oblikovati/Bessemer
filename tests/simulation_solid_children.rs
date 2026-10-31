//! Component reader parity and separately scoped static folder vectors (#149).
use bessemer::simulation::*;
#[path = "simulation_solid_children/boundaries.rs"]
mod boundaries;
#[path = "simulation_solid_children/edits.rs"]
mod edits;
#[path = "simulation_solid_children/history.rs"]
mod history;
#[path = "support/binary_load_edit_case.rs"]
mod load_edit_case;
#[path = "support/binary_load_history.rs"]
mod load_history;
#[path = "support/binary_load_mutations.rs"]
mod load_mutations;
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{
    consume_expected_fields, project_field, project_packet, project_word, read_word,
    take_projection,
};

const LIMITS: SimulationSolidChildLimits = SimulationSolidChildLimits {
    maximum_bytes: 1000000,
    maximum_fields: 250000,
    maximum_entries: 200000,
    maximum_text_bytes: 1000000,
};
const COMPONENT: SimulationSolidChildKind = SimulationSolidChildKind::Component;
const FOLDER: SimulationSolidChildKind = SimulationSolidChildKind::Folder;
const INPUT: &[u8] = include_bytes!("fixtures/simulation-solid-children/component.bin");
const EXPECTED: &[u8] =
    include_bytes!("fixtures/simulation-solid-children/component-projection.bin");
const FOLDERS: &[u8] = include_bytes!("fixtures/simulation-solid-children/folder.bin");

fn read_original(
    cursor: &mut &[u8],
    kind: SimulationSolidChildKind,
) -> SimulationSolidChildProperty {
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    SimulationSolidChildProperty::from_bytes(take_projection(cursor, size), version, kind, LIMITS)
        .unwrap()
}

fn assert_constructed(header: &SimulationSolidChildProperty) {
    let rebuilt = SimulationSolidChildProperty::from_fields(
        header.fields().to_vec(),
        header.version(),
        header.kind(),
        LIMITS,
    )
    .unwrap();
    assert_eq!(rebuilt.to_bytes(), header.to_bytes());
    assert_eq!(rebuilt.fields(), header.fields());
    assert_eq!(rebuilt.field_offsets(), header.field_offsets());
    assert_ne!(rebuilt.field_id(0).unwrap(), header.field_id(0).unwrap());
    assert_coverage(header);
}

fn assert_coverage(header: &SimulationSolidChildProperty) {
    let coverage = header.coverage();
    assert_eq!(coverage.kind, header.kind());
    assert_eq!(coverage.behavior_profile, 1);
    assert_eq!(coverage.checked_bytes, header.to_bytes().len());
    assert_eq!(coverage.checked_fields, header.fields().len());
    assert_eq!(
        coverage.completion,
        SimulationSolidChildCompletion::CompletedBody
    );
    assert_eq!(coverage.requested_pattern_error_bits, 0);
    assert_eq!(coverage.diagnostics.len(), 4);
}

fn project_kind(cursor: &mut &[u8], output: &mut Vec<u8>, kind: SimulationSolidChildKind) {
    let header = read_original(cursor, kind);
    assert_constructed(&header);
    project_word(output, header.version().value() as usize);
    let raw = header.to_bytes();
    project_word(output, raw.len());
    output.extend_from_slice(&raw);
    project_word(output, header.fields().len());
    for (field, span) in header.fields().iter().zip(header.field_offsets()) {
        project_field(output, field, *span);
    }
    consume_expected_fields(cursor);
}

fn project_component(cursor: &mut &[u8], output: &mut Vec<u8>) {
    project_kind(cursor, output, COMPONENT);
}

fn project_folder(cursor: &mut &[u8], output: &mut Vec<u8>) {
    project_kind(cursor, output, FOLDER);
}

fn check_input_case(cursor: &mut &[u8]) -> bool {
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let expected = read_word(cursor) != 0;
    let size = read_word(cursor) as usize;
    let raw = take_projection(cursor, size);
    let result = SimulationSolidChildProperty::from_bytes(raw, version, COMPONENT, LIMITS);
    assert_eq!(result.is_ok(), expected, "{version:?}");
    let spans: Vec<(usize, usize)> = (0..read_word(cursor)).map(|_| read_span(cursor)).collect();
    if let Ok(header) = result {
        assert_eq!(header.to_bytes(), raw);
        assert_eq!(header.field_offsets(), spans);
    }
    expected
}

fn read_span(cursor: &mut &[u8]) -> (usize, usize) {
    (read_word(cursor) as usize, read_word(cursor) as usize)
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
fn owned_components_match_all_actual_reader_outcomes_and_full_fields() {
    assert_eq!(check_input(INPUT), (48, 0));
    project_packet(EXPECTED, 48, project_component);
}

#[test]
fn static_folder_vectors_match_all_literal_fields_without_native_claims() {
    project_packet(FOLDERS, 48, project_folder);
    let mut cursor = FOLDERS;
    let header = read_original(&mut cursor, FOLDER);
    assert_eq!(
        header.coverage().evidence,
        SimulationSolidChildEvidence::StaticFolderProfile
    );
    let mut cursor = EXPECTED;
    let header = read_original(&mut cursor, COMPONENT);
    assert_eq!(
        header.coverage().evidence,
        SimulationSolidChildEvidence::BoundComponentReader
    );
}

#[test]
#[ignore = "requires every freshly recovered native component argument"]
fn native_components_preserve_every_accepted_and_failed_alternative() {
    let path = std::env::var_os("BESSEMER_SOLID_COMPONENT_NATIVE_INPUT")
        .expect("set complete native input packet");
    assert_eq!(check_input(&std::fs::read(path).unwrap()), (699, 53));
}

#[test]
#[ignore = "requires independently generated native component fields"]
fn native_component_fields_match_actual_reader_with_optional_export() {
    let path = std::env::var_os("BESSEMER_SOLID_COMPONENT_NATIVE_PROJECTION_INPUT")
        .expect("set native field expectations");
    let mut packet = project_packet(EXPECTED, 48, project_component);
    packet.extend(project_packet(FOLDERS, 48, project_folder));
    packet.extend(project_packet(
        &std::fs::read(path).unwrap(),
        699,
        project_component,
    ));
    if let Some(path) = std::env::var_os("BESSEMER_SOLID_CHILD_PROJECTION_OUTPUT") {
        std::fs::write(path, packet).unwrap();
    }
}
