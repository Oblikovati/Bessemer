//! Ordinary text width records and material classification follow independent gates.
use super::*;

fn ordinary_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/ordinary-text-roots.bin"
    ))
}

#[test]
fn complete_ordinary_text_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(ordinary_cases().len(), 158);
    assert_eq!(
        ordinary_cases().into_iter().map(check_case).sum::<usize>(),
        43403
    );
}

fn check_ordinary_gates(case: OldCase) {
    let archive = parsed(case);
    for index in 9..13 {
        let prefix = format!("root.views[0].display_items.items[{index}].data");
        assert_eq!(
            field(&archive, &format!("{prefix}.has_widths")).is_some(),
            case.version > 2563
        );
        check_ordinary_word_count(&archive, &prefix, case.version, index);
    }
    check_material_gates(&archive, case.version);
}

#[test]
fn ordinary_widths_and_material_classification_are_absent_through_their_bounds() {
    for case in ordinary_cases() {
        check_ordinary_gates(case);
    }
}

fn check_ordinary_edit(case: OldCase) {
    let source = parsed(case);
    let expected_case = expected_ordinary_edit(case);
    let expected = parsed(expected_case);
    let contents = resumed_inline::resumed_edited_contents(&source, &expected);
    let edited = source.with_contents(contents, LIMITS).unwrap();
    assert_eq!(edited.to_bytes().unwrap(), expected_case.bytes);
    assert_eq!(
        edited
            .with_contents(source.contents().clone(), LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        case.bytes
    );
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn ordinary_text_and_inline_facet_edits_match_independent_bytes_and_revert() {
    for case in ordinary_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3 | 32 | 48 | 64 | 80 | 96 | 112))
    {
        check_ordinary_edit(case);
    }
}

#[test]
fn complete_old_inline_faces_remain_independently_verified() {
    for case in ordinary_cases()
        .into_iter()
        .filter(|case| case.scenario >= 96)
    {
        resumed_inline::check_inline_payloads(case);
    }
}

#[test]
fn truncated_ordinary_text_roots_never_publish_partial_results() {
    for case in ordinary_cases() {
        for length in (0..case.bytes.len())
            .step_by(97)
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
fn complete_ordinary_text_documents_edit_reopen_and_revert() {
    for (kind, raw, expected) in ORDINARY_DOCUMENTS {
        resumed_inline::check_owned_document(kind, raw, expected);
    }
}

const ORDINARY_DOCUMENTS: [(DisplayDocumentKind, &[u8], &[u8]); 2] = [
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/ordinary-text-document-part.bin"),
        include_bytes!("../fixtures/display-lists/ordinary-text-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/ordinary-text-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/ordinary-text-edited-document-assembly.bin"),
    ),
];

fn expected_ordinary_edit(case: OldCase) -> OldCase {
    ordinary_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

fn check_ordinary_word_count(archive: &DisplayArchive, prefix: &str, version: u32, index: usize) {
    let width_prefix = format!("{prefix}.unknown_48[");
    let widths = archive
        .contents()
        .fields
        .iter()
        .filter(|field| field.path.starts_with(&width_prefix))
        .count();
    assert_eq!(
        widths,
        if version > 2563 && index.is_multiple_of(2) {
            5
        } else {
            0
        }
    );
}

fn check_material_gates(archive: &DisplayArchive, version: u32) {
    for member in &archive.contents().fields {
        if let Some(prefix) = member.path.strip_suffix(".material.user_name") {
            let classification = format!("{prefix}.material.classification");
            assert_eq!(field(archive, &classification).is_some(), version > 2565);
        }
    }
}
