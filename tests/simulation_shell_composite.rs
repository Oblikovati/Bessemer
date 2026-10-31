//! Complete composite profiles against independent primitive and nested-history oracles (#157).
use bessemer::{model_archives::ModelArchiveLimits, simulation::*};
#[path = "simulation_shell_composite/boundaries.rs"]
mod boundaries;
#[path = "simulation_shell_composite/concurrency.rs"]
mod concurrency;
#[path = "simulation_shell_composite/contracts.rs"]
mod contracts;
#[path = "simulation_shell_composite/history.rs"]
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
    reason = "Shared primitive/history helpers also serve other load value families."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "simulation_shell_composite/malformed.rs"]
mod malformed;
#[path = "simulation_shell_composite/typed_edits.rs"]
mod typed_edits;
use load_projection::{
    consume_expected_fields, project_field, project_packet, project_word, read_word,
    take_projection,
};

const PROFILES: [SimulationShellCompositeProfile; 4] = [
    SimulationShellCompositeProfile::FlipTurnReader,
    SimulationShellCompositeProfile::FlipTurnWriter,
    SimulationShellCompositeProfile::ExtraReader,
    SimulationShellCompositeProfile::ExtraWriter,
];
const LIMITS: SimulationShellCompositeLimits = SimulationShellCompositeLimits {
    maximum_bytes: 8_000_000,
    maximum_fields: 1_000_000,
    maximum_entries: 100_000,
    maximum_text_bytes: 1_000_000,
    embedded: ModelArchiveLimits {
        maximum_bytes: 1_000_000,
        maximum_fields: 100_000,
        maximum_entries: 100_000,
        maximum_depth: 128,
        maximum_objects: 100_000,
    },
};
const EXPECTED: &[u8] = include_bytes!("fixtures/simulation-shell-composite/projection.bin");

fn read_original(cursor: &mut &[u8]) -> SimulationShellComposite {
    let profile = PROFILES[read_word(cursor) as usize];
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    SimulationShellComposite::from_bytes(take_projection(cursor, size), version, profile, LIMITS)
        .unwrap()
}

fn context(profile: SimulationShellCompositeProfile) -> usize {
    PROFILES
        .iter()
        .position(|candidate| *candidate == profile)
        .unwrap()
}

fn assert_constructed(body: &SimulationShellComposite) {
    let constructed = SimulationShellComposite::from_fields(
        body.fields().to_vec(),
        body.version(),
        body.profile(),
        LIMITS,
    )
    .unwrap();
    assert_eq!(constructed.to_bytes(), body.to_bytes());
    assert_eq!(constructed.fields(), body.fields());
    assert_eq!(constructed.field_offsets(), body.field_offsets());
    assert_eq!(constructed.profile(), body.profile());
    assert_eq!(constructed.version(), body.version());
    assert_ne!(constructed.field_id(0).unwrap(), body.field_id(0).unwrap());
}

fn assert_coverage(body: &SimulationShellComposite) {
    let coverage = body.coverage();
    assert_eq!(coverage.version, body.version());
    assert_eq!(coverage.profile, body.profile());
    assert_eq!(coverage.limits.maximum_bytes, LIMITS.maximum_bytes);
    assert_eq!(coverage.limits.maximum_fields, LIMITS.maximum_fields);
    assert_eq!(coverage.limits.maximum_entries, LIMITS.maximum_entries);
    assert_eq!(
        coverage.limits.maximum_text_bytes,
        LIMITS.maximum_text_bytes
    );
    assert_eq!(
        coverage.check_mode,
        SimulationShellCompositeCheckMode::ExactSelectedProfiles
    );
    assert_eq!(coverage.checked_bytes, body.to_bytes().len());
    assert_eq!(coverage.checked_fields, body.fields().len());
    assert_eq!(coverage.body_count, 1);
    assert_eq!(coverage.requested_pattern_error_bits, 0);
    assert_eq!(coverage.diagnostics.len(), 4);
}

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let body = read_original(cursor);
    assert_constructed(&body);
    assert_coverage(&body);
    project_word(output, context(body.profile()));
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

#[test]
fn all_owned_profiles_match_every_independent_field_and_complete_constructed_byte() {
    project_packet(EXPECTED, 270, project_case);
}

#[test]
#[ignore = "requires all original fresh composite named-profile input arguments"]
fn every_native_argument_keeps_both_profiles_and_the_historical_failure() {
    let path = std::env::var_os("BESSEMER_SHELL_COMPOSITE_NATIVE_INPUT")
        .expect("set complete original composite arguments");
    let bytes = std::fs::read(path).unwrap();
    assert_native_arguments(&bytes);
}

fn assert_native_arguments(bytes: &[u8]) {
    let mut cursor = bytes;
    let mut counts = (0, 0);
    while !cursor.is_empty() {
        let kind = read_word(&mut cursor) as usize;
        let direction = read_word(&mut cursor) as usize;
        let version = SimulationPersistVersion::new(read_word(&mut cursor) as i32).unwrap();
        let expected = read_word(&mut cursor) != 0;
        let size = read_word(&mut cursor) as usize;
        let result = SimulationShellComposite::from_bytes(
            take_projection(&mut cursor, size),
            version,
            PROFILES[kind * 2 + direction],
            LIMITS,
        );
        assert_eq!(result.is_ok(), expected, "{kind} {direction} {version:?}");
        counts.0 += usize::from(expected);
        counts.1 += usize::from(!expected);
    }
    assert_eq!(counts, (30, 10));
}

#[test]
#[ignore = "requires complete independent native composite primitive expectations"]
fn every_native_accepted_profile_matches_full_fields_with_optional_export() {
    let path = std::env::var_os("BESSEMER_SHELL_COMPOSITE_NATIVE_PROJECTION_INPUT")
        .expect("set native composite fields");
    let native = std::fs::read(path).unwrap();
    let mut output = project_packet(EXPECTED, 270, project_case);
    output.extend(project_packet(&native, 30, project_case));
    if let Some(path) = std::env::var_os("BESSEMER_SHELL_COMPOSITE_PROJECTION_OUTPUT") {
        std::fs::write(path, output).unwrap();
    }
}
