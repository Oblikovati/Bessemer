//! Bucket coordinates remain three-dimensional before a stored 2D scope exists.
use super::*;
use bessemer::tessellation::{FacetArchive, FacetContainer, TessellationLimits};

fn early_scope_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-scope-roots.bin"
    ))
}

#[test]
fn complete_early_scope_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(early_scope_cases().len(), 154);
    assert_eq!(
        early_scope_cases()
            .into_iter()
            .map(check_case)
            .sum::<usize>(),
        49017
    );
}

fn check_early_scope_members(case: OldCase) {
    let archive = parsed(case);
    legacy_ghost::check_early_ghost_members(case);
    for member in &archive.contents().fields {
        if let Some(prefix) = member.path.strip_suffix(".tess_number") {
            assert_eq!(
                field(&archive, &format!("{prefix}.has_2d_info")).is_some(),
                case.version > 1180
            );
            if case.version <= 1180 {
                assert!(field(&archive, &format!("{prefix}.xform_2d")).is_none());
            }
        }
        if let Some(prefix) = member.path.strip_suffix(".smooth") {
            check_scope_polygon(&archive, prefix);
        }
    }
}

fn check_scope_polygon(archive: &DisplayArchive, prefix: &str) {
    let Some(DisplayFieldValue::I32(count)) = field(archive, &format!("{prefix}.count")) else {
        panic!("expected polygon count");
    };
    if *count <= 0 {
        return;
    }
    let (owner, _) = prefix.rsplit_once(".bucket.").unwrap();
    let two_dimensional = matches!(field(archive,&format!("{owner}.bucket.has_2d_info")),Some(DisplayFieldValue::I32(value)) if *value != 0);
    let actual = scope_coordinate_count(archive, prefix);
    assert_eq!(
        actual,
        *count as usize * if two_dimensional { 2 } else { 3 }
    );
}

#[test]
fn old_buckets_omit_scope_fields_and_every_polygon_uses_the_correct_coordinate_arity() {
    for case in early_scope_cases() {
        check_early_scope_members(case);
    }
}

#[test]
fn converted_old_2d_profiles_store_and_edit_explicit_z_coordinates() {
    for case in early_scope_cases()
        .into_iter()
        .filter(|case| case.version == 1180 && matches!(case.scenario, 112 | 113))
    {
        check_converted_z_words(case);
    }
}

fn check_converted_z_words(case: OldCase) {
    let archive = parsed(case);
    for index in [2, 5] {
        let path = format!("root.views[0].view_bucket.bucket.silh.items[0].points_f32[{index}]");
        let Some(DisplayFieldValue::F32(value)) = field(&archive, &path) else {
            panic!("expected stored Z coordinate");
        };
        let expected = if case.scenario == 113 {
            2.0_f32
        } else {
            1.0_f32
        };
        assert_eq!(value.bits(), expected.to_bits());
    }
}

fn expected_early_scope_edit(case: OldCase) -> OldCase {
    early_scope_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

#[test]
fn complete_scope_coordinate_text_and_facet_edits_match_independent_bytes_and_revert() {
    for case in early_scope_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3 | 32 | 48 | 64 | 80 | 96 | 112))
    {
        ordinary_text::check_complete_edit(case, expected_early_scope_edit(case));
    }
}

fn early_scope_facet_bytes(version: u32, index: usize, edited: usize) -> &'static [u8] {
    FACETS.iter().find(|row| row.0 == version).unwrap().1[index][edited]
}

fn expected_early_scope_facet(version: u32, index: usize, edited: usize) -> FacetArchive {
    let limits = TessellationLimits {
        maximum_components: 0,
        maximum_faces: 1,
        maximum_array_elements: 1000,
    };
    FacetArchive::from_bytes(
        early_scope_facet_bytes(version, index, edited),
        legacy_rendering::facet_context(version),
        limits,
    )
    .unwrap()
}

fn check_early_scope_facets(case: OldCase) {
    let source = parsed(case);
    let facets = resumed_inline::inline_facets(&source);
    assert_eq!(facets.len(), 2);
    for (index, facet) in facets.into_iter().enumerate() {
        check_scope_facet_record(facet, case, index);
    }
}

fn check_scope_facet_record(facet: &FacetContainer, case: OldCase, index: usize) {
    let edited = usize::from(case.scenario % 16 == 1);
    assert_eq!(
        facet,
        expected_early_scope_facet(case.version, index, edited).contents()
    );
    let rebuilt =
        FacetArchive::new(legacy_rendering::facet_context(case.version), facet.clone()).unwrap();
    assert_eq!(
        rebuilt.to_bytes().unwrap(),
        early_scope_facet_bytes(case.version, index, edited)
    );
    assert_eq!(facet.legacy_word.is_some(), case.version > 1181);
}

#[test]
fn complete_inline_facets_match_independent_records_before_the_discarded_word() {
    for case in early_scope_cases()
        .into_iter()
        .filter(|case| case.scenario >= 96)
    {
        check_early_scope_facets(case);
    }
}

#[test]
fn truncated_early_scope_roots_never_publish_partial_results() {
    for case in early_scope_cases() {
        for length in (0..case.bytes.len())
            .step_by(167)
            .chain(case.bytes.len() - 8..case.bytes.len())
        {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..length], case_context(case), LIMITS)
                    .is_err()
            );
        }
    }
}

#[test]
fn new_contexts_require_coordinate_scopes_and_discarded_facet_words() {
    for case in early_scope_cases()
        .into_iter()
        .filter(|case| matches!(case.version, 1180 | 1181) && case.scenario == 96)
    {
        let source = parsed(case);
        let context = DisplayArchiveContext {
            version: DisplayVersion::new(case.version + 1),
            ..case_context(case)
        };
        assert!(DisplayArchive::new(source.contents().clone(), context, LIMITS).is_err());
        assert_eq!(source.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn old_3d_and_scoped_2d_documents_edit_reopen_and_revert() {
    for (kind, raw, expected) in DOCUMENTS {
        resumed_inline::check_owned_document(kind, raw, expected);
    }
}

const DOCUMENTS: [(DisplayDocumentKind, &[u8], &[u8]); 4] = [
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-scope-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-scope-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-scope-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-scope-edited-document-assembly.bin"),
    ),
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-scope-rich-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-scope-rich-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-scope-rich-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-scope-rich-edited-document-assembly.bin"),
    ),
];

const FACETS: [(u32, [[&[u8]; 2]; 2]); 4] = [
    (
        1180,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-scope-facet-1180-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-scope-facet-1180-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-scope-facet-1180-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-scope-facet-1180-1-1.bin"),
            ],
        ],
    ),
    (
        1181,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-scope-facet-1181-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-scope-facet-1181-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-scope-facet-1181-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-scope-facet-1181-1-1.bin"),
            ],
        ],
    ),
    (
        1182,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-scope-facet-1182-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-scope-facet-1182-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-scope-facet-1182-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-scope-facet-1182-1-1.bin"),
            ],
        ],
    ),
    (
        1215,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-scope-facet-1215-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-scope-facet-1215-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-scope-facet-1215-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-scope-facet-1215-1-1.bin"),
            ],
        ],
    ),
];

fn scope_coordinate_count(archive: &DisplayArchive, prefix: &str) -> usize {
    let point_prefix = format!("{prefix}.points_f32[");
    archive
        .contents()
        .fields
        .iter()
        .filter(|member| member.path.starts_with(&point_prefix))
        .count()
}
