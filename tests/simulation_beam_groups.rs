//! Complete separate C-726 bodies against independent physical field and history oracles (#144).
use bessemer::model_archives::ModelArchiveLimits;
use bessemer::simulation::*;
#[path = "simulation_beam_groups/concurrency.rs"]
mod concurrency;
#[path = "simulation_beam_groups/edits.rs"]
mod edits;
#[path = "simulation_beam_groups/history.rs"]
mod history;
#[path = "support/binary_load_edit_case.rs"]
mod load_edit_case;
#[path = "support/binary_load_history.rs"]
mod load_history;
#[path = "support/binary_load_limits.rs"]
mod load_limits;
#[path = "support/binary_load_mutations.rs"]
mod load_mutations;
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "support/binary_load_strings.rs"]
mod load_strings;
#[path = "simulation_beam_groups/malformed.rs"]
mod malformed;
use load_projection::{
    consume_expected_fields, project_field, project_packet, project_word, read_word,
    take_projection,
};

const LIMITS: SimulationBeamGroupLimits = SimulationBeamGroupLimits {
    maximum_bytes: 1000000,
    maximum_fields: 100000,
    maximum_entries: 2000000,
    maximum_text_bytes: 1000000,
    embedded: ModelArchiveLimits {
        maximum_bytes: 100000,
        maximum_fields: 50000,
        maximum_entries: 100000,
        maximum_depth: 256,
        maximum_objects: 20000,
    },
};
const EXPECTED: &[u8] = include_bytes!("fixtures/simulation-beam-groups/projection.bin");

fn kind(index: u32, expected: i32) -> SimulationBeamGroupKind {
    assert!(expected >= -1);
    let expected_joint_count = usize::try_from(expected).ok();
    match index {
        0 => {
            assert_eq!(expected_joint_count, None);
            SimulationBeamGroupKind::JointGroup
        }
        1 => {
            assert_eq!(expected_joint_count, None);
            SimulationBeamGroupKind::Joint
        }
        2 => SimulationBeamGroupKind::JointList {
            expected_joint_count,
        },
        3 => {
            assert_eq!(expected_joint_count, None);
            SimulationBeamGroupKind::BeamFolder
        }
        _ => panic!("kind {index}; expected 0..=3"),
    }
}

fn kind_index(kind: SimulationBeamGroupKind) -> usize {
    match kind {
        SimulationBeamGroupKind::JointGroup => 0,
        SimulationBeamGroupKind::Joint => 1,
        SimulationBeamGroupKind::JointList { .. } => 2,
        SimulationBeamGroupKind::BeamFolder => 3,
    }
}

fn expected_count(kind: SimulationBeamGroupKind) -> Option<usize> {
    match kind {
        SimulationBeamGroupKind::JointList {
            expected_joint_count,
        } => expected_joint_count,
        _ => None,
    }
}

fn read_body(cursor: &mut &[u8]) -> SimulationBeamGroupBody {
    let index = read_word(cursor);
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let kind = kind(index, read_word(cursor) as i32);
    let size = read_word(cursor) as usize;
    let raw = take_projection(cursor, size);
    let body = SimulationBeamGroupBody::from_bytes(raw, version, kind, LIMITS).unwrap();
    assert_eq!(body.to_bytes(), raw);
    body
}

fn assert_construction(body: &SimulationBeamGroupBody) {
    let constructed = SimulationBeamGroupBody::from_fields(
        body.fields().to_vec(),
        body.version(),
        body.kind(),
        LIMITS,
    )
    .unwrap();
    assert_eq!(constructed.to_bytes(), body.to_bytes());
    assert_eq!(constructed.fields(), body.fields());
    assert_eq!(constructed.field_offsets(), body.field_offsets());
    assert_eq!(constructed.kind(), body.kind());
    assert_ne!(constructed.field_id(0).unwrap(), body.field_id(0).unwrap());
}

