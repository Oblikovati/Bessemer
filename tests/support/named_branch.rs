//! Named collections exercise their later gates inside complete object graphs.
use super::*;
use complete_cases::framing;
#[path = "../../examples/support/named_trace.rs"]
mod named_trace;

fn named_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/named-branch-roots.bin"
    ))
}

#[test]
fn full_named_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(named_cases().len(), 100);
    assert_eq!(
        named_cases().into_iter().map(check_case).sum::<usize>(),
        39740
    );
}

fn named_collection(archive: &DisplayArchive) -> NamedViews {
    let Some(DisplayFieldValue::NamedViews(views)) = field(archive, "root.named_view_transform")
    else {
        panic!("expected named collection")
    };
    let limits = DisplayViewLimits {
        maximum_bytes: LIMITS.maximum_bytes,
        maximum_views: LIMITS.maximum_entries,
        maximum_nested_entries: LIMITS.maximum_entries,
    };
    NamedViews::new(views.clone(), archive.context().version, limits).unwrap()
}

#[test]
fn nested_named_values_match_independent_context_bound_oracles() {
    let mut remaining: &'static [u8] =
        include_bytes!("../fixtures/display-lists/named-branch-oracles.bin");
    assert_eq!(framing::take_u32(&mut remaining).unwrap(), 100);
    let mut fields = 0;
    for case in named_cases() {
        fields += check_named_oracle(case, &mut remaining);
    }
    assert_eq!(fields, 10698);
    framing::require_finished(remaining).unwrap();
}

fn check_named_oracle(case: OldCase, remaining: &mut &'static [u8]) -> usize {
    assert_eq!(framing::take_u32(remaining).unwrap(), case.version);
    let kind = u8::from(case.kind == DisplayDocumentKind::Assembly);
    assert_eq!(framing::take(remaining, 1).unwrap()[0], kind);
    assert_eq!(framing::take(remaining, 1).unwrap()[0], case.scenario);
    let length = framing::take_u32(remaining).unwrap() as usize;
    let expected = framing::take(remaining, length).unwrap();
    assert_eq!(
        named_trace::trace(&named_collection(&parsed(case))),
        expected
    );
    expected.iter().filter(|byte| **byte == b'\n').count()
}

fn selected_named(case: OldCase, scenario: u8) -> OldCase {
    named_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version && row.kind == case.kind && row.scenario == scenario
        })
        .unwrap()
}

fn populated_named_cases() -> impl Iterator<Item = OldCase> {
    named_cases()
        .into_iter()
        .filter(|case| case.scenario == 195)
}

fn edited_named_contents(archive: &DisplayArchive) -> DisplayArchiveContents {
    let mut contents = archive.contents().clone();
    let named = contents
        .fields
        .iter_mut()
        .find(|value| value.path.as_ref() == "root.named_view_transform")
        .unwrap();
    let DisplayFieldValue::NamedViews(Some(views)) = &mut named.value else {
        panic!("expected populated named collection")
    };
    let plane = &mut views[0].section_planes.as_mut().unwrap()[0];
    assert_eq!(plane.unknown_i0, -17);
    plane.unknown_i0 = 23;
    contents
}

#[test]
fn isolated_plane_edits_match_independent_wire_and_revert_without_source_mutation() {
    for case in populated_named_cases() {
        let expected = selected_named(case, 196);
        let source = parsed(case);
        let edited = source
            .with_contents(edited_named_contents(&source), LIMITS)
            .unwrap();
        assert_eq!(edited.to_bytes().unwrap(), expected.bytes);
        let reverted = edited
            .with_contents(source.contents().clone(), LIMITS)
            .unwrap();
        assert_eq!(reverted.to_bytes().unwrap(), case.bytes);
        assert_eq!(source.to_bytes().unwrap(), case.bytes);
    }
}

