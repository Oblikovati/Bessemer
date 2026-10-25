//! Old environment tails and complete pre-coordinate inline facets.
use super::*;
use bessemer::tessellation::{FacetArchive, TessellationLimits};

fn environment_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-environment-roots.bin"
    ))
}

#[test]
fn complete_environment_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(environment_cases().len(), 190);
    assert_eq!(
        environment_cases()
            .into_iter()
            .map(check_case)
            .sum::<usize>(),
        51031
    );
}

#[test]
fn environment_tail_words_are_physically_absent_through_2277() {
    for case in environment_cases() {
        legacy_texture::check_texture_gates(case);
        let archive = parsed(case);
        for name in ["unknown_1b8", "unknown_1bc"] {
            let path = format!("root.user_model_env.{name}");
            assert_eq!(field(&archive, &path).is_some(), case.version > 2277);
        }
    }
}

fn expected_environment_edit(case: OldCase) -> OldCase {
    environment_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

#[test]
fn complete_environment_text_edge_and_facet_edits_match_independent_bytes_and_revert() {
    for case in environment_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3 | 32 | 48 | 64 | 80 | 96 | 112))
    {
        ordinary_text::check_complete_edit(case, expected_environment_edit(case));
    }
}

pub(super) fn expected_facet(version: u32, index: usize, edited: usize) -> FacetArchive {
    let bytes = facet_bytes(version, index, edited);
    let limits = TessellationLimits {
        maximum_components: 0,
        maximum_faces: 1,
        maximum_array_elements: 1000,
    };
    FacetArchive::from_bytes(bytes, legacy_rendering::facet_context(version), limits).unwrap()
}

fn facet_bytes(version: u32, index: usize, edited: usize) -> &'static [u8] {
    FACETS.iter().find(|row| row.0 == version).unwrap().1[index][edited]
}

fn check_environment_facets(case: OldCase) {
    let source = parsed(case);
    let facets = resumed_inline::inline_facets(&source);
    assert_eq!(facets.len(), 2);
    for (index, facet) in facets.into_iter().enumerate() {
        check_facet_record(facet, case, index);
    }
}

fn check_facet_record(facet: &bessemer::tessellation::FacetContainer, case: OldCase, index: usize) {
    let context = legacy_rendering::facet_context(case.version);
    let edited = usize::from(case.scenario % 16 == 1);
    assert_eq!(
        facet,
        expected_facet(case.version, index, edited).contents()
    );
    let rebuilt = FacetArchive::new(context, facet.clone()).unwrap();
    assert_eq!(
        rebuilt.to_bytes().unwrap(),
        facet_bytes(case.version, index, edited)
    );
    assert_eq!(facet.texture_coords.is_some(), case.version > 2304);
    assert_eq!(facet.tangents_binormals.is_some(), case.version > 2304);
}

#[test]
fn complete_inline_facets_match_independent_records_across_texture_coordinate_bounds() {
    for case in environment_cases()
        .into_iter()
        .filter(|case| case.scenario >= 96)
    {
        check_environment_facets(case);
    }
}

#[test]
fn truncated_environment_roots_never_publish_partial_results() {
    for case in environment_cases() {
        for length in (0..case.bytes.len())
            .step_by(113)
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
fn old_environment_contents_cannot_be_republished_with_required_tail_words() {
    for case in environment_cases()
        .into_iter()
        .filter(|case| case.version == 2277 && case.scenario == 96)
    {
        let source = parsed(case);
        let context = DisplayArchiveContext {
            version: DisplayVersion::new(2278),
            ..case_context(case)
        };
        assert!(DisplayArchive::new(source.contents().clone(), context, LIMITS).is_err());
        assert_eq!(source.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn complete_environment_documents_edit_reopen_and_revert() {
    for (kind, raw, expected) in DOCUMENTS {
        resumed_inline::check_owned_document(kind, raw, expected);
    }
}

const DOCUMENTS: [(DisplayDocumentKind, &[u8], &[u8]); 2] = [
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-environment-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-environment-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-environment-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-environment-edited-document-assembly.bin"),
    ),
];

const FACETS: [(u32, [[&[u8]; 2]; 2]); 5] = [
    (
        2277,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-environment-facet-2277-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-environment-facet-2277-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-environment-facet-2277-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-environment-facet-2277-1-1.bin"),
            ],
        ],
    ),
    (
        2278,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-environment-facet-2278-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-environment-facet-2278-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-environment-facet-2278-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-environment-facet-2278-1-1.bin"),
            ],
        ],
    ),
    (
        2304,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-environment-facet-2304-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-environment-facet-2304-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-environment-facet-2304-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-environment-facet-2304-1-1.bin"),
            ],
        ],
    ),
    (
        2305,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-environment-facet-2305-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-environment-facet-2305-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-environment-facet-2305-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-environment-facet-2305-1-1.bin"),
            ],
        ],
    ),
    (
        2359,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-environment-facet-2359-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-environment-facet-2359-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-environment-facet-2359-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-environment-facet-2359-1-1.bin"),
            ],
        ],
    ),
];
