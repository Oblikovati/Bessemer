//! Complete model and component payloads without unstored old metadata arrays.
use super::*;
use bessemer::tessellation::FacetArchive;

fn model_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-model-roots.bin"
    ))
}

#[test]
fn complete_model_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(model_cases().len(), 226);
    assert_eq!(
        model_cases().into_iter().map(check_case).sum::<usize>(),
        60857
    );
}

pub(super) fn check_model_gates(case: OldCase) {
    legacy_texture::check_texture_gates(case);
    check_model_members(&parsed(case), case.version);
}

fn check_model_members(archive: &DisplayArchive, version: u32) {
    for member in &archive.contents().fields {
        if let Some(prefix) = member.path.strip_suffix(".ddata_time_stamp") {
            assert_eq!(
                field(archive, &format!("{prefix}.unknown_30.count")).is_some(),
                version > 2130
            );
        }
        if let Some(prefix) = member.path.strip_suffix(".num_user_comp_bodies") {
            assert_eq!(
                field(archive, &format!("{prefix}.gobbled_up_atom_ids.count")).is_some(),
                version > 2148
            );
        }
        if member.path.contains(".bodies[") && member.path.ends_with(".unknown_24") {
            let path = member.path.replace(".unknown_24", ".unknown_2c");
            assert_eq!(field(archive, &path).is_some(), version > 2130);
        }
    }
}

#[test]
fn body_part_and_component_metadata_follow_their_independent_storage_bounds() {
    for case in model_cases() {
        check_model_gates(case);
    }
}

fn expected_model_edit(case: OldCase) -> OldCase {
    model_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

#[test]
fn complete_model_text_edge_and_facet_edits_match_independent_bytes_and_revert() {
    for case in model_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3 | 32 | 48 | 64 | 80 | 96 | 112))
    {
        ordinary_text::check_complete_edit(case, expected_model_edit(case));
    }
}

fn check_model_facets(case: OldCase) {
    let source = parsed(case);
    let facets = resumed_inline::inline_facets(&source);
    assert_eq!(facets.len(), 2);
    let context = legacy_rendering::facet_context(case.version);
    for (index, facet) in facets.into_iter().enumerate() {
        let expected =
            legacy_environment::expected_facet(2277, index, usize::from(case.scenario % 16 == 1));
        assert_eq!(facet, expected.contents());
        let rebuilt = FacetArchive::new(context, facet.clone()).unwrap();
        assert_eq!(rebuilt.to_bytes().unwrap(), expected.to_bytes().unwrap());
    }
}

#[test]
fn complete_old_inline_faces_match_the_independent_pre_coordinate_records() {
    for case in model_cases().into_iter().filter(|case| case.scenario >= 96) {
        check_model_facets(case);
    }
}

#[test]
fn truncated_model_roots_never_publish_partial_results() {
    for case in model_cases() {
        for length in (0..case.bytes.len())
            .step_by(127)
            .chain(case.bytes.len() - 8..case.bytes.len())
        {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..length], case_context(case), LIMITS)
                    .is_err()
            );
        }
    }
}

fn check_model_transition(case: OldCase) {
    let source = parsed(case);
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(case.version + 1),
        ..case_context(case)
    };
    let result = DisplayArchive::new(source.contents().clone(), context, LIMITS);
    if case.version == 2148 && case.kind == DisplayDocumentKind::Part {
        assert_eq!(result.unwrap().to_bytes().unwrap(), case.bytes);
    } else {
        assert!(result.is_err());
    }
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn context_changes_require_new_model_fields_only_when_their_payload_is_reachable() {
    for case in model_cases()
        .into_iter()
        .filter(|case| matches!(case.version, 2130 | 2148) && case.scenario == 96)
    {
        check_model_transition(case);
    }
}

#[test]
fn complete_model_documents_edit_reopen_and_revert() {
    for (kind, raw, expected) in DOCUMENTS {
        resumed_inline::check_owned_document(kind, raw, expected);
    }
}

const DOCUMENTS: [(DisplayDocumentKind, &[u8], &[u8]); 2] = [
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-model-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-model-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-model-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-model-edited-document-assembly.bin"),
    ),
];
