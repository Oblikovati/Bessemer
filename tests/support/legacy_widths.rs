//! Implicit widths count Unicode characters; later layouts retain their stored count.
use super::*;

fn width_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-width-roots.bin"
    ))
}

#[test]
fn complete_width_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(width_cases().len(), 34);
    assert_eq!(
        width_cases().into_iter().map(check_case).sum::<usize>(),
        4912
    );
}

fn width_word_count(archive: &DisplayArchive, index: usize) -> usize {
    let prefix = format!("root.views[0].display_items.items[{index}].data.unknown_30[");
    archive
        .contents()
        .fields
        .iter()
        .filter(|field| field.path.starts_with(&prefix))
        .count()
}

fn check_width_counts(case: OldCase) {
    let archive = parsed(case);
    let explicit = case.version > 0xaeb;
    for (index, expected) in [
        (4, if explicit { 3 } else { 5 }),
        (5, 0),
        (6, 0),
        (7, 5),
        (8, usize::from(!explicit)),
    ] {
        assert_eq!(width_word_count(&archive, index), expected);
        let path = format!("root.views[0].display_items.items[{index}].data.width_count");
        assert_eq!(field(&archive, &path).is_some(), explicit && index != 6);
    }
}

#[test]
fn astral_combining_empty_and_disabled_text_widths_follow_their_physical_count_layout() {
    for case in width_cases() {
        check_width_counts(case);
    }
}

fn width_edited_contents(
    source: &DisplayArchive,
    expected: &DisplayArchive,
) -> DisplayArchiveContents {
    let mut contents = legacy_panes::legacy_edited_contents(source, expected);
    for target in &mut contents.fields {
        if matches!(
            target.path.as_ref(),
            "root.views[0].display_items.items[4].data.text"
                | "root.views[0].display_items.items[4].data.unknown_30[0]"
        ) {
            target.value = field(expected, &target.path).unwrap().clone();
        }
    }
    contents
}

fn expected_width_edit(case: OldCase) -> OldCase {
    width_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

fn check_width_edit(case: OldCase) {
    let source = parsed(case);
    let expected_case = expected_width_edit(case);
    let expected = parsed(expected_case);
    let edited = source
        .with_contents(width_edited_contents(&source, &expected), LIMITS)
        .unwrap();
    assert_eq!(edited.to_bytes().unwrap(), expected_case.bytes);
    let reverted = edited
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    assert_eq!(reverted.to_bytes().unwrap(), case.bytes);
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn text_and_width_edits_match_independent_bytes_and_authentically_revert() {
    for case in width_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3))
    {
        check_width_edit(case);
    }
}

fn check_text_length_change(case: OldCase) {
    let source = parsed(case);
    let mut contents = source.contents().clone();
    let empty = field(&source, "root.views[0].display_items.items[5].data.text").unwrap();
    let target = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.views[0].display_items.items[4].data.text")
        .unwrap();
    target.value = empty.clone();
    assert_eq!(
        source.with_contents(contents, LIMITS).is_ok(),
        case.version > 0xaeb
    );
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn text_length_changes_require_matching_implicit_widths_but_keep_independent_explicit_counts() {
    for case in width_cases().into_iter().filter(|case| case.scenario == 0) {
        check_text_length_change(case);
    }
}

#[test]
fn implicit_and_explicit_width_shapes_cannot_be_published_under_each_others_versions() {
    for case in width_cases()
        .into_iter()
        .filter(|case| case.version != 2771)
    {
        let source = parsed(case);
        let version = if case.version == 2795 { 2796 } else { 2795 };
        let context = DisplayArchiveContext {
            version: DisplayVersion::new(version),
            ..case_context(case)
        };
        assert!(DisplayArchive::new(source.contents().clone(), context, LIMITS).is_err());
        assert_eq!(source.to_bytes().unwrap(), case.bytes);
    }
}

fn check_invalid_width_counts(case: OldCase) {
    let source = parsed(case);
    for count in [-1, 16777217] {
        let mut contents = source.contents().clone();
        archive_support::set_integer(
            &mut contents,
            "root.views[0].display_items.items[4].data.width_count",
            count,
        );
        let failure = source.with_contents(contents, LIMITS).unwrap_err();
        assert_eq!(failure.class, FailureClass::Invalid);
        assert!(
            failure
                .cause
                .contains(&format!("width count {count}; expected 0..=16777216"))
        );
    }
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn negative_and_excessive_explicit_counts_fail_with_the_stored_value_and_expected_range() {
    for case in width_cases()
        .into_iter()
        .filter(|case| case.version == 2796 && case.scenario == 0)
    {
        check_invalid_width_counts(case);
    }
}

const WIDTH_DOCUMENTS: [(u32, &[u8], &[u8]); 2] = [
    (
        2771,
        include_bytes!("../fixtures/display-lists/legacy-width-external-2771-0.SLDASM"),
        include_bytes!("../fixtures/display-lists/legacy-width-external-2771-1.SLDASM"),
    ),
    (
        2795,
        include_bytes!("../fixtures/display-lists/legacy-width-external-2795-0.SLDASM"),
        include_bytes!("../fixtures/display-lists/legacy-width-external-2795-1.SLDASM"),
    ),
];

#[test]
fn cached_width_documents_keep_all_faces_through_text_edits_reopen_and_revert() {
    for (version, source, expected) in WIDTH_DOCUMENTS {
        external_faces::documents::check_edit(version, source, expected, width_edited_contents);
    }
}

#[test]
fn incomplete_implicit_and_explicit_width_roots_never_publish_partial_fields() {
    for case in width_cases() {
        for length in (0..case.bytes.len())
            .step_by(83)
            .chain(case.bytes.len() - 8..case.bytes.len())
        {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..length], case_context(case), LIMITS)
                    .is_err()
            );
        }
    }
}
