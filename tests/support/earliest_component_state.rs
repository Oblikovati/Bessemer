//! Missing component members remain distinct from stored zero and null members.
use super::*;

fn check_component_members(case: OldCase) {
    let archive = parsed(case);
    let prefix = "root.tessellation.components[0]";
    for (member, limit) in [
        ("visible", 0xdca),
        ("ref_configuration_id", 0xdde),
        ("file_type", 0xdde),
    ] {
        let path = format!("{prefix}.viewer_comp_data.{member}");
        assert_eq!(has_field(&archive, &path), case.version > limit);
    }
    assert_eq!(
        has_field(&archive, &format!("{prefix}.children.count_word")),
        case.version > 0xdde
    );
    assert_eq!(
        has_field(&archive, &format!("{prefix}.owner")),
        case.version > 0xdde
    );
}

#[test]
fn component_visibility_configuration_and_hierarchy_follow_their_own_gates() {
    let selected = cases()
        .into_iter()
        .filter(|case| case.kind == DisplayDocumentKind::Assembly && case.scenario >= 208);
    for case in selected {
        check_component_members(case);
    }
}

fn check_component_context_gate(case: OldCase, before: u32, after: u32) {
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
    assert_eq!(result.is_err(), case.kind == DisplayDocumentKind::Assembly);
    if let Ok(rebuilt) = result {
        assert_eq!(rebuilt.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn component_boundaries_reject_changed_assembly_shapes_and_keep_part_bytes() {
    for (before, after) in [(0xdca, 0xdcb), (0xdde, 0xddf), (0xde2, 0xde3)] {
        let selected = cases()
            .into_iter()
            .filter(|case| [before, after].contains(&case.version) && case.scenario >= 208);
        for case in selected {
            check_component_context_gate(case, before, after);
        }
    }
}

fn hierarchy_template_case() -> OldCase {
    cases()
        .into_iter()
        .find(|case| {
            case.version == 0xddf
                && case.kind == DisplayDocumentKind::Assembly
                && case.scenario == 208
        })
        .unwrap()
}

fn hierarchy_template() -> Vec<DisplayField> {
    let archive = parsed(hierarchy_template_case());
    let paths = [
        "root.tessellation.components[0].children.count_word",
        "root.tessellation.components[0].owner",
    ];
    archive
        .contents()
        .fields
        .iter()
        .filter(|field| paths.contains(&field.path.as_ref()))
        .cloned()
        .collect()
}

#[test]
fn stored_empty_hierarchies_retain_their_zero_count_and_null_pointer() {
    let archive = parsed(hierarchy_template_case());
    let prefix = "root.tessellation.components[0]";
    let count = format!("{prefix}.children.count_word=i:0\n");
    assert_eq!(archive_support::check_scalar_table(&archive, &count), 1);
    assert_eq!(
        object_target(&archive, &format!("{prefix}.owner")),
        DisplayObjectPointer::Null
    );
}
fn reject_synthetic_hierarchy(case: OldCase) {
    let archive = parsed(case);
    let mut contents = archive.contents().clone();
    let after = "root.tessellation.components[0].unknown_320";
    let index = contents
        .fields
        .iter()
        .position(|field| field.path.as_ref() == after)
        .unwrap();
    let inserted = hierarchy_template();
    assert_eq!(inserted.len(), 2);
    contents.fields.splice(index + 1..index + 1, inserted);
    assert!(archive.with_contents(contents, LIMITS).is_err());
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}

#[test]
fn absent_hierarchies_cannot_be_replaced_with_stored_empty_arrays_and_null_owners() {
    let selected = cases().into_iter().filter(|case| {
        case.version == 3508 && case.kind == DisplayDocumentKind::Assembly && case.scenario == 208
    });
    for case in selected {
        reject_synthetic_hierarchy(case);
    }
}
