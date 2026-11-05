use super::property_domains_3086_tests::{
    assert_property_splice, assert_signed_property_domain, assert_stored_double_domain,
};
use super::property_domains_3087_tests::byte_domain;
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::records_with_count;
use super::{ModelField, ModelFieldValue};

const BLEND_LABELS: [&str; 13] = [
    "raw_110", "raw_114", "raw_118", "raw_120", "raw_420", "raw_428", "raw_60", "raw_64",
    "raw_378", "raw_300", "raw_374", "raw_304", "raw_308",
];

fn joined_blend_field(field: &&ModelField) -> bool {
    let Some((prefix, label)) = field.path.rsplit_once('.') else {
        return false;
    };
    prefix.ends_with(".payload.data") && BLEND_LABELS.contains(&label)
}

fn matching_control_domain(
    version: u32,
    fields: &[ModelField],
    field: &ModelField,
    original: i32,
) -> usize {
    let low = (original as u32) & 255;
    let high = [0, 0x100, 0x200, 0x300, 0x7fffff00, 0x80000000, 0xffffff00];
    for upper in high {
        let value = (upper | low) as i32;
        let expected = ModelFieldValue::I32(value);
        assert_property_splice(version, fields, &field.path, &value.to_le_bytes(), expected);
    }
    high.len()
}

fn blend_scalar_domain(version: u32, fields: &[ModelField], field: &ModelField) -> usize {
    match field.value {
        ModelFieldValue::U8(_) => {
            byte_domain(version, fields, &field.path);
            256
        }
        ModelFieldValue::F64(_) => {
            assert_stored_double_domain(version, fields, &field.path);
            12
        }
        ModelFieldValue::I32(value) => blend_integer_domain(version, fields, field, value),
        _ => panic!("{}: expected byte, signed32 or binary64", field.path),
    }
}

fn blend_graph_patches(
    version: u32,
    fields: &[ModelField],
) -> (usize, std::collections::BTreeSet<String>) {
    let mut patches = 0;
    let mut labels = std::collections::BTreeSet::new();
    for field in fields.iter().filter(joined_blend_field) {
        patches += blend_scalar_domain(version, fields, field);
        labels.insert(field.path.rsplit('.').next().unwrap().to_owned());
    }
    (patches, labels)
}

#[test]
fn blend_named_fields_preserve_matching_high_bits_raw_bytes_and_unclamped_doubles() {
    let packet = include_bytes!("../fixtures/blends/cases.bin");
    let mut patches = 0;
    let mut covered = std::collections::BTreeSet::new();
    for (_, version, raw, _, _) in records_with_count(packet, b"BLN1", 68) {
        let fields = decode_graph(version, raw, true).unwrap().0;
        let (count, labels) = blend_graph_patches(version, &fields);
        patches += count;
        covered.extend(labels);
    }
    assert_eq!(covered.len(), BLEND_LABELS.len());
    eprintln!("BLEND_NAMED_PROPERTY_PATCHES\t{patches}");
}

#[test]
fn circular_direction_and_symmetry_fields_retain_every_octet_despite_native_bool_views() {
    let packet = include_bytes!("../fixtures/circular-patterns/cases.bin");
    let mut patches = 0;
    for (_, version, raw, _, _) in records_with_count(packet, b"CPF1", 22) {
        let fields = decode_graph(version, raw, true).unwrap().0;
        for field in fields.iter().filter(circular_property_byte) {
            byte_domain(version, &fields, &field.path);
            patches += 256;
        }
    }
    assert!(patches > 0);
    eprintln!("CIRCULAR_PROPERTY_BYTE_PATCHES\t{patches}");
}

fn circular_property_byte(field: &&ModelField) -> bool {
    let labels = ["raw_1160", "raw_11c0", "raw_11c4"];
    let Some((prefix, label)) = field.path.rsplit_once('.') else {
        return false;
    };
    prefix.ends_with(".payload") && labels.contains(&label)
}

fn blend_integer_domain(
    version: u32,
    fields: &[ModelField],
    field: &ModelField,
    value: i32,
) -> usize {
    if field.path.ends_with(".raw_60") || field.path.ends_with(".raw_64") {
        return matching_control_domain(version, fields, field, value);
    }
    assert_signed_property_domain(version, fields, &field.path);
    6
}
