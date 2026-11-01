//! Complete ShellManager header fields and edits against independent reference packets (#156).
use bessemer::simulation::*;
#[path = "simulation_shell_manager/contracts.rs"]
mod contracts;
#[path = "simulation_shell_manager/history.rs"]
mod history;
#[path = "support/binary_load_edit_case.rs"]
mod load_edit_case;
#[path = "support/binary_load_history.rs"]
mod load_history;
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{
    consume_expected_fields, project_field, project_packet, project_word, read_word,
    take_projection,
};

const LIMITS: SimulationShellManagerLimits = SimulationShellManagerLimits {
    maximum_bytes: 8,
    maximum_fields: 2,
    maximum_entries: 0,
    maximum_text_bytes: 0,
};
const PROFILES: [SimulationShellWireProfile; 2] = [
    SimulationShellWireProfile::Reader,
    SimulationShellWireProfile::Writer,
];
const EXPECTED: &[u8] = include_bytes!("fixtures/simulation-shell-manager/header-projection.bin");

fn read_original(cursor: &mut &[u8]) -> SimulationShellManagerProperty {
    let profile = PROFILES[read_word(cursor) as usize];
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    SimulationShellManagerProperty::from_bytes(
        take_projection(cursor, size),
        version,
        profile,
        LIMITS,
    )
    .unwrap()
}

fn assert_constructed(header: &SimulationShellManagerProperty) {
    let rebuilt = SimulationShellManagerProperty::from_fields(
        header.fields().to_vec(),
        header.version(),
        header.wire_profile(),
        LIMITS,
    )
    .unwrap();
    assert_eq!(rebuilt.to_bytes(), header.to_bytes());
    assert_eq!(rebuilt.fields(), header.fields());
    assert_eq!(rebuilt.field_offsets(), header.field_offsets());
    assert_eq!(rebuilt.version(), header.version());
    assert_eq!(rebuilt.wire_profile(), header.wire_profile());
    assert_ne!(rebuilt.field_id(0).unwrap(), header.field_id(0).unwrap());
}

fn assert_coverage(header: &SimulationShellManagerProperty) {
    let coverage = header.coverage();
    assert_eq!(
        coverage.completion,
        SimulationShellManagerCompletion::CompletedBody
    );
    assert_eq!(
        coverage.check_mode,
        SimulationShellManagerCheckMode::ExactStandalone
    );
    assert_eq!(coverage.checked_bytes, header.to_bytes().len());
    assert_eq!(coverage.checked_fields, header.fields().len());
    assert_eq!(coverage.version, header.version());
    assert_eq!(coverage.wire_profile, header.wire_profile());
    assert_eq!(coverage.limits, LIMITS);
    assert_eq!(
        (coverage.body_count, coverage.requested_pattern_error_bits),
        (1, 0)
    );
    assert_eq!(coverage.diagnostics.len(), 4);
}

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let header = read_original(cursor);
    assert_constructed(&header);
    assert_coverage(&header);
    project_word(
        output,
        usize::from(header.wire_profile() == SimulationShellWireProfile::Writer),
    );
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

#[test]
fn every_owned_reader_and_current_writer_profile_matches_all_fields_offsets_and_constructed_bytes()
{
    project_packet(EXPECTED, 60, project_case);
}

#[test]
#[ignore = "requires all original freshly recovered ShellManager direct-header arguments"]
fn all_native_header_arguments_keep_accepted_and_failed_alternatives() {
    let path = std::env::var_os("BESSEMER_SHELL_HEADER_NATIVE_INPUT")
        .expect("set complete native header packet");
    let raw = std::fs::read(path).unwrap();
    let mut cursor = raw.as_slice();
    let mut counts = (0, 0);
    while !cursor.is_empty() {
        let version = SimulationPersistVersion::new(read_word(&mut cursor) as i32).unwrap();
        let accepted = read_word(&mut cursor) != 0;
        let size = read_word(&mut cursor) as usize;
        let bytes = take_projection(&mut cursor, size);
        let result =
            SimulationShellManagerProperty::from_bytes(bytes, version, PROFILES[0], LIMITS);
        assert_eq!(result.is_ok(), accepted, "{version:?}");
        counts.0 += usize::from(accepted);
        counts.1 += usize::from(!accepted);
    }
    assert_eq!(counts, (365, 30));
}

#[test]
#[ignore = "requires complete independently generated native header primitive expectations"]
fn all_native_header_fields_match_complete_independent_projection_with_optional_export() {
    let path = std::env::var_os("BESSEMER_SHELL_HEADER_NATIVE_PROJECTION_INPUT")
        .expect("set complete native header fields");
    let native = std::fs::read(path).unwrap();
    let mut packet = project_packet(EXPECTED, 60, project_case);
    packet.extend(project_packet(&native, 365, project_case));
    if let Some(path) = std::env::var_os("BESSEMER_SHELL_HEADER_PROJECTION_OUTPUT") {
        std::fs::write(path, packet).unwrap();
    }
}

#[test]
fn all_published_header_and_wire_profile_types_are_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<SimulationShellManagerProperty>();
    check::<SimulationShellManagerLimits>();
    check::<SimulationShellManagerEdit>();
    check::<SimulationShellManagerCoverage>();
    check::<SimulationShellManagerCheckMode>();
    check::<SimulationShellManagerCompletion>();
    check::<SimulationShellManagerDiagnostic>();
    check::<SimulationShellManagerField>();
    check::<SimulationShellManagerValue>();
    check::<SimulationShellManagerFieldId>();
    check::<SimulationShellManagerHistory>();
    check::<SimulationShellManagerHistoryParent>();
    check::<SimulationShellManagerRelation>();
    check::<SimulationShellWireProfile>();
}
