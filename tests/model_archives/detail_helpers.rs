use super::lightweight_section_tests::next_case;
use super::reader::ModelArchiveContext;
use super::{
    ModelDecoder, ModelField, ModelFieldLimits, ModelObjectRole, detail_helpers, encode_fields,
};
use crate::Result;
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/detail-helper-cases.bin");
const NATIVE: &[u8] = include_bytes!("../fixtures/model-archives/native-detail-helpers.bin");
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

fn decode(kind: u8, version: u32, bytes: &[u8]) -> Result<Vec<ModelField>> {
    let mut reader = decoder(bytes, version);
    let mut map = ObjectReader::new();
    match kind {
        0 => detail_helpers::surface_finish(&mut reader)?,
        1 => detail_helpers::note(&mut reader, &mut map)?,
        2 => reader.object(
            "object",
            &mut map,
            ModelObjectRole::Declared(&["moNoteDataHelper_c"]),
        )?,
        3 => detail_helpers::read(&mut reader, &mut map)?,
        _ => panic!("unexpected helper fixture kind {kind}"),
    }
    reader.finish()
}

#[test]
fn helper_fields_nested_maps_and_cycles_match_independent_reference_cases() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(kind, version, bytes).unwrap();
        fields += super::node_units::assert_oracle(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 10000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (21, 1827));
}

#[test]
fn native_surface_finish_and_note_helpers_reconstruct_every_field() {
    let decoded = decode(3, 19000, NATIVE).unwrap();
    let oracle = include_str!("../fixtures/model-archives/native-detail-helpers.txt");
    assert_eq!(super::node_units::assert_oracle(&decoded, oracle), 156);
    assert_eq!(encode_fields(&decoded, 10000).unwrap(), NATIVE);
}

#[test]
fn every_helper_case_prefix_and_trailing_byte_fails_complete_record_publication() {
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
fn native_size_t_counts_preserve_all_bits_and_reject_impossible_helper_maps() {
    for value in [0_u64, 1, u64::MAX, 1 << 63] {
        let bytes = value.to_le_bytes();
        let mut reader = decoder(&bytes, 19000);
        assert_eq!(reader.u64("count").unwrap(), value);
        assert_eq!(encode_fields(&reader.finish().unwrap(), 8).unwrap(), bytes);
    }
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, _) = next_case(&mut cursor);
        if kind != 2 {
            continue;
        }
        reject_large_note_count(kind, version, bytes);
    }
}

fn reject_large_note_count(kind: u8, version: u32, bytes: &[u8]) {
    let fields = decode(kind, version, bytes).unwrap();
    let index = fields
        .iter()
        .position(|field| field.path.ends_with(".helper_map_count"))
        .unwrap();
    let offset = encode_fields(&fields[..index], 10000).unwrap().len();
    for count in [i32::MAX as u64 + 1, 1 << 63, u64::MAX] {
        let mut altered = bytes.to_vec();
        altered[offset..offset + 8].copy_from_slice(&count.to_le_bytes());
        assert!(decode(kind, version, &altered).is_err());
    }
}
