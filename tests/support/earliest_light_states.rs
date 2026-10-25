//! Every old-light class keeps independently gated PV and scene words.
use super::*;

fn check_named_light_state(archive: &DisplayArchive, index: usize, version: u32) {
    let prefix = format!("root.old_lights.items[{index}]");
    for member in ["user_name", "user_property.count"] {
        assert!(has_field(archive, &format!("{prefix}.{member}")));
    }
    for (member, limit) in [("enabled_in_pv", 0xcc0), ("keep_light_in_scene", 0xd01)] {
        let path = format!("{prefix}.{member}");
        assert_eq!(has_field(archive, &path), version > limit);
    }
}

fn check_light_state_members(case: OldCase) {
    let archive = parsed(case);
    for index in 0..2 {
        check_named_light_state(&archive, index, case.version);
    }
}
#[test]
fn enabled_and_keep_scene_words_follow_their_independent_boundaries() {
    for case in cases().into_iter().filter(|case| case.scenario >= 208) {
        check_light_state_members(case);
    }
}

const OLD_LIGHT_NAMES: [&str; 5] = [
    "moAmbientLight_c",
    "moLightFeature_c",
    "moDirectionLight_c",
    "moSpotLight_c",
    "moPointLight_c",
];

fn light_runtime_name(archive: &DisplayArchive) -> &str {
    let path = "root.old_lights.items[1]";
    let field = archive
        .contents()
        .fields
        .iter()
        .find(|field| field.path.as_ref() == path)
        .unwrap();
    let DisplayFieldValue::ObjectTag(tag) = &field.value else {
        panic!("expected light tag")
    };
    &tag.runtime_class.as_ref().unwrap().name
}

fn check_derived_light_members(archive: &DisplayArchive, kind: usize) {
    let prefix = "root.old_lights.items[1]";
    for (member, present) in [
        ("direction[0]", kind == 2),
        ("position[0]", kind >= 3),
        ("target[0]", kind == 3),
    ] {
        assert_eq!(has_field(archive, &format!("{prefix}.{member}")), present);
    }
    assert!(has_field(
        archive,
        &format!("{prefix}.user_property.entries[3].type")
    ));
}

fn check_light_class(case: OldCase) {
    let archive = parsed(case);
    let kind = usize::from((case.scenario - 224) / 2);
    assert_eq!(light_runtime_name(&archive), OLD_LIGHT_NAMES[kind]);
    check_derived_light_members(&archive, kind);
}
#[test]
fn all_five_classes_keep_their_identity_and_derived_geometry() {
    let selected = cases()
        .into_iter()
        .filter(|case| (224..234).contains(&case.scenario));
    assert_eq!(selected.clone().count(), 50);
    for case in selected {
        check_light_class(case);
    }
}

fn check_light_context_boundary(case: OldCase, before: u32, after: u32) {
    let archive = parsed(case);
    let version = opposite_light_version(case.version, before, after);
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(version),
        ..case_context(case)
    };
    let result = DisplayArchive::new(archive.contents().clone(), context, LIMITS);
    assert_eq!(
        result.is_err(),
        has_field(&archive, "root.old_lights.items[0].user_name")
    );
    if let Ok(rebuilt) = result {
        assert_eq!(rebuilt.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn light_boundaries_reject_changed_shapes_and_preserve_unshaded_roots() {
    for (before, after) in [(0xcc0, 0xcc1), (0xd01, 0xd02)] {
        let selected = cases()
            .into_iter()
            .filter(|case| [before, after].contains(&case.version));
        for case in selected {
            check_light_context_boundary(case, before, after);
        }
    }
}

fn reject_synthetic_light_word(archive: &DisplayArchive, member: &str) {
    let mut contents = archive.contents().clone();
    let prefix = "root.old_lights.items[1]";
    let index = contents
        .fields
        .iter()
        .position(|field| field.path.as_ref() == format!("{prefix}.user_name"))
        .unwrap();
    let field = DisplayField {
        path: format!("{prefix}.{member}").into(),
        value: DisplayFieldValue::I32(0),
    };
    contents.fields.insert(index + 1, field);
    assert!(archive.with_contents(contents, LIMITS).is_err());
}

#[test]
fn missing_light_state_words_cannot_be_replaced_with_stored_zeros() {
    let selected = cases()
        .into_iter()
        .filter(|case| case.version == 3238 && (224..234).contains(&case.scenario));
    for case in selected {
        let archive = parsed(case);
        reject_synthetic_light_word(&archive, "enabled_in_pv");
        reject_synthetic_light_word(&archive, "keep_light_in_scene");
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}

fn opposite_light_version(version: u32, before: u32, after: u32) -> u32 {
    if version == before {
        return after;
    }
    before
}
