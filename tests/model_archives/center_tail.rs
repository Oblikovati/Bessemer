use super::lightweight_section_tests::next_case;
use super::reader::ModelArchiveContext;
use super::{
    ModelDecoder, ModelField, ModelFieldLimits, center_helpers, detail_tail, encode_fields,
};
use crate::Result;
use crate::archive_codec::ArchiveCursor;
use crate::header::ArchiveVersion;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/center-tail-cases.bin");
const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 10000,
    maximum_fields: 3000,
    maximum_entries: 3000,
    maximum_depth: 32,
    maximum_objects: 128,
};

fn decode(kind: u8, version: u32, bytes: &[u8]) -> Result<Vec<ModelField>> {
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(version),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )?;
    if kind == 0 {
        center_helpers::read(&mut reader)?;
    } else {
        detail_tail::read(&mut reader)?;
    }
    reader.finish()
}

const NATIVE_GROUPS: [(u8, &[u8], &str, usize); 2] = [
    (
        0,
        include_bytes!("../fixtures/model-archives/native-center.bin"),
        include_str!("../fixtures/model-archives/native-center.txt"),
        281,
    ),
    (
        1,
        include_bytes!("../fixtures/model-archives/native-detail-tail.bin"),
        include_str!("../fixtures/model-archives/native-detail-tail.txt"),
        81,
    ),
];

#[test]
fn center_swift_and_detail_tail_cases_match_reference_fields_and_exact_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(kind, version, bytes).unwrap();
        fields += super::node_units::assert_oracle(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 10000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (30, 4484));
}

#[test]
fn native_center_helpers_and_detail_tail_preserve_every_stored_value() {
    for (kind, bytes, oracle, count) in NATIVE_GROUPS {
        let fields = decode(kind, 19000, bytes).unwrap();
        assert_eq!(super::node_units::assert_oracle(&fields, oracle), count);
        assert_eq!(encode_fields(&fields, 10000).unwrap(), bytes);
    }
}

#[test]
fn every_center_and_tail_case_prefix_and_trailing_byte_fails_complete_publication() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(kind, version, &bytes[..length]).is_err(),
                "kind {kind} version {version} prefix {length}"
            );
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(decode(kind, version, &trailing).is_err());
    }
}
