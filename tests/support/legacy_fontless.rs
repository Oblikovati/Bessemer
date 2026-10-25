//! Font-manager omission preserves graph meaning after compact and extended tag relocation.
use super::*;
use bessemer::tessellation::{DynamicArray, FacetArchive, FacetCurvature, TessellationLimits};
#[path = "legacy_fontless_facets.rs"]
mod facets;
#[path = "legacy_fontless_graph.rs"]
mod graph;

fn fontless_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-fontless-roots.bin"
    ))
}

fn control_case(case: OldCase) -> OldCase {
    fontless_cases()
        .into_iter()
        .find(|row| row.version == 767 && row.kind == case.kind && row.scenario == case.scenario)
        .unwrap()
}

#[test]
fn all_fontless_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(fontless_cases().len(), 618);
    assert_eq!(
        fontless_cases().into_iter().map(check_case).sum::<usize>(),
        164547
    );
}

#[test]
fn removing_font_slots_preserves_all_remaining_fields_and_relocates_every_graph_tag() {
    for case in fontless_cases()
        .into_iter()
        .filter(|case| !matches!(case.scenario, 16..=20))
    {
        graph::check_control_graph(case);
    }
}

fn check_fontless_members(case: OldCase) {
    let archive = parsed(case);
    assert_eq!(
        field(&archive, "root.line_font_mgr").is_some(),
        case.version > 766
    );
    for member in &archive.contents().fields {
        if let Some(prefix) = member.path.strip_suffix(".smooth") {
            let count = field(&archive, &format!("{prefix}.count"));
            let populated = matches!(count, Some(DisplayFieldValue::I32(value)) if *value > 0);
            assert_eq!(
                field(&archive, &format!("{prefix}.has_user_line_property")).is_some(),
                populated && case.version > 760
            );
        }
    }
}

#[test]
fn font_manager_and_polygon_line_properties_follow_their_independent_gates() {
    for case in fontless_cases() {
        check_fontless_members(case);
    }
}

fn expected_fontless_edit(case: OldCase) -> OldCase {
    fontless_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

const EDIT_SCENARIOS: [u8; 18] = [
    0, 3, 32, 48, 64, 80, 96, 112, 144, 146, 148, 150, 152, 160, 162, 164, 176, 180,
];

#[test]
fn relocated_graphs_support_complete_independent_edits_and_exact_reverts_without_source_mutation() {
    for case in fontless_cases()
        .into_iter()
        .filter(|case| EDIT_SCENARIOS.contains(&case.scenario))
    {
        ordinary_text::check_complete_edit(case, expected_fontless_edit(case));
    }
}

fn check_fontless_facet(
    case: OldCase,
    facet: &bessemer::tessellation::FacetContainer,
    index: usize,
) {
    let edited = usize::from(case.scenario % 2 == 1);
    let expected = facets::expected_fontless_facet(case.version, index, edited);
    assert_eq!(facet, expected.contents());
    let FacetCurvature::Legacy(DynamicArray::Present(values)) = &facet.curvature else {
        panic!("expected complete legacy array");
    };
    let width = if case.version < 717 { 4 } else { 24 };
    facets::check_array_values(values, width, index, edited);
    let rebuilt =
        FacetArchive::new(legacy_rendering::facet_context(case.version), facet.clone()).unwrap();
    assert_eq!(
        rebuilt.to_bytes().unwrap(),
        facets::fontless_facet_bytes(case.version, index, edited)
    );
}

#[test]
fn narrow_and_wide_legacy_arrays_keep_explicit_metadata_payload_and_complete_facet_records() {
    for case in fontless_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 96 | 97 | 112 | 113))
    {
        let archive = parsed(case);
        let faces = resumed_inline::inline_facets(&archive);
        assert_eq!(faces.len(), 2);
        for (index, facet) in faces.into_iter().enumerate() {
            check_fontless_facet(case, facet, index);
        }
    }
}

#[test]
fn extended_class_and_object_references_keep_their_framing_after_the_font_slots_are_removed() {
    for case in fontless_cases()
        .into_iter()
        .filter(|case| case.scenario == 180)
    {
        let archive = parsed(case);
        check_extended_tags(&archive);
    }
}

fn check_extended_tags(archive: &DisplayArchive) {
    let tags = object_tags(archive);
    assert!(
        tags.iter()
            .any(|tag| tag.extended.is_some_and(|word| word & 0x8000_0000 != 0))
    );
    assert!(
        tags.iter()
            .any(|tag| tag.extended.is_some_and(|word| word & 0x8000_0000 == 0))
    );
}

fn object_tags(archive: &DisplayArchive) -> Vec<&DisplayObjectTag> {
    archive
        .contents()
        .fields
        .iter()
        .filter_map(|member| {
            if let DisplayFieldValue::ObjectTag(tag) = &member.value {
                Some(tag)
            } else {
                None
            }
        })
        .collect()
}

fn check_fontless_context(case: OldCase) {
    let source = parsed(case);
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(case.version + 1),
        ..case_context(case)
    };
    let result = DisplayArchive::new(source.contents().clone(), context, LIMITS);
    if case.version == 766 || (case.version == 760 && case.scenario == 112) {
        assert!(result.is_err());
    } else {
        assert_eq!(result.unwrap().to_bytes().unwrap(), case.bytes);
    }
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn new_contexts_require_omitted_members_but_do_not_impose_an_unknown_array_element_domain() {
    for case in fontless_cases()
        .into_iter()
        .filter(|case| matches!(case.version, 716 | 760 | 766) && matches!(case.scenario, 0 | 112))
    {
        check_fontless_context(case);
    }
}

#[test]
fn incomplete_fontless_roots_never_publish_partial_results() {
    for case in fontless_cases() {
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

#[test]
fn fontless_inline_arrays_and_extended_graphs_are_deterministic_across_threads() {
    for case in fontless_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 112 | 180))
    {
        resumed_inline::concurrent_inline_case(case);
    }
}

#[test]
fn documents_without_font_managers_edit_reopen_and_authentically_revert() {
    for (kind, raw, expected) in DOCUMENTS {
        resumed_inline::check_owned_document(kind, raw, expected);
    }
}

const DOCUMENTS: [(DisplayDocumentKind, &[u8], &[u8]); 4] = [
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-fontless-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-fontless-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-fontless-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-fontless-edited-document-assembly.bin"),
    ),
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-fontless-rich-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-fontless-rich-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-fontless-rich-document-assembly.bin"),
        include_bytes!(
            "../fixtures/display-lists/legacy-fontless-rich-edited-document-assembly.bin"
        ),
    ),
];
