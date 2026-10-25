//! Older buckets omit thread maps and per-polygon pattern-transform IDs together.
use super::*;

fn thread_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-thread-roots.bin"
    ))
}

#[test]
fn complete_thread_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(thread_cases().len(), 94);
    assert_eq!(
        thread_cases().into_iter().map(check_case).sum::<usize>(),
        19392
    );
}

fn check_thread_fields(case: OldCase) {
    let archive = parsed(case);
    let fields = &archive.contents().fields;
    let maps = fields
        .iter()
        .any(|field| field.path.contains(".cthread_maps["));
    assert_eq!(maps, case.scenario >= 32 && case.version > 0xad2);
    let patterns = fields
        .iter()
        .filter(|field| field.path.ends_with(".thread_ptn_xform_id"))
        .count();
    let expected = if maps && case.scenario % 16 != 2 {
        5
    } else {
        0
    };
    assert_eq!(patterns, expected);
}

#[test]
fn thread_maps_and_polygon_transform_ids_follow_the_exact_storage_boundary() {
    for case in thread_cases() {
        check_thread_fields(case);
    }
}

fn thread_edited_contents(
    source: &DisplayArchive,
    expected: &DisplayArchive,
) -> DisplayArchiveContents {
    let mut contents = legacy_widths::width_edited_contents(source, expected);
    let path = "root.views[0].view_bucket.bucket.silh.items[0].smooth";
    if let Some(target) = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
    {
        target.value = field(expected, path).unwrap().clone();
    }
    contents
}

fn expected_thread_edit(case: OldCase) -> OldCase {
    thread_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

fn check_thread_edit(case: OldCase) {
    let source = parsed(case);
    let expected_case = expected_thread_edit(case);
    let expected = parsed(expected_case);
    let edited = source
        .with_contents(thread_edited_contents(&source, &expected), LIMITS)
        .unwrap();
    assert_eq!(edited.to_bytes().unwrap(), expected_case.bytes);
    let reverted = edited
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    assert_eq!(reverted.to_bytes().unwrap(), case.bytes);
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn polygon_and_text_edits_match_independent_bytes_and_authentically_revert() {
    for case in thread_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3 | 32 | 48 | 64 | 80))
    {
        check_thread_edit(case);
    }
}

fn check_thread_boundary(case: OldCase) {
    let source = parsed(case);
    let version = if case.version == 2770 { 2771 } else { 2770 };
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(version),
        ..case_context(case)
    };
    let result = DisplayArchive::new(source.contents().clone(), context, LIMITS);
    assert_eq!(result.is_err(), case.scenario >= 32);
    if let Ok(rebuilt) = result {
        assert_eq!(rebuilt.to_bytes().unwrap(), case.bytes);
    }
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn incompatible_thread_shapes_fail_while_null_bucket_roots_remain_unchanged() {
    for case in thread_cases()
        .into_iter()
        .filter(|case| case.version != 2749)
    {
        check_thread_boundary(case);
    }
}

fn coordinate_fields(archive: &DisplayArchive, path: &str) -> usize {
    archive
        .contents()
        .fields
        .iter()
        .filter(|field| field.path.starts_with(path))
        .count()
}

fn check_root_thread_alias(source: &DisplayArchive) {
    let root = "root.views[0].view_bucket";
    let key = field(
        source,
        &format!("{root}.bucket.cthread_maps[1].entries[0].key"),
    )
    .unwrap();
    let alias = field(source, &format!("{root}.sub_buckets[0]")).unwrap();
    assert_eq!(key, alias);
}

fn check_thread_alias_and_dimensions(case: OldCase) {
    let source = parsed(case);
    check_root_thread_alias(&source);
    let root = "root.views[0].view_bucket";
    let polygon = format!("{root}.bucket.silh.items[0]");
    assert_eq!(
        coordinate_fields(&source, &format!("{polygon}.points_f32[")),
        if case.scenario % 32 >= 16 { 4 } else { 6 }
    );
    let thread = format!("{root}.bucket.cthread_maps[0].entries[0].value.items[0].points_f32[");
    assert_eq!(coordinate_fields(&source, &thread), 3);
    let width = format!("{polygon}.user_line_property.width.standard_width");
    assert_eq!(
        field(&source, &width),
        Some(&DisplayFieldValue::I32(i32::MIN))
    );
}

#[test]
fn thread_keys_keep_root_aliases_and_restore_three_dimensions_after_two_dimensional_lists() {
    for case in thread_cases()
        .into_iter()
        .filter(|case| case.version == 2771 && case.scenario >= 32 && case.scenario % 16 != 2)
    {
        check_thread_alias_and_dimensions(case);
    }
}

const THREAD_DOCUMENTS: [(u32, &[u8], &[u8]); 2] = [
    (
        2749,
        include_bytes!("../fixtures/display-lists/legacy-thread-external-2749-0.SLDASM"),
        include_bytes!("../fixtures/display-lists/legacy-thread-external-2749-1.SLDASM"),
    ),
    (
        2771,
        include_bytes!("../fixtures/display-lists/legacy-thread-external-2771-0.SLDASM"),
        include_bytes!("../fixtures/display-lists/legacy-thread-external-2771-1.SLDASM"),
    ),
];

#[test]
fn populated_bucket_documents_keep_all_faces_through_edits_reopen_and_revert() {
    for (version, source, expected) in THREAD_DOCUMENTS {
        external_faces::documents::check_edit(version, source, expected, thread_edited_contents);
    }
}

#[test]
fn incomplete_thread_bucket_roots_never_publish_partial_fields() {
    for case in thread_cases() {
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

fn concurrent_thread_case(case: OldCase) {
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
fn shaded_assembly_bucket_graphs_are_deterministic_across_concurrent_constructors() {
    for case in thread_cases().into_iter().filter(|case| {
        case.kind == DisplayDocumentKind::Assembly && case.scenario == 80 && case.version != 2749
    }) {
        concurrent_thread_case(case);
    }
}
