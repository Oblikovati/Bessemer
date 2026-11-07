use super::property_domains_3086_tests::{
    assert_property_splice, assert_signed_property_domain, assert_stored_double_domain,
};
use super::property_domains_3087_tests::{byte_domain, integer_family};
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::records_with_count;
use super::{ModelField, ModelFieldValue};

fn draft_multiple_angles_domain(version: u32, fields: &[ModelField], field: &ModelField) -> usize {
    let ModelFieldValue::I32(original) = field.value else {
        panic!("{}: expected signed32 MultipleAngles", field.path);
    };
    let values: &[i32] = if original == 0 {
        &[0]
    } else {
        &[i32::MIN, -1, 1, 2, i32::MAX]
    };
    for value in values {
        let expected = ModelFieldValue::I32(*value);
        assert_property_splice(version, fields, &field.path, &value.to_le_bytes(), expected);
    }
    values.len()
}

fn draft_multiple_angles_patches() -> usize {
    let packet = include_bytes!("../fixtures/drafts/cases.bin");
    let mut patches = 0;
    for (_, version, raw, _, _) in records_with_count(packet, b"DRF1", 90) {
        let fields = decode_graph(version, raw, true).unwrap().0;
        for field in fields
            .iter()
            .filter(|field| field.path.ends_with(".payload.raw_870"))
        {
            patches += draft_multiple_angles_domain(version, &fields, field);
        }
    }
    assert!(patches > 0);
    patches
}

#[test]
fn draft_named_settings_and_preconversion_word_preserve_signed_values_and_array_shape() {
    let packet = include_bytes!("../fixtures/drafts/cases.bin");
    let labels = [
        "raw_830", "raw_840", "raw_848", "raw_880", "raw_810", "raw_814", "raw_844",
    ];
    let words = integer_family(packet, b"DRF1", 90, &labels) * 6;
    let selectors = draft_multiple_angles_patches();
    eprintln!("DRAFT_PROPERTY_WORD_PATCHES\t{words}\nDRAFT_MULTIPLE_ANGLE_PATCHES\t{selectors}");
}

#[test]
fn shell_outward_keeps_every_saved_octet_without_trimming_following_lists() {
    let packet = include_bytes!("../fixtures/shell-features/cases.bin");
    let mut patches = 0;
    for (_, version, raw, _, _) in records_with_count(packet, b"SHF1", 18) {
        let fields = decode_graph(version, raw, true).unwrap().0;
        for field in fields
            .iter()
            .filter(|field| field.path.ends_with(".payload.raw_818"))
        {
            byte_domain(version, &fields, &field.path);
            patches += 256;
        }
    }
    assert!(patches > 0);
    eprintln!("SHELL_OUTWARD_BYTE_PATCHES\t{patches}");
}

fn scale_property_patches(fields: &[ModelField], field: &ModelField) -> usize {
    if ["root.object.raw_808", "root.object.raw_80c"].contains(&field.path.as_ref()) {
        assert_signed_property_domain(19000, fields, &field.path);
        return 6;
    }
    if field.path.starts_with("root.object.vector_810[") {
        assert_stored_double_domain(19000, fields, &field.path);
        return 12;
    }
    0
}

#[test]
fn scale_properties_preserve_signed_settings_and_independent_ieee_vector_components() {
    let cases = [
        include_bytes!("../fixtures/resolved-features/scale-empty.bin").as_slice(),
        include_bytes!("../fixtures/resolved-features/scale-coordinate.bin").as_slice(),
    ];
    let mut patches = 0;
    for raw in cases {
        let fields = decode_graph(19000, raw, true).unwrap().0;
        for field in &fields {
            patches += scale_property_patches(&fields, field);
        }
    }
    assert_eq!(patches, 96);
    eprintln!("SCALE_PROPERTY_PATCHES\t{patches}");
}
