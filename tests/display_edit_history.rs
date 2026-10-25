//! Exact stored-archive edits publish revision-bound structural provenance.
use bessemer::{FailureClass, display_lists::*};
#[allow(
    dead_code,
    reason = "Shared archive helpers also serve other integration tests."
)]
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::{LIMITS, context};

fn history_source() -> DisplayArchive {
    archive_support::decode_document(
        include_bytes!("fixtures/blank-part.SLDPRT"),
        DisplayDocumentKind::Part,
    )
}

fn changed_contents(source: &DisplayArchive) -> DisplayArchiveContents {
    let mut contents = source.contents().clone();
    archive_support::set_integer(
        &mut contents,
        "root.markup_ink_views.configuration_count",
        7,
    );
    contents
}

fn unshaded_contents(source: &DisplayArchive) -> DisplayArchiveContents {
    let mut contents = source.contents().clone();
    let position = contents
        .fields
        .iter()
        .position(|field| field.path.as_ref() == "root.has_shaded_data")
        .unwrap();
    contents.fields[position].value = DisplayFieldValue::I32(0);
    contents.fields.truncate(position + 1);
    contents
}

#[test]
fn completed_unchanged_edits_have_exact_coverage_and_no_changes() {
    let source = history_source();
    let outcome = source
        .edit_with_history(source.contents().clone(), LIMITS)
        .unwrap();
    assert_eq!(outcome.completion(), DisplayEditCompletion::Unchanged);
    assert_eq!(outcome.fidelity(), DisplayEditFidelity::ExactStoredFields);
    assert_eq!(
        outcome.coverage().mode,
        DisplayEditCheckMode::ExactStoredArchive
    );
    assert_eq!(outcome.coverage().wire_bytes, 6316);
    assert_eq!(outcome.coverage().loaded_objects, 46);
    assert_eq!(outcome.coverage().object_aliases, 0);
    assert_eq!(outcome.coverage().unchecked_external_faces, 0);
    check_unchanged_report(&source, &outcome);
}

fn check_unchanged_report(source: &DisplayArchive, outcome: &DisplayArchiveEdit) {
    assert_eq!(
        outcome.coverage().typed_fields,
        source.contents().fields.len()
    );
    assert_eq!(outcome.unchanged_fields(), outcome.coverage().typed_fields);
    assert_eq!(
        outcome.diagnostics(),
        [DisplayEditDiagnostic::GeometryOutsideScope]
    );
    assert!(outcome.changes().is_empty());
    assert_eq!(
        outcome.archive().to_bytes().unwrap(),
        source.to_bytes().unwrap()
    );
}

fn check_derived_change(source: &DisplayArchive, outcome: &DisplayArchiveEdit) {
    assert_eq!(outcome.completion(), DisplayEditCompletion::Changed);
    assert_eq!(outcome.changes().len(), 1);
    let change = &outcome.changes()[0];
    assert_eq!(change.path(), "root.markup_ink_views.configuration_count");
    assert_eq!(change.relation(), DisplayFieldRelation::Derived);
    assert_eq!(change.parents().len(), 1);
    let parent = source.resolve_field(&change.parents()[0]).unwrap();
    let result = outcome
        .archive()
        .resolve_field(change.result().unwrap())
        .unwrap();
    assert_eq!(parent.value, DisplayFieldValue::I32(1));
    assert_eq!(result.value, DisplayFieldValue::I32(7));
    assert_eq!(parent.path, result.path);
    assert_eq!(
        outcome.unchanged_fields() + 1,
        source.contents().fields.len()
    );
}

#[test]
fn derived_fields_resolve_only_in_their_owning_source_and_result_revisions() {
    let source = history_source();
    let outcome = source
        .edit_with_history(changed_contents(&source), LIMITS)
        .unwrap();
    check_derived_change(&source, &outcome);
    let change = &outcome.changes()[0];
    let parent = &change.parents()[0];
    let target = change.result().unwrap();
    assert_eq!(
        source.clone().resolve_field(parent).unwrap(),
        source.resolve_field(parent).unwrap()
    );
    check_foreign_field(outcome.archive(), parent);
    check_foreign_field(&source, target);
    check_foreign_field(&history_source(), parent);
}