fn replace_named_stream(
    original: &bessemer::Document,
    archive: &DisplayArchive,
) -> bessemer::Document {
    let mut result = original.clone();
    result
        .replace_stream("Contents/DisplayLists", archive.to_bytes().unwrap())
        .unwrap();
    let reopened = bessemer::Document::from_bytes(&result.to_bytes().unwrap()).unwrap();
    for stream in original
        .streams()
        .filter(|stream| stream.name != "Contents/DisplayLists")
    {
        assert_eq!(
            reopened.read_stream(stream.name).unwrap(),
            original.read_stream(stream.name).unwrap()
        );
    }
    reopened
}

fn check_named_document(raw: &[u8], expected_raw: &[u8], kind: DisplayDocumentKind) {
    let original = bessemer::Document::from_bytes(raw).unwrap();
    let source = archive_support::decode_document(raw, kind);
    let expected = archive_support::decode_document(expected_raw, kind);
    let outcome = source
        .edit_with_history(edited_named_contents(&source), LIMITS)
        .unwrap();
    assert_eq!(
        outcome.archive().to_bytes().unwrap(),
        expected.to_bytes().unwrap()
    );
    let reopened = replace_named_stream(&original, outcome.archive());
    let result = archive_support::decode_document(&reopened.to_bytes().unwrap(), kind);
    assert_eq!(result.to_bytes().unwrap(), expected.to_bytes().unwrap());
    check_named_restore(&original, &source, &reopened, &result);
}

fn check_named_restore(
    original: &bessemer::Document,
    source: &DisplayArchive,
    reopened: &bessemer::Document,
    result: &DisplayArchive,
) {
    let reverted = result
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    let restored = replace_named_stream(reopened, &reverted);
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
fn whole_named_documents_edit_reopen_and_restore_unrelated_streams() {
    check_named_document(
        include_bytes!("../fixtures/display-lists/named-branch-document-part.bin"),
        include_bytes!("../fixtures/display-lists/named-branch-edited-document-part.bin"),
        DisplayDocumentKind::Part,
    );
    check_named_document(
        include_bytes!("../fixtures/display-lists/named-branch-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/named-branch-edited-document-assembly.bin"),
        DisplayDocumentKind::Assembly,
    );
}

#[test]
fn truncated_named_roots_fail_before_publication() {
    for case in populated_named_cases() {
        let cuts = (0..case.bytes.len())
            .step_by(173)
            .chain(case.bytes.len().saturating_sub(8)..case.bytes.len());
        for length in cuts {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..length], case_context(case), LIMITS)
                    .is_err(),
                "{} {:?} prefix {length}",
                case.version,
                case.kind
            );
        }
    }
}

#[test]
fn named_collections_preserve_every_other_field_and_object_identity() {
    for case in populated_named_cases() {
        let control = selected_named(case, 192);
        let source = parsed(case);
        let baseline = parsed(control);
        assert_eq!(other_named_fields(&source), other_named_fields(&baseline));
    }
}

fn other_named_fields(archive: &DisplayArchive) -> Vec<DisplayField> {
    archive
        .contents()
        .fields
        .iter()
        .filter(|value| value.path.as_ref() != "root.named_view_transform")
        .cloned()
        .collect()
}

#[test]
fn named_plane_history_retains_independent_wire_and_revision_bound_parents() {
    for case in populated_named_cases() {
        let source = parsed(case);
        let expected = selected_named(case, 196);
        let outcome = source
            .edit_with_history(edited_named_contents(&source), LIMITS)
            .unwrap();
        assert_eq!(outcome.archive().to_bytes().unwrap(), expected.bytes);
        assert_eq!(outcome.changes().len(), 1);
        let change = &outcome.changes()[0];
        assert_eq!(change.path(), "root.named_view_transform");
        assert_eq!(change.relation(), DisplayFieldRelation::Derived);
        let parent = source.resolve_field(&change.parents()[0]).unwrap();
        assert_eq!(Some(&parent.value), field(&source, change.path()));
        let result = outcome
            .archive()
            .resolve_field(change.result().unwrap())
            .unwrap();
        assert_eq!(Some(&result.value), field(&parsed(expected), change.path()));
    }
}
