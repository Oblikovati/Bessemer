//! Full roots cross rendering flags and both auxiliary facet-texture boundaries.
use super::*;
use bessemer::tessellation::{
    ArchiveVersion, FacetArchive, FacetArchiveMode, TessellationContext, TessellationLimits,
};

fn rendering_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-rendering-roots.bin"
    ))
}

#[test]
fn complete_rendering_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(rendering_cases().len(), 262);
    assert_eq!(
        rendering_cases().into_iter().map(check_case).sum::<usize>(),
        73967
    );
}

fn check_rendering_fields(case: OldCase) {
    let archive = parsed(case);
    ordinary_text::check_ordinary_gates(case);
    for member in &archive.contents().fields {
        if let Some(prefix) = member.path.strip_suffix(".material.user_name") {
            for name in [
                "use_advanced_graphics",
                "use_defined_texture",
                "use_defined_shader",
                "use_defined_color",
                "use_defined_hatch",
            ] {
                assert_eq!(
                    field(&archive, &format!("{prefix}.material.{name}")).is_some(),
                    case.version > 2388
                );
            }
        }
    }
}

#[test]
fn material_rendering_words_are_absent_through_2388() {
    for case in rendering_cases() {
        check_rendering_fields(case);
    }
}

fn expected_rendering_edit(case: OldCase) -> OldCase {
    rendering_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

#[test]
fn full_material_text_edge_and_facet_edits_match_independent_bytes_and_revert() {
    for case in rendering_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3 | 32 | 48 | 64 | 80 | 96 | 112))
    {
        ordinary_text::check_complete_edit(case, expected_rendering_edit(case));
    }
}

fn facet_context(version: u32) -> TessellationContext {
    TessellationContext {
        archive_version: ArchiveVersion::new(version),
        mode: FacetArchiveMode::Full,
    }
}

fn expected_facet(version: u32, index: usize, edited: usize) -> FacetArchive {
    let bytes = facet_bytes(version, index, edited);
    let limits = TessellationLimits {
        maximum_components: 0,
        maximum_faces: 1,
        maximum_array_elements: 1000,
    };
    FacetArchive::from_bytes(bytes, facet_context(version), limits).unwrap()
}

fn facet_bytes(version: u32, index: usize, edited: usize) -> &'static [u8] {
    FACETS.iter().find(|row| row.0 == version).unwrap().1[index][edited]
}

fn check_inline_texture(case: OldCase) {
    let source = parsed(case);
    let facets = resumed_inline::inline_facets(&source);
    assert_eq!(facets.len(), 2);
    let edited = usize::from(case.scenario % 16 == 1);
    for (index, facet) in facets.into_iter().enumerate() {
        assert_eq!(
            facet,
            expected_facet(case.version, index, edited).contents()
        );
        let forced = FacetArchive::new(facet_context(case.version), facet.clone()).unwrap();
        assert_eq!(
            forced.to_bytes().unwrap(),
            facet_bytes(case.version, index, edited)
        );
        assert_eq!(facet.texture_tessellation.is_some(), case.version > 2406);
        check_texture_vectors(facet, case.version, index);
    }
}

#[test]
fn complete_inline_facets_match_independent_versioned_records() {
    for case in rendering_cases()
        .into_iter()
        .filter(|case| case.scenario >= 96)
    {
        check_inline_texture(case);
    }
}

#[test]
fn truncated_rendering_roots_never_publish_partial_results() {
    for case in rendering_cases() {
        for length in (0..case.bytes.len())
            .step_by(107)
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
fn complete_rendering_documents_edit_reopen_and_revert() {
    for (kind, raw, expected) in DOCUMENTS {
        resumed_inline::check_owned_document(kind, raw, expected);
    }
}

const DOCUMENTS: [(DisplayDocumentKind, &[u8], &[u8]); 2] = [
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-rendering-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-rendering-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-rendering-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-rendering-edited-document-assembly.bin"),
    ),
];

const FACETS: [(u32, [[&[u8]; 2]; 2]); 7] = [
    (
        2388,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2388-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2388-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2388-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2388-1-1.bin"),
            ],
        ],
    ),
    (
        2389,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2389-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2389-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2389-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2389-1-1.bin"),
            ],
        ],
    ),
    (
        2406,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2406-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2406-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2406-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2406-1-1.bin"),
            ],
        ],
    ),
    (
        2407,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2407-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2407-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2407-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2407-1-1.bin"),
            ],
        ],
    ),
    (
        2417,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2417-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2417-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2417-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2417-1-1.bin"),
            ],
        ],
    ),
    (
        2418,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2418-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2418-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2418-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2418-1-1.bin"),
            ],
        ],
    ),
    (
        2563,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2563-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2563-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2563-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-rendering-facet-2563-1-1.bin"),
            ],
        ],
    ),
];

fn check_texture_vectors(
    facet: &bessemer::tessellation::FacetContainer,
    version: u32,
    index: usize,
) {
    if let Some(texture) = &facet.texture_tessellation {
        assert_eq!(texture.value.is_some(), index == 1);
        if let Some(payload) = &texture.value {
            assert_eq!(payload.vectors.is_some(), version > 2417);
            if let Some(vectors) = &payload.vectors {
                assert!(vectors.value.is_some());
            }
        }
    }
}
