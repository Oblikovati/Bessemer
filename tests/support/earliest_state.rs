//! Earlier appearance and component identifiers are absent rather than zero-filled.
use super::*;

#[test]
fn stored_appearance_words_begin_at_e55() {
    for case in cases().into_iter().filter(|case| case.scenario >= 208) {
        let archive = parsed(case);
        assert!(has_field(
            &archive,
            "root.visual_properties.hardcoded_facet_display"
        ));
        let path = "root.visual_properties.unknown_old_5c";
        assert_eq!(
            has_field(&archive, path),
            (0xe55..0x1052).contains(&case.version)
        );
    }
}

#[test]
fn component_reference_ids_begin_after_e11_without_removing_neighboring_words() {
    for case in cases()
        .into_iter()
        .filter(|case| case.kind == DisplayDocumentKind::Assembly && case.scenario >= 208)
    {
        let archive = parsed(case);
        let prefix = "root.tessellation.components[0].viewer_comp_data";
        assert!(has_field(&archive, &format!("{prefix}.has_pattern_ref")));
        assert!(has_field(&archive, &format!("{prefix}.unknown_1a8")));
        assert_eq!(
            has_field(&archive, &format!("{prefix}.reference_id")),
            case.version > 0xe11
        );
    }
}

fn check_state_context_boundary(case: OldCase, before: u32, after: u32) {
    let archive = parsed(case);
    let version = if case.version == before {
        after
    } else {
        before
    };
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(version),
        ..case_context(case)
    };
    let changed = before == 0xe54 || case.kind == DisplayDocumentKind::Assembly;
    let result = DisplayArchive::new(archive.contents().clone(), context, LIMITS);
    assert_eq!(result.is_err(), changed);
    if let Ok(rebuilt) = result {
        assert_eq!(rebuilt.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn state_boundaries_require_only_the_fields_stored_for_each_document_kind() {
    for (before, after) in [(0xe11, 0xe12), (0xe54, 0xe55)] {
        let selected = cases()
            .into_iter()
            .filter(|case| [before, after].contains(&case.version) && case.scenario >= 208);
        for case in selected {
            check_state_context_boundary(case, before, after);
        }
    }
}

fn inserted_appearance_word(archive: &DisplayArchive) -> DisplayArchiveContents {
    let mut contents = archive.contents().clone();
    let after = "root.visual_properties.hardcoded_facet_display";
    let index = contents
        .fields
        .iter()
        .position(|field| field.path.as_ref() == after)
        .unwrap();
    let field = DisplayField {
        path: "root.visual_properties.unknown_old_5c".into(),
        value: DisplayFieldValue::I32(0),
    };
    contents.fields.insert(index + 1, field);
    contents
}

#[test]
fn a_missing_appearance_word_cannot_be_replaced_with_a_stored_zero() {
    for case in cases()
        .into_iter()
        .filter(|case| case.version == 3555 && case.scenario == 208)
    {
        let archive = parsed(case);
        assert!(
            archive
                .with_contents(inserted_appearance_word(&archive), LIMITS)
                .is_err()
        );
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}
