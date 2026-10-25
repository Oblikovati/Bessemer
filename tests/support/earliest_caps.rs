//! Stored caps and pointers are distinct from the reference reader's defaults.
use super::*;

#[test]
fn font_caps_and_front_transforms_follow_separate_boundaries() {
    for case in cases() {
        let archive = parsed(case);
        let cap = "root.line_font_mgr.line_fonts[0].value.end_cap";
        assert_eq!(has_field(&archive, cap), case.version >= 0xc9f);
        assert_eq!(
            has_field(&archive, "root.front_view_transform"),
            case.version > 0xc9c
        );
    }
}

#[test]
fn bucket_pointer_is_absent_before_its_gate_and_stored_null_after_it() {
    for case in cases()
        .into_iter()
        .filter(|case| case.scenario < 208 && case.version <= 0xf5e)
    {
        let archive = parsed(case);
        let path = "root.views[0].view_bucket.visual_properties";
        assert_eq!(has_field(&archive, path), case.version > 0xc90);
        if case.version > 0xc90 && case.version <= 0xf5e {
            assert_eq!(object_target(&archive, path), DisplayObjectPointer::Null);
        }
    }
}

#[test]
fn pen_caps_keep_the_layer_and_style_fields_at_their_original_positions() {
    for case in cases().into_iter().filter(|case| case.scenario == 240) {
        check_pen_caps(case);
    }
}

fn check_pen_caps(case: OldCase) {
    let archive = parsed(case);
    let pen = "root.views[0].display_items.items[6].data";
    let user = "root.views[0].display_items.items[7].data";
    let caps = [
        (format!("{pen}.end_cap"), case.version > 0xca5),
        (format!("{user}.end_cap_style"), case.version >= 0xca6),
    ];
    for (path, present) in caps {
        assert_eq!(has_field(&archive, &path), present);
    }
    assert!(has_field(&archive, &format!("{user}.layer_id")));
    assert!(has_field(&archive, &format!("{user}.style")));
}

fn opposite_cap_boundary(version: u32) -> u32 {
    match version {
        0xc90 => 0xc91,
        0xc91 => 0xc90,
        0xc9c => 0xc9d,
        0xc9d => 0xc9c,
        0xc9e => 0xc9f,
        0xc9f => 0xc9e,
        0xca5 => 0xca6,
        _ => 0xca5,
    }
}

fn boundary_changes_case(case: OldCase) -> bool {
    match case.version {
        0xc90 | 0xc91 => case.scenario < 208,
        0xca5 | 0xca6 => case.scenario >= 240,
        _ => true,
    }
}

#[test]
fn cap_boundaries_reject_incompatible_shapes_and_preserve_unchanged_roots() {
    let boundaries = [0xc90, 0xc91, 0xc9c, 0xc9d, 0xc9e, 0xc9f, 0xca5, 0xca6];
    for case in cases()
        .into_iter()
        .filter(|case| boundaries.contains(&case.version))
    {
        check_cap_context(case);
    }
}

fn check_cap_context(case: OldCase) {
    let archive = parsed(case);
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(opposite_cap_boundary(case.version)),
        ..case_context(case)
    };
    let result = DisplayArchive::new(archive.contents().clone(), context, LIMITS);
    assert_eq!(result.is_err(), boundary_changes_case(case));
    if let Ok(rebuilt) = result {
        assert_eq!(rebuilt.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn reference_default_font_cap_cannot_be_published_as_a_stored_zero() {
    for case in cases().into_iter().filter(|case| case.version == 3196) {
        let archive = parsed(case);
        let contents = synthetic_font_cap(&archive);
        assert!(archive.with_contents(contents, LIMITS).is_err());
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}

fn synthetic_font_cap(archive: &DisplayArchive) -> DisplayArchiveContents {
    let mut contents = archive.contents().clone();
    let prefix = "root.line_font_mgr.line_fonts[0].value";
    let index = contents
        .fields
        .iter()
        .position(|field| field.path.as_ref() == format!("{prefix}.standard_width"))
        .unwrap();
    let cap = DisplayField {
        path: format!("{prefix}.end_cap").into(),
        value: DisplayFieldValue::I16(0),
    };
    contents.fields.insert(index + 1, cap);
    contents
}
