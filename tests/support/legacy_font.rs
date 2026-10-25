//! Font suffixes and solid-fill lists retain their physical version layouts.
use super::*;
use bessemer::tessellation::{FacetArchive, TessellationLimits};

fn font_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-font-roots.bin"
    ))
}

#[test]
fn complete_font_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(font_cases().len(), 262);
    assert_eq!(
        font_cases().into_iter().map(check_case).sum::<usize>(),
        79851
    );
}

pub(super) fn check_font_gates(case: OldCase) {
    let archive = parsed(case);
    legacy_boundary::check_boundary_members(case);
    for index in 13..17 {
        let prefix = format!("root.views[0].display_items.items[{index}].data");
        assert_eq!(
            field(&archive, &format!("{prefix}.unknown_28")).is_some(),
            case.version > 1326
        );
        assert_eq!(
            field(&archive, &format!("{prefix}.unknown_30")).is_some(),
            case.version > 2015
        );
    }
    for member in &archive.contents().fields {
        if let Some(prefix) = member.path.strip_suffix(".has_2d_info") {
            assert_eq!(
                field(&archive, &format!("{prefix}.solid_fill.count")).is_some(),
                case.version > 2032
            );
        }
    }
}

#[test]
fn font_words_and_solid_fill_lists_follow_their_independent_storage_bounds() {
    for case in font_cases() {
        check_font_gates(case);
    }
}

fn expected_font_edit(case: OldCase) -> OldCase {
    font_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

#[test]
fn complete_font_text_edge_and_facet_edits_match_independent_bytes_and_revert() {
    for case in font_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3 | 32 | 48 | 64 | 80 | 96 | 112))
    {
        ordinary_text::check_complete_edit(case, expected_font_edit(case));
    }
}

fn font_facet_bytes(version: u32, index: usize, edited: usize) -> &'static [u8] {
    FACETS.iter().find(|row| row.0 == version).unwrap().1[index][edited]
}

pub(super) fn expected_font_facet(version: u32, index: usize, edited: usize) -> FacetArchive {
    let limits = TessellationLimits {
        maximum_components: 0,
        maximum_faces: 1,
        maximum_array_elements: 1000,
    };
    FacetArchive::from_bytes(
        font_facet_bytes(version, index, edited),
        legacy_rendering::facet_context(version),
        limits,
    )
    .unwrap()
}

fn check_font_facets(case: OldCase) {
    let source = parsed(case);
    let facets = resumed_inline::inline_facets(&source);
    assert_eq!(facets.len(), 2);
    let edited = usize::from(case.scenario % 16 == 1);
    for (index, facet) in facets.into_iter().enumerate() {
        assert_eq!(
            facet,
            expected_font_facet(case.version, index, edited).contents()
        );
        let rebuilt =
            FacetArchive::new(legacy_rendering::facet_context(case.version), facet.clone())
                .unwrap();
        assert_eq!(
            rebuilt.to_bytes().unwrap(),
            font_facet_bytes(case.version, index, edited)
        );
        assert_eq!(facet.legacy_word.is_some(), case.version < 2012);
    }
}

#[test]
fn complete_inline_facets_match_independent_records_with_the_temporary_legacy_word() {
    for case in font_cases().into_iter().filter(|case| case.scenario >= 96) {
        check_font_facets(case);
    }
}

#[test]
fn truncated_font_roots_never_publish_partial_results() {
    for case in font_cases() {
        for length in (0..case.bytes.len())
            .step_by(137)
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
fn contexts_cannot_republish_missing_font_or_fill_fields_or_an_extra_facet_word() {
    for case in font_cases()
        .into_iter()
        .filter(|case| matches!(case.version, 2011 | 2015 | 2032) && case.scenario == 96)
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
fn complete_font_documents_edit_reopen_and_revert() {
    for (kind, raw, expected) in DOCUMENTS {
        resumed_inline::check_owned_document(kind, raw, expected);
    }
}

const DOCUMENTS: [(DisplayDocumentKind, &[u8], &[u8]); 2] = [
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-font-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-font-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-font-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-font-edited-document-assembly.bin"),
    ),
];

const FACETS: [(u32, [[&[u8]; 2]; 2]); 7] = [
    (
        2011,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2011-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2011-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2011-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2011-1-1.bin"),
            ],
        ],
    ),
    (
        2012,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2012-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2012-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2012-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2012-1-1.bin"),
            ],
        ],
    ),
    (
        2015,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2015-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2015-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2015-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2015-1-1.bin"),
            ],
        ],
    ),
    (
        2016,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2016-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2016-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2016-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2016-1-1.bin"),
            ],
        ],
    ),
    (
        2032,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2032-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2032-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2032-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2032-1-1.bin"),
            ],
        ],
    ),
    (
        2033,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2033-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2033-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2033-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2033-1-1.bin"),
            ],
        ],
    ),
    (
        2078,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2078-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2078-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2078-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-font-facet-2078-1-1.bin"),
            ],
        ],
    ),
];
