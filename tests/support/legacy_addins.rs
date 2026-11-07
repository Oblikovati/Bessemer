//! Add-in identity and body appearance pointers have independent storage boundaries.
use super::*;

fn addin_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-addin-roots.bin"
    ))
}

#[test]
fn complete_addin_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(addin_cases().len(), 50);
    assert_eq!(
        addin_cases().into_iter().map(check_case).sum::<usize>(),
        5192
    );
}

fn check_addin_fields(case: OldCase) {
    let archive = parsed(case);
    for name in ["addin_identifier", "valid_only_for_addin"] {
        let path = format!("root.visual_properties.{name}");
        assert_eq!(
            field(&archive, &path).is_some(),
            case.scenario != 2 && case.version > 0xb2b
        );
    }
    assert_eq!(
        body_property_count(&archive),
        usize::from(matches!(case.scenario, 3 | 4) && case.version >= 0xb4b)
    );
}

fn body_property_count(archive: &DisplayArchive) -> usize {
    archive
        .contents()
        .fields
        .iter()
        .filter(|field| field.path.ends_with(".body_prop_infos[0].value.viz_prop"))
        .count()
}

#[test]
fn addin_fields_and_single_body_pointers_follow_their_exact_presence_gates() {
    for case in addin_cases() {
        check_addin_fields(case);
    }
}

fn addin_edited_contents(
    source: &DisplayArchive,
    expected: &DisplayArchive,
) -> DisplayArchiveContents {
    let mut contents = legacy_panes::legacy_edited_contents(source, expected);
    for target in &mut contents.fields {
        if matches!(
            target.path.as_ref(),
            "root.visual_properties.addin_identifier"
                | "root.visual_properties.valid_only_for_addin"
        ) {
            target.value = field(expected, &target.path).unwrap().clone();
        }
    }
    contents
}

fn expected_addin_edit(case: OldCase) -> OldCase {
    addin_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

fn check_addin_edit(case: OldCase) {
    let source = parsed(case);
    let expected_case = expected_addin_edit(case);
    let expected = parsed(expected_case);
    let edited = source
        .with_contents(addin_edited_contents(&source, &expected), LIMITS)
        .unwrap();
    assert_eq!(edited.to_bytes().unwrap(), expected_case.bytes);
    let reverted = edited
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    assert_eq!(reverted.to_bytes().unwrap(), case.bytes);
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn signed_addin_flags_and_unicode_identifiers_edit_and_revert_without_changing_inputs() {
    for case in addin_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3))
    {
        check_addin_edit(case);
    }
}

fn opposite_addin_version(version: u32) -> u32 {
    match version {
        2859 => 2860,
        2860 => 2859,
        2890 => 2891,
        _ => 2890,
    }
}

fn check_addin_boundary(case: OldCase) {
    let source = parsed(case);
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(opposite_addin_version(case.version)),
        ..case_context(case)
    };
    let changed = if case.version < 2890 {
        case.scenario != 2
    } else {
        matches!(case.scenario, 3 | 4)
    };
    let result = DisplayArchive::new(source.contents().clone(), context, LIMITS);
    assert_eq!(result.is_err(), changed);
    if let Ok(rebuilt) = result {
        assert_eq!(rebuilt.to_bytes().unwrap(), case.bytes);
    }
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn body_pointer_boundaries_change_only_bodies_and_addin_boundaries_only_shaded_roots() {
    for case in addin_cases()
        .into_iter()
        .filter(|case| case.version != 2796)
    {
        check_addin_boundary(case);
    }
}

const ADDIN_DOCUMENTS: [(u32, &[u8], &[u8]); 2] = [
    (
        2796,
        include_bytes!("../fixtures/display-lists/legacy-addin-external-2796-0.SLDASM"),
        include_bytes!("../fixtures/display-lists/legacy-addin-external-2796-1.SLDASM"),
    ),
    (
        2890,
        include_bytes!("../fixtures/display-lists/legacy-addin-external-2890-0.SLDASM"),
        include_bytes!("../fixtures/display-lists/legacy-addin-external-2890-1.SLDASM"),
    ),
];

#[test]
fn cached_addin_documents_keep_all_faces_through_edits_reopen_and_revert() {
    for (version, source, expected) in ADDIN_DOCUMENTS {
        external_faces::documents::check_edit(version, source, expected, addin_edited_contents);
    }
}

#[test]
fn incomplete_addin_and_body_roots_never_publish_partial_fields() {
    for case in addin_cases() {
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