fn assert_coverage(body: &SimulationBeamGroupBody) {
    let coverage = body.coverage();
    assert_eq!(coverage.kind, body.kind());
    assert_eq!(
        coverage.completion,
        SimulationBeamGroupCompletion::CompletedBody
    );
    assert_eq!(
        coverage.check_mode,
        SimulationBeamGroupCheckMode::ExactStoredProfile {
            expected_joint_count: expected_count(body.kind())
        }
    );
    assert_eq!(coverage.checked_bytes, body.to_bytes().len());
    assert_eq!(coverage.checked_fields, body.fields().len());
    assert_eq!(coverage.body_count, 1);
    assert_eq!(coverage.requested_pattern_error_bits, 0);
    assert_eq!(coverage.diagnostics.len(), 4);
}

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let body = read_body(cursor);
    assert_construction(&body);
    assert_coverage(&body);
    project_word(output, kind_index(body.kind()));
    project_word(output, body.version().value() as usize);
    output.extend_from_slice(
        &expected_count(body.kind())
            .map(|count| count as i32)
            .unwrap_or(-1)
            .to_le_bytes(),
    );
    let bytes = body.to_bytes();
    project_word(output, bytes.len());
    output.extend_from_slice(&bytes);
    project_word(output, body.fields().len());
    for (field, span) in body.fields().iter().zip(body.field_offsets()) {
        project_field(output, field, *span);
    }
    consume_expected_fields(cursor);
}

#[test]
fn all_owned_bodies_match_complete_independent_fields_and_forced_construction() {
    project_packet(EXPECTED, 560, project_case);
}

#[test]
#[ignore = "requires explicit complete native joint/group/list field expectations"]
fn every_native_body_matches_complete_independent_projection_with_optional_export() {
    let path = std::env::var_os("BESSEMER_BEAM_GROUPS_NATIVE_PROJECTION_INPUT")
        .expect("set the complete native projection packet");
    let native = std::fs::read(path).unwrap();
    let mut packet = project_packet(EXPECTED, 560, project_case);
    packet.extend(project_packet(&native, 414, project_case));
    if let Some(path) = std::env::var_os("BESSEMER_BEAM_GROUPS_PROJECTION_OUTPUT") {
        std::fs::write(path, packet).unwrap();
    }
}

#[test]
fn every_owned_kind_retains_all_original_span_boundaries() {
    let packet = include_bytes!("fixtures/simulation-beam-groups/owned.bin");
    assert_eq!(span_counts(packet), [140, 140, 140, 140]);
}

fn span_counts(mut cursor: &[u8]) -> [usize; 4] {
    let mut counts = [0; 4];
    while !cursor.is_empty() {
        let body = read_body(&mut cursor);
        counts[kind_index(body.kind())] += 1;
        let mut end = 0;
        for _ in 0..read_word(&mut cursor) {
            let start = read_word(&mut cursor) as usize;
            let next = read_word(&mut cursor) as usize;
            assert_eq!(start, end);
            assert_source_span(&body, start, next);
            end = next;
        }
        assert_eq!(end, body.to_bytes().len());
    }
    counts
}

#[test]
#[ignore = "requires all fresh original native acceptance/span outcomes"]
fn all_native_original_spans_and_counts_match_without_inventing_folder_fixtures() {
    let path = std::env::var_os("BESSEMER_BEAM_GROUPS_NATIVE_INPUT")
        .expect("set fresh native input/span packet");
    assert_eq!(span_counts(&std::fs::read(path).unwrap()), [36, 367, 11, 0]);
}

fn assert_source_span(body: &SimulationBeamGroupBody, start: usize, end: usize) {
    if start == end {
        assert!(
            body.field_offsets()
                .iter()
                .any(|span| span.0 == start || span.1 == end),
            "empty source span {start}..{end}; expected a physical boundary (#145)"
        );
        return;
    }
    assert!(body.field_offsets().iter().any(|span| span.0 == start));
    assert!(body.field_offsets().iter().any(|span| span.1 == end));
}

