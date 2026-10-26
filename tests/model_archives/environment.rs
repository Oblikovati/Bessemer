use super::lightweight_section_tests::next_case;
use super::reader::ModelArchiveContext;
use super::{
    ModelDecoder, ModelField, ModelFieldLimits, ModelObjectRole, detail_definitions, encode_fields,
    environment, features,
};
use crate::Result;
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/environment-cases.bin");
const NATIVE: &[u8] = include_bytes!("../fixtures/model-archives/native-environment.bin");
const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 30000,
    maximum_fields: 6000,
    maximum_entries: 6000,
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

fn decode(kind: u8, version: u32, bytes: &[u8], mut map: ObjectReader) -> Result<Vec<ModelField>> {
    let mut reader = decoder(bytes, version);
    match kind {
        0 => features::read(&mut reader, &mut map)?,
        1 => reader.object("object", &mut map, ModelObjectRole::Any)?,
        2 => environment::read(&mut reader, &mut map)?,
        _ => panic!("unexpected environment fixture kind {kind}"),
    }
    reader.finish()
}

pub(super) fn inherited_map() -> ObjectReader {
    let mut map = super::complete_detail_tests::inherited_map();
    let raw = include_bytes!("../fixtures/model-archives/native-complete-details.bin");
    let mut reader = decoder(raw, 19000);
    detail_definitions::read(&mut reader, &mut map).unwrap();
    reader.finish().unwrap();
    map
}

#[test]
fn feature_model_light_and_environment_cases_match_independent_fields_and_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(kind, version, bytes, ObjectReader::new()).unwrap();
        fields += super::node_units::assert_oracle(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (24, 1768));
}

#[test]
fn native_environment_uses_inherited_identity_and_preserves_every_scalar() {
    let fields = decode(2, 19000, NATIVE, inherited_map()).unwrap();
    let oracle = include_str!("../fixtures/model-archives/native-environment.txt");
    assert_eq!(super::node_units::assert_oracle(&fields, oracle), 158);
    assert_eq!(encode_fields(&fields, 30000).unwrap(), NATIVE);
    assert!(decode(2, 19000, NATIVE, ObjectReader::new()).is_err());
}

#[test]
fn every_feature_light_and_environment_prefix_and_trailing_byte_fails() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(kind, version, &bytes[..length], ObjectReader::new()).is_err(),
                "kind {kind} version {version} prefix {length}"
            );
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(decode(kind, version, &trailing, ObjectReader::new()).is_err());
    }
}

#[test]
fn native_environment_fields_own_inputs_and_parallel_encoding_is_deterministic() {
    let mut bytes = NATIVE.to_vec();
    let fields = decode(2, 19000, &bytes, inherited_map()).unwrap();
    bytes.fill(0xa5);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| encode_fields(&fields, 30000).unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), NATIVE);
        }
    });
}