fn check_foreign_field(archive: &DisplayArchive, handle: &DisplayFieldHandle) {
    let error = archive.resolve_field(handle).unwrap_err();
    assert_eq!(error.class, FailureClass::Invalid);
    assert!(
        error
            .cause
            .contains("expected a handle from this archive revision")
    );
}

fn check_deleted_changes(source: &DisplayArchive, outcome: &DisplayArchiveEdit) {
    let mut deleted = 0;
    for change in outcome.changes() {
        let parent = source.resolve_field(&change.parents()[0]).unwrap();
        assert_eq!(change.path(), parent.path.as_ref());
        if change.relation() == DisplayFieldRelation::Deleted {
            assert!(change.result().is_none());
            deleted += 1;
        } else {
            assert_eq!(change.relation(), DisplayFieldRelation::Derived);
            assert_eq!(change.path(), "root.has_shaded_data");
        }
    }
    assert_eq!(
        deleted,
        source.contents().fields.len() - outcome.archive().contents().fields.len()
    );
}

#[test]
fn removed_and_restored_shading_have_complete_deletion_and_creation_parents() {
    let source = history_source();
    let removed = source
        .edit_with_history(unshaded_contents(&source), LIMITS)
        .unwrap();
    check_deleted_changes(&source, &removed);
    let restored = removed
        .archive()
        .edit_with_history(source.contents().clone(), LIMITS)
        .unwrap();
    check_created_changes(removed.archive(), &restored);
    assert_eq!(
        restored.archive().to_bytes().unwrap(),
        source.to_bytes().unwrap()
    );
    assert_eq!(source.to_bytes().unwrap().len(), 6316);
}

fn check_created_changes(source: &DisplayArchive, outcome: &DisplayArchiveEdit) {
    let mut created = 0;
    for change in outcome.changes() {
        let result = outcome
            .archive()
            .resolve_field(change.result().unwrap())
            .unwrap();
        assert_eq!(change.path(), result.path.as_ref());
        created += check_creation_parent(source, change);
    }
    assert_eq!(
        created,
        outcome.archive().contents().fields.len() - source.contents().fields.len()
    );
}

fn check_creation_parent(source: &DisplayArchive, change: &DisplayFieldChange) -> usize {
    if change.relation() == DisplayFieldRelation::Created {
        assert!(change.parents().is_empty());
        return 1;
    }
    assert_eq!(change.relation(), DisplayFieldRelation::Derived);
    assert_eq!(
        source.resolve_field(&change.parents()[0]).unwrap().value,
        DisplayFieldValue::I32(0)
    );
    0
}

#[test]
fn invalid_and_over_budget_edits_publish_no_report_and_preserve_the_source() {
    let source = history_source();
    let original = source.to_bytes().unwrap();
    let mut invalid = changed_contents(&source);
    invalid.fields[0].value = DisplayFieldValue::I32(99);
    assert_eq!(
        source.edit_with_history(invalid, LIMITS).unwrap_err().class,
        FailureClass::Invalid
    );
    let short = DisplayArchiveLimits {
        maximum_fields: source.contents().fields.len() - 1,
        ..LIMITS
    };
    assert!(
        source
            .edit_with_history(changed_contents(&source), short)
            .is_err()
    );
    assert_eq!(source.to_bytes().unwrap(), original);
}

#[test]
fn reports_and_handles_are_owned_send_sync_and_concurrent_results_are_ordered() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<DisplayArchiveEdit>();
    assert_send_sync::<DisplayFieldHandle>();
    assert_send_sync::<DisplayFieldChange>();
    let source = history_source();
    let jobs: Vec<_> = (0..8)
        .map(|_| {
            let cloned = source.clone();
            std::thread::spawn(move || {
                cloned
                    .edit_with_history(changed_contents(&cloned), LIMITS)
                    .unwrap()
            })
        })
        .collect();
    for job in jobs {
        check_derived_change(&source, &job.join().unwrap());
    }
}

