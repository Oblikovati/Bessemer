//! Shader-related words start at separate native layout boundaries.
use super::*;

fn require_shader_neighbors(archive: &DisplayArchive) {
    for member in [
        "appearance_name",
        "material.classification",
        "material.user_property.count",
    ] {
        let path = format!("root.visual_properties.{member}");
        assert!(has_field(archive, &path));
    }
}

fn check_shader_members(case: OldCase) {
    let archive = parsed(case);
    require_shader_neighbors(&archive);
    for (member, limit) in [
        ("hardcoded_facet_display", 0xdb1),
        ("material.use_new_shaders", 0xdb3),
    ] {
        let path = format!("root.visual_properties.{member}");
        assert_eq!(has_field(&archive, &path), case.version > limit);
    }
}
#[test]
fn facet_and_shader_words_have_independent_boundaries_without_removing_neighboring_fields() {
    for case in cases().into_iter().filter(|case| case.scenario >= 208) {
        check_shader_members(case);
    }
}

fn check_shader_context(case: OldCase, before: u32, after: u32) {
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
    let result = DisplayArchive::new(archive.contents().clone(), context, LIMITS);
    assert_eq!(
        result.is_err(),
        has_field(&archive, "root.visual_properties")
    );
    if let Ok(rebuilt) = result {
        assert_eq!(rebuilt.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn context_changes_reject_changed_material_shapes_and_preserve_unshaded_roots() {
    for (before, after) in [(0xdb1, 0xdb2), (0xdb3, 0xdb4)] {
        let selected = cases()
            .into_iter()
            .filter(|case| [before, after].contains(&case.version));
        for case in selected {
            check_shader_context(case, before, after);
        }
    }
}

fn inserted_zero_word(
    archive: &DisplayArchive,
    after: &str,
    member: &str,
) -> DisplayArchiveContents {
    let mut contents = archive.contents().clone();
    let index = contents
        .fields
        .iter()
        .position(|field| field.path.as_ref() == after)
        .unwrap();
    let field = DisplayField {
        path: member.into(),
        value: DisplayFieldValue::I32(0),
    };
    contents.fields.insert(index + 1, field);
    contents
}

fn reject_synthetic_shader_words(case: OldCase) {
    let archive = parsed(case);
    for (after, member) in [
        ("appearance_name", "hardcoded_facet_display"),
        ("material.classification", "material.use_new_shaders"),
    ] {
        reject_inserted_shader_word(&archive, after, member);
    }
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}

#[test]
fn missing_facet_and_shader_words_cannot_be_replaced_with_stored_zero_values() {
    let selected = cases()
        .into_iter()
        .filter(|case| case.version == 3330 && case.scenario == 208);
    for case in selected {
        reject_synthetic_shader_words(case);
    }
}

fn reject_inserted_shader_word(archive: &DisplayArchive, after: &str, member: &str) {
    let prefix = "root.visual_properties";
    let contents = inserted_zero_word(
        archive,
        &format!("{prefix}.{after}"),
        &format!("{prefix}.{member}"),
    );
    assert!(archive.with_contents(contents, LIMITS).is_err());
}
