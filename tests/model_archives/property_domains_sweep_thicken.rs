use super::property_domains_3086_tests::{
    assert_property_splice, assert_signed_property_domain, assert_stored_double_domain,
};
use super::property_domains_3087_tests::byte_domain;
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::records_with_count;
use super::{ModelField, ModelFieldValue};

fn sweep_saved_byte_domain(version: u32, fields: &[ModelField], field: &ModelField) -> usize {
    if field.path.ends_with(".raw_78") {
        byte_domain(version, fields, &field.path);
        return 256;
    }
    let ModelFieldValue::U8(original) = field.value else {
        panic!("{}: expected saved thin-sweep octet", field.path);
    };
    let values = if original == 0 { 0..=0 } else { 1..=255 };
    for value in values {
        assert_property_splice(
            version,
            fields,
            &field.path,
            &[value],
            ModelFieldValue::U8(value),
        );
    }
    if original == 0 { 1 } else { 255 }
}

fn sweep_property_patches(version: u32, fields: &[ModelField], field: &ModelField) -> usize {
    if field.path.ends_with(".raw_230") {
        assert_signed_property_domain(version, fields, &field.path);
        return 6;
    }
    if field.path.ends_with(".raw_78") || field.path.ends_with(".raw_234") {
        return sweep_saved_byte_domain(version, fields, field);
    }
    0
}

#[test]
fn sweep_saved_settings_preserve_octets_signed_words_and_thin_dimension_shape() {
    let packet = include_bytes!("../fixtures/sweeps/cases.bin");
    let mut patches = 0;
    for (_, version, raw, _, _) in records_with_count(packet, b"SWF1", 160) {
        let fields = decode_graph(version, raw, true).unwrap().0;
        for field in fields
            .iter()
            .filter(|field| field.path.contains(".payload."))
        {
            patches += sweep_property_patches(version, &fields, field);
        }
    }
    assert!(patches > 0);
    eprintln!("SWEEP_SAVED_SETTING_PATCHES\t{patches}");
}

fn thicken_property_patches(version: u32, fields: &[ModelField], field: &ModelField) -> usize {
    let Some(label) = field.path.strip_prefix("root.object.items[1].") else {
        return 0;
    };
    if ["raw_818", "raw_81c", "raw_820", "raw_828"].contains(&label) {
        assert_signed_property_domain(version, fields, &field.path);
        return 6;
    }
    if label == "raw_810" {
        assert_stored_double_domain(version, fields, &field.path);
        return 12;
    }
    0
}

#[test]
fn thicken_positive_version_settings_preserve_signed_words_and_ieee_thickness() {
    let packet = include_bytes!("../fixtures/thickening/cases.bin");
    let mut patches = 0;
    let mut cases = 0;
    // #213: high-bit synthetic replay does not bind the native signed effective version.
    for (_, version, raw, _, _) in records_with_count(packet, b"THK1", 126)
        .into_iter()
        .filter(|row| row.0 < 6 && row.1 <= i32::MAX as u32)
    {
        let fields = decode_graph(version, raw, true).unwrap().0;
        for field in &fields {
            patches += thicken_property_patches(version, &fields, field);
        }
        cases += 1;
    }
    assert_eq!((cases, patches), (36, 1296));
    eprintln!("THICKEN_POSITIVE_VERSION_SETTING_PATCHES\t{patches}");
}
