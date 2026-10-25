//! All old-light classes keep their legacy bases, names and derived layouts.
use super::*;

fn old_light_kind(scenario: u8) -> usize {
    match scenario {
        16..=20 => usize::from(scenario - 16),
        144..=153 => usize::from((scenario - 144) / 2),
        _ => 2,
    }
}

const OLD_LIGHT_WORDS: [&str; 6] = [
    "unknown_old_a8",
    "unknown_old_a0",
    "unknown_old_b0",
    "unknown_old_88[0]",
    "unknown_old_50[0]",
    "unknown_old_68[0]",
];
const LIGHT_CLASSES: [&str; 5] = [
    "moAmbientLight_c",
    "moLightFeature_c",
    "moDirectionLight_c",
    "moSpotLight_c",
    "moPointLight_c",
];

fn check_old_light_base(archive: &DisplayArchive, prefix: &str, version: u32) {
    for member in OLD_LIGHT_WORDS {
        assert_eq!(
            field(archive, &format!("{prefix}.{member}")).is_some(),
            version <= 1063
        );
    }
    for member in ["in_model_space", "unknown_364"] {
        assert_eq!(
            field(archive, &format!("{prefix}.{member}")).is_some(),
            version > 1063
        );
    }
    check_old_light_names(archive, prefix, version);
}

fn check_old_light_names(archive: &DisplayArchive, prefix: &str, version: u32) {
    assert_eq!(
        field(archive, &format!("{prefix}.use_cart_display")).is_some(),
        version >= 1106
    );
    assert_eq!(
        field(archive, &format!("{prefix}.user_name")).is_some(),
        !(1031..=1107).contains(&version)
    );
}

fn check_old_light_derived(archive: &DisplayArchive, prefix: &str, version: u32, kind: usize) {
    for member in [
        "position[0]",
        "target[0]",
        "cone_angle",
        "exponent",
        "direction[0]",
    ] {
        let present = match member {
            "direction[0]" => kind == 2 && version > 1063,
            "position[0]" => matches!(kind, 3 | 4) && version > 1063,
            "target[0]" | "cone_angle" => kind == 3 && version > 1063,
            _ => kind == 3 && version >= 1087,
        };
        assert_eq!(
            field(archive, &format!("{prefix}.{member}")).is_some(),
            present
        );
    }
}

fn check_complete_old_light(case: OldCase) {
    let archive = parsed(case);
    let mut definitions = 0;
    for member in &archive.contents().fields {
        if !member.path.starts_with("root.old_lights.items[") {
            continue;
        }
        if let Some(prefix) = member.path.strip_suffix(".ambient") {
            check_old_light_definition(&archive, prefix, case);
            definitions += 1;
        }
    }
    check_light_definition_count(&archive, definitions, case);
}

fn check_light_definition_count(archive: &DisplayArchive, definitions: usize, case: OldCase) {
    let present = matches!(field(archive,"root.old_lights.has_lights"),Some(DisplayFieldValue::I32(value)) if *value != 0);
    assert_eq!(
        definitions,
        if present { 2 } else { 0 },
        "version {} kind {:?} scenario {}",
        case.version,
        case.kind,
        case.scenario
    );
}

fn check_old_light_definition(archive: &DisplayArchive, prefix: &str, case: OldCase) {
    let kind = if prefix.ends_with("items[0]") {
        0
    } else {
        old_light_kind(case.scenario)
    };
    let Some(DisplayFieldValue::ObjectTag(tag)) = field(archive, prefix) else {
        panic!("expected light object");
    };
    assert_eq!(
        tag.runtime_class.as_ref().unwrap().name,
        LIGHT_CLASSES[kind]
    );
    check_old_light_base(archive, prefix, case.version);
    check_old_light_derived(archive, prefix, case.version, kind);
}

#[test]
fn every_old_light_class_keeps_its_base_names_and_subtype_fields_at_each_boundary() {
    for case in model_viz_cases() {
        check_complete_old_light(case);
    }
}

fn check_old_light_transition(case: OldCase) {
    let source = parsed(case);
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(case.version + 1),
        ..case_context(case)
    };
    let result = DisplayArchive::new(source.contents().clone(), context, LIMITS);
    if case.version == 1086 && old_light_kind(case.scenario) != 3 {
        assert_eq!(result.unwrap().to_bytes().unwrap(), case.bytes);
    } else {
        assert!(result.is_err());
    }
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn contexts_keep_unchanged_light_subtypes_compatible_and_reject_missing_or_extra_fields() {
    for case in model_viz_cases().into_iter().filter(|case| {
        matches!(case.version, 1030 | 1063 | 1086 | 1105 | 1107)
            && matches!(case.scenario, 144 | 146 | 148 | 150 | 152)
    }) {
        check_old_light_transition(case);
    }
}
