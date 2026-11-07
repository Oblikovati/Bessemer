//! Generic bucket keys reach early PW layouts before document-root scenes (#41).
use super::*;
const PW_PATH: &str = "root.views[0].view_bucket.bucket.cthread_maps[0].entries[0].key";

fn pw_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/early-pw-scene-roots.bin"
    ))
}

#[test]
fn all_generic_caller_pw_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(pw_cases().len(), 130);
    assert_eq!(
        pw_cases().into_iter().map(check_case).sum::<usize>(),
        115921
    );
}

fn pw_selected(case: OldCase, scenario: u8) -> OldCase {
    pw_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version && row.kind == case.kind && row.scenario == scenario
        })
        .unwrap()
}

fn pw_field<'a>(archive: &'a DisplayArchive, suffix: &str) -> Option<&'a DisplayFieldValue> {
    field(archive, &format!("{PW_PATH}.{suffix}"))
}

fn check_pw_members(case: OldCase) {
    let archive = parsed(case);
    assert_eq!(
        pw_field(&archive, "background_image_type").is_some(),
        case.version > 3595
    );
    assert_eq!(
        pw_field(&archive, "background_fit_to_camera_fov").is_some(),
        case.version > 3897
    );
    check_pw_wall_members(&archive, case);
}

fn check_pw_wall_members(archive: &DisplayArchive, case: OldCase) {
    let count = pw_wall_count(case);
    assert_eq!(
        pw_field(archive, "wall_info.count"),
        Some(&DisplayFieldValue::I32(count))
    );
    if case.scenario == 226 {
        return;
    }
    assert_eq!(
        pw_field(archive, "wall_info[0].key").is_some(),
        case.version >= 3576
    );
    assert_eq!(
        pw_field(archive, "wall_info[0].value.mapping_fit_to_size").is_some(),
        case.version >= 3601
    );
}

fn pw_wall_count(case: OldCase) -> i32 {
    match case.scenario {
        226 => -3,
        227 | 228 => 256,
        _ => 3,
    }
}

#[test]
fn early_pw_members_follow_their_own_guards_without_synthetic_wall_keys() {
    for case in pw_cases() {
        check_pw_members(case);
    }
}

#[test]
fn distinct_view_activity_and_unknown_words_and_shared_pw_aliases_keep_their_identities() {
    for case in pw_cases() {
        let archive = parsed(case);
        check_pw_alias(&archive);
        check_pw_activity(&archive);
    }
}

fn check_pw_alias(archive: &DisplayArchive) {
    let Some(DisplayFieldValue::ObjectTag(definition)) = field(archive, PW_PATH) else {
        panic!("PW definition")
    };
    let alias = "root.views[0].view_bucket.bucket.cthread_maps[1].entries[0].key";
    let Some(DisplayFieldValue::ObjectTag(alias)) = field(archive, alias) else {
        panic!("PW alias")
    };
    let DisplayObjectPointer::New(id) = definition.target else {
        panic!("new PW object")
    };
    assert_eq!(alias.target, DisplayObjectPointer::Reference(id));
}

fn check_pw_activity(archive: &DisplayArchive) {
    assert_eq!(
        field(archive, "root.views[0].view_bucket.unknown_68"),
        Some(&DisplayFieldValue::I32(6))
    );
    assert_eq!(
        field(archive, "root.views[0].view_bucket.flags"),
        Some(&DisplayFieldValue::I32(-7))
    );
}

fn pw_edited_contents(source: &DisplayArchive) -> DisplayArchiveContents {
    let mut contents = source.contents().clone();
    archive_support::set_integer(&mut contents, &format!("{PW_PATH}.shadow_type"), -2);
    contents
}

fn check_pw_edit(case: OldCase) {
    let source = parsed(case);
    let expected = pw_selected(case, case.scenario + 1);
    let outcome = source
        .edit_with_history(pw_edited_contents(&source), LIMITS)
        .unwrap();
    assert_eq!(outcome.archive().to_bytes().unwrap(), expected.bytes);
    assert_eq!(outcome.changes().len(), 1);
    assert_eq!(
        outcome.changes()[0].path(),
        format!("{PW_PATH}.shadow_type")
    );
    let reverted = outcome
        .archive()
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    assert_eq!(reverted.to_bytes().unwrap(), case.bytes);
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn isolated_early_pw_edits_match_independent_bytes_and_authentically_revert() {
    for case in pw_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 224 | 227))
    {
        check_pw_edit(case);
    }
}

#[test]
fn impossible_pw_wall_counts_fail_before_allocating_or_publishing_an_archive() {
    for case in pw_cases().into_iter().filter(|case| case.scenario == 224) {
        let source = parsed(case);
        let mut contents = source.contents().clone();
        archive_support::set_integer(
            &mut contents,
            &format!("{PW_PATH}.wall_info.count"),
            i32::MAX,
        );
        let error = source.edit_with_history(contents, LIMITS).unwrap_err();
        assert_eq!(error.class, FailureClass::Invalid);
        assert!(error.cause.contains("wall_info.count count 2147483647"));
        assert_eq!(source.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn incomplete_pw_roots_are_rejected_at_sampled_prefixes_and_every_last_eight_bytes() {
    for case in pw_cases().into_iter().filter(|case| case.scenario == 224) {
        let cuts = (0..case.bytes.len())
            .step_by(173)
            .chain(case.bytes.len() - 8..case.bytes.len());
        for length in cuts {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..length], case_context(case), LIMITS)
                    .is_err()
            );
        }
    }
}