#[test]
fn shared_map_aliases_are_counted_without_becoming_new_objects() {
    let source = DisplayArchive::from_bytes(
        include_bytes!("fixtures/display-lists/shared-font-part.bin"),
        context(),
        LIMITS,
    )
    .unwrap();
    let outcome = source
        .edit_with_history(source.contents().clone(), LIMITS)
        .unwrap();
    assert_eq!(outcome.coverage().loaded_objects, 45);
    assert_eq!(outcome.coverage().object_aliases, 1);
    assert_eq!(
        outcome.coverage().wire_bytes,
        source.to_bytes().unwrap().len()
    );
}

fn external_history_source() -> DisplayArchive {
    DisplayArchive::from_bytes(
        include_bytes!("fixtures/display-lists/external-shaded-assembly.bin"),
        DisplayArchiveContext {
            kind: DisplayDocumentKind::Assembly,
            face_tessellation_components: std::sync::Arc::new(["Assem1".to_owned()].into()),
            ..context()
        },
        LIMITS,
    )
    .unwrap()
}

#[test]
fn external_face_references_are_reported_as_unchecked_not_certified() {
    let source = external_history_source();
    let outcome = source
        .edit_with_history(source.contents().clone(), LIMITS)
        .unwrap();
    assert_eq!(outcome.coverage().unchecked_external_faces, 2);
    assert_eq!(
        outcome.diagnostics(),
        [
            DisplayEditDiagnostic::GeometryOutsideScope,
            DisplayEditDiagnostic::ExternalFaceLinksNotChecked
        ]
    );
}

#[test]
fn consecutive_edits_and_authentic_reverts_keep_each_intermediate_parent() {
    let source = history_source();
    let first = source
        .edit_with_history(changed_contents(&source), LIMITS)
        .unwrap();
    let restored = first
        .archive()
        .edit_with_history(source.contents().clone(), LIMITS)
        .unwrap();
    check_reverted_parent(&source, &first, &restored);
    assert_eq!(
        restored.archive().to_bytes().unwrap(),
        source.to_bytes().unwrap()
    );
}

fn check_reverted_parent(
    source: &DisplayArchive,
    first: &DisplayArchiveEdit,
    restored: &DisplayArchiveEdit,
) {
    let change = &restored.changes()[0];
    let parent = &change.parents()[0];
    assert_eq!(
        first.archive().resolve_field(parent).unwrap().value,
        DisplayFieldValue::I32(7)
    );
    assert!(source.resolve_field(parent).is_err());
    let result = restored
        .archive()
        .resolve_field(change.result().unwrap())
        .unwrap();
    assert_eq!(result.value, DisplayFieldValue::I32(1));
}

fn history_order(outcome: &DisplayArchiveEdit) -> Vec<(String, DisplayFieldRelation)> {
    outcome
        .changes()
        .iter()
        .map(|change| (change.path().to_owned(), change.relation()))
        .collect()
}

#[test]
fn every_edit_report_type_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<DisplayArchiveEdit>();
    assert_send_sync::<DisplayFieldHandle>();
    assert_send_sync::<DisplayFieldChange>();
    assert_send_sync::<DisplayFieldRelation>();
    assert_send_sync::<DisplayEditCompletion>();
    assert_send_sync::<DisplayEditFidelity>();
    assert_send_sync::<DisplayEditDiagnostic>();
    assert_send_sync::<DisplayEditCoverage>();
    assert_send_sync::<DisplayEditCheckMode>();
}

#[test]
fn parallel_complete_deletions_match_serial_history_order_and_resolve_all_parents() {
    let source = history_source();
    let serial = source
        .edit_with_history(unshaded_contents(&source), LIMITS)
        .unwrap();
    let jobs: Vec<_> = (0..8).map(|_| spawn_unshaded_edit(&source)).collect();
    for job in jobs {
        let outcome = job.join().unwrap();
        assert_eq!(history_order(&outcome), history_order(&serial));
        check_deleted_changes(&source, &outcome);
    }
}

fn spawn_unshaded_edit(source: &DisplayArchive) -> std::thread::JoinHandle<DisplayArchiveEdit> {
    let cloned = source.clone();
    std::thread::spawn(move || {
        cloned
            .edit_with_history(unshaded_contents(&cloned), LIMITS)
            .unwrap()
    })
}
