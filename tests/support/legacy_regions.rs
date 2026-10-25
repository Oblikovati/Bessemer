//! Region flags retain their physical word only after the documented boundary.
use super::*;

fn region_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-region-roots.bin"
    ))
}

#[test]
fn complete_region_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(region_cases().len(), 94);
    assert_eq!(
        region_cases().into_iter().map(check_case).sum::<usize>(),
        18896
    );
}

fn check_region_fields(case: OldCase) {
    let source = parsed(case);
    let words: Vec<_> = source
        .contents()
        .fields
        .iter()
        .filter(|field| field.path.ends_with(".new_region"))
        .collect();
    let present = case.version > 0xabc && case.scenario >= 32 && case.scenario % 16 != 2;
    assert_eq!(words.len(), if present { 4 } else { 0 });
    let expected = if case.scenario % 16 == 1 { 0 } else { i32::MIN };
    for word in words {
        assert_eq!(word.value, DisplayFieldValue::I32(expected));
    }
}

#[test]
fn region_words_are_absent_until_their_gate_and_keep_noncanonical_signed_values() {
    for case in region_cases() {
        check_region_fields(case);
    }
}

fn region_edited_contents(
    source: &DisplayArchive,
    expected: &DisplayArchive,
) -> DisplayArchiveContents {
    let mut contents = legacy_threads::thread_edited_contents(source, expected);
    for target in &mut contents.fields {
        if target.path.ends_with(".new_region") {
            target.value = field(expected, &target.path).unwrap().clone();
        }
    }
    contents
}

fn expected_region_edit(case: OldCase) -> OldCase {
    region_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

fn check_region_edit(case: OldCase) {
    let source = parsed(case);
    let expected_case = expected_region_edit(case);
    let expected = parsed(expected_case);
    let edited = source
        .with_contents(region_edited_contents(&source, &expected), LIMITS)
        .unwrap();
    assert_eq!(edited.to_bytes().unwrap(), expected_case.bytes);
    let reverted = edited
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    assert_eq!(reverted.to_bytes().unwrap(), case.bytes);
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn region_polygon_and_text_edits_match_independent_bytes_and_authentically_revert() {
    for case in region_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3 | 32 | 48 | 64 | 80))
    {
        check_region_edit(case);
    }
}

fn check_region_boundary(case: OldCase) {
    let source = parsed(case);
    let version = if case.version == 2748 { 2749 } else { 2748 };
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
fn region_shapes_change_only_positive_polygons_while_empty_buckets_keep_their_layout() {
    for case in region_cases()
        .into_iter()
        .filter(|case| case.version != 2725)
    {
        check_region_boundary(case);
    }
}

const REGION_DOCUMENTS: [(u32, &[u8], &[u8]); 2] = [
    (
        2725,
        include_bytes!("../fixtures/display-lists/legacy-region-external-2725-0.SLDASM"),
        include_bytes!("../fixtures/display-lists/legacy-region-external-2725-1.SLDASM"),
    ),
    (
        2749,
        include_bytes!("../fixtures/display-lists/legacy-region-external-2749-0.SLDASM"),
        include_bytes!("../fixtures/display-lists/legacy-region-external-2749-1.SLDASM"),
    ),
];

#[test]
fn populated_region_documents_keep_all_faces_through_raw_flag_edits_reopen_and_revert() {
    for (version, source, expected) in REGION_DOCUMENTS {
        external_faces::documents::check_edit(version, source, expected, region_edited_contents);
    }
}

#[test]
fn incomplete_region_bucket_roots_never_publish_partial_fields() {
    for case in region_cases() {
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

fn concurrent_region_case(case: OldCase) {
    let source = parsed(case);
    std::thread::scope(|scope| {
        for _ in 0..4 {
            let archive = &source;
            scope.spawn(move || {
                let rebuilt =
                    DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS)
                        .unwrap();
                assert_eq!(rebuilt.to_bytes().unwrap(), case.bytes);
            });
        }
    });
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn shaded_region_graphs_are_deterministic_across_concurrent_constructors() {
    for case in region_cases().into_iter().filter(|case| {
        case.kind == DisplayDocumentKind::Assembly && case.scenario == 80 && case.version != 2725
    }) {
        concurrent_region_case(case);
    }
}
