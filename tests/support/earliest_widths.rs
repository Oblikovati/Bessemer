//! The restore caller determines the physical legacy line-width integer size.
use super::*;

fn width_field<'a>(archive: &'a DisplayArchive, path: &str) -> &'a DisplayFieldValue {
    &archive
        .contents()
        .fields
        .iter()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value
}

#[test]
fn font_widths_remain_signed_words_without_synthetic_custom_widths() {
    let prefix = "root.line_font_mgr.line_fonts[0].value";
    for case in cases() {
        let archive = parsed(case);
        let standard = width_field(&archive, &format!("{prefix}.standard_width"));
        assert!(matches!(standard, DisplayFieldValue::I16(_)));
        assert_eq!(
            has_field(&archive, &format!("{prefix}.custom_width")),
            case.version >= 0xf41
        );
    }
}

fn check_pen_width(case: OldCase, index: usize, positive: bool) {
    let archive = parsed(case);
    let member = if positive { "width" } else { "line_width" };
    let prefix = format!("root.views[0].display_items.items[{index}].data.{member}");
    let value = width_field(&archive, &format!("{prefix}.standard_width"));
    let expected = pen_boundary_value(case, positive);
    if case.version < 0xf41 {
        assert_eq!(value, &DisplayFieldValue::I32(expected));
    } else {
        assert!(matches!(value, DisplayFieldValue::I16(_)));
    }
    assert_eq!(
        has_field(&archive, &format!("{prefix}.custom_width")),
        case.version >= 0xf41
    );
}

#[test]
fn legacy_display_item_pens_retain_full_range_signed_integers() {
    for case in cases()
        .into_iter()
        .filter(|case| [240, 241].contains(&case.scenario))
    {
        check_pen_width(case, 6, false);
        check_pen_width(case, 7, true);
    }
}

#[test]
fn polygon_properties_use_integer_widths_while_embedded_fonts_use_words() {
    let prefix = "root.views[0].view_bucket.bucket.cthread_maps[0].entries[0].value.items[0].user_line_property.width";
    for case in cases()
        .into_iter()
        .filter(|case| case.version < 0xf41 && case.scenario < 208)
    {
        let archive = parsed(case);
        let value = width_field(&archive, &format!("{prefix}.standard_width"));
        assert_eq!(value, &DisplayFieldValue::I32(i32::MIN));
        assert!(!has_field(&archive, &format!("{prefix}.custom_width")));
        assert!(embedded_word_widths(&archive));
    }
}

fn embedded_word_widths(archive: &DisplayArchive) -> bool {
    archive
        .contents()
        .fields
        .iter()
        .filter(|field| {
            field.path.contains(".line_fonts[") && field.path.ends_with(".standard_width")
        })
        .all(|field| matches!(field.value, DisplayFieldValue::I16(_)))
}

#[test]
fn old_and_new_width_layouts_require_different_owned_field_shapes() {
    for case in cases()
        .into_iter()
        .filter(|case| [0xf40, 0xf41].contains(&case.version))
    {
        let archive = parsed(case);
        let version = if case.version == 0xf40 { 0xf41 } else { 0xf40 };
        let context = DisplayArchiveContext {
            version: DisplayVersion::new(version),
            ..case_context(case)
        };
        assert!(DisplayArchive::new(archive.contents().clone(), context, LIMITS).is_err());
    }
}

fn pen_boundary_value(case: OldCase, positive: bool) -> i32 {
    let edited = i32::from(case.scenario == 241);
    if positive {
        return i32::MAX - edited;
    }
    i32::MIN + edited
}

fn edited_width_case(case: OldCase) -> OldCase {
    cases()
        .into_iter()
        .find(|edited| {
            edited.version == case.version && edited.kind == case.kind && edited.scenario == 241
        })
        .unwrap()
}

fn replace_pen_widths(contents: &mut DisplayArchiveContents, expected: &DisplayArchive) {
    for (index, member) in [(6, "line_width"), (7, "width")] {
        let path =
            format!("root.views[0].display_items.items[{index}].data.{member}.standard_width");
        let field = contents
            .fields
            .iter_mut()
            .find(|field| field.path.as_ref() == path)
            .unwrap();
        field.value = width_field(expected, &path).clone();
    }
}

fn check_independent_pen_edit(case: OldCase) {
    let source = parsed(case);
    let expected_case = edited_width_case(case);
    let expected = parsed(expected_case);
    let mut contents = source.contents().clone();
    replace_pen_widths(&mut contents, &expected);
    let edited = source.with_contents(contents, LIMITS).unwrap();
    assert_eq!(edited.to_bytes().unwrap(), expected_case.bytes);
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
    let reverted = edited
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    assert_eq!(reverted.to_bytes().unwrap(), case.bytes);
}

#[test]
fn pen_edits_match_independent_bytes_keep_the_source_and_authentically_revert() {
    for case in cases().into_iter().filter(|case| case.scenario == 240) {
        check_independent_pen_edit(case);
    }
}
