use super::lightweight_section_tests::next_case;
use super::reader::ModelArchiveContext;
use super::{
    ModelDecoder, ModelField, ModelFieldLimits, dimension_defaults, encode_fields, resolved_start,
};
use crate::Result;
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/dimension-default-cases.bin");
const NATIVE: &[u8] = include_bytes!("../fixtures/model-archives/native-dimension-defaults.bin");
const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 10000,
    maximum_fields: 3000,
    maximum_entries: 3000,
    maximum_depth: 32,
    maximum_objects: 128,
};

fn decoder(bytes: &[u8], version: u32) -> ModelDecoder<'_> {
    ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(version),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )
    .unwrap()
}

fn decode(version: u32, bytes: &[u8], mut map: ObjectReader) -> Result<Vec<ModelField>> {
    let mut reader = decoder(bytes, version);
    dimension_defaults::read(&mut reader, &mut map)?;
    reader.finish()
}

fn inherited_map() -> ObjectReader {
    let mut map = super::resolved_start_tests::inherited_map();
    let raw = include_bytes!("../fixtures/model-archives/native-resolved-start.bin");
    let mut reader = decoder(raw, 19000);
    resolved_start::read(&mut reader, &mut map).unwrap();
    reader.finish().unwrap();
    map
}

#[test]
fn all_ten_dimension_kinds_match_independent_boundary_values_and_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut count, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, oracle) = next_case(&mut cursor);
        assert_eq!(kind, 0);
        let decoded = decode(version, bytes, ObjectReader::new()).unwrap();
        fields += super::node_units::assert_oracle(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 10000).unwrap(), bytes);
        count += 1;
    }
    assert_eq!((count, fields), (11, 8951));
}

#[test]
fn native_dimension_defaults_require_the_actual_map_and_reconstruct_all_fields() {
    let fields = decode(19000, NATIVE, inherited_map()).unwrap();
    let oracle = include_str!("../fixtures/model-archives/native-dimension-defaults.txt");
    assert_eq!(super::node_units::assert_oracle(&fields, oracle), 931);
    assert_eq!(encode_fields(&fields, 10000).unwrap(), NATIVE);
    assert!(decode(19000, NATIVE, ObjectReader::new()).is_err());
}

#[test]
fn every_dimension_group_prefix_and_trailing_byte_rejects_partial_publication() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(version, &bytes[..length], ObjectReader::new()).is_err(),
                "version {version} prefix {length}"
            );
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(decode(version, &trailing, ObjectReader::new()).is_err());
    }
}

#[test]
fn native_dimension_fields_own_their_bytes_and_encode_deterministically() {
    let mut bytes = NATIVE.to_vec();
    let fields = decode(19000, &bytes, inherited_map()).unwrap();
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
