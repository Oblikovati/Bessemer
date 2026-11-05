use super::property_domains_3086_tests::{assert_property_splice, assert_signed_property_domain};
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::records_with_count;
use super::{ModelField, ModelFieldValue};
use crate::archive_codec::ArchiveCursor;

fn complete_pattern_cases() -> Vec<(u32, &'static [u8])> {
    let mut cursor = ArchiveCursor::new(include_bytes!(
        "../fixtures/model-archives/linear-pattern-cases.bin"
    ));
    let mut cases = Vec::new();
    while cursor.remaining() != 0 {
        let (mode, version, raw, _) = super::lightweight_section_tests::next_case(&mut cursor);
        if mode == 0 {
            cases.push((version, raw));
        }
    }
    cases
}

fn pattern_entities_field(field: &&ModelField) -> bool {
    let path = field.path.as_ref();
    path.ends_with(".header.byte_940") || path.ends_with(".header.word_940")
}

fn assert_pattern_entities_word(
    version: u32,
    fields: &[ModelField],
    field: &ModelField,
) -> (usize, usize) {
    if matches!(field.value, ModelFieldValue::I32(_)) {
        assert_signed_property_domain(version, fields, &field.path);
        return (0, 6);
    }
    assert!(matches!(field.value, ModelFieldValue::U8(_)));
    for byte in 0..=u8::MAX {
        let expected = ModelFieldValue::U8(byte);
        assert_property_splice(version, fields, &field.path, &[byte], expected);
    }
    (256, 0)
}

#[test]
fn pattern_entities_type_retains_legacy_octets_and_modern_signed_words_without_predicate_projection()
 {
    let mut totals = (0, 0);
    for (version, raw) in complete_pattern_cases() {
        let fields = decode_graph(version, raw, true).unwrap().0;
        for field in fields.iter().filter(pattern_entities_field) {
            let (bytes, words) = assert_pattern_entities_word(version, &fields, field);
            totals.0 += bytes;
            totals.1 += words;
        }
    }
    assert!(totals.0 > 0 && totals.1 > 0);
    eprintln!(
        "PATTERN_ENTITIES_BYTE_PATCHES\t{}\nPATTERN_ENTITIES_WORD_PATCHES\t{}",
        totals.0, totals.1
    );
}

fn trim_option_patterns() -> Vec<u32> {
    let mut patterns = vec![
        0,
        u32::MAX,
        0xffff0000,
        0x0000ffff,
        0x12345678,
        0xabcd0040,
        0xabcd0010,
    ];
    for bit in 0..32 {
        patterns.push(1u32 << bit);
        patterns.push(!(1u32 << bit));
    }
    patterns
}

fn assert_trim_packed_domain(version: u32, fields: &[ModelField], path: &str) -> usize {
    let patterns = trim_option_patterns();
    for value in &patterns {
        assert_property_splice(
            version,
            fields,
            path,
            &value.to_le_bytes(),
            ModelFieldValue::U32(*value),
        );
    }
    patterns.len()
}

fn surface_scalar_patches(version: u32, fields: &[ModelField], field: &ModelField) -> usize {
    if field.path.ends_with(".trim_data_800.raw_10") {
        return assert_trim_packed_domain(version, fields, &field.path);
    }
    let signed = [".raw_818", ".trim_data_800.raw_08", ".trim_data_800.raw_0c"];
    if !signed.iter().any(|suffix| field.path.ends_with(suffix)) {
        return 0;
    }
    assert_signed_property_domain(version, fields, &field.path);
    6
}

#[test]
fn trim_packed_components_and_surface_sheet_indices_preserve_all_saved_fragments() {
    let packet = include_bytes!("../fixtures/surface-operations/cases.bin");
    let mut patches = 0;
    for (_, version, raw, _, _) in records_with_count(packet, b"SOP1", 84) {
        let fields = decode_graph(version, raw, true).unwrap().0;
        for field in &fields {
            patches += surface_scalar_patches(version, &fields, field);
        }
    }
    assert!(patches > 0);
    eprintln!("TRIM_SURFACE_SCALAR_PATCHES\t{patches}");
}
