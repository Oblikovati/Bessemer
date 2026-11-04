use super::annotation_view_tests::decode;
use super::encode_fields;
use super::lightweight_section_tests::next_case;
use super::{ModelField, ModelFieldValue};
use crate::archive_codec::ArchiveCursor;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/folder-parameter-cases.bin");

#[test]
fn folder_and_parameter_variants_match_independent_values_classes_and_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (_, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(version, bytes).unwrap();
        fields += super::node_units::assert_oracle(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (63, 1938));
}

#[test]
fn folder_and_parameter_prefixes_and_trailing_bytes_reject_partial_bodies() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(version, &bytes[..length]).is_err(),
                "version {version} prefix {length}"
            );
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(decode(version, &trailing).is_err());
    }
}

#[test]
fn parameter_and_folder_snapshots_own_strings_flags_and_float_bits() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        let mut source = bytes.to_vec();
        let fields = decode(version, &source).unwrap();
        source.fill(0xa5);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
    }
}

#[test]
fn folder_property_words_keep_signed32_values_without_native_boolean_queries() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut words) = (0, 0);
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        words += assert_folder_property_domains(version, bytes);
        cases += 1;
    }
    assert_eq!((cases, words, words * 6), (63, 72, 432));
}

fn assert_folder_property_domains(version: u32, bytes: &[u8]) -> usize {
    let fields = decode(version, bytes).unwrap();
    let selected: Vec<usize> = fields
        .iter()
        .enumerate()
        .filter_map(|(index, field)| {
            (field.path.ends_with(".unknown_358") || field.path.ends_with(".unknown_368"))
                .then_some(index)
        })
        .collect();
    // Exact own-class property names do not close the stored integer domain (schema C-3506).
    for &index in &selected {
        for value in [i32::MIN, -1, 0, 1, 2, i32::MAX] {
            assert_folder_property_splice(version, bytes, &fields, index, value);
        }
    }
    selected.len()
}

fn assert_folder_property_splice(
    version: u32,
    original: &[u8],
    fields: &[ModelField],
    selected: usize,
    value: i32,
) {
    let mut changed = fields.to_vec();
    changed[selected].value = ModelFieldValue::I32(value);
    let offset = encode_fields(&fields[..selected], 30000).unwrap().len();
    let mut expected = original.to_vec();
    expected[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    let encoded = encode_fields(&changed, 30000).unwrap();
    assert_eq!(encoded, expected);
    assert_eq!(decode(version, &encoded).unwrap(), changed);
    assert_eq!(encode_fields(fields, 30000).unwrap(), original);
}
