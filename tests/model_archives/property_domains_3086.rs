use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::records_with_count;
use super::{ModelField, ModelFieldValue, StoredDouble};

const SPLIT_PACKET: &[u8] = include_bytes!("../fixtures/split-features/cases.bin");
const HELIX_PACKET: &[u8] = include_bytes!("../fixtures/helices/cases.bin");

const SPLIT_SETTINGS: [&str; 9] = [
    "910", "914", "918", "91c", "920", "924", "928", "9d8", "a20",
];
const SIGNED_BOUNDARIES: [i32; 6] = [i32::MIN, -1, 0, 1, 2, i32::MAX];
const NUMREV_BITS: [u64; 12] = [
    0,
    0x8000_0000_0000_0000,
    1,
    0x8000_0000_0000_0001,
    0x3ff0_0000_0000_0000,
    0xbff0_0000_0000_0000,
    0x7ff0_0000_0000_0000,
    0xfff0_0000_0000_0000,
    0x7ff8_0000_0000_1234,
    0xfff8_0000_0000_5678,
    0x7ff0_0000_0000_0001,
    0xfff0_0000_0000_0001,
];

fn assert_property_splice(
    version: u32,
    fields: &[ModelField],
    path: &str,
    replacement: &[u8],
    expected_value: ModelFieldValue,
) {
    let changed = super::manager_context_tests::replace_field(fields, path, replacement);
    let decoded = decode_graph(version, &changed, true).unwrap().0;
    let mut expected = fields.to_vec();
    expected
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value = expected_value;
    assert_eq!(decoded, expected);
    assert_eq!(super::encode_fields(&decoded, 30000).unwrap(), changed);
}

fn assert_split_setting_domain(version: u32, fields: &[ModelField], path: &str) {
    for value in SIGNED_BOUNDARIES {
        assert_property_splice(
            version,
            fields,
            path,
            &value.to_le_bytes(),
            ModelFieldValue::I32(value),
        );
    }
}

fn split_setting_paths(fields: &[ModelField]) -> Vec<(usize, &str)> {
    let mut paths = Vec::new();
    for (index, label) in SPLIT_SETTINGS.iter().enumerate() {
        let suffix = format!(".payload.raw_{label}");
        paths.extend(
            fields
                .iter()
                .filter(|field| field.path.ends_with(&suffix))
                .map(|field| (index, field.path.as_ref())),
        );
    }
    paths
}

#[test]
fn split_named_parent_settings_preserve_full_signed32_without_boolean_or_enum_projection() {
    let rows = records_with_count(SPLIT_PACKET, b"SPL1", 312);
    let mut occurrences = [0usize; 9];
    for (_, version, raw, _, _) in rows.into_iter().filter(|row| row.0 < 6) {
        let fields = decode_graph(version, raw, true).unwrap().0;
        for (index, path) in split_setting_paths(&fields) {
            assert_split_setting_domain(version, &fields, path);
            occurrences[index] += 1;
        }
    }

    assert!(occurrences.iter().all(|count| *count > 0));
    eprintln!("SPLIT_PROPERTY_OCCURRENCES\t{occurrences:?}");
}

#[test]
fn helix_numrev_preserves_ieee_bits_without_count_unit_or_finiteness_constraints() {
    let rows = records_with_count(HELIX_PACKET, b"HLX1", 243);
    let path = "root.object.payload.data_788.raw_20";
    let mut cases = 0;
    for (_, version, raw, _, _) in rows.into_iter().filter(|row| row.0 == 0) {
        let fields = decode_graph(version, raw, true).unwrap().0;
        assert_numrev_domain(version, &fields, path);
        cases += NUMREV_BITS.len();
    }

    assert!(cases > 0);
    eprintln!("HELIX_NUMREV_SPLICES\t{cases}");
}

fn assert_numrev_domain(version: u32, fields: &[ModelField], path: &str) {
    for bits in NUMREV_BITS {
        let expected = ModelFieldValue::F64(StoredDouble::from_bits(bits));
        assert_property_splice(version, fields, path, &bits.to_le_bytes(), expected);
    }
}
