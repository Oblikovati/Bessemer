//! Normalize only the independently measured removed font subtree's map slots.
use super::*;

fn shifted_index(index: u32) -> u32 {
    assert!(!(3..46).contains(&index));
    if index >= 46 { index - 43 } else { index }
}

fn shifted_tag(tag: &DisplayObjectTag) -> DisplayObjectTag {
    let mut result = tag.clone();
    result.target = match tag.target {
        DisplayObjectPointer::Null => DisplayObjectPointer::Null,
        DisplayObjectPointer::New(identity) => DisplayObjectPointer::New(
            DisplayObjectId::new(shifted_index(identity.value())).unwrap(),
        ),
        DisplayObjectPointer::Reference(identity) => DisplayObjectPointer::Reference(
            DisplayObjectId::new(shifted_index(identity.value())).unwrap(),
        ),
    };
    if tag.tag == 0x7fff {
        let raw = tag.extended.unwrap();
        result.extended = Some((raw & 0x8000_0000) | shifted_index(raw & 0x7fff_ffff));
    } else if !matches!(tag.tag, 0 | 0xffff) {
        result.tag = (tag.tag & 0x8000) | shifted_index(u32::from(tag.tag & 0x7fff)) as u16;
    }
    result
}

fn retained_control_field(member: &DisplayField, version: u32) -> bool {
    !matches!(member.value, DisplayFieldValue::Facet(_))
        && !(version <= 766 && member.path.starts_with("root.line_font_mgr"))
        && !(version <= 760
            && (member.path.ends_with(".has_user_line_property")
                || member.path.contains(".user_line_property.")))
}

fn normalized_control_field(member: &DisplayField, version: u32) -> DisplayField {
    let mut result = member.clone();
    if version <= 766
        && let DisplayFieldValue::ObjectTag(tag) = &member.value
    {
        result.value = DisplayFieldValue::ObjectTag(shifted_tag(tag));
    }
    result
}

fn loaded_objects(archive: &DisplayArchive) -> usize {
    archive
        .contents()
        .fields
        .iter()
        .filter(|member| {
            matches!(&member.value, DisplayFieldValue::ObjectTag(tag)
            if matches!(tag.target, DisplayObjectPointer::New(_)))
        })
        .count()
}

pub(super) fn check_control_graph(case: OldCase) {
    let source = parsed(case);
    let control = parsed(control_case(case));
    assert_eq!(
        without_facets(&source),
        expected_survivors(&control, case.version)
    );
    assert_eq!(
        loaded_objects(&source),
        loaded_objects(&control) - 41 * usize::from(case.version <= 766)
    );
}

fn expected_survivors(control: &DisplayArchive, version: u32) -> Vec<DisplayField> {
    control
        .contents()
        .fields
        .iter()
        .filter(|member| retained_control_field(member, version))
        .map(|member| normalized_control_field(member, version))
        .collect()
}

fn without_facets(source: &DisplayArchive) -> Vec<DisplayField> {
    source
        .contents()
        .fields
        .iter()
        .filter(|member| !matches!(member.value, DisplayFieldValue::Facet(_)))
        .cloned()
        .collect()
}
