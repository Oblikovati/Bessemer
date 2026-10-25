//! Appearance names and lightweight-rendering state have different first versions.
use super::*;

fn visual_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-visual-roots.bin"
    ))
}

#[test]
fn complete_legacy_appearance_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(visual_cases().len(), 50);
    assert_eq!(
        visual_cases().into_iter().map(check_case).sum::<usize>(),
        5260
    );
}

fn check_visual_fields(case: OldCase) {
    let archive = parsed(case);
    for (member, threshold) in [
        ("use_in_sw_lw_rendering", 0xb53),
        ("appearance_name", 0xb55),
    ] {
        let path = format!("root.visual_properties.{member}");
        assert_eq!(
            field(&archive, &path).is_some(),
            case.scenario != 2 && case.version > threshold
        );
    }
}

#[test]
fn appearance_names_and_lightweight_flags_follow_their_separate_version_gates() {
    for case in visual_cases() {
        check_visual_fields(case);
    }
}

fn visual_edited_contents(
    source: &DisplayArchive,
    expected: &DisplayArchive,
) -> DisplayArchiveContents {
    let mut contents = legacy_panes::legacy_edited_contents(source, expected);
    for target in &mut contents.fields {
        if target.path.as_ref() == "root.visual_properties.use_in_sw_lw_rendering"
            || target.path.as_ref() == "root.visual_properties.appearance_name"
        {
            target.value = field(expected, &target.path).unwrap().clone();
        }
    }
    contents
}

fn edited_case(case: OldCase) -> OldCase {
    visual_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

fn check_visual_edit(case: OldCase) {
    let source = parsed(case);
    let expected_case = edited_case(case);
    let expected = parsed(expected_case);
    let edited = source
        .with_contents(visual_edited_contents(&source, &expected), LIMITS)
        .unwrap();
    assert_eq!(edited.to_bytes().unwrap(), expected_case.bytes);
    let reverted = edited
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    assert_eq!(reverted.to_bytes().unwrap(), case.bytes);
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn noncanonical_appearance_edits_match_independent_bytes_and_authentically_revert() {
    for case in visual_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3))
    {
        check_visual_edit(case);
    }
}

fn check_visual_boundary(case: OldCase) {
    let version = match case.version {
        2899 => 2900,
        2900 => 2899,
        2901 => 2902,
        _ => 2901,
    };
    let source = parsed(case);
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(version),
        ..case_context(case)
    };
    assert!(DisplayArchive::new(source.contents().clone(), context, LIMITS).is_err());
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn incompatible_appearance_shapes_fail_without_changing_the_source() {
    for case in visual_cases()
        .into_iter()
        .filter(|case| case.version != 2891 && case.scenario != 2)
    {
        check_visual_boundary(case);
    }
}

fn cached_documents() -> [(u32, &'static [u8], &'static [u8]); 2] {
    [
        (
            2891,
            include_bytes!("../fixtures/display-lists/legacy-visual-external-2891-0.SLDASM"),
            include_bytes!("../fixtures/display-lists/legacy-visual-external-2891-1.SLDASM"),
        ),
        (
            2901,
            include_bytes!("../fixtures/display-lists/legacy-visual-external-2901-0.SLDASM"),
            include_bytes!("../fixtures/display-lists/legacy-visual-external-2901-1.SLDASM"),
        ),
    ]
}

#[test]
fn cached_appearance_documents_keep_complete_faces_through_edit_reopen_and_revert() {
    for (version, source, expected) in cached_documents() {
        external_faces::documents::check_edit(version, source, expected, visual_edited_contents);
    }
}

#[test]
fn incomplete_appearance_roots_never_publish_partial_fields() {
    for case in visual_cases() {
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
