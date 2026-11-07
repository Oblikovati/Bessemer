use super::ModelField;
use super::direct_collection_tests::decode_collection;
use super::property_domains_3086_tests::{
    assert_signed_property_domain, assert_stored_double_domain,
};
use super::sketch_offsets::sketch_offset;
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::records_with_count;

fn deform_named_setting(version: u32, fields: &[ModelField], field: &ModelField) -> usize {
    if field.path.ends_with(".geometry_8f8.raw_d8") || field.path.ends_with(".geometry_8f8.raw_678")
    {
        assert_signed_property_domain(version, fields, &field.path);
        return 6;
    }
    if field.path.contains(".payload.point_8a8[") {
        assert_stored_double_domain(version, fields, &field.path);
        return 12;
    }
    0
}

#[test]
fn deform_properties_preserve_full_words_and_three_ieee_drag_point_components() {
    let packet = include_bytes!("../fixtures/deformations/cases.bin");
    let mut patches = 0;
    let mut paths = std::collections::BTreeSet::new();
    for (_, version, raw, _, _) in records_with_count(packet, b"DFM1", 384) {
        let fields = decode_graph(version, raw, true).unwrap().0;
        for field in &fields {
            let count = deform_named_setting(version, &fields, field);
            if count > 0 {
                paths.insert(field.path.rsplit('.').next().unwrap().to_owned());
            }
            patches += count;
        }
    }
    assert_eq!(paths.len(), 5);
    eprintln!("DEFORM_NAMED_PROPERTY_PATCHES\t{patches}");
}

#[test]
fn sketch_relation_indices_keep_signed_native_archive_bits_in_the_unsigned_view() {
    for index in [0u32, 1, i32::MAX as u32, 0x80000000, u32::MAX] {
        let mut raw = 1i32.to_le_bytes().to_vec();
        raw.extend_from_slice(&index.to_le_bytes());
        raw.extend_from_slice(&[0; 76]);
        let fields = decode_collection(19000, &raw, sketch_offset).unwrap();
        let field = fields
            .iter()
            .find(|field| field.path.ends_with("indices[0]"))
            .unwrap();
        assert_eq!(field.value, super::ModelFieldValue::U32(index));
        assert_eq!(super::encode_fields(&fields, raw.len()).unwrap(), raw);
    }
}
