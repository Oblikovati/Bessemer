use super::property_domains_3086_tests::{
    assert_signed_property_domain, assert_stored_double_domain,
};
use super::property_domains_3087_tests::byte_domain;
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::records_with_count;
use super::{ModelField, ModelFieldValue};

fn root_setting_path(kind: u8, label: &str) -> String {
    let root = if matches!(kind % 6, 2 | 5) {
        "root.object.items[0]"
    } else {
        "root.object"
    };
    format!("{root}.{label}")
}

fn saved_setting_domain(version: u32, fields: &[ModelField], path: &str) -> usize {
    let Some(field) = fields.iter().find(|field| field.path.as_ref() == path) else {
        return 0;
    };
    match field.value {
        ModelFieldValue::I32(_) => {
            assert_signed_property_domain(version, fields, path);
            6
        }
        ModelFieldValue::U8(_) => {
            byte_domain(version, fields, path);
            256
        }
        ModelFieldValue::F64(_) => {
            assert_stored_double_domain(version, fields, path);
            12
        }
        _ => panic!("{path}: expected saved numeric scalar, got {field:?}"),
    }
}

#[test]
fn combine_saved_operation_and_transparency_retain_raw_domains_before_native_conversions() {
    let packet = include_bytes!("../fixtures/combine-bodies/cases.bin");
    let mut patches = 0;
    for (kind, version, raw, _, _) in records_with_count(packet, b"CMB1", 90)
        .into_iter()
        .filter(|row| row.0 < 6)
    {
        let fields = decode_graph(version, raw, true).unwrap().0;
        for label in ["raw_828", "raw_838"] {
            patches += saved_setting_domain(version, &fields, &root_setting_path(kind, label));
        }
    }
    assert_eq!(patches, 4788);
    eprintln!("COMBINE_PROPERTY_PATCHES\t{patches}");
}

#[test]
fn mid_surface_data_knit_flag_retains_signed_words_without_querying_placement_provider() {
    let packet = include_bytes!("../fixtures/mid-surfaces/cases.bin");
    let mut patches = 0;
    for (_, version, raw, _, _) in records_with_count(packet, b"MSF1", 432)
        .into_iter()
        .filter(|row| (18..24).contains(&row.0))
    {
        let fields = decode_graph(version, raw, true).unwrap().0;
        patches += saved_setting_domain(version, &fields, "root.object.raw_10");
    }
    assert_eq!(patches, 324);
    eprintln!("MID_SURFACE_KNIT_FLAG_PATCHES\t{patches}");
}

#[test]
fn thin_specification_named_scalars_retain_full_words_and_ieee_end_thickness() {
    let packet = include_bytes!("../fixtures/thin-extrusions/cases.bin");
    let mut patches = 0;
    for (kind, version, raw, _, _) in records_with_count(packet, b"THN1", 360)
        .into_iter()
        .filter(|row| row.0 < 6)
    {
        let fields = decode_graph(version, raw, true).unwrap().0;
        for label in ["raw_28", "raw_2c", "raw_50", "raw_30"] {
            patches += saved_setting_domain(version, &fields, &root_setting_path(kind, label));
        }
    }
    assert_eq!(patches, 2160);
    eprintln!("THIN_SPECIFICATION_PROPERTY_PATCHES\t{patches}");
}
