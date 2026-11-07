//! Earlier environments and property graphs have distinct stored boundaries.
use super::*;

#[test]
fn earlier_environment_scene_and_assembly_words_are_absent_until_their_gates() {
    for case in cases().into_iter().filter(|case| case.scenario >= 208) {
        let archive = parsed(case);
        assert_eq!(
            has_field(&archive, "root.user_model_env.unknown_1c0"),
            case.version > 0xf55
        );
        assert_eq!(has_field(&archive, "root.scene_type"), case.version > 0xf5e);
        if case.kind == DisplayDocumentKind::Assembly {
            let path = "root.tessellation.components[0].has_shrink_wrap_config";
            assert_eq!(has_field(&archive, path), case.version > 0xf76);
        }
    }
}

#[test]
fn bucket_properties_keep_each_versioned_complete_font_array() {
    for case in cases()
        .into_iter()
        .filter(|case| case.version <= 0xf5e && case.scenario < 208)
    {
        let archive = parsed(case);
        let last = embedded_font_exists(&archive, 23);
        let prior = embedded_font_exists(&archive, 22);
        assert!(embedded_font_exists(&archive, 14));
        assert_eq!(prior, case.version >= 0xf32);
        assert_eq!(last, case.version >= 0xf5c);
        assert!(!has_field(&archive, "root.scene_type"));
    }
}

fn embedded_font_exists(archive: &DisplayArchive, index: usize) -> bool {
    let suffix = format!(".line_fonts[{index}].name");
    archive
        .contents()
        .fields
        .iter()
        .any(|field| field.path.ends_with(&suffix))
}

fn check_bucket_holder(case: OldCase) {
    let archive = parsed(case);
    let path = "root.views[0].view_bucket.bucket.cthread_maps[0].entries[0].key";
    let DisplayObjectPointer::New(identity) = object_target(&archive, path) else {
        panic!("expected holder identity")
    };
    let reference = object_target(&archive, &format!("{path}.unknown_8"));
    assert_eq!(reference, DisplayObjectPointer::Reference(identity));
    let last = if case.version < 0xf32 { 14 } else { 22 };
    assert!(has_field(
        &archive,
        &format!("{path}.property.line_fonts[{last}].name")
    ));
}

#[test]
fn pre_scene_holders_keep_their_cobject_self_references_in_bucket_keys() {
    let matching = cases()
        .into_iter()
        .filter(|case| case.version <= 0xf5e && case.scenario == 16);
    for case in matching {
        check_bucket_holder(case);
    }
}
fn check_earlier_root_gate(case: OldCase, before: u32, after: u32) {
    let archive = parsed(case);
    let version = opposite_root_version(case.version, before, after);
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(version),
        ..case_context(case)
    };
    let changed = before != 0xf76 || case.kind == DisplayDocumentKind::Assembly;
    let result = DisplayArchive::new(archive.contents().clone(), context, LIMITS);
    assert_eq!(result.is_err(), changed, "{} {:?}", case.version, case.kind);
    if let Ok(rebuilt) = result {
        assert_eq!(rebuilt.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn earlier_root_gates_follow_the_stored_layout_for_each_document_kind() {
    for (before, after) in [(0xf55, 0xf56), (0xf5e, 0xf5f), (0xf76, 0xf77)] {
        let matching = cases()
            .into_iter()
            .filter(|case| [before, after].contains(&case.version) && case.scenario >= 208);
        for case in matching {
            check_earlier_root_gate(case, before, after);
        }
    }
}

fn opposite_root_version(version: u32, before: u32, after: u32) -> u32 {
    if version == before {
        return after;
    }
    before
}
