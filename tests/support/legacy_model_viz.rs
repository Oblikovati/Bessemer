//! Complete old model visual pointers, pane quality and facet edge-array gates.
use super::*;
use bessemer::tessellation::{FacetArchive, FacetContainer};
#[path = "legacy_model_viz_facets.rs"]
mod facets;
#[path = "legacy_old_lights.rs"]
mod lights;

pub(super) fn model_viz_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-model-viz-roots.bin"
    ))
}

#[test]
fn complete_model_viz_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(model_viz_cases().len(), 962);
    assert_eq!(
        model_viz_cases().into_iter().map(check_case).sum::<usize>(),
        284873
    );
}

fn check_model_viz_members(case: OldCase) {
    let archive = parsed(case);
    legacy_scope::check_early_scope_members(case);
    for member in &archive.contents().fields {
        if let Some(prefix) = member.path.strip_suffix(".bodies.count_word") {
            assert_eq!(
                field(&archive, &format!("{prefix}.model_viz_props[0].value")).is_some(),
                case.version >= 998
            );
        }
        if member.path.contains(".panes[") {
            check_model_viz_pane(&archive, &member.path, case.version);
        }
    }
}

fn check_model_viz_pane(archive: &DisplayArchive, path: &str, version: u32) {
    let Some(prefix) = path.strip_suffix(".display_mode") else {
        return;
    };
    assert_eq!(
        field(archive, &format!("{prefix}.hlr_display_quality")).is_some(),
        version > 1116
    );
    assert!(field(archive, &format!("{prefix}.perspective")).is_some());
}

#[test]
fn model_visual_pointers_and_pane_quality_words_follow_their_independent_storage_bounds() {
    for case in model_viz_cases() {
        check_model_viz_members(case);
    }
}

fn expected_model_viz_edit(case: OldCase) -> OldCase {
    model_viz_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

#[test]
fn complete_old_light_model_text_and_facet_edits_match_independent_bytes_and_revert() {
    for case in model_viz_cases().into_iter().filter(|case| {
        matches!(
            case.scenario,
            0 | 3 | 32 | 48 | 64 | 80 | 96 | 112 | 144 | 146 | 148 | 150 | 152
        )
    }) {
        ordinary_text::check_complete_edit(case, expected_model_viz_edit(case));
    }
}

fn check_model_viz_facets(case: OldCase) {
    let source = parsed(case);
    let payloads = resumed_inline::inline_facets(&source);
    assert_eq!(payloads.len(), 2);
    for (index, facet) in payloads.into_iter().enumerate() {
        check_model_viz_facet_record(facet, case, index);
    }
}

fn check_model_viz_facet_record(facet: &FacetContainer, case: OldCase, index: usize) {
    let edited = usize::from(case.scenario % 16 == 1);
    assert_eq!(
        facet,
        facets::expected_model_viz_facet(case.version, index, edited).contents()
    );
    let rebuilt =
        FacetArchive::new(legacy_rendering::facet_context(case.version), facet.clone()).unwrap();
    assert_eq!(
        rebuilt.to_bytes().unwrap(),
        facets::model_viz_facet_bytes(case.version, index, edited)
    );
    assert_eq!(facet.edge_arrays.is_some(), case.version >= 990);
}

#[test]
fn complete_inline_facets_match_independent_records_before_edge_arrays() {
    for case in model_viz_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 96 | 97 | 112 | 113))
    {
        check_model_viz_facets(case);
    }
}

#[test]
fn truncated_model_viz_roots_never_publish_partial_results() {
    for case in model_viz_cases() {
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
fn new_contexts_require_model_pointers_pane_words_and_facet_edges() {
    for case in model_viz_cases()
        .into_iter()
        .filter(|case| matches!(case.version, 989 | 997 | 1116) && case.scenario == 96)
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
fn old_and_stored_model_visual_documents_edit_reopen_and_revert() {
    for (kind, raw, expected) in DOCUMENTS {
        resumed_inline::check_owned_document(kind, raw, expected);
    }
}

const DOCUMENTS: [(DisplayDocumentKind, &[u8], &[u8]); 4] = [
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-model-viz-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-model-viz-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-model-viz-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-model-viz-edited-document-assembly.bin"),
    ),
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-model-viz-rich-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-model-viz-rich-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-model-viz-rich-document-assembly.bin"),
        include_bytes!(
            "../fixtures/display-lists/legacy-model-viz-rich-edited-document-assembly.bin"
        ),
    ),
];
