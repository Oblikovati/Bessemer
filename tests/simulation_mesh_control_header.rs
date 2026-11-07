//! Complete MeshControlManager header fields and edits against independent reference packets (#161).
use bessemer::simulation::SimulationPersistVersion;
use bessemer::simulation::{
    SimulationMeshControlHeader as HeaderBody,
    SimulationMeshControlHeaderCheckMode as HeaderCheckMode,
    SimulationMeshControlHeaderCompletion as HeaderCompletion,
    SimulationMeshControlHeaderCoverage as HeaderCoverage,
    SimulationMeshControlHeaderDiagnostic as HeaderDiagnostic,
    SimulationMeshControlHeaderEdit as HeaderEdit, SimulationMeshControlHeaderField as HeaderField,
    SimulationMeshControlHeaderFieldId as HeaderFieldId,
    SimulationMeshControlHeaderHistory as HeaderHistory,
    SimulationMeshControlHeaderHistoryParent as HeaderHistoryParent,
    SimulationMeshControlHeaderLimits as HeaderLimits,
    SimulationMeshControlHeaderProfile as HeaderProfile,
    SimulationMeshControlHeaderRelation as HeaderRelation,
    SimulationMeshControlHeaderValue as HeaderValue,
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

const OWNED_CASES: usize = 80;
const PARALLEL_ROUNDS: usize = OWNED_CASES * 3;
const NATIVE_ACCEPTED: usize = 361;
const NATIVE_REJECTED: usize = 31;
const EXPECTED_HISTORY: &[u8] =
    include_bytes!("fixtures/simulation-mesh-control/header-history.bin");
const NATIVE_INPUT_ENV: &str = "BESSEMER_MESH_CONTROL_HEADER_NATIVE_INPUT";
const NATIVE_PROJECTION_ENV: &str = "BESSEMER_MESH_CONTROL_HEADER_NATIVE_PROJECTION_INPUT";
const PROJECTION_OUTPUT_ENV: &str = "BESSEMER_MESH_CONTROL_HEADER_PROJECTION_OUTPUT";
const NATIVE_HISTORY_ENV: &str = "BESSEMER_MESH_CONTROL_HEADER_NATIVE_HISTORY_INPUT";
const HISTORY_OUTPUT_ENV: &str = "BESSEMER_MESH_CONTROL_HEADER_HISTORY_OUTPUT";

const LIMITS: HeaderLimits = HeaderLimits {
    maximum_bytes: 12,
    maximum_fields: 3,
    maximum_entries: 0,
    maximum_text_bytes: 0,
};
const PROFILES: [HeaderProfile; 2] = [HeaderProfile::Reader, HeaderProfile::Writer];
const EXPECTED: &[u8] = include_bytes!("fixtures/simulation-mesh-control/header-projection.bin");

fn assert_caller_field_isolation(header: &HeaderBody) {
    let mut fields = header.fields().to_vec();
    let constructed = HeaderBody::from_fields(
        fields.clone(),
        header.version(),
        header.wire_profile(),
        LIMITS,
    )
    .unwrap();
    fields[0].path = "changed-caller-path".into();
    fields[0].value = HeaderValue::Signed(91);
    assert!(
        HeaderBody::from_fields(fields, header.version(), header.wire_profile(), LIMITS).is_err()
    );
    assert_eq!(constructed.fields(), header.fields());
    assert_eq!(constructed.field_offsets(), header.field_offsets());
    assert_eq!(constructed.to_bytes(), header.to_bytes());
}

#[test]
fn every_constructed_header_is_isolated_from_caller_field_mutation_and_failure() {
    let mut cursor = EXPECTED;
    while !cursor.is_empty() {
        assert_caller_field_isolation(&read_original(&mut cursor));
        consume_expected_fields(&mut cursor);
    }
}

#[test]
fn early_reader_and_current_writer_never_select_each_other_by_body_extent() {
    for (version, words) in [(0, 1), (800027, 1), (800028, 2), (840012, 2)] {
        let version = SimulationPersistVersion::new(version).unwrap();
        let reader = vec![0; words * 4];
        let writer = [0; 12];
        assert!(HeaderBody::from_bytes(&reader, version, HeaderProfile::Reader, LIMITS).is_ok());
        assert!(HeaderBody::from_bytes(&reader, version, HeaderProfile::Writer, LIMITS).is_err());
        assert!(HeaderBody::from_bytes(&writer, version, HeaderProfile::Writer, LIMITS).is_ok());
        assert!(HeaderBody::from_bytes(&writer, version, HeaderProfile::Reader, LIMITS).is_err());
    }
}

#[test]
fn historical_document_info_candidate_stays_invalid_under_the_study_reader() {
    let version = SimulationPersistVersion::new(890620).unwrap();
    let error =
        HeaderBody::from_bytes(&[0; 4], version, HeaderProfile::Reader, LIMITS).unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
}

#[test]
fn every_owned_reader_and_current_writer_profile_matches_all_fields_offsets_and_constructed_bytes()
{
    project_packet(EXPECTED, OWNED_CASES, project_case);
}

#[test]
#[ignore = "requires all original freshly recovered MeshControlManager direct-header arguments"]
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
