use super::lightweight_section_tests::next_case;
use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelFieldLimits, encode_fields};
use crate::Result;
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/model-tail-cases.bin");
const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 30000,
    maximum_fields: 6000,
    maximum_entries: 6000,
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
    let mut map = ObjectReader::new();
    match kind {
        0 => super::post_views::read(&mut reader, &mut map)?,
        1 => super::model_arrays::read(&mut reader, &mut map)?,
        2 => super::layers::read(&mut reader, &mut map)?,
        3 | 4 => {
            super::reference_tail::read(&mut reader, &mut map, if kind == 4 { 0x100 } else { 0 })?
        }
        5 => super::shared_tail::read(&mut reader, &mut map)?,
        6 | 7 => super::part_tail::read(&mut reader, &mut map, i32::from(kind == 7))?,
        _ => panic!("unexpected model tail kind {kind}"),
    }
    reader.finish()
}

#[test]
fn model_tail_cases_match_reference_fields_and_owned_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(kind, version, bytes).unwrap();
        fields += super::node_units::assert_oracle(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (30, 1741));
}

#[test]
fn model_tail_prefixes_and_trailing_bytes_are_rejected() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(kind, version, &bytes[..length]).is_err(),
                "tail {kind} version {version} prefix {length}"
            );
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(decode(kind, version, &trailing).is_err());
    }
}
