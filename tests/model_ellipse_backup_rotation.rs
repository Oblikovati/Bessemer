//! Full signed ellipse-backup snapshots against independent complete-root literals (#192).
#[path = "model_ellipse_backup_rotation/concurrency.rs"]
mod concurrency;
#[path = "model_ellipse_backup_rotation/contracts.rs"]
mod contracts;
#[path = "model_ellipse_backup_rotation/exports.rs"]
mod exports;
#[path = "../examples/support/framing.rs"]
mod framing;
#[path = "model_ellipse_backup_rotation/historical.rs"]
mod historical;
#[path = "support/model_operation_snapshot.rs"]
mod model_operation_snapshot;
#[path = "model_ellipse_backup_rotation/native.rs"]
mod native;
#[path = "support/native_kernel_context.rs"]
mod native_kernel_context;
#[path = "support/owned_resolved_document.rs"]
mod owned_resolved_document;
#[path = "support/unselected_document_streams.rs"]
mod unselected_document_streams;

use bessemer::FailureClass;
use bessemer::model_archives::*;
use owned_resolved_document::{prepared_owned_resolved_document, read_owned_resolved_document};

const ROOTS: &[u8] = include_bytes!("fixtures/ellipse-backup-rotation/roots.bin");
const ORACLES: &str = include_str!("fixtures/ellipse-backup-rotation/cases.tsv");
const REQUESTS: [i32; 5] = [i32::MIN, -1, 0, 1, i32::MAX];
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 64 * 1024 * 1024,
    maximum_fields: 2_000_000,
    maximum_entries: 2_000_000,
    maximum_depth: 128,
    maximum_objects: 100_000,
};

struct EllipseCase {
    version: u32,
    raw: &'static [u8],
    offsets: [usize; 3],
    snapshots: [i32; 3],
    slots: usize,
}

fn ellipse_cases() -> Vec<EllipseCase> {
    assert_eq!(&ROOTS[..4], b"EBR1");
    let mut cursor = &ROOTS[4..];
    let count = framing::take_u32(&mut cursor).unwrap();
    assert_eq!(count, 108);
    framing::require_count(count, cursor.len(), 8).unwrap();
    let mut cases = Vec::new();
    for (index, line) in ORACLES.lines().enumerate() {
        let columns: Vec<_> = line.split('\t').collect();
        assert_eq!(columns.len(), 10);
        assert_eq!(columns[0].parse::<usize>().unwrap(), index);
        cases.push(next_ellipse_case(&mut cursor, &columns));
    }
    framing::require_finished(cursor).unwrap();
    assert_eq!(cases.len(), 108);
    cases
}

fn next_ellipse_case(cursor: &mut &'static [u8], columns: &[&str]) -> EllipseCase {
    let version = framing::take_u32(cursor).unwrap();
    assert_eq!(version, columns[1].parse::<u32>().unwrap());
    let length = framing::take_u32(cursor).unwrap() as usize;
    EllipseCase {
        version,
        raw: framing::take(cursor, length).unwrap(),
        offsets: std::array::from_fn(|index| columns[3 + index].parse().unwrap()),
        snapshots: std::array::from_fn(|index| columns[6 + index].parse().unwrap()),
        slots: columns[9].parse().unwrap(),
    }
}

fn current_ellipse_archive(case: &EllipseCase) -> PartResolvedFeaturesArchive {
    assert_eq!(case.version, 19000);
    let document = prepared_owned_resolved_document(case.raw);
    let source = read_owned_resolved_document(&document, LIMITS);
    assert_eq!(source.to_bytes().unwrap(), case.raw);
    assert_eq!(source.object_slots(), (97, case.slots));
    source
}

fn verify_ellipse_projection(source: &PartResolvedFeaturesArchive, case: &EllipseCase) {
    let states = source.ellipse_backup_rotation_states().unwrap();
    assert_eq!(states.len(), 2);
    for (index, state) in states.iter().enumerate() {
        assert_eq!(state.rotation_direction_snapshot(), case.snapshots[index]);
        assert_eq!(
            source.resolve_field(state.source_field()).unwrap().value,
            ModelFieldValue::I32(case.snapshots[index])
        );
        assert_eq!(
            source
                .ellipse_backup_rotation_state(state.owner_field())
                .unwrap(),
            *state
        );
    }
    assert_eq!(source.to_bytes().unwrap(), case.raw);
}

fn verify_ellipse_edit(
    source: &PartResolvedFeaturesArchive,
    case: &EllipseCase,
    owner: usize,
    snapshot: i32,
) -> Vec<u8> {
    let states = source.ellipse_backup_rotation_states().unwrap();
    let state = &states[owner];
    let edit = source
        .edit_ellipse_backup_rotation(state, snapshot)
        .unwrap();
    let expected = ellipse_expected_bytes(case, owner, snapshot);
    assert_eq!(edit.archive().to_bytes().unwrap(), expected);
    let mut fields = source.fields().to_vec();
    fields[state.source_field().index()].value = ModelFieldValue::I32(snapshot);
    assert_eq!(edit.archive().fields(), fields);
    contracts::verify_ellipse_result(source, state, &edit, snapshot);
    verify_ellipse_reversal(
        source,
        case.raw,
        &edit,
        owner,
        state.rotation_direction_snapshot(),
    );
    assert_eq!(source.to_bytes().unwrap(), case.raw);
    expected
}