fn pw_document_archive(document: &bessemer::Document, kind: DisplayDocumentKind) -> DisplayArchive {
    let version =
        DisplayVersion::from_stream_names(document.streams().map(|stream| stream.name)).unwrap();
    assert_eq!(version.value(), 2771);
    assert!(
        !document
            .streams()
            .any(|stream| stream.name == "FaceTessellations/Directory")
    );
    let context = DisplayArchiveContext {
        version,
        kind,
        ..context()
    };
    let raw = document.read_stream("Contents/DisplayLists").unwrap();
    DisplayArchive::from_bytes(&raw, context, LIMITS).unwrap()
}

fn check_pw_document(kind: DisplayDocumentKind, raw: &[u8], expected_raw: &[u8]) {
    let original = bessemer::Document::from_bytes(raw).unwrap();
    let source = pw_document_archive(&original, kind);
    let expected =
        pw_document_archive(&bessemer::Document::from_bytes(expected_raw).unwrap(), kind);
    let outcome = source
        .edit_with_history(pw_edited_contents(&source), LIMITS)
        .unwrap();
    assert_eq!(
        outcome.archive().to_bytes().unwrap(),
        expected.to_bytes().unwrap()
    );
    let reopened = named_branch::replace_named_stream(&original, outcome.archive());
    check_pw_document_restore(&original, &source, &reopened, kind);
    assert_eq!(original.to_bytes().unwrap(), raw);
}

fn check_pw_document_restore(
    original: &bessemer::Document,
    source: &DisplayArchive,
    reopened: &bessemer::Document,
    kind: DisplayDocumentKind,
) {
    let result = pw_document_archive(reopened, kind);
    let reverted = result
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    let restored = named_branch::replace_named_stream(reopened, &reverted);
    assert_eq!(
        restored.read_stream("Contents/DisplayLists").unwrap(),
        source.to_bytes().unwrap()
    );
    assert_eq!(
        original.read_stream("Contents/DisplayLists").unwrap(),
        source.to_bytes().unwrap()
    );
}

#[test]
fn complete_pw_documents_resolve_their_marker_edit_reopen_and_revert_unrelated_streams() {
    check_pw_document(
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/early-pw-scene-document-part.bin"),
        include_bytes!("../fixtures/display-lists/early-pw-scene-edited-document-part.bin"),
    );
    check_pw_document(
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/early-pw-scene-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/early-pw-scene-edited-document-assembly.bin"),
    );
}

fn concurrent_pw_edit(case: OldCase) -> std::thread::JoinHandle<Vec<u8>> {
    let source = parsed(case);
    std::thread::spawn(move || {
        source
            .edit_with_history(pw_edited_contents(&source), LIMITS)
            .unwrap()
            .archive()
            .to_bytes()
            .unwrap()
    })
}

#[test]
fn parallel_pw_edits_match_independent_serial_bytes_in_all_version_contexts() {
    for case in pw_cases().into_iter().filter(|case| case.scenario == 224) {
        let expected = pw_selected(case, 225);
        let jobs: Vec<_> = (0..4).map(|_| concurrent_pw_edit(case)).collect();
        for job in jobs {
            assert_eq!(job.join().unwrap(), expected.bytes);
        }
    }
}

#[test]
fn early_pw_roots_keep_exact_field_and_object_budgets_including_shared_aliases() {
    for case in pw_cases().into_iter().filter(|case| case.scenario == 227) {
        check_pw_budget_case(case);
    }
}

fn check_pw_budget_case(case: OldCase) {
    let source = parsed(case);
    let report = source
        .edit_with_history(source.contents().clone(), LIMITS)
        .unwrap();
    let exact = DisplayArchiveLimits {
        maximum_bytes: case.bytes.len(),
        maximum_fields: report.coverage().typed_fields,
        maximum_objects: report.coverage().loaded_objects,
        ..LIMITS
    };
    assert!(DisplayArchive::from_bytes(case.bytes, case_context(case), exact).is_ok());
    check_short_pw_budgets(&source, case, exact);
}

fn check_short_pw_budgets(source: &DisplayArchive, case: OldCase, exact: DisplayArchiveLimits) {
    let short = DisplayArchiveLimits {
        maximum_objects: exact.maximum_objects - 1,
        ..exact
    };
    assert!(DisplayArchive::from_bytes(case.bytes, case_context(case), short).is_err());
    let short = DisplayArchiveLimits {
        maximum_fields: exact.maximum_fields - 1,
        ..exact
    };
    assert!(
        source
            .edit_with_history(source.contents().clone(), short)
            .is_err()
    );
}
