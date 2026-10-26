use super::annotation_view_tests::decode;
use super::lightweight_section_tests::next_case;
use super::{ModelFieldValue, encode_fields};
use crate::archive_codec::ArchiveCursor;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/position-sheetmetal-cases.bin");

#[test]
fn position_groups_and_sheet_metal_bridges_match_independent_fields_and_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (_, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(version, bytes).unwrap();
        fields += super::node_units::assert_oracle(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (40, 1872));
}

#[test]
fn position_and_sheet_metal_prefixes_and_trailing_bytes_fail_complete_publication() {
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
fn position_direction_cardinality_is_checked_before_reading_arrays() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (_, version, bytes, _) = next_case(&mut cursor);
    let mut fields = decode(version, bytes).unwrap();
    let count = fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.direction_count")
        .unwrap();
    count.value = ModelFieldValue::U16(u16::MAX);
    let error = decode(version, &encode_fields(&fields, 30000).unwrap()).unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Unsupported);
    assert!(error.cause.contains("65535"));
}
