use super::lightweight_section_tests::next_case;
use super::reader::ModelArchiveContext;
use super::{
    ModelDecoder, ModelField, ModelFieldLimits, ModelObjectRole, detail_definitions, encode_fields,
    resolved_start,
};
use crate::Result;
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/complete-detail-cases.bin");
const NATIVE: &[u8] = include_bytes!("../fixtures/model-archives/native-complete-details.bin");
const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 50000,
    maximum_fields: 10000,
    maximum_entries: 10000,
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

fn decode(
    tagged: bool,
    version: u32,
    bytes: &[u8],
    mut map: ObjectReader,
) -> Result<Vec<ModelField>> {
    let mut reader = decoder(bytes, version);
    if tagged {
        reader.object(
            "object",
            &mut map,
            ModelObjectRole::Declared(&["moDetailDefs_c"]),
        )?;
    } else {
        detail_definitions::read(&mut reader, &mut map)?;
    }
    reader.finish()
}

pub(super) fn inherited_map() -> ObjectReader {
    let mut map = super::resolved_start_tests::inherited_map();
    let prefix = include_bytes!("../fixtures/model-archives/native-resolved-start.bin");
    let mut reader = decoder(prefix, 19000);
    resolved_start::read(&mut reader, &mut map).unwrap();
    reader.finish().unwrap();
    map
}

#[test]
fn complete_direct_and_tagged_detail_objects_match_independent_fields_and_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(kind == 1, version, bytes, ObjectReader::new()).unwrap();
        fields += super::node_units::assert_oracle(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 50000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (4, 10980));
}

#[test]
fn whole_native_detail_definition_body_reconstructs_all_bytes_and_values() {
    let fields = decode(false, 19000, NATIVE, inherited_map()).unwrap();
    let oracle = include_str!("../fixtures/model-archives/native-complete-details.txt");
    assert_eq!(super::node_units::assert_oracle(&fields, oracle), 2910);
    assert_eq!(encode_fields(&fields, 50000).unwrap(), NATIVE);
    assert!(decode(false, 19000, NATIVE, ObjectReader::new()).is_err());
}

#[test]
fn complete_detail_object_prefixes_and_trailing_bytes_cannot_publish_partial_results() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(kind == 1, version, &bytes[..length], ObjectReader::new()).is_err(),
                "kind {kind} version {version} prefix {length}"
            );
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(decode(kind == 1, version, &trailing, ObjectReader::new()).is_err());
    }
}

#[test]
fn complete_detail_fields_own_their_inputs_and_reconstruct_in_parallel() {
    let mut bytes = NATIVE.to_vec();
    let fields = decode(false, 19000, &bytes, inherited_map()).unwrap();
    bytes.fill(0xa5);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| encode_fields(&fields, 50000).unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), NATIVE);
        }
    });
}
