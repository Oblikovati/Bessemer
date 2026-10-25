//! Older pane, line and symbol layouts retain physical fields rather than restore defaults.
use super::*;

fn legacy_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-pane-roots.bin"
    ))
}

#[test]
fn complete_legacy_pane_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(legacy_cases().len(), 66);
    assert_eq!(
        legacy_cases().into_iter().map(check_case).sum::<usize>(),
        7487
    );
}

fn check_line_word(case: OldCase) {
    let archive = parsed(case);
    let value = field(
        &archive,
        "root.views[0].display_items.items[2].data.unknown_30",
    )
    .unwrap();
    let edited = i32::from(matches!(case.scenario, 1 | 4));
    let expected = if case.version <= 0xbf8 {
        DisplayFieldValue::I32(i32::MIN + edited)
    } else {
        DisplayFieldValue::U8((255 - edited) as u8)
    };
    assert_eq!(value, &expected);
}

#[test]
fn noncanonical_line_flags_keep_all_four_bytes_before_the_byte_width_boundary() {
    for case in legacy_cases() {
        check_line_word(case);
    }
}

fn check_pane_and_symbol(case: OldCase) {
    let archive = parsed(case);
    let panes = archive
        .contents()
        .fields
        .iter()
        .filter(|field| {
            field.path.starts_with("root.user_model_env.") && field.path.ends_with(".unknown_a8")
        })
        .count();
    assert_eq!(panes, if case.version > 0xbda { 3 } else { 0 });
    let prefix = "root.views[0].display_items.items[3].data";
    for member in ["is_3d_symbol", "plane_xform"] {
        assert_eq!(
            field(&archive, &format!("{prefix}.{member}")).is_some(),
            case.version > 0xbbd
        );
    }
}

#[test]
fn pane_text_and_symbol_plane_fields_are_absent_until_their_separate_gates() {
    for case in legacy_cases() {
        check_pane_and_symbol(case);
    }
}

fn opposite_legacy_version(version: u32) -> u32 {
    match version {
        0xbbd => 0xbbe,
        0xbbe => 0xbbd,
        0xbda => 0xbdb,
        0xbdb => 0xbda,
        0xbf8 => 0xbf9,
        _ => 0xbf8,
    }
}

#[test]
fn incompatible_pane_symbol_and_line_shapes_cannot_be_published_under_neighboring_versions() {
    for case in legacy_cases()
        .into_iter()
        .filter(|case| case.version != 2999)
    {
        let archive = parsed(case);
        let context = DisplayArchiveContext {
            version: DisplayVersion::new(opposite_legacy_version(case.version)),
            ..case_context(case)
        };
        assert!(DisplayArchive::new(archive.contents().clone(), context, LIMITS).is_err());
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}

fn expected_legacy_edit(case: OldCase) -> OldCase {
    let scenario = if case.scenario == 3 { 4 } else { 1 };
    legacy_cases()
        .into_iter()
        .find(|expected| {
            expected.version == case.version
                && expected.kind == case.kind
                && expected.scenario == scenario
        })
        .unwrap()
}

fn edit_line_flag(contents: &mut DisplayArchiveContents, expected: &DisplayArchive) {
    let path = "root.views[0].display_items.items[2].data.unknown_30";
    let target = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap();
    target.value = field(expected, path).unwrap().clone();
}

fn check_legacy_edit(case: OldCase) {
    let source = parsed(case);
    let expected_case = expected_legacy_edit(case);
    let expected = parsed(expected_case);
    let contents = legacy_edited_contents(&source, &expected);
    let edited = source.with_contents(contents, LIMITS).unwrap();
    assert_eq!(edited.to_bytes().unwrap(), expected_case.bytes);
    let reverted = edited
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    assert_eq!(reverted.to_bytes().unwrap(), case.bytes);
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

fn legacy_edited_contents(
    source: &DisplayArchive,
    expected: &DisplayArchive,
) -> DisplayArchiveContents {
    let mut contents = edited_contents(source, expected);
    edit_line_flag(&mut contents, expected);
    contents
}

#[test]
fn complete_cached_documents_preserve_old_line_words_through_edit_reopen_and_revert() {
    let rows = [
        (
            2999,
            include_bytes!("../fixtures/display-lists/legacy-pane-external-2999-0.SLDASM")
                .as_slice(),
            include_bytes!("../fixtures/display-lists/legacy-pane-external-2999-1.SLDASM")
                .as_slice(),
        ),
        (
            3064,
            include_bytes!("../fixtures/display-lists/legacy-pane-external-3064-0.SLDASM")
                .as_slice(),
            include_bytes!("../fixtures/display-lists/legacy-pane-external-3064-1.SLDASM")
                .as_slice(),
        ),
    ];
    for (version, source, expected) in rows {
        external_faces::documents::check_edit(version, source, expected, legacy_edited_contents);
    }
}

#[test]
fn raw_line_and_pen_edits_match_independent_bytes_and_authentically_revert() {
    for case in legacy_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3))
    {
        check_legacy_edit(case);
    }
}

#[test]
fn legacy_roots_reject_sampled_truncations_and_every_short_tail() {
    for case in legacy_cases() {
        for length in (0..case.bytes.len())
            .step_by(53)
            .chain(case.bytes.len() - 8..case.bytes.len())
        {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..length], case_context(case), LIMITS)
                    .is_err()
            );
        }
        let mut trailing = case.bytes.to_vec();
        trailing.push(0);
        assert!(DisplayArchive::from_bytes(&trailing, case_context(case), LIMITS).is_err());
    }
}
