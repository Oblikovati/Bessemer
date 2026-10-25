//! Complete material-texture and shaded-edge layouts before their stored flags.
use super::*;
use bessemer::tessellation::FacetArchive;

fn texture_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-texture-roots.bin"
    ))
}

#[test]
fn complete_texture_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(texture_cases().len(), 190);
    assert_eq!(
        texture_cases().into_iter().map(check_case).sum::<usize>(),
        52439
    );
}

fn check_texture_gates(case: OldCase) {
    let archive = parsed(case);
    legacy_rendering::check_rendering_fields(case);
    assert_eq!(
        field(&archive, "root.views[0].edges_in_shaded_mode").is_some(),
        case.version > 2374
    );
    for member in &archive.contents().fields {
        if let Some(prefix) = member.path.strip_suffix(".material.user_name") {
            assert_eq!(
                field(&archive, &format!("{prefix}.material.has_texture")).is_some(),
                case.version > 2359
            );
            assert_eq!(
                field(&archive, &format!("{prefix}.material.texture.unknown_8")).is_some(),
                case.version > 2359
            );
        }
    }
}

#[test]
fn material_texture_and_view_edge_fields_are_physically_absent_at_their_bounds() {
    for case in texture_cases() {
        check_texture_gates(case);
    }
}

fn expected_texture_edit(case: OldCase) -> OldCase {
    texture_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

#[test]
fn full_text_edge_and_facet_edits_match_independent_bytes_and_revert() {
    for case in texture_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3 | 32 | 48 | 64 | 80 | 96 | 112))
    {
        ordinary_text::check_complete_edit(case, expected_texture_edit(case));
    }
}

fn check_texture_facets(case: OldCase) {
    let source = parsed(case);
    let facets = resumed_inline::inline_facets(&source);
    assert_eq!(facets.len(), 2);
    let context = legacy_rendering::facet_context(case.version);
    for (index, facet) in facets.into_iter().enumerate() {
        let expected =
            legacy_rendering::expected_facet(2388, index, usize::from(case.scenario % 16 == 1));
        assert_eq!(facet, expected.contents());
        assert_eq!(
            FacetArchive::new(context, facet.clone())
                .unwrap()
                .to_bytes()
                .unwrap(),
            expected.to_bytes().unwrap()
        );
    }
}

#[test]
fn complete_inline_facets_retain_the_independent_pre_texture_layout() {
    for case in texture_cases()
        .into_iter()
        .filter(|case| case.scenario >= 96)
    {
        check_texture_facets(case);
    }
}

#[test]
fn truncated_texture_roots_never_publish_partial_results() {
    for case in texture_cases() {
        for length in (0..case.bytes.len())
            .step_by(109)
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
fn older_roots_cannot_be_republished_with_newly_required_texture_or_edge_words() {
    for case in texture_cases()
        .into_iter()
        .filter(|case| case.scenario == 96 && matches!(case.version, 2359 | 2374))
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
fn complete_texture_documents_edit_reopen_and_revert() {
    for (kind, raw, expected) in DOCUMENTS {
        resumed_inline::check_owned_document(kind, raw, expected);
    }
}

const DOCUMENTS: [(DisplayDocumentKind, &[u8], &[u8]); 2] = [
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-texture-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-texture-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-texture-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-texture-edited-document-assembly.bin"),
    ),
];
