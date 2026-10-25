//! Complete old ghost lists, hidden counts, part timestamps and facet arrays.
use super::*;
use bessemer::tessellation::{FacetArchive, FacetContainer, TessellationLimits};

fn early_ghost_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-ghost-roots.bin"
    ))
}

#[test]
fn complete_early_ghost_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(early_ghost_cases().len(), 406);
    assert_eq!(
        early_ghost_cases()
            .into_iter()
            .map(check_case)
            .sum::<usize>(),
        133273
    );
}

pub(super) fn check_early_ghost_members(case: OldCase) {
    let archive = parsed(case);
    legacy_ui::check_early_ui_members(case);
    for member in &archive.contents().fields {
        if let Some(prefix) = member.path.strip_suffix(".tess_number") {
            check_ghost_suffix(&archive, prefix, case.version);
        }
        if let Some(prefix) = member.path.strip_suffix(".bodies.count_word") {
            assert_eq!(
                field(&archive, &format!("{prefix}.ddata_time_stamp")).is_some(),
                case.version >= 1271
            );
        }
    }
}

fn check_ghost_suffix(archive: &DisplayArchive, prefix: &str, version: u32) {
    assert_eq!(
        field(archive, &format!("{prefix}.show_hidden_count")).is_some(),
        version >= 1236
    );
    let count = field(archive, &format!("{prefix}.dl_ghost_bucket.count_word"));
    if version <= 1215 {
        assert!(count.is_none());
        return;
    }
    let silh = field(archive, &format!("{prefix}.silh.count")).unwrap();
    let DisplayFieldValue::I32(silh) = silh else {
        panic!("expected polygon count");
    };
    assert_eq!(count, Some(&DisplayFieldValue::U16(u16::from(*silh > 0))));
}

#[test]
fn ghost_items_hidden_words_and_timestamps_follow_their_independent_storage_bounds() {
    for case in early_ghost_cases() {
        check_early_ghost_members(case);
    }
}

fn expected_early_ghost_edit(case: OldCase) -> OldCase {
    early_ghost_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

#[test]
fn complete_ghost_item_text_node_and_facet_edits_match_independent_bytes_and_revert() {
    for case in early_ghost_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3 | 32 | 48 | 64 | 80 | 96 | 112))
    {
        ordinary_text::check_complete_edit(case, expected_early_ghost_edit(case));
    }
}

fn early_ghost_facet_bytes(version: u32, index: usize, edited: usize) -> &'static [u8] {
    FACETS.iter().find(|row| row.0 == version).unwrap().1[index][edited]
}

fn expected_early_ghost_facet(version: u32, index: usize, edited: usize) -> FacetArchive {
    let limits = TessellationLimits {
        maximum_components: 0,
        maximum_faces: 1,
        maximum_array_elements: 1000,
    };
    FacetArchive::from_bytes(
        early_ghost_facet_bytes(version, index, edited),
        legacy_rendering::facet_context(version),
        limits,
    )
    .unwrap()
}

fn check_early_ghost_facets(case: OldCase) {
    let source = parsed(case);
    let facets = resumed_inline::inline_facets(&source);
    assert_eq!(facets.len(), 2);
    for (index, facet) in facets.into_iter().enumerate() {
        check_ghost_facet_record(facet, case, index);
    }
}

fn check_ghost_facet_record(facet: &FacetContainer, case: OldCase, index: usize) {
    let edited = usize::from(case.scenario % 16 == 1);
    assert_eq!(
        facet,
        expected_early_ghost_facet(case.version, index, edited).contents()
    );
    let rebuilt =
        FacetArchive::new(legacy_rendering::facet_context(case.version), facet.clone()).unwrap();
    assert_eq!(
        rebuilt.to_bytes().unwrap(),
        early_ghost_facet_bytes(case.version, index, edited)
    );
    assert_eq!(facet.legacy_triangles.is_some(), case.version > 1253);
    assert_eq!(facet.planar.is_some(), case.version > 1253);
    assert_eq!(facet.legacy_array.is_some(), case.version > 1275);
}

#[test]
fn complete_inline_facets_match_independent_records_before_legacy_arrays_and_planar_words() {
    for case in early_ghost_cases()
        .into_iter()
        .filter(|case| case.scenario >= 96)
    {
        check_early_ghost_facets(case);
    }
}

#[test]
fn truncated_early_ghost_roots_never_publish_partial_results() {
    for case in early_ghost_cases() {
        for length in (0..case.bytes.len())
            .step_by(163)
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
fn new_contexts_require_ghost_hidden_time_and_facet_fields_at_each_boundary() {
    for case in early_ghost_cases().into_iter().filter(|case| {
        matches!(case.version, 1215 | 1235 | 1253 | 1270 | 1275) && case.scenario == 96
    }) {
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
fn early_and_populated_ghost_documents_edit_reopen_and_revert() {
    for (kind, raw, expected) in DOCUMENTS {
        resumed_inline::check_owned_document(kind, raw, expected);
    }
}

const DOCUMENTS: [(DisplayDocumentKind, &[u8], &[u8]); 4] = [
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-ghost-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-ghost-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-ghost-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-ghost-edited-document-assembly.bin"),
    ),
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-ghost-rich-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-ghost-rich-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-ghost-rich-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-ghost-rich-edited-document-assembly.bin"),
    ),
];

const FACETS: [(u32, [[&[u8]; 2]; 2]); 11] = [
    (
        1215,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1215-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1215-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1215-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1215-1-1.bin"),
            ],
        ],
    ),
    (
        1216,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1216-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1216-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1216-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1216-1-1.bin"),
            ],
        ],
    ),
    (
        1235,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1235-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1235-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1235-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1235-1-1.bin"),
            ],
        ],
    ),
    (
        1236,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1236-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1236-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1236-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1236-1-1.bin"),
            ],
        ],
    ),
    (
        1253,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1253-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1253-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1253-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1253-1-1.bin"),
            ],
        ],
    ),
    (
        1254,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1254-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1254-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1254-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1254-1-1.bin"),
            ],
        ],
    ),
    (
        1270,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1270-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1270-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1270-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1270-1-1.bin"),
            ],
        ],
    ),
    (
        1271,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1271-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1271-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1271-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1271-1-1.bin"),
            ],
        ],
    ),
    (
        1275,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1275-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1275-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1275-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1275-1-1.bin"),
            ],
        ],
    ),
    (
        1276,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1276-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1276-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1276-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1276-1-1.bin"),
            ],
        ],
    ),
    (
        1286,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1286-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1286-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1286-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ghost-facet-1286-1-1.bin"),
            ],
        ],
    ),
];
