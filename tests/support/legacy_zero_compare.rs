//! Compare all retained typed fields against the complete version-596 control.
use super::*;

fn visual_prefixes(archive: &DisplayArchive) -> Vec<&str> {
    archive
        .contents()
        .fields
        .iter()
        .filter_map(|member| {
            let DisplayFieldValue::ObjectTag(tag) = &member.value else {
                return None;
            };
            let class = tag.runtime_class.as_ref()?;
            (class.name == "moVisualProperties_c"
                && matches!(tag.target, DisplayObjectPointer::New(_)))
            .then_some(member.path.as_ref())
        })
        .collect()
}

fn retained_appearance(path: &str, version: u32, visuals: &[&str]) -> bool {
    for (threshold, member) in [(144, "color"), (211, "wire_color"), (172, "hlr_color")] {
        if version <= threshold
            && visuals
                .iter()
                .any(|prefix| path == format!("{prefix}.{member}"))
        {
            return false;
        }
    }
    if version <= 370
        && (path.ends_with(".material.transparency") || path.ends_with(".material.emission"))
    {
        return false;
    }
    retained_light(path, version)
}

fn retained_light(path: &str, version: u32) -> bool {
    if version > 380 || !path.starts_with("root.old_lights.items[") {
        return true;
    }
    !path.ends_with(".id_name")
        && !path.ends_with(".user_name")
        && !path.contains(".user_property.")
}

fn retained_ui(path: &str, version: u32) -> bool {
    !(version <= 437 && path.contains(".panes[") && path.ends_with(".perspective"))
        && !(version <= 595 && path.contains(".dl_bucket."))
        && !(version <= 561 && path.contains(".edge_vis_data[") && path.ends_with(".unknown_c"))
}

pub(super) fn check_unchanged_fields(case: OldCase) {
    let source = parsed(case);
    let control = parsed(control_case(case));
    let visuals = visual_prefixes(&control);
    let expected: Vec<DisplayField> = control
        .contents()
        .fields
        .iter()
        .filter(|member| {
            retained_ui(&member.path, case.version)
                && retained_appearance(&member.path, case.version, &visuals)
        })
        .cloned()
        .collect();
    assert_eq!(source.contents().fields, expected);
}