fn ellipse_expected_bytes(case: &EllipseCase, owner: usize, snapshot: i32) -> Vec<u8> {
    patch_ellipse_word(case.raw, case.offsets[owner], snapshot)
}

fn patch_ellipse_word(raw: &[u8], offset: usize, snapshot: i32) -> Vec<u8> {
    let mut expected = raw.to_vec();
    expected[offset..offset + 4].copy_from_slice(&snapshot.to_le_bytes());
    expected
}

fn verify_ellipse_reversal(
    source: &PartResolvedFeaturesArchive,
    original: &[u8],
    edit: &PartResolvedFeaturesEdit,
    owner: usize,
    snapshot: i32,
) {
    let fresh = edit.archive().ellipse_backup_rotation_states().unwrap();
    let reverse = edit
        .archive()
        .edit_ellipse_backup_rotation(&fresh[owner], snapshot)
        .unwrap();
    assert_eq!(reverse.archive().to_bytes().unwrap(), original);
    assert_eq!(reverse.archive().fields(), source.fields());
}

fn verify_ellipse_case_requests(source: &PartResolvedFeaturesArchive, case: &EllipseCase) -> usize {
    let mut operations = 0;
    for owner in 0..2 {
        for snapshot in REQUESTS {
            verify_ellipse_edit(source, case, owner, snapshot);
            operations += 1;
        }
    }
    operations
}

#[test]
fn current_contexts_project_two_physical_ellipses_without_alias_or_arc_roles() {
    for case in ellipse_cases()
        .into_iter()
        .filter(|case| case.version == 19000)
    {
        let source = current_ellipse_archive(&case);
        verify_ellipse_projection(&source, &case);
        contracts::verify_non_ellipse_handles(&source);
    }
}

#[test]
fn current_signed_edits_match_literal_bytes_all_fields_history_and_exact_reversals() {
    let mut operations = 0;
    for case in ellipse_cases()
        .into_iter()
        .filter(|case| case.version == 19000)
    {
        let source = current_ellipse_archive(&case);
        operations += verify_ellipse_case_requests(&source, &case);
    }
    assert_eq!(operations, 360);
}

#[test]
fn current_revisions_reject_foreign_and_stale_states_without_input_mutation() {
    for case in ellipse_cases()
        .into_iter()
        .filter(|case| case.version == 19000)
    {
        let source = current_ellipse_archive(&case);
        let foreign = current_ellipse_archive(&case);
        for state in source.ellipse_backup_rotation_states().unwrap() {
            contracts::verify_ellipse_revision_guards(&source, &foreign, &state);
        }
        assert_eq!(source.to_bytes().unwrap(), case.raw);
        assert_eq!(foreign.to_bytes().unwrap(), case.raw);
    }
}

#[test]
fn ellipse_snapshot_state_is_send_sync() {
    fn require<T: Send + Sync>() {}
    require::<ModelEllipseBackupRotationState>();
}

#[test]
fn complete_arc_only_archives_have_no_ellipse_roles() {
    for case in ellipse_cases()
        .into_iter()
        .filter(|case| case.version == 19000)
    {
        let start = case.offsets[2] - 12 - 6 - "moArcBackedUpData_c".len();
        let mut raw = case.raw[..6].to_vec();
        raw[4..6].copy_from_slice(&1_u16.to_le_bytes());
        raw.extend_from_slice(&case.raw[start..]);
        let document = prepared_owned_resolved_document(&raw);
        let source = read_owned_resolved_document(&document, LIMITS);
        assert!(source.ellipse_backup_rotation_states().unwrap().is_empty());
        assert_eq!(source.root_count(), 1);
        assert_eq!(source.to_bytes().unwrap(), raw);
    }
}

#[test]
fn generic_valid_other_schema_does_not_gain_the_canonical_ellipse_role() {
    let case = ellipse_cases()
        .into_iter()
        .find(|case| case.version == 19000)
        .unwrap();
    let source = current_ellipse_archive(&case);
    let noncanonical = noncanonical_ellipse_archive(&case);
    let error = noncanonical.ellipse_backup_rotation_states().unwrap_err();
    assert_eq!(error.class, FailureClass::Invalid);
    assert!(error.cause.contains("schema1 moEllipseBackedUpData_c"));
    assert_eq!(source.to_bytes().unwrap(), case.raw);
}

fn noncanonical_ellipse_archive(case: &EllipseCase) -> PartResolvedFeaturesArchive {
    let mut raw = case.raw.to_vec();
    assert_eq!(&raw[6..10], &[255, 255, 1, 0]);
    raw[8..10].copy_from_slice(&7_u16.to_le_bytes());
    let document = prepared_owned_resolved_document(&raw);
    let source = read_owned_resolved_document(&document, LIMITS);
    assert_eq!(source.to_bytes().unwrap(), raw);
    source
}
