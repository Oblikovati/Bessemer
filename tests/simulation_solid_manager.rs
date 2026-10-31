//! Complete direct SolidManager Property against full independent source projections (#148).
use bessemer::simulation::*;
#[path = "simulation_solid_manager/concurrency.rs"]
mod concurrency;
#[path = "simulation_solid_manager/edits.rs"]
mod edits;
#[path = "simulation_solid_manager/hardcaps.rs"]
mod hardcaps;
#[path = "simulation_solid_manager/history.rs"]
mod history;
#[path = "support/binary_load_edit_case.rs"]
mod load_edit_case;
#[path = "support/binary_load_history.rs"]
mod load_history;
#[path = "support/binary_load_mutations.rs"]
mod load_mutations;
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "simulation_solid_manager/malformed.rs"]
mod malformed;
use load_projection::{
    consume_expected_fields, project_field, project_packet, project_word, read_word,
    take_projection,
};

const LIMITS: SimulationSolidManagerLimits = SimulationSolidManagerLimits {
    maximum_bytes: 1000000,
    maximum_fields: 250000,
    maximum_entries: 200000,
    maximum_text_bytes: 1000000,
};
const INPUT: &[u8] = include_bytes!("fixtures/simulation-solid-manager/owned.bin");
const EXPECTED: &[u8] = include_bytes!("fixtures/simulation-solid-manager/projection.bin");

fn read_original(cursor: &mut &[u8]) -> SimulationSolidManagerProperty {
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    SimulationSolidManagerProperty::from_bytes(take_projection(cursor, size), version, LIMITS)
        .unwrap()
}

fn assert_constructed(header: &SimulationSolidManagerProperty) {
    let rebuilt = SimulationSolidManagerProperty::from_fields(
        header.fields().to_vec(),
        header.version(),
        LIMITS,
    )
    .unwrap();
    assert_eq!(rebuilt.to_bytes(), header.to_bytes());
    assert_eq!(rebuilt.fields(), header.fields());
    assert_eq!(rebuilt.field_offsets(), header.field_offsets());
    assert_eq!(rebuilt.version(), header.version());
    assert_ne!(rebuilt.field_id(0).unwrap(), header.field_id(0).unwrap());
}

fn assert_coverage(header: &SimulationSolidManagerProperty) {
    let coverage = header.coverage();
    assert_eq!(
        coverage.completion,
        SimulationSolidManagerCompletion::CompletedBody
    );
    assert_eq!(
        coverage.check_mode,
        SimulationSolidManagerCheckMode::ExactStandalone
    );
    assert_eq!(coverage.checked_bytes, header.to_bytes().len());
    assert_eq!(coverage.checked_fields, header.fields().len());
    assert_eq!(coverage.version, header.version());
    assert_eq!(coverage.limits, LIMITS);
    assert_eq!(coverage.body_count, 1);
    assert_eq!(coverage.requested_pattern_error_bits, 0);
    assert_eq!(coverage.diagnostics.len(), 4);
}

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let header = read_original(cursor);
    assert_constructed(&header);
    assert_coverage(&header);
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

fn check_input_case(cursor: &mut &[u8]) -> bool {
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let expected = read_word(cursor) != 0;
    let size = read_word(cursor) as usize;
    let raw = take_projection(cursor, size);
    let result = SimulationSolidManagerProperty::from_bytes(raw, version, LIMITS);
    assert_eq!(result.is_ok(), expected, "{version:?}");
    let spans: Vec<(usize, usize)> = (0..read_word(cursor))
        .map(|_| (read_word(cursor) as usize, read_word(cursor) as usize))
        .collect();
    if let Ok(header) = result {
        assert_eq!(header.to_bytes(), raw);
        assert_eq!(header.field_offsets(), spans);
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
fn every_owned_historical_and_raw_list_profile_matches_complete_source_spans() {
    assert_eq!(check_input(INPUT), (68, 0));
}

#[test]
fn every_owned_field_matches_independent_types_paths_bits_offsets_and_constructed_bytes() {
    project_packet(EXPECTED, 68, project_case);
}

#[test]
#[ignore = "requires all original freshly extracted native SolidManager arguments/spans"]
fn all_native_arguments_retain_all_accepted_and_failed_alternatives() {
    let path = std::env::var_os("BESSEMER_SOLID_MANAGER_NATIVE_INPUT")
        .expect("set complete fresh native input/span packet");
    assert_eq!(check_input(&std::fs::read(path).unwrap()), (365, 30));
}

#[test]
#[ignore = "requires complete independently generated native typed-field expectations"]
fn all_native_fields_match_complete_source_with_optional_export() {
    let path = std::env::var_os("BESSEMER_SOLID_MANAGER_NATIVE_PROJECTION_INPUT")
        .expect("set complete native typed projection packet");
    let native = std::fs::read(path).unwrap();
    let mut packet = project_packet(EXPECTED, 68, project_case);
    packet.extend(project_packet(&native, 365, project_case));
    if let Some(path) = std::env::var_os("BESSEMER_SOLID_MANAGER_PROJECTION_OUTPUT") {
        std::fs::write(path, packet).unwrap();
    }
}

fn owned_header(version: i32, entries: usize) -> SimulationSolidManagerProperty {
    let mut cursor = EXPECTED;
    while !cursor.is_empty() {
        let header = read_original(&mut cursor);
        consume_expected_fields(&mut cursor);
        if header.version().value() == version && header.coverage().list_entries == entries {
            return header;
        }
    }
    panic!("version {version}, entries {entries}; expected complete owned header");
}

#[test]
fn all_published_header_types_are_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<SimulationSolidManagerProperty>();
    check::<SimulationSolidManagerLimits>();
    check::<SimulationSolidManagerEdit>();
    check::<SimulationSolidManagerCoverage>();
    check::<SimulationSolidManagerCheckMode>();
    check::<SimulationSolidManagerCompletion>();
    check::<SimulationSolidManagerDiagnostic>();
    check::<SimulationSolidManagerField>();
    check::<SimulationSolidManagerValue>();
    check::<SimulationSolidManagerFieldId>();
    check::<SimulationSolidManagerHistory>();
    check::<SimulationSolidManagerHistoryParent>();
    check::<SimulationSolidManagerRelation>();
}

#[test]
fn empty_text_lists_remain_one_complete_header_with_zero_entry_budget() {
    let header = owned_header(870008, 0);
    let mut limits = LIMITS;
    limits.maximum_entries = 0;
    let decoded =
        SimulationSolidManagerProperty::from_bytes(&header.to_bytes(), header.version(), limits)
            .unwrap();
    assert_eq!(
        decoded.coverage().completion,
        SimulationSolidManagerCompletion::CompletedBody
    );
    assert_eq!(decoded.coverage().list_entries, 0);
    assert_eq!(decoded.fields().len(), 6);
    assert_eq!(decoded.to_bytes(), header.to_bytes());
}
