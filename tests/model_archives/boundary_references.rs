use super::annotation_view_tests::decode;
use super::lightweight_section_tests::next_case;
use super::{ModelFieldValue, encode_fields};
use crate::archive_codec::ArchiveCursor;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/boundary-reference-cases.bin");

#[test]
fn boundary_reference_cases_match_independent_fields_aliases_and_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (_, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(version, bytes).unwrap();
        fields += super::transform_rows::assert_fields(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (48, 412));
}

#[test]
fn boundary_reference_prefixes_and_trailing_bytes_reject_incomplete_records() {
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
fn boundary_entity_counts_are_unsigned_and_checked_before_iteration() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (_, version, bytes, _) = next_case(&mut cursor);
    let mut fields = decode(version, bytes).unwrap();
    let count = fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.entity_count")
        .unwrap();
    count.value = ModelFieldValue::U32(u32::MAX);
    let error = decode(version, &encode_fields(&fields, 30000).unwrap()).unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Invalid);
    assert!(error.cause.contains("4294967295"));
}
