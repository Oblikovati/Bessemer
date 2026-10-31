//! Complete direct ShellManagerGroup fields and typed edits with retained MFC frames (#156).
use bessemer::{
    history::{ArchiveString, TextEncoding},
    simulation::*,
};
#[path = "simulation_shell_groups/budgets.rs"]
mod budgets;
#[path = "simulation_shell_groups/contracts.rs"]
mod contracts;
#[path = "simulation_shell_groups/hardcaps.rs"]
mod hardcaps;
#[path = "simulation_shell_groups/history.rs"]
mod history;
#[allow(
    dead_code,
    reason = "Common primitive IO and history identity helpers also support other value families."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "simulation_shell_groups/malformed.rs"]
mod malformed;
#[path = "simulation_shell_groups/projection.rs"]
mod projection;
use load_projection::{project_packet, project_word, read_word, take_projection};

const LIMITS: SimulationShellGroupLimits = SimulationShellGroupLimits {
    maximum_bytes: 8_000_000,
    maximum_fields: 1_100_001,
    maximum_entries: 100_000,
    maximum_text_bytes: 8_000_000,
};
const PROFILES: [SimulationShellWireProfile; 2] = [
    SimulationShellWireProfile::Reader,
    SimulationShellWireProfile::Writer,
];
const EXPECTED: &[u8] = include_bytes!("fixtures/simulation-shell-manager/group-projection.bin");

fn read_group(cursor: &mut &[u8]) -> SimulationShellGroup {
    let profile = PROFILES[read_word(cursor) as usize];
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    SimulationShellGroup::from_bytes(take_projection(cursor, size), version, profile, LIMITS)
        .unwrap()
}

fn assert_constructed(group: &SimulationShellGroup) {
    let rebuilt = SimulationShellGroup::from_fields(
        group.fields().to_vec(),
        group.version(),
        group.wire_profile(),
        LIMITS,
    )
    .unwrap();
    assert_eq!(rebuilt.to_bytes(), group.to_bytes());
    assert_eq!(rebuilt.fields(), group.fields());
    assert_eq!(rebuilt.field_offsets(), group.field_offsets());
    assert_eq!(rebuilt.version(), group.version());
    assert_eq!(rebuilt.wire_profile(), group.wire_profile());
    assert_ne!(rebuilt.field_id(0).unwrap(), group.field_id(0).unwrap());
}

fn assert_coverage(group: &SimulationShellGroup) {
    let coverage = group.coverage();
    let completion = if coverage.record_count == 0 {
        SimulationShellGroupCompletion::CompletedEmptyRecords
    } else {
        SimulationShellGroupCompletion::CompletedRecords
    };
    assert_eq!(coverage.completion, completion);
    assert_eq!(
        coverage.check_mode,
        SimulationShellGroupCheckMode::ExactSelectedProfiles
    );
    assert_eq!(coverage.checked_bytes, group.to_bytes().len());
    assert_eq!(coverage.checked_fields, group.fields().len());
    assert_eq!(coverage.version, group.version());
    assert_eq!(coverage.wire_profile, group.wire_profile());
    assert_eq!(coverage.limits, LIMITS);
    assert_eq!(coverage.requested_pattern_error_bits, 0);
    assert_eq!(coverage.diagnostics.len(), 4);
}

#[test]
fn every_owned_group_matches_all_independent_fields_and_retained_cstring_frames() {
    project_packet(EXPECTED, 72, projection::project_case);
}

#[test]
#[ignore = "requires every freshly recovered native ShellManagerGroup argument"]
fn all_original_native_group_arguments_preserve_the_complete_empty_boundary() {
    let path = std::env::var_os("BESSEMER_SHELL_GROUP_NATIVE_INPUT")
        .expect("set complete native group packet");
    let bytes = std::fs::read(path).unwrap();
    let mut cursor = bytes.as_slice();
    let mut count = 0;
    while !cursor.is_empty() {
        let version = SimulationPersistVersion::new(read_word(&mut cursor) as i32).unwrap();
        let expected = read_word(&mut cursor) != 0;
        let size = read_word(&mut cursor) as usize;
        let result = SimulationShellGroup::from_bytes(
            take_projection(&mut cursor, size),
            version,
            PROFILES[0],
            LIMITS,
        );
        assert_eq!(result.is_ok(), expected);
        assert_eq!(result.unwrap().coverage().record_count, 0);
        count += 1;
    }
    assert_eq!(count, 79);
}

#[test]
#[ignore = "requires complete independently generated native group primitive expectations"]
fn all_native_group_fields_match_complete_projection_with_optional_export() {
    let path = std::env::var_os("BESSEMER_SHELL_GROUP_NATIVE_PROJECTION_INPUT")
        .expect("set complete native group fields");
    let native = std::fs::read(path).unwrap();
    let mut packet = project_packet(EXPECTED, 72, projection::project_case);
    packet.extend(project_packet(&native, 79, projection::project_case));
    if let Some(path) = std::env::var_os("BESSEMER_SHELL_GROUP_PROJECTION_OUTPUT") {
        std::fs::write(path, packet).unwrap();
    }
}

#[test]
fn all_group_wire_identity_history_and_cstring_types_are_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<SimulationShellGroup>();
    check::<SimulationShellGroupLimits>();
    check::<SimulationShellGroupEdit>();
    check::<SimulationShellGroupCoverage>();
    check::<SimulationShellGroupCheckMode>();
    check::<SimulationShellGroupCompletion>();
    check::<SimulationShellGroupDiagnostic>();
    check::<SimulationShellGroupField>();
    check::<SimulationShellGroupValue>();
    check::<SimulationShellGroupFieldId>();
    check::<SimulationShellGroupHistory>();
    check::<SimulationShellGroupHistoryParent>();
    check::<SimulationShellGroupRelation>();
    check::<SimulationShellWireProfile>();
    check::<ArchiveString>();
    check::<TextEncoding>();
}
