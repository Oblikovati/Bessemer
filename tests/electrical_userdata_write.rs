//! Isolated text mutations validate the entire stored database and source checkpoints.
#[path = "electrical_userdata_write/fixture.rs"]
mod fixture;
#[path = "electrical_userdata_write/native.rs"]
mod native;
#[path = "electrical_userdata_write/projection.rs"]
mod projection;
#[path = "support/sqlite_row_wire.rs"]
mod sqlite_row_wire;
#[path = "support/sqlite_wire.rs"]
mod sqlite_wire;
use bessemer::{electrical::*, sqlite_image::SqliteValue};

#[test]
fn every_owned_success_matches_independent_full_database_values() {
    let cases = fixture::owned_cases();
    assert_eq!(cases.len(), 15);
    for case in cases.iter().filter(|case| case.success) {
        let (edit, member) = perform_owned_write(case);
        verify_owned_projection(case, &edit);
        verify_edit_contract(&edit, &member);
    }
}

fn perform_owned_write(case: &fixture::OwnedCase) -> (ElectricalUserdataWrite, ElectricalMemberId) {
    let archive = fixture::archive(&case.raw);
    let member = fixture::owned_member(&archive);
    let edit = write(&archive, &member, &case.replacement, fixture::limits()).unwrap();
    assert_eq!(archive.to_bytes(), case.raw);
    (edit, member)
}

fn verify_owned_projection(case: &fixture::OwnedCase, edit: &ElectricalUserdataWrite) {
    let output = edit
        .result()
        .sqlite_member(&edit.history().result_member, fixture::limits())
        .unwrap();
    assert!(
        projection::logical_snapshot(&output) == case.expected,
        "{}",
        case.name
    );
    assert_eq!(edit.source().to_bytes(), case.raw);
    assert_eq!(
        edit.history().after,
        SqliteValue::Text(case.replacement.as_bytes().into())
    );
}

fn write(
    archive: &ElectricalArchive,
    member: &ElectricalMemberId,
    text: &str,
    limits: bessemer::sqlite_image::SqliteLimits,
) -> bessemer::Result<ElectricalUserdataWrite> {
    archive.edit_userdata_text(member, fixture::key(), slot(0), text, limits)
}

fn verify_edit_contract(edit: &ElectricalUserdataWrite, member: &ElectricalMemberId) {
    assert_eq!(&edit.history().source_member, member);
    assert_eq!(edit.history().key, fixture::key());
    assert_eq!(edit.history().slot.index(), 0);
    assert_eq!(
        edit.coverage().mode,
        ElectricalUserdataWriteCheckMode::ExactTextAndStoredDatabase
    );
    assert_eq!(edit.coverage().achieved_byte_error, 0);
    assert!(edit.coverage().checked_tables >= 2);
    assert!(edit.coverage().checked_cells >= 25);
    assert!(edit.coverage().checked_rows >= 2);
    assert!(edit.coverage().accounted_bytes > 0);
    assert_eq!(edit.diagnostics().len(), 4);
    let opaque = edit.result().member_named("opaque.bin").unwrap();
    assert_eq!(
        edit.result().read_member(&opaque).unwrap(),
        b"untouched\0\xff"
    );
}

#[test]
fn all_unsupported_or_side_effect_cases_fail_without_publishing_or_mutating_inputs() {
    for case in fixture::owned_cases().iter().filter(|case| !case.success) {
        let archive = fixture::archive(&case.raw);
        let member = fixture::owned_member(&archive);
        let error = archive
            .edit_userdata_text(
                &member,
                fixture::key(),
                slot(0),
                &case.replacement,
                fixture::limits(),
            )
            .unwrap_err();
        assert_eq!(
            error.operation, "Electrical userdata write",
            "{}",
            case.name
        );
        assert!(error.cause.contains("expected"), "{}: {}", case.name, error);
        assert_eq!(archive.to_bytes(), case.raw);
    }
}

#[test]
fn unchanged_text_preserves_exact_original_bytes_and_copied_history() {
    let case = fixture::owned_cases()
        .into_iter()
        .find(|case| case.name == "no-op")
        .unwrap();
    let archive = fixture::archive(&case.raw);
    let member = fixture::owned_member(&archive);
    let edit = archive
        .edit_userdata_text(&member, fixture::key(), slot(0), "old", fixture::limits())
        .unwrap();
    assert_eq!(edit.result().to_bytes(), case.raw);
    assert_eq!(edit.source().to_bytes(), case.raw);
    assert_eq!(edit.history().before, edit.history().after);
    assert_eq!(edit.history().relation, ElectricalMemberRelation::Copied);
    assert!(
        edit.carrier_edit()
            .history()
            .iter()
            .all(|entry| entry.relation == ElectricalMemberRelation::Copied)
    );
}

#[test]
fn slot_limits_missing_keys_page_growth_and_snapshot_budgets_are_explicit() {
    let case = fixture::owned_cases().remove(0);
    let archive = fixture::archive(&case.raw);
    let member = fixture::owned_member(&archive);
    let missing = ElectricalUserdataKey {
        object_id: 999,
        object_no: 0,
    };
    assert!(
        archive
            .edit_userdata_text(&member, missing, slot(0), "new", fixture::limits())
            .is_err()
    );
    let mut limits = fixture::limits();
    limits.maximum_image_bytes = archive.read_member(&member).unwrap().len();
    assert!(write(&archive, &member, &"µ".repeat(4096), limits).is_err());
    limits = fixture::limits();
    limits.maximum_vm_steps = 1;
    assert!(write(&archive, &member, "new", limits).is_err());
    assert_eq!(archive.to_bytes(), case.raw);
}

