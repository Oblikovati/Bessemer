use super::lightweight_section_tests::next_case;
use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelFieldLimits, detail_prefix, encode_fields};
use crate::Result;
use crate::archive_codec::ArchiveCursor;
use crate::header::ArchiveVersion;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/detail-prefix-cases.bin");
const NATIVE: &[u8] = include_bytes!("../fixtures/model-archives/native-detail-prefix.bin");
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
    match kind {
        0 => detail_prefix::read(&mut reader)?,
        1 => detail_prefix::character_format(&mut reader)?,
        _ => panic!("unexpected detail fixture kind {kind}"),
    }
    reader.finish()
}

#[test]
fn detail_prefix_and_character_format_match_independent_boundary_cases() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(kind, version, bytes).unwrap();
        fields += super::node_units::assert_oracle(&decoded, oracle);
        assert_eq!(
            encode_fields(&decoded, LIMITS.maximum_bytes).unwrap(),
            bytes
        );
        cases += 1;
    }
    assert_eq!((cases, fields), (22, 1451));
}

#[test]
fn native_detail_prefix_preserves_every_independent_scalar_and_wire_byte() {
    let fields = decode(0, 19000, NATIVE).unwrap();
    let oracle = include_str!("../fixtures/model-archives/native-detail-prefix.txt");
    assert_eq!(super::node_units::assert_oracle(&fields, oracle), 143);
    assert_eq!(
        encode_fields(&fields, LIMITS.maximum_bytes).unwrap(),
        NATIVE
    );
}

#[test]
fn every_detail_case_prefix_and_trailing_byte_fails_complete_record_publication() {
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

#[test]
fn detail_fields_survive_input_overwrite_and_parallel_reconstruction() {
    let mut bytes = NATIVE.to_vec();
    let fields = decode(0, 19000, &bytes).unwrap();
    bytes.fill(0xa5);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| encode_fields(&fields, 10000).unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), NATIVE);
        }
    });
}
