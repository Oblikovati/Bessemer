use super::property_domains_3086_tests::{
    assert_property_splice, assert_signed_property_domain, assert_stored_double_domain,
};
use super::property_domains_3087_tests::integer_family;
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::records_with_count;
use super::{ModelField, ModelFieldValue};

fn option_byte_domain(
    version: u32,
    fields: &[ModelField],
    field: &ModelField,
    previous: u8,
) -> usize {
    let selector = field.path.ends_with("want_thin_solid_18");
    let (first, last) = match (selector, previous == 0) {
        (true, true) => (0, 0),
        (true, false) => (1, u8::MAX),
        _ => (0, u8::MAX),
    };
    for byte in first..=last {
        let expected = ModelFieldValue::U8(byte);
        assert_property_splice(version, fields, &field.path, &[byte], expected);
    }
    usize::from(last) - usize::from(first) + 1
}

fn option_scalar_domain(version: u32, fields: &[ModelField], field: &ModelField) -> usize {
    match field.value {
        ModelFieldValue::U8(previous) => option_byte_domain(version, fields, field, previous),
        ModelFieldValue::I32(3) if field.path.ends_with("raw_30") => 0,
        ModelFieldValue::I32(_) => {
            assert_signed_property_domain(version, fields, &field.path);
            6
        }
        ModelFieldValue::F64(_) => {
            assert_stored_double_domain(version, fields, &field.path);
            12
        }
        _ => panic!(
            "{}: expected a body-option byte, signed32 or binary64",
            field.path
        ),
    }
}

#[test]
fn netblend_body_options_keep_full_byte_domains_without_native_setter_coupling() {
    let packet = include_bytes!("../fixtures/net-blends/cases.bin");
    let mut patches = 0;
    let mut covered = std::collections::BTreeSet::new();
    for (_, version, raw, _, _) in records_with_count(packet, b"NBL1", 570)
        .into_iter()
        .filter(|row| row.0 < 12 && row.1 == 19000)
    {
        let fields = decode_graph(version, raw, true).unwrap().0;
        let (count, labels) = option_graph_patches(version, &fields);
        patches += count;
        covered.extend(labels);
    }

    assert_eq!(covered.len(), 8);
    eprintln!("NETBLEND_BODY_OPTION_PATCHES\t{patches}");
}

#[test]
fn punch_named_options_keep_signed32_values_at_every_reached_internal_revision_gate() {
    let packet = include_bytes!("../fixtures/punches/cases.bin");
    let labels = ["raw_864", "raw_910", "raw_914", "raw_918"];
    let occurrences = integer_family(packet, b"PCH1", 300, &labels);
    eprintln!("PUNCH_OPTION_PATCHES\t{}", occurrences * 6);
}

fn option_graph_patches(
    version: u32,
    fields: &[ModelField],
) -> (usize, std::collections::BTreeSet<String>) {
    let mut patches = 0;
    let mut covered = std::collections::BTreeSet::new();
    for field in fields
        .iter()
        .filter(|field| field.path.contains(".body_options_848."))
    {
        patches += option_scalar_domain(version, fields, field);
        covered.insert(field.path.rsplit('.').next().unwrap().to_owned());
    }
    (patches, covered)
}
