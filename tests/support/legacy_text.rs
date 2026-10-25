//! Physical polygon nodes survive old logical resets; text and facet fields stay gated.
use super::*;
use bessemer::tessellation::{FacetArchive, TessellationLimits};

fn early_text_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-text-roots.bin"
    ))
}

#[test]
fn complete_early_text_roots_match_independent_logical_and_physical_fields_and_bytes() {
    assert_eq!(early_text_cases().len(), 262);
    assert_eq!(
        early_text_cases()
            .into_iter()
            .map(check_case)
            .sum::<usize>(),
        78250
    );
}

pub(super) fn check_early_text_members(case: OldCase) {
    let archive = parsed(case);
    legacy_discard::check_hidden_silhouettes(case);
    for index in 9..13 {
        let prefix = format!("root.views[0].display_items.items[{index}].data");
        for name in ["unknown_34", "unknown_38", "unknown_40"] {
            assert_eq!(
                field(&archive, &format!("{prefix}.{name}")).is_some(),
                case.version > 1834
            );
        }
    }
}

#[test]
fn ordinary_text_tails_are_absent_through_1834_for_both_item_kinds() {
    for case in early_text_cases() {
        check_early_text_members(case);
    }
}

fn check_stored_node_words(case: OldCase) {
    let archive = parsed(case);
    let edited = case.scenario == 4 || case.scenario % 16 == 1;
    let expected = DisplayFieldValue::I32(if edited { i32::MAX } else { i32::MIN });
    let nodes: Vec<_> = archive
        .contents()
        .fields
        .iter()
        .filter(|member| member.path.contains(".view_bucket.") && member.path.ends_with(".node_id"))
        .collect();
    if case.scenario == 96 {
        assert!(!nodes.is_empty());
    }
    for node in nodes {
        assert_eq!(node.value, expected);
    }
}

#[test]
fn polygon_node_words_retain_both_signed_extremes_despite_old_logical_resets() {
    for case in early_text_cases() {
        check_stored_node_words(case);
    }
}

fn expected_early_text_edit(case: OldCase) -> OldCase {
    early_text_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

#[test]
fn complete_early_text_node_and_facet_edits_match_independent_bytes_and_revert() {
    for case in early_text_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3 | 32 | 48 | 64 | 80 | 96 | 112))
    {
        ordinary_text::check_complete_edit(case, expected_early_text_edit(case));
    }
}

fn early_text_facet_bytes(version: u32, index: usize, edited: usize) -> &'static [u8] {
    FACETS.iter().find(|row| row.0 == version).unwrap().1[index][edited]
}

fn expected_early_text_facet(version: u32, index: usize, edited: usize) -> FacetArchive {
    let limits = TessellationLimits {
        maximum_components: 0,
        maximum_faces: 1,
        maximum_array_elements: 1000,
    };
    FacetArchive::from_bytes(
        early_text_facet_bytes(version, index, edited),
        legacy_rendering::facet_context(version),
        limits,
    )
    .unwrap()
}

fn check_early_text_facets(case: OldCase) {
    let source = parsed(case);
    let facets = resumed_inline::inline_facets(&source);
    assert_eq!(facets.len(), 2);
    let edited = usize::from(case.scenario % 16 == 1);
    for (index, facet) in facets.into_iter().enumerate() {
        assert_eq!(
            facet,
            expected_early_text_facet(case.version, index, edited).contents()
        );
        let rebuilt =
            FacetArchive::new(legacy_rendering::facet_context(case.version), facet.clone())
                .unwrap();
        assert_eq!(
            rebuilt.to_bytes().unwrap(),
            early_text_facet_bytes(case.version, index, edited)
        );
        assert_eq!(facet.edge_ids_valid.is_some(), case.version > 1673);
    }
}

#[test]
fn complete_inline_facets_match_independent_records_across_the_edge_id_flag_boundary() {
    for case in early_text_cases()
        .into_iter()
        .filter(|case| case.scenario >= 96)
    {
        check_early_text_facets(case);
    }
}

#[test]
fn truncated_early_text_roots_never_publish_partial_results() {
    for case in early_text_cases() {
        for length in (0..case.bytes.len())
            .step_by(151)
            .chain(case.bytes.len() - 8..case.bytes.len())
        {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..length], case_context(case), LIMITS)
                    .is_err()
            );
        }
    }
}

fn check_early_text_transition(case: OldCase) {
    let source = parsed(case);
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(case.version + 1),
        ..case_context(case)
    };
    let result = DisplayArchive::new(source.contents().clone(), context, LIMITS);
    if case.version == 1645 {
        assert_eq!(result.unwrap().to_bytes().unwrap(), case.bytes);
    } else {
        assert!(result.is_err());
    }
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn contexts_preserve_physical_nodes_but_require_new_text_and_facet_words() {
    for case in early_text_cases()
        .into_iter()
        .filter(|case| matches!(case.version, 1645 | 1673 | 1834) && case.scenario == 96)
    {
        check_early_text_transition(case);
    }
}

#[test]
fn complete_early_text_node_documents_edit_reopen_and_revert() {
    for (kind, raw, expected) in DOCUMENTS {
        resumed_inline::check_owned_document(kind, raw, expected);
    }
}

const DOCUMENTS: [(DisplayDocumentKind, &[u8], &[u8]); 2] = [
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-text-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-text-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-text-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-text-edited-document-assembly.bin"),
    ),
];

const FACETS: [(u32, [[&[u8]; 2]; 2]); 7] = [
    (
        1645,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1645-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1645-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1645-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1645-1-1.bin"),
            ],
        ],
    ),
    (
        1646,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1646-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1646-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1646-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1646-1-1.bin"),
            ],
        ],
    ),
    (
        1673,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1673-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1673-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1673-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1673-1-1.bin"),
            ],
        ],
    ),
    (
        1674,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1674-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1674-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1674-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1674-1-1.bin"),
            ],
        ],
    ),
    (
        1834,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1834-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1834-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1834-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1834-1-1.bin"),
            ],
        ],
    ),
    (
        1835,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1835-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1835-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1835-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1835-1-1.bin"),
            ],
        ],
    ),
    (
        1857,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1857-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1857-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1857-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-text-facet-1857-1-1.bin"),
            ],
        ],
    ),
];