#[test]
fn slot_and_value_byte_limits_are_checked_before_the_update() {
    assert!(ElectricalUserdataSlot::new(20).is_err());
    assert!(ElectricalUserdataSlot::new(usize::MAX).is_err());
    let case = fixture::owned_cases().remove(0);
    let archive = fixture::archive(&case.raw);
    let member = fixture::owned_member(&archive);
    let mut limits = fixture::limits();
    limits.maximum_value_bytes = 4096;
    assert!(write(&archive, &member, &"µ".repeat(4096), limits).is_err());
    assert_eq!(archive.to_bytes(), case.raw);
}

#[test]
fn bound_sql_text_and_revision_handles_cannot_target_other_statements_or_sources() {
    let case = fixture::owned_cases().remove(0);
    let archive = fixture::archive(&case.raw);
    let member = fixture::owned_member(&archive);
    let text = "'); DELETE FROM tew_userdata; --\0µ";
    let edit = write(&archive, &member, text, fixture::limits()).unwrap();
    assert_eq!(
        edit.history().after,
        SqliteValue::Text(text.as_bytes().into())
    );
    assert!(write(edit.result(), &member, "stale", fixture::limits()).is_err());
    assert_eq!(archive.to_bytes(), case.raw);
}

#[test]
fn full_snapshot_accounting_accepts_its_exact_bound_and_rejects_one_less() {
    let case = fixture::owned_cases().remove(0);
    let archive = fixture::archive(&case.raw);
    let member = fixture::owned_member(&archive);
    let edit = archive
        .edit_userdata_text(&member, fixture::key(), slot(0), "new", fixture::limits())
        .unwrap();
    let mut limits = fixture::limits();
    limits.maximum_output_bytes = edit.coverage().accounted_bytes;
    assert!(
        archive
            .edit_userdata_text(&member, fixture::key(), slot(0), "new", limits)
            .is_ok()
    );
    limits.maximum_output_bytes -= 1;
    assert!(
        archive
            .edit_userdata_text(&member, fixture::key(), slot(0), "new", limits)
            .is_err()
    );
    assert_eq!(archive.to_bytes(), case.raw);
}

#[test]
fn immutable_checkpoints_and_eight_mixed_workers_isolate_callers() {
    verify_send_sync();
    let mut case = fixture::owned_cases().remove(0);
    let archive = fixture::archive(&case.raw);
    let member = fixture::owned_member(&archive);
    let mut text = String::from("caller\0µ");
    let edit = write(&archive, &member, &text, fixture::limits()).unwrap();
    text.clear();
    case.raw.fill(0);
    assert_eq!(
        edit.history().after,
        SqliteValue::Text(b"caller\0\xc2\xb5".as_slice().into())
    );
    run_write_workers(&archive, &member);
}

fn run_write_workers(archive: &ElectricalArchive, member: &ElectricalMemberId) {
    let expected = write(archive, member, "thread", fixture::limits())
        .unwrap()
        .result()
        .to_bytes();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| mixed_worker(archive, member, &expected)))
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
}

fn mixed_worker(archive: &ElectricalArchive, member: &ElectricalMemberId, expected: &[u8]) {
    let next = write(archive, member, "thread", fixture::limits()).unwrap();
    assert_eq!(next.source().to_bytes(), archive.to_bytes());
    assert_eq!(next.result().to_bytes(), expected);
    let missing = ElectricalUserdataKey {
        object_id: 999,
        object_no: 0,
    };
    assert!(
        archive
            .edit_userdata_text(member, missing, slot(0), "bad", fixture::limits())
            .is_err()
    );
}

#[test]
#[ignore = "Complete 680 native text edits and 680 no-ops; run explicitly for write acceptance"]
fn complete_native_key_and_slot_corpus_reopens_preserves_rows_and_source_checkpoints() {
    let cases = fixture::native_cases();
    assert_eq!(cases.len(), 4);
    let edits: usize = cases.iter().map(|case| native::run_case(case, true)).sum();
    assert_eq!(edits, 680);
}

#[test]
fn representative_native_first_and_last_slots_preserve_complete_carriers() {
    let cases = fixture::native_cases();
    let edits: usize = cases.iter().map(|case| native::run_case(case, false)).sum();
    assert_eq!(edits, 8);
}

fn verify_send_sync() {
    assert_send_sync::<ElectricalUserdataWrite>();
    assert_send_sync::<ElectricalUserdataSlot>();
    assert_send_sync::<ElectricalUserdataCellHistory>();
    assert_send_sync::<ElectricalUserdataWriteCoverage>();
    assert_send_sync::<ElectricalUserdataWriteCheckMode>();
    assert_send_sync::<ElectricalUserdataWriteDiagnostic>();
}
fn assert_send_sync<T: Send + Sync>() {}
fn slot(index: usize) -> ElectricalUserdataSlot {
    ElectricalUserdataSlot::new(index).unwrap()
}
