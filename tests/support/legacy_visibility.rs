//! Bucket line properties preserve visibility only where a word was stored.
use super::*;

fn visibility_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-visibility-roots.bin"
    ))
}

#[test]
fn complete_visibility_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(visibility_cases().len(), 150);
    assert_eq!(
        visibility_cases()
            .into_iter()
            .map(check_case)
            .sum::<usize>(),
        30175
    );
}

fn check_visibility_fields(case: OldCase) {
    let source = parsed(case);
    let values: Vec<_> = source
        .contents()
        .fields
        .iter()
        .filter(|field| field.path.ends_with(".user_line_property.visibility"))
        .collect();
    let present = case.version >= 0xa80 && case.scenario >= 32 && case.scenario % 16 != 2;
    assert_eq!(values.len(), if present { 3 } else { 0 });
    let edited = i32::from(case.scenario == 4 || case.scenario % 16 == 1);
    for value in values {
        assert_eq!(value.value, DisplayFieldValue::I32(i32::MIN + edited));
    }
}

#[test]
fn line_visibility_words_are_absent_before_the_gate_and_keep_all_signed_bits_afterward() {
    for case in visibility_cases() {
        check_visibility_fields(case);
    }
}

pub(super) fn visibility_edited_contents(
    source: &DisplayArchive,
    expected: &DisplayArchive,
) -> DisplayArchiveContents {
    let mut contents = legacy_modes::mode_edited_contents(source, expected);
    for target in &mut contents.fields {
        if target.path.ends_with(".user_line_property.visibility") {
            target.value = field(expected, &target.path).unwrap().clone();
        }
    }
    contents
}

fn expected_visibility_edit(case: OldCase) -> OldCase {
    visibility_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

fn check_visibility_edit(case: OldCase) {
    let source = parsed(case);
    let expected_case = expected_visibility_edit(case);
    let expected = parsed(expected_case);
    let edited = source
        .with_contents(visibility_edited_contents(&source, &expected), LIMITS)
        .unwrap();
    assert_eq!(edited.to_bytes().unwrap(), expected_case.bytes);
    let reverted = edited
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    assert_eq!(reverted.to_bytes().unwrap(), case.bytes);
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn visibility_polygon_and_text_edits_match_independent_bytes_and_authentically_revert() {
    for case in visibility_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3 | 32 | 48 | 64 | 80))
    {
        check_visibility_edit(case);
    }
}

fn check_visibility_boundary(case: OldCase) {
    let source = parsed(case);
    let version = if case.version == 2687 { 2688 } else { 2687 };
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(version),
        ..case_context(case)
    };
    let result = DisplayArchive::new(source.contents().clone(), context, LIMITS);
    assert_eq!(
        result.is_err(),
        case.scenario >= 32 && case.scenario % 16 != 2
    );
    if let Ok(rebuilt) = result {
        assert_eq!(rebuilt.to_bytes().unwrap(), case.bytes);
    }
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn visibility_shapes_change_only_records_that_have_line_properties() {
    for case in visibility_cases()
        .into_iter()
        .filter(|case| matches!(case.version, 2687 | 2688))
    {
        check_visibility_boundary(case);
    }
}

#[test]
fn holder_name_boundary_does_not_change_reachable_archive_layouts() {
    for case in visibility_cases()
        .into_iter()
        .filter(|case| matches!(case.version, 2710 | 2711))
    {
        let source = parsed(case);
        let version = if case.version == 2710 { 2711 } else { 2710 };
        let context = DisplayArchiveContext {
            version: DisplayVersion::new(version),
            ..case_context(case)
        };
        let rebuilt = DisplayArchive::new(source.contents().clone(), context, LIMITS).unwrap();
        assert_eq!(rebuilt.to_bytes().unwrap(), case.bytes);
    }
}

fn check_holder_role_rejection(case: OldCase) {
    let source = parsed(case);
    let mut contents = source.contents().clone();
    let target = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.views[0].view_bucket")
        .unwrap();
    let DisplayFieldValue::ObjectTag(tag) = &mut target.value else {
        panic!("expected bucket object tag");
    };
    tag.runtime_class.as_mut().unwrap().name = "moComponentPropertyHolder".into();
    tag.new_class.as_mut().unwrap().name = "moComponentPropertyHolder".into();
    let failure = source.with_contents(contents, LIMITS).unwrap_err();
    assert!(failure.cause.contains("moComponentPropertyHolder"));
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn component_holders_cannot_replace_typed_bucket_entry_points_in_the_lower_interval() {
    for case in visibility_cases()
        .into_iter()
        .filter(|case| case.version == 2710 && case.scenario == 32)
    {
        check_holder_role_rejection(case);
    }
}

const VISIBILITY_DOCUMENTS: [(u32, &[u8], &[u8]); 2] = [
    (
        2650,
        include_bytes!("../fixtures/display-lists/legacy-visibility-external-2650-0.SLDASM"),
        include_bytes!("../fixtures/display-lists/legacy-visibility-external-2650-1.SLDASM"),
    ),
    (
        2688,
        include_bytes!("../fixtures/display-lists/legacy-visibility-external-2688-0.SLDASM"),
        include_bytes!("../fixtures/display-lists/legacy-visibility-external-2688-1.SLDASM"),
    ),
];

#[test]
fn visibility_documents_keep_all_faces_through_raw_flag_edits_reopen_and_revert() {
    for (version, source, expected) in VISIBILITY_DOCUMENTS {
        external_faces::documents::check_edit(
            version,
            source,
            expected,
            visibility_edited_contents,
        );
    }
}

#[test]
fn incomplete_line_visibility_roots_never_publish_partial_fields() {
    for case in visibility_cases() {
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
