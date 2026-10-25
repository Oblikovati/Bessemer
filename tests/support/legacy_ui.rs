//! Complete old UI and sketch-text layouts alongside independent facet edge arrays.
use super::*;
use bessemer::tessellation::{FacetArchive, FacetContainer, TessellationLimits};

fn early_ui_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-ui-roots.bin"
    ))
}

#[test]
fn complete_early_ui_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(early_ui_cases().len(), 442);
    assert_eq!(
        early_ui_cases().into_iter().map(check_case).sum::<usize>(),
        146786
    );
}

pub(super) fn check_early_ui_members(case: OldCase) {
    let archive = parsed(case);
    legacy_text::check_early_text_members(case);
    assert!(field(&archive, "root.user_model_env.splitter_10.rows").is_some());
    assert_eq!(
        field(&archive, "root.user_model_env.splitter_e0.rows").is_some(),
        case.version >= 1301
    );
    for index in 17..19 {
        assert_eq!(
            field(
                &archive,
                &format!("root.views[0].display_items.items[{index}].data.unknown_a8")
            )
            .is_some(),
            case.version > 1325
        );
    }
    check_early_ui_graph_fields(&archive, case.version);
}

fn check_early_ui_graph_fields(archive: &DisplayArchive, version: u32) {
    for member in &archive.contents().fields {
        if let Some(prefix) = member.path.strip_suffix(".bucket.has_2d_info") {
            assert_eq!(
                field(archive, &format!("{prefix}.has_ghost_dl_xform")).is_some(),
                version > 1289
            );
        }
        check_ui_body_word(archive, &member.path, version);
    }
}

#[test]
fn splitter_ghost_sketch_font_and_body_fields_follow_their_exact_storage_bounds() {
    for case in early_ui_cases() {
        check_early_ui_members(case);
    }
}

fn expected_early_ui_edit(case: OldCase) -> OldCase {
    early_ui_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

#[test]
fn complete_early_ui_sketch_node_and_facet_edits_match_independent_bytes_and_revert() {
    for case in early_ui_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3 | 32 | 48 | 64 | 80 | 96 | 112))
    {
        ordinary_text::check_complete_edit(case, expected_early_ui_edit(case));
    }
}

fn early_ui_facet_bytes(version: u32, index: usize, edited: usize) -> &'static [u8] {
    FACETS.iter().find(|row| row.0 == version).unwrap().1[index][edited]
}

fn expected_early_ui_facet(version: u32, index: usize, edited: usize) -> FacetArchive {
    let limits = TessellationLimits {
        maximum_components: 0,
        maximum_faces: 1,
        maximum_array_elements: 1000,
    };
    FacetArchive::from_bytes(
        early_ui_facet_bytes(version, index, edited),
        legacy_rendering::facet_context(version),
        limits,
    )
    .unwrap()
}

fn check_early_ui_facets(case: OldCase) {
    let source = parsed(case);
    let facets = resumed_inline::inline_facets(&source);
    assert_eq!(facets.len(), 2);
    for (index, facet) in facets.into_iter().enumerate() {
        check_ui_facet_record(facet, case, index);
    }
}

fn check_ui_facet_record(facet: &FacetContainer, case: OldCase, index: usize) {
    let edited = usize::from(case.scenario % 16 == 1);
    assert_eq!(
        facet,
        expected_early_ui_facet(case.version, index, edited).contents()
    );
    let rebuilt =
        FacetArchive::new(legacy_rendering::facet_context(case.version), facet.clone()).unwrap();
    assert_eq!(
        rebuilt.to_bytes().unwrap(),
        early_ui_facet_bytes(case.version, index, edited)
    );
    assert_eq!(
        facet.edge_arrays.as_ref().unwrap().edge_types.is_some(),
        case.version > 1331
    );
}

#[test]
fn complete_inline_facets_match_independent_records_before_the_edge_type_array() {
    for case in early_ui_cases()
        .into_iter()
        .filter(|case| case.scenario >= 96)
    {
        check_early_ui_facets(case);
    }
}

#[test]
fn truncated_early_ui_roots_never_publish_partial_results() {
    for case in early_ui_cases() {
        for length in (0..case.bytes.len())
            .step_by(157)
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
fn new_contexts_require_ui_sketch_font_body_and_facet_fields_at_each_boundary() {
    for case in early_ui_cases().into_iter().filter(|case| {
        matches!(case.version, 1286 | 1289 | 1300 | 1325 | 1326 | 1331) && case.scenario == 96
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
fn early_and_rich_ui_documents_edit_reopen_and_revert() {
    for (kind, raw, expected) in DOCUMENTS {
        resumed_inline::check_owned_document(kind, raw, expected);
    }
}

const DOCUMENTS: [(DisplayDocumentKind, &[u8], &[u8]); 4] = [
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-ui-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-ui-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-ui-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-ui-edited-document-assembly.bin"),
    ),
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-ui-rich-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-ui-rich-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-ui-rich-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-ui-rich-edited-document-assembly.bin"),
    ),
];

const FACETS: [(u32, [[&[u8]; 2]; 2]); 12] = [
    (
        1286,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1286-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1286-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1286-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1286-1-1.bin"),
            ],
        ],
    ),
    (
        1287,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1287-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1287-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1287-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1287-1-1.bin"),
            ],
        ],
    ),
    (
        1289,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1289-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1289-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1289-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1289-1-1.bin"),
            ],
        ],
    ),
    (
        1290,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1290-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1290-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1290-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1290-1-1.bin"),
            ],
        ],
    ),
    (
        1300,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1300-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1300-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1300-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1300-1-1.bin"),
            ],
        ],
    ),
    (
        1301,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1301-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1301-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1301-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1301-1-1.bin"),
            ],
        ],
    ),
    (
        1325,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1325-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1325-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1325-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1325-1-1.bin"),
            ],
        ],
    ),
    (
        1326,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1326-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1326-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1326-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1326-1-1.bin"),
            ],
        ],
    ),
    (
        1327,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1327-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1327-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1327-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1327-1-1.bin"),
            ],
        ],
    ),
    (
        1331,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1331-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1331-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1331-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1331-1-1.bin"),
            ],
        ],
    ),
    (
        1332,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1332-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1332-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1332-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1332-1-1.bin"),
            ],
        ],
    ),
    (
        1645,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1645-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1645-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1645-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-ui-facet-1645-1-1.bin"),
            ],
        ],
    ),
];

fn check_ui_body_word(archive: &DisplayArchive, path: &str, version: u32) -> bool {
    if !path.contains(".bodies[") {
        return false;
    }
    let Some(prefix) = path.strip_suffix(".faces.count_word") else {
        return false;
    };
    assert_eq!(
        field(archive, &format!("{prefix}.unknown_20")).is_some(),
        version > 1286
    );
    true
}

#[test]
fn body_metadata_assertions_visit_real_count_word_headers_on_both_sides_of_the_gate() {
    for case in early_ui_cases()
        .into_iter()
        .filter(|case| matches!(case.version, 1286 | 1287) && case.scenario == 96)
    {
        let archive = parsed(case);
        let visits = archive
            .contents()
            .fields
            .iter()
            .filter(|member| check_ui_body_word(&archive, &member.path, case.version))
            .count();
        assert_eq!(visits, 1, "version {} kind {:?}", case.version, case.kind);
    }
}
