//! Complete archives preserve old curvature arrays without interpreting their elements.
use super::*;
use bessemer::tessellation::{
    ArrayElementSize, DynamicArray, DynamicArrayValues, FacetArchive, FacetContainer,
    FacetCurvature, Flagged, StoredFloat, TessellationLimits,
};
#[path = "legacy_curvature_facets.rs"]
mod facets;

fn curvature_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-curvature-roots.bin"
    ))
}

fn legacy_array(index: usize, edited: usize) -> DynamicArray {
    let count = 1 + index + edited;
    let mut payload: Vec<u8> = (0..count).flat_map(|_| 0..24).collect();
    if edited != 0 {
        *payload.last_mut().unwrap() = 255;
    }
    DynamicArray::Present(DynamicArrayValues {
        element_size: ArrayElementSize::new(24).unwrap(),
        initial_capacity: 17 + edited as u32,
        growth_step: 23 + edited as u32,
        count: count as u32,
        payload,
    })
}

fn expected_curvature(version: u32, index: usize, edited: usize) -> FacetCurvature {
    if version <= 816 {
        return FacetCurvature::Legacy(legacy_array(index, edited));
    }
    FacetCurvature::Calculated(Flagged {
        flag: if index == 0 { 0 } else { -7 },
        value: (index != 0).then(|| {
            vec![
                StoredFloat::from_bits(0x7fc0_0042),
                StoredFloat::from_bits(0x8000_0000),
            ]
        }),
    })
}

fn expected_facet(case: OldCase, index: usize, edited: usize) -> FacetArchive {
    FacetArchive::from_bytes(
        facets::curvature_facet_bytes(case.version, index, edited),
        legacy_rendering::facet_context(case.version),
        TessellationLimits {
            maximum_components: 0,
            maximum_faces: 1,
            maximum_array_elements: 1000,
        },
    )
    .unwrap()
}

pub(super) fn check_curvature_facet(case: OldCase, facet: &FacetContainer, index: usize) {
    let edited = usize::from(case.scenario % 16 == 1);
    assert_eq!(
        facet.curvature,
        expected_curvature(case.version, index, edited)
    );
    assert_eq!(facet, expected_facet(case, index, edited).contents());
    let rebuilt =
        FacetArchive::new(legacy_rendering::facet_context(case.version), facet.clone()).unwrap();
    assert_eq!(
        rebuilt.to_bytes().unwrap(),
        facets::curvature_facet_bytes(case.version, index, edited)
    );
}

#[test]
fn complete_curvature_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(curvature_cases().len(), 290);
    assert_eq!(
        curvature_cases().into_iter().map(check_case).sum::<usize>(),
        86015
    );
}

#[test]
fn prior_component_environment_bucket_and_text_gates_still_hold_for_earlier_contexts() {
    for case in curvature_cases() {
        legacy_component::check_component_members(case);
    }
}

#[test]
fn legacy_arrays_and_calculated_flags_match_explicit_metadata_payload_and_float_oracles() {
    for case in curvature_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 96 | 97 | 112 | 113))
    {
        let archive = parsed(case);
        let facets = resumed_inline::inline_facets(&archive);
        assert_eq!(facets.len(), 2);
        for (index, facet) in facets.into_iter().enumerate() {
            check_curvature_facet(case, facet, index);
        }
    }
}

fn expected_curvature_edit(case: OldCase) -> OldCase {
    curvature_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

#[test]
fn complete_array_metadata_payload_text_and_light_edits_match_independent_bytes_and_revert() {
    for case in curvature_cases().into_iter().filter(|case| {
        matches!(
            case.scenario,
            0 | 3 | 32 | 48 | 64 | 80 | 96 | 112 | 144 | 146 | 148 | 150 | 152
        )
    }) {
        ordinary_text::check_complete_edit(case, expected_curvature_edit(case));
    }
}

#[test]
fn truncated_curvature_roots_never_publish_partial_results() {
    for case in curvature_cases() {
        for length in (0..case.bytes.len())
            .step_by(173)
            .chain(case.bytes.len() - 8..case.bytes.len())
        {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..length], case_context(case), LIMITS)
                    .is_err()
            );
        }
    }
}

fn check_curvature_context(case: OldCase) {
    let source = parsed(case);
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(if case.version == 816 { 817 } else { 816 }),
        ..case_context(case)
    };
    let result = DisplayArchive::new(source.contents().clone(), context, LIMITS);
    if matches!(case.scenario, 96 | 112) {
        let error = result.unwrap_err();
        assert!(error.cause.contains("curvature"));
    } else {
        assert_eq!(result.unwrap().to_bytes().unwrap(), case.bytes);
    }
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn contexts_require_the_right_curvature_representation_only_for_inline_facets() {
    for case in curvature_cases().into_iter().filter(|case| {
        matches!(case.version, 816 | 817) && matches!(case.scenario, 0 | 3 | 96 | 112)
    }) {
        check_curvature_context(case);
    }
}

#[test]
fn legacy_and_calculated_inline_arrays_reconstruct_identically_across_threads() {
    for case in curvature_cases()
        .into_iter()
        .filter(|case| case.scenario == 112)
    {
        resumed_inline::concurrent_inline_case(case);
    }
}

#[test]
fn legacy_array_documents_edit_reopen_and_authentically_revert() {
    for (kind, raw, expected) in DOCUMENTS {
        resumed_inline::check_owned_document(kind, raw, expected);
    }
}

const DOCUMENTS: [(DisplayDocumentKind, &[u8], &[u8]); 4] = [
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-curvature-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-curvature-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-curvature-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-curvature-edited-document-assembly.bin"),
    ),
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-curvature-rich-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-curvature-rich-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-curvature-rich-document-assembly.bin"),
        include_bytes!(
            "../fixtures/display-lists/legacy-curvature-rich-edited-document-assembly.bin"
        ),
    ),
];
