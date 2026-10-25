//! Related old polygon, body and inline facet layouts in complete roots.
use super::*;
use bessemer::tessellation::{FacetArchive, TessellationLimits};

fn boundary_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-boundary-roots.bin"
    ))
}

#[test]
fn complete_boundary_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(boundary_cases().len(), 370);
    assert_eq!(
        boundary_cases().into_iter().map(check_case).sum::<usize>(),
        99441
    );
}

fn check_boundary_members(case: OldCase) {
    let archive = parsed(case);
    legacy_model::check_model_gates(case);
    for member in &archive.contents().fields {
        if member.path.contains(".bodies[") && member.path.ends_with(".unknown_20") {
            let path = member.path.replace(".unknown_20", ".unknown_24");
            assert_eq!(field(&archive, &path).is_some(), case.version >= 2091);
        }
        if let Some(prefix) = member.path.strip_suffix(".smooth") {
            check_polygon_fields(&archive, prefix, case.version);
        }
    }
}

fn check_polygon_fields(archive: &DisplayArchive, prefix: &str, version: u32) {
    let count = field(archive, &format!("{prefix}.count")).unwrap();
    let DisplayFieldValue::I32(count) = count else {
        panic!("expected polygon count");
    };
    for (name, threshold) in [("new_boundary", 2078), ("atom_id", 2101)] {
        assert_eq!(
            field(archive, &format!("{prefix}.{name}")).is_some(),
            *count > 0 && version > threshold
        );
    }
}

#[test]
fn body_and_polygon_words_follow_their_independent_storage_bounds() {
    for case in boundary_cases() {
        check_boundary_members(case);
    }
}

fn expected_boundary_edit(case: OldCase) -> OldCase {
    boundary_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

#[test]
fn complete_boundary_text_edge_and_facet_edits_match_independent_bytes_and_revert() {
    for case in boundary_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3 | 32 | 48 | 64 | 80 | 96 | 112))
    {
        ordinary_text::check_complete_edit(case, expected_boundary_edit(case));
    }
}

fn facet_bytes(version: u32, index: usize, edited: usize) -> &'static [u8] {
    FACETS.iter().find(|row| row.0 == version).unwrap().1[index][edited]
}

fn expected_facet(version: u32, index: usize, edited: usize) -> FacetArchive {
    let limits = TessellationLimits {
        maximum_components: 0,
        maximum_faces: 1,
        maximum_array_elements: 1000,
    };
    FacetArchive::from_bytes(
        facet_bytes(version, index, edited),
        legacy_rendering::facet_context(version),
        limits,
    )
    .unwrap()
}

fn check_boundary_facets(case: OldCase) {
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
    assert_eq!(facet.num_facets, 1 + edited as i32);
    assert_eq!(facet.stamp.is_some(), case.version >= 2090);
    assert_eq!(facet.cone_cull.is_some(), case.version >= 2107);
}

#[test]
fn complete_inline_facets_match_independent_records_before_stamps_and_cone_culling() {
    for case in boundary_cases()
        .into_iter()
        .filter(|case| case.scenario >= 96)
    {
        check_boundary_facets(case);
    }
}

#[test]
fn truncated_boundary_roots_never_publish_partial_results() {
    for case in boundary_cases() {
        for length in (0..case.bytes.len())
            .step_by(131)
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
fn context_changes_require_new_polygon_body_and_facet_fields_at_each_boundary() {
    for case in boundary_cases().into_iter().filter(|case| {
        matches!(case.version, 2078 | 2089 | 2090 | 2101 | 2106) && case.scenario == 96
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
fn complete_boundary_documents_edit_reopen_and_revert() {
    for (kind, raw, expected) in DOCUMENTS {
        resumed_inline::check_owned_document(kind, raw, expected);
    }
}

const DOCUMENTS: [(DisplayDocumentKind, &[u8], &[u8]); 2] = [
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-boundary-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-boundary-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-boundary-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-boundary-edited-document-assembly.bin"),
    ),
];

const FACETS: [(u32, [[&[u8]; 2]; 2]); 10] = [
    (
        2078,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2078-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2078-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2078-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2078-1-1.bin"),
            ],
        ],
    ),
    (
        2079,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2079-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2079-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2079-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2079-1-1.bin"),
            ],
        ],
    ),
    (
        2089,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2089-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2089-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2089-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2089-1-1.bin"),
            ],
        ],
    ),
    (
        2090,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2090-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2090-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2090-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2090-1-1.bin"),
            ],
        ],
    ),
    (
        2091,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2091-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2091-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2091-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2091-1-1.bin"),
            ],
        ],
    ),
    (
        2101,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2101-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2101-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2101-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2101-1-1.bin"),
            ],
        ],
    ),
    (
        2102,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2102-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2102-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2102-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2102-1-1.bin"),
            ],
        ],
    ),
    (
        2106,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2106-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2106-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2106-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2106-1-1.bin"),
            ],
        ],
    ),
    (
        2107,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2107-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2107-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2107-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2107-1-1.bin"),
            ],
        ],
    ),
    (
        2130,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2130-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2130-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2130-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-boundary-facet-2130-1-1.bin"),
            ],
        ],
    ),
];
