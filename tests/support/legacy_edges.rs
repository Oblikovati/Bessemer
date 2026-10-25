//! Temporary edge records and view triples retain every stored legacy word.
use super::*;

fn edge_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-edge-roots.bin"
    ))
}

#[test]
fn complete_edge_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(edge_cases().len(), 206);
    assert_eq!(
        edge_cases().into_iter().map(check_case).sum::<usize>(),
        43739
    );
}

fn edge_field_count(archive: &DisplayArchive, suffix: &str) -> usize {
    archive
        .contents()
        .fields
        .iter()
        .filter(|field| field.path.ends_with(suffix))
        .count()
}

fn expected_edge_counts(case: OldCase) -> [usize; 4] {
    let buckets = if case.scenario < 32 {
        0
    } else if case.scenario % 16 == 2 {
        1
    } else {
        2
    };
    let records = usize::from(case.scenario >= 32 && case.scenario % 16 != 2) * 2;
    [
        if case.version < 2650 { buckets } else { 0 },
        if case.version < 2635 { records } else { 0 },
        if case.version >= 2650 { buckets } else { 0 },
        if (2627..2650).contains(&case.version) {
            buckets
        } else {
            0
        },
    ]
}

fn check_edge_fields(case: OldCase) {
    let source = parsed(case);
    let suffixes = [
        ".edge_vis_data.count",
        ".discarded_old_8",
        ".component_str_map.count_word",
        ".unknown_old_triples.count",
    ];
    for (suffix, count) in suffixes.into_iter().zip(expected_edge_counts(case)) {
        assert_eq!(edge_field_count(&source, suffix), count);
    }
}

#[test]
fn edge_arrays_discarded_words_component_maps_and_triples_follow_their_separate_gates() {
    for case in edge_cases() {
        check_edge_fields(case);
    }
}

pub(super) fn edge_edited_contents(
    source: &DisplayArchive,
    expected: &DisplayArchive,
) -> DisplayArchiveContents {
    let mut contents = legacy_visibility::visibility_edited_contents(source, expected);
    for target in &mut contents.fields {
        if target.path.contains(".edge_vis_data[") || target.path.contains(".unknown_old_triples[")
        {
            target.value = field(expected, &target.path).unwrap().clone();
        }
    }
    contents
}

fn expected_edge_edit(case: OldCase) -> OldCase {
    edge_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

fn check_edge_edit(case: OldCase) {
    let source = parsed(case);
    let expected_case = expected_edge_edit(case);
    let expected = parsed(expected_case);
    let edited = source
        .with_contents(edge_edited_contents(&source, &expected), LIMITS)
        .unwrap();
    assert_eq!(edited.to_bytes().unwrap(), expected_case.bytes);
    let reverted = edited
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    assert_eq!(reverted.to_bytes().unwrap(), case.bytes);
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn temporary_edge_and_triple_edits_match_independent_bytes_and_authentically_revert() {
    for case in edge_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3 | 32 | 48 | 64 | 80))
    {
        check_edge_edit(case);
    }
}

fn opposite_edge_version(version: u32) -> u32 {
    match version {
        2626 => 2627,
        2627 => 2626,
        2634 => 2635,
        2635 => 2634,
        2649 => 2650,
        _ => 2649,
    }
}

fn check_edge_boundary(case: OldCase) {
    let source = parsed(case);
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(opposite_edge_version(case.version)),
        ..case_context(case)
    };
    let changed =
        case.scenario >= 32 && (!matches!(case.version, 2634 | 2635) || case.scenario % 16 != 2);
    let result = DisplayArchive::new(source.contents().clone(), context, LIMITS);
    assert_eq!(result.is_err(), changed);
    if let Ok(rebuilt) = result {
        assert_eq!(rebuilt.to_bytes().unwrap(), case.bytes);
    }
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn incompatible_edge_shapes_fail_while_unchanged_empty_records_remain_compatible() {
    for case in edge_cases().into_iter().filter(|case| case.version != 2606) {
        check_edge_boundary(case);
    }
}

fn check_edge_dimensions(case: OldCase) {
    let source = parsed(case);
    let prefix = "root.views[0].view_bucket.bucket.edge_vis_data[0].polys.items[0].points_f32[";
    let coordinates = source
        .contents()
        .fields
        .iter()
        .filter(|field| field.path.starts_with(prefix))
        .count();
    assert_eq!(coordinates, if case.scenario % 32 >= 16 { 4 } else { 6 });
    if case.version < 2635 {
        let edited = i32::from(case.scenario % 16 == 1);
        let path = "root.views[0].view_bucket.bucket.edge_vis_data[0].discarded_old_8";
        assert_eq!(
            field(&source, path),
            Some(&DisplayFieldValue::I32(i32::MIN + edited))
        );
    }
}

