//! Optional material bodies and fixed property arrays retain their stored boundaries.
use super::*;

fn check_material_members(case: OldCase) {
    let archive = parsed(case);
    let prefix = "root.visual_properties.material";
    assert!(has_field(
        &archive,
        &format!("{prefix}.texture.ignore_color_material")
    ));
    for suffix in [
        "has_rv_app_props",
        "rv_app_props.face_width",
        "texture.texture_trans_x",
        "texture.texture_trans_y",
    ] {
        assert_eq!(
            has_field(&archive, &format!("{prefix}.{suffix}")),
            case.version > 0xf1d
        );
    }
}

#[test]
fn early_materials_omit_the_rv_presence_word_body_and_texture_translations() {
    for case in cases().into_iter().filter(|case| case.scenario >= 208) {
        check_material_members(case);
    }
}

fn check_property_font_boundary(case: OldCase) {
    let archive = parsed(case);
    let prefix = "root.views[0].view_bucket.bucket.cthread_maps[0].entries[0].key";
    let property = if case.scenario == 16 {
        format!("{prefix}.property")
    } else {
        prefix.to_owned()
    };
    let count = if case.version < 0xf32 { 15 } else { 23 };
    assert!(has_field(
        &archive,
        &format!("{property}.line_fonts[{}].name", count - 1)
    ));
    assert!(!has_field(
        &archive,
        &format!("{property}.line_fonts[{count}].name")
    ));
}

#[test]
fn fixed_embedded_font_arrays_grow_from_fifteen_to_twenty_three_at_f32() {
    let selected = cases()
        .into_iter()
        .filter(|case| [0xf31, 0xf32].contains(&case.version) && case.scenario < 208);
    for case in selected {
        check_property_font_boundary(case);
    }
}

fn reject_other_material_layout(case: OldCase, version: u32) {
    let archive = parsed(case);
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(version),
        ..case_context(case)
    };
    assert!(DisplayArchive::new(archive.contents().clone(), context, LIMITS).is_err());
}

#[test]
fn material_and_font_shapes_cannot_be_republished_across_their_boundaries() {
    for case in cases() {
        match (case.version, case.scenario) {
            (0xf1d, 208..) => reject_other_material_layout(case, 0xf1e),
            (0xf1e, 208..) => reject_other_material_layout(case, 0xf1d),
            (0xf31, 0 | 16) => reject_other_material_layout(case, 0xf32),
            (0xf32, 0 | 16) => reject_other_material_layout(case, 0xf31),
            _ => {}
        }
    }
}

#[test]
fn an_absent_rv_flag_cannot_be_replaced_with_a_stored_false_word() {
    for case in cases()
        .into_iter()
        .filter(|case| case.version == 3800 && case.scenario == 208)
    {
        let archive = parsed(case);
        let mut contents = archive.contents().clone();
        insert_false_rv_word(&mut contents);
        assert!(archive.with_contents(contents, LIMITS).is_err());
    }
}

fn insert_false_rv_word(contents: &mut DisplayArchiveContents) {
    let prefix = "root.visual_properties.material";
    let index = contents
        .fields
        .iter()
        .position(|field| field.path.as_ref() == format!("{prefix}.use_new_shaders"))
        .unwrap();
    contents.fields.insert(
        index + 1,
        DisplayField {
            path: format!("{prefix}.has_rv_app_props").into(),
            value: DisplayFieldValue::I32(0),
        },
    );
}
