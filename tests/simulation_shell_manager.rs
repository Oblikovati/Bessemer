//! Complete ShellManager header fields and edits against independent reference packets (#156).
use bessemer::simulation::SimulationPersistVersion;
use bessemer::simulation::{
    SimulationShellManagerCheckMode as HeaderCheckMode,
    SimulationShellManagerCompletion as HeaderCompletion,
    SimulationShellManagerCoverage as HeaderCoverage,
    SimulationShellManagerDiagnostic as HeaderDiagnostic, SimulationShellManagerEdit as HeaderEdit,
    SimulationShellManagerField as HeaderField, SimulationShellManagerFieldId as HeaderFieldId,
    SimulationShellManagerHistory as HeaderHistory,
    SimulationShellManagerHistoryParent as HeaderHistoryParent,
    SimulationShellManagerLimits as HeaderLimits, SimulationShellManagerProperty as HeaderBody,
    SimulationShellManagerRelation as HeaderRelation, SimulationShellManagerValue as HeaderValue,
    SimulationShellWireProfile as HeaderProfile,
};

#[path = "support/signed_header_contracts.rs"]
mod contracts;
#[path = "support/signed_header_history.rs"]
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

#[path = "support/signed_header_projection.rs"]
mod header_projection;
use header_projection::{project_case, read_original};

const OWNED_CASES: usize = 60;
const PARALLEL_ROUNDS: usize = 32;
const NATIVE_ACCEPTED: usize = 365;
const NATIVE_REJECTED: usize = 30;
const EXPECTED_HISTORY: &[u8] =
    include_bytes!("fixtures/simulation-shell-manager/header-history.bin");
const NATIVE_INPUT_ENV: &str = "BESSEMER_SHELL_HEADER_NATIVE_INPUT";
const NATIVE_PROJECTION_ENV: &str = "BESSEMER_SHELL_HEADER_NATIVE_PROJECTION_INPUT";
const PROJECTION_OUTPUT_ENV: &str = "BESSEMER_SHELL_HEADER_PROJECTION_OUTPUT";
const NATIVE_HISTORY_ENV: &str = "BESSEMER_SHELL_HEADER_NATIVE_HISTORY_INPUT";
const HISTORY_OUTPUT_ENV: &str = "BESSEMER_SHELL_HEADER_HISTORY_OUTPUT";

const LIMITS: HeaderLimits = HeaderLimits {
    maximum_bytes: 8,
    maximum_fields: 2,
    maximum_entries: 0,
    maximum_text_bytes: 0,
};
const PROFILES: [HeaderProfile; 2] = [HeaderProfile::Reader, HeaderProfile::Writer];
const EXPECTED: &[u8] = include_bytes!("fixtures/simulation-shell-manager/header-projection.bin");

#[test]
fn every_owned_reader_and_current_writer_profile_matches_all_fields_offsets_and_constructed_bytes()
{
    project_packet(EXPECTED, OWNED_CASES, project_case);
}

#[test]
#[ignore = "requires all original freshly recovered ShellManager direct-header arguments"]
fn all_native_header_arguments_keep_accepted_and_failed_alternatives() {
    let path = std::env::var_os(NATIVE_INPUT_ENV).expect("set complete native header packet");
    let raw = std::fs::read(path).unwrap();
    let mut cursor = raw.as_slice();
    let mut counts = (0, 0);
    while !cursor.is_empty() {
        let version = SimulationPersistVersion::new(read_word(&mut cursor) as i32).unwrap();
        let accepted = read_word(&mut cursor) != 0;
        let size = read_word(&mut cursor) as usize;
        let bytes = take_projection(&mut cursor, size);
        let result = HeaderBody::from_bytes(bytes, version, PROFILES[0], LIMITS);
        assert_eq!(result.is_ok(), accepted, "{version:?}");
        counts.0 += usize::from(accepted);
        counts.1 += usize::from(!accepted);
    }
    assert_eq!(counts, (NATIVE_ACCEPTED, NATIVE_REJECTED));
}

#[test]
#[ignore = "requires complete independently generated native header primitive expectations"]
fn all_native_header_fields_match_complete_independent_projection_with_optional_export() {
    let path = std::env::var_os(NATIVE_PROJECTION_ENV).expect("set complete native header fields");
    let native = std::fs::read(path).unwrap();
    let mut packet = project_packet(EXPECTED, OWNED_CASES, project_case);
    packet.extend(project_packet(&native, NATIVE_ACCEPTED, project_case));
    if let Some(path) = std::env::var_os(PROJECTION_OUTPUT_ENV) {
        std::fs::write(path, packet).unwrap();
    }
}

#[test]
fn all_published_header_and_wire_profile_types_are_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<HeaderBody>();
    check::<HeaderLimits>();
    check::<HeaderEdit>();
    check::<HeaderCoverage>();
    check::<HeaderCheckMode>();
    check::<HeaderCompletion>();
    check::<HeaderDiagnostic>();
    check::<HeaderField>();
    check::<HeaderValue>();
    check::<HeaderFieldId>();
    check::<HeaderHistory>();
    check::<HeaderHistoryParent>();
    check::<HeaderRelation>();
    check::<HeaderProfile>();
}