#[test]
fn empty_neighbor_arrays_at_eof_remain_completed_without_invented_physical_fields() {
    let body = empty_neighbor_list();
    assert_source_span(&body, 8, 8);
    assert_source_span(&body, 12, 12);
    assert_eq!(body.fields().len(), 3);
    assert_eq!(body.to_bytes().len(), 12);
    assert_eq!(
        body.coverage().completion,
        SimulationBeamGroupCompletion::CompletedBody
    );
}

fn empty_neighbor_list() -> SimulationBeamGroupBody {
    let raw = [2_i32, 0, 0]
        .into_iter()
        .flat_map(i32::to_le_bytes)
        .collect::<Vec<u8>>();
    let kind = SimulationBeamGroupKind::JointList {
        expected_joint_count: Some(2),
    };
    SimulationBeamGroupBody::from_bytes(
        &raw,
        SimulationPersistVersion::new(891107).unwrap(),
        kind,
        LIMITS,
    )
    .unwrap()
}

#[test]
fn joint_list_count_scope_is_explicit_and_mismatches_never_reconstruct_adjacency() {
    let body = empty_neighbor_list();
    let raw = body.to_bytes();
    let unchecked_kind = SimulationBeamGroupKind::JointList {
        expected_joint_count: None,
    };
    let unchecked =
        SimulationBeamGroupBody::from_bytes(&raw, body.version(), unchecked_kind, LIMITS).unwrap();
    assert_eq!(unchecked.fields(), body.fields());
    assert_ne!(unchecked.coverage().check_mode, body.coverage().check_mode);
    for count in [1, 10001] {
        let kind = SimulationBeamGroupKind::JointList {
            expected_joint_count: Some(count),
        };
        let error =
            SimulationBeamGroupBody::from_bytes(&raw, body.version(), kind, LIMITS).unwrap_err();
        assert!(error.cause.contains(&count.to_string()));
    }
    assert_eq!(body.to_bytes(), raw);
}

#[test]
fn foreign_and_structural_edit_requests_preserve_inputs_and_count_scope() {
    let body = empty_neighbor_list();
    let original = body.to_bytes();
    let foreign = empty_neighbor_list().field_id(0).unwrap();
    assert!(body.field(&foreign).is_err());
    assert!(body.field_id(body.fields().len()).is_err());
    let id = body.field_id(0).unwrap();
    assert!(
        body.edit_field(&id, SimulationBeamGroupValue::Signed(1))
            .is_err()
    );
    assert!(
        body.edit_field(&id, SimulationBeamGroupValue::Unsigned(2))
            .is_err()
    );
    assert_eq!(body.to_bytes(), original);
    assert_count_noop(&body, &id);
}

fn assert_count_noop(body: &SimulationBeamGroupBody, id: &SimulationBeamGroupFieldId) {
    let unchanged = body
        .edit_field(id, SimulationBeamGroupValue::Signed(2))
        .unwrap();
    assert_eq!(unchanged.parent(), unchanged.result());
    assert_eq!(
        unchanged.coverage().check_mode,
        SimulationBeamGroupCheckMode::ExactStoredProfileAndPreservation {
            expected_joint_count: Some(2)
        }
    );
    assert_eq!(
        unchanged.replacement(),
        &SimulationBeamGroupValue::Signed(2)
    );
}

#[test]
fn every_published_group_type_is_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<SimulationBeamGroupBody>();
    check::<SimulationBeamGroupKind>();
    check::<SimulationBeamGroupEdit>();
    check::<SimulationBeamGroupCoverage>();
    check::<SimulationBeamGroupCompletion>();
    check::<SimulationBeamGroupCheckMode>();
    check::<SimulationBeamGroupDiagnostic>();
    check::<SimulationBeamGroupField>();
    check::<SimulationBeamGroupValue>();
    check::<SimulationBeamGroupLimits>();
    check::<SimulationBeamGroupFieldId>();
    check::<SimulationBeamGroupHistory>();
    check::<SimulationBeamGroupHistoryParent>();
    check::<SimulationBeamGroupNestedHistory>();
    check::<SimulationBeamGroupNestedParent>();
    check::<SimulationBeamGroupRelation>();
}