#[test]
fn edge_polygons_inherit_bucket_dimensions_and_keep_discarded_word_bits() {
    for case in edge_cases()
        .into_iter()
        .filter(|case| case.version < 2650 && case.scenario >= 32 && case.scenario % 16 != 2)
    {
        check_edge_dimensions(case);
    }
}

fn check_root_triples(source: &DisplayArchive, root: &str, edited: i32) {
    assert_eq!(
        field(source, &format!("{root}.unknown_old_triples[0][0]")),
        Some(&DisplayFieldValue::I32(-1 + edited))
    );
    assert_eq!(
        field(source, &format!("{root}.unknown_old_triples[1][0]")),
        Some(&DisplayFieldValue::I32(i32::MIN))
    );
}

fn check_nested_triples(case: OldCase) {
    let source = parsed(case);
    let root = "root.views[0].view_bucket";
    check_root_triples(&source, root, i32::from(case.scenario % 16 == 1));
    assert_eq!(
        field(
            &source,
            &format!("{root}.sub_buckets[1].unknown_old_triples.count")
        ),
        Some(&DisplayFieldValue::I32(-7))
    );
    assert!(
        field(
            &source,
            &format!("{root}.sub_buckets[0].unknown_old_triples.count")
        )
        .is_none()
    );
}

#[test]
fn temporary_triples_belong_to_each_declared_view_and_never_to_aliases() {
    for case in edge_cases().into_iter().filter(|case| {
        (2627..2650).contains(&case.version) && case.scenario >= 32 && case.scenario % 16 != 2
    }) {
        check_nested_triples(case);
    }
}

fn reject_large_count(source: &DisplayArchive, path: &str) {
    let mut contents = source.contents().clone();
    archive_support::set_integer(&mut contents, path, i32::MAX);
    let failure = source.with_contents(contents, LIMITS).unwrap_err();
    assert_eq!(failure.class, FailureClass::Invalid);
    assert!(failure.cause.contains("2147483647"));
}

#[test]
fn oversized_edge_and_triple_counts_fail_before_allocating_or_publishing_results() {
    let case = edge_cases()
        .into_iter()
        .find(|case| {
            case.version == 2634
                && case.kind == DisplayDocumentKind::Assembly
                && case.scenario == 80
        })
        .unwrap();
    let source = parsed(case);
    reject_large_count(
        &source,
        "root.views[0].view_bucket.bucket.edge_vis_data.count",
    );
    reject_large_count(
        &source,
        "root.views[0].view_bucket.unknown_old_triples.count",
    );
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

const EDGE_DOCUMENTS: [(u32, &[u8], &[u8]); 5] = [
    (
        2606,
        include_bytes!("../fixtures/display-lists/legacy-edge-external-2606-0.SLDASM"),
        include_bytes!("../fixtures/display-lists/legacy-edge-external-2606-1.SLDASM"),
    ),
    (
        2634,
        include_bytes!("../fixtures/display-lists/legacy-edge-external-2634-0.SLDASM"),
        include_bytes!("../fixtures/display-lists/legacy-edge-external-2634-1.SLDASM"),
    ),
    (
        2635,
        include_bytes!("../fixtures/display-lists/legacy-edge-external-2635-0.SLDASM"),
        include_bytes!("../fixtures/display-lists/legacy-edge-external-2635-1.SLDASM"),
    ),
    (
        2649,
        include_bytes!("../fixtures/display-lists/legacy-edge-external-2649-0.SLDASM"),
        include_bytes!("../fixtures/display-lists/legacy-edge-external-2649-1.SLDASM"),
    ),
    (
        2650,
        include_bytes!("../fixtures/display-lists/legacy-edge-external-2650-0.SLDASM"),
        include_bytes!("../fixtures/display-lists/legacy-edge-external-2650-1.SLDASM"),
    ),
];

#[test]
fn temporary_edge_documents_keep_all_faces_through_edits_reopen_and_revert() {
    for (version, source, expected) in EDGE_DOCUMENTS {
        external_faces::documents::check_edit(version, source, expected, edge_edited_contents);
    }
}

#[test]
fn incomplete_legacy_edge_roots_never_publish_partial_fields() {
    for case in edge_cases() {
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
