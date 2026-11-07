//! Explicit member-presence checks for every low-version branch.
use super::*;

fn check_visual(archive: &DisplayArchive, prefix: &str, version: u32) {
    for (threshold, member) in [(144, "color"), (211, "wire_color"), (172, "hlr_color")] {
        assert_eq!(
            field(archive, &format!("{prefix}.{member}")).is_some(),
            version > threshold
        );
    }
    for member in ["transparency", "emission"] {
        assert_eq!(
            field(archive, &format!("{prefix}.material.{member}")).is_some(),
            version > 370
        );
    }
    for member in ["ambient", "diffuse", "specular", "shininess"] {
        assert!(field(archive, &format!("{prefix}.material.{member}")).is_some());
    }
}

fn check_named_light(archive: &DisplayArchive, prefix: &str, version: u32) {
    for member in ["id_name", "user_name", "user_property.count"] {
        assert_eq!(
            field(archive, &format!("{prefix}.{member}")).is_some(),
            version > 380
        );
    }
}

fn check_pane_member(archive: &DisplayArchive, path: &str, version: u32) {
    if path.contains(".panes[")
        && let Some(prefix) = path.strip_suffix(".display_mode")
    {
        assert_eq!(
            field(archive, &format!("{prefix}.perspective")).is_some(),
            version > 437
        );
    }
}

fn check_ui_member(archive: &DisplayArchive, path: &str, version: u32) {
    check_pane_member(archive, path, version);
    if let Some(prefix) = path.strip_suffix(".tess_number") {
        assert_eq!(
            field(archive, &format!("{prefix}.dl_bucket.count_word")).is_some(),
            version > 595
        );
    }
    if path.contains(".edge_vis_data[")
        && let Some(prefix) = path.strip_suffix(".unknown_8")
    {
        assert_eq!(
            field(archive, &format!("{prefix}.unknown_c")).is_some(),
            version > 561
        );
    }
}

pub(super) fn check_members(case: OldCase) {
    let archive = parsed(case);
    for member in &archive.contents().fields {
        check_ui_member(&archive, &member.path, case.version);
        if member.path.starts_with("root.old_lights.items[")
            && let Some(prefix) = member.path.strip_suffix(".ambient")
        {
            check_named_light(&archive, prefix, case.version);
        }
        if let DisplayFieldValue::ObjectTag(tag) = &member.value
            && matches!(tag.target, DisplayObjectPointer::New(_))
            && tag
                .runtime_class
                .as_ref()
                .is_some_and(|class| class.name == "moVisualProperties_c")
        {
            check_visual(&archive, &member.path, case.version);
        }
    }
}

fn check_empty_edge_record(archive: &DisplayArchive, prefix: &str, version: u32, edited: bool) {
    for index in 0..128 {
        let row = format!("{prefix}.edge_vis_data[{index}]");
        assert_eq!(
            field(archive, &format!("{row}.polys.count")),
            Some(&DisplayFieldValue::I32(0))
        );
        assert_eq!(
            field(archive, &format!("{row}.unknown_8")),
            Some(&DisplayFieldValue::I32(if edited { 23 } else { -8 }))
        );
        assert_eq!(
            field(archive, &format!("{row}.unknown_c")).is_some(),
            version > 561
        );
    }
}

pub(super) fn check_minimal_edges(case: OldCase) {
    let archive = parsed(case);
    let mut buckets = 0;
    for member in &archive.contents().fields {
        if let Some(prefix) = member.path.strip_suffix(".edge_vis_data.count") {
            assert_eq!(member.value, DisplayFieldValue::I32(128));
            check_empty_edge_record(&archive, prefix, case.version, case.scenario == 189);
            buckets += 1;
        }
    }
    assert_eq!(buckets, 2);
}
