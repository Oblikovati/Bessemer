//! Viewer modes distinguish absent, one-word and three-word physical layouts.
use super::*;

fn mode_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-mode-roots.bin"
    ))
}

#[test]
fn complete_mode_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(mode_cases().len(), 94);
    assert_eq!(
        mode_cases().into_iter().map(check_case).sum::<usize>(),
        18823
    );
}

fn expected_mode_words(case: OldCase) -> usize {
    let shaded = (case.scenario < 32 && case.scenario != 2) || case.scenario >= 64;
    if case.kind != DisplayDocumentKind::Assembly || !shaded || case.version <= 0xaa4 {
        return 0;
    }
    if matches!(case.scenario, 3 | 4) || case.scenario >= 64 {
        3
    } else {
        1
    }
}

fn expected_mode_values(case: OldCase) -> Vec<DisplayFieldValue> {
    let edited = i32::from(case.scenario == 4 || case.scenario % 16 == 1);
    match expected_mode_words(case) {
        0 => Vec::new(),
        1 => vec![DisplayFieldValue::I32(6)],
        _ => vec![
            DisplayFieldValue::I32(i32::MIN + edited),
            DisplayFieldValue::I32(i32::MAX),
            DisplayFieldValue::I32(i32::MIN),
        ],
    }
}

fn check_mode_fields(case: OldCase) {
    let source = parsed(case);
    let words: Vec<_> = source
        .contents()
        .fields
        .iter()
        .filter(|field| field.path.contains(".viewer_comp_data.display_mode_data["))
        .map(|field| field.value.clone())
        .collect();
    assert_eq!(words, expected_mode_values(case));
}

#[test]
fn viewer_modes_keep_their_absence_sentinel_and_full_width_payload_shapes() {
    for case in mode_cases() {
        check_mode_fields(case);
    }
}

pub(super) fn mode_edited_contents(
    source: &DisplayArchive,
    expected: &DisplayArchive,
) -> DisplayArchiveContents {
    let mut contents = legacy_regions::region_edited_contents(source, expected);
    for target in &mut contents.fields {
        if target.path.contains(".viewer_comp_data.display_mode_data[") {
            target.value = field(expected, &target.path).unwrap().clone();
        }
    }
    contents
}

fn expected_mode_edit(case: OldCase) -> OldCase {
    mode_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

fn check_mode_edit(case: OldCase) {
    let source = parsed(case);
    let expected_case = expected_mode_edit(case);
    let expected = parsed(expected_case);
    let edited = source
        .with_contents(mode_edited_contents(&source, &expected), LIMITS)
        .unwrap();
    assert_eq!(edited.to_bytes().unwrap(), expected_case.bytes);
    let reverted = edited
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    assert_eq!(reverted.to_bytes().unwrap(), case.bytes);
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn mode_polygon_and_text_edits_match_independent_bytes_and_authentically_revert() {
    for case in mode_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3 | 32 | 48 | 64 | 80))
    {
        check_mode_edit(case);
    }
}

fn check_mode_boundary(case: OldCase) {
    let source = parsed(case);
    let version = if case.version == 2724 { 2725 } else { 2724 };
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(version),
        ..case_context(case)
    };
    let changed = expected_mode_words(case) != expected_mode_words(OldCase { version, ..case });
    let result = DisplayArchive::new(source.contents().clone(), context, LIMITS);
    assert_eq!(result.is_err(), changed);
    if let Ok(rebuilt) = result {
        assert_eq!(rebuilt.to_bytes().unwrap(), case.bytes);
    }
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn mode_boundaries_change_only_shaded_assemblies_even_when_component_parts_are_null() {
    for case in mode_cases().into_iter().filter(|case| case.version != 2711) {
        check_mode_boundary(case);
    }
}

fn check_invalid_mode_shape(case: OldCase) {
    let source = parsed(case);
    let mut contents = source.contents().clone();
    let value = if expected_mode_words(case) == 1 {
        i32::MIN
    } else {
        6
    };
    archive_support::set_integer(
        &mut contents,
        "root.tessellation.components[0].viewer_comp_data.display_mode_data[0]",
        value,
    );
    assert!(source.with_contents(contents, LIMITS).is_err());
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn changing_the_mode_sentinel_requires_adding_or_removing_both_extra_words() {
    for case in mode_cases().into_iter().filter(|case| {
        case.version == 2725
            && case.kind == DisplayDocumentKind::Assembly
            && matches!(case.scenario, 0 | 3)
    }) {
        check_invalid_mode_shape(case);
    }
}

fn mode_field_range(contents: &DisplayArchiveContents, prefix: &str) -> std::ops::Range<usize> {
    let start = contents
        .fields
        .iter()
        .position(|field| field.path.starts_with(prefix))
        .unwrap();
    let count = contents.fields[start..]
        .iter()
        .take_while(|field| field.path.starts_with(prefix))
        .count();
    start..start + count
}

fn mode_shape_contents(
    source: &DisplayArchive,
    expected: &DisplayArchive,
) -> DisplayArchiveContents {
    let mut contents = source.contents().clone();
    let prefix = "root.tessellation.components[0].viewer_comp_data.display_mode_data[";
    let range = mode_field_range(&contents, prefix);
    let replacement = expected
        .contents()
        .fields
        .iter()
        .filter(|field| field.path.starts_with(prefix))
        .cloned();
    contents.fields.splice(range, replacement);
    contents
}

fn check_shape_conversion(source: &DisplayArchive, expected: &DisplayArchive) {
    let edited = source
        .with_contents(mode_shape_contents(source, expected), LIMITS)
        .unwrap();
    assert_eq!(edited.to_bytes().unwrap(), expected.to_bytes().unwrap());
    let reverted = edited
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    assert_eq!(reverted.to_bytes().unwrap(), source.to_bytes().unwrap());
}

#[test]
fn complete_single_and_three_word_mode_edits_match_frozen_independent_archives() {
    let selected = |case: &OldCase| {
        case.version == 2725 && case.kind == DisplayDocumentKind::Assembly && case.scenario == 3
    };
    let short = parsed(
        legacy_regions::region_cases()
            .into_iter()
            .find(selected)
            .unwrap(),
    );
    let long = parsed(mode_cases().into_iter().find(selected).unwrap());
    check_shape_conversion(&short, &long);
    check_shape_conversion(&long, &short);
}

const MODE_DOCUMENTS: [(u32, &[u8], &[u8]); 2] = [
    (
        2711,
        include_bytes!("../fixtures/display-lists/legacy-mode-external-2711-0.SLDASM"),
        include_bytes!("../fixtures/display-lists/legacy-mode-external-2711-1.SLDASM"),
    ),
    (
        2725,
        include_bytes!("../fixtures/display-lists/legacy-mode-external-2725-0.SLDASM"),
        include_bytes!("../fixtures/display-lists/legacy-mode-external-2725-1.SLDASM"),
    ),
];

#[test]
fn populated_mode_documents_keep_all_faces_through_mode_edits_reopen_and_revert() {
    for (version, source, expected) in MODE_DOCUMENTS {
        external_faces::documents::check_edit(version, source, expected, mode_edited_contents);
    }
}

#[test]
fn incomplete_component_mode_roots_never_publish_partial_fields() {
    for case in mode_cases() {
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
