use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelFieldLimits, ModelObjectRole, encode_fields};
use super::{lightweight_part, lightweight_post, lightweight_shared};
use crate::Result;
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/lightweight-sections.bin");
const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 10000,
    maximum_fields: 3000,
    maximum_entries: 3000,
    maximum_depth: 32,
    maximum_objects: 128,
};

fn read_section(kind: u8, reader: &mut ModelDecoder<'_>) -> Result<()> {
    let mut map = ObjectReader::new();
    match kind {
        0 => lightweight_post::array(reader),
        1 => lightweight_post::flags(reader),
        2 => lightweight_post::collections(reader, &mut map),
        3 => lightweight_post::tail(reader, &mut map),
        4 => lightweight_part::tail(reader).map(|_| ()),
        5 => lightweight_shared::spatial(reader, &mut map),
        6 => lightweight_shared::metadata(reader, &mut map),
        7 => lightweight_shared::manager(reader, &mut map),
        _ => panic!("unknown fixture section {kind}"),
    }
}

fn decode(kind: u8, version: u32, bytes: &[u8]) -> Result<Vec<ModelField>> {
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(version),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )?;
    read_section(kind, &mut reader)?;
    reader.finish()
}

pub(super) fn next_case<'a>(cursor: &mut ArchiveCursor<'a>) -> (u8, u32, &'a [u8], &'a str) {
    let kind = cursor.u8().unwrap();
    let version = cursor.u32().unwrap();
    let size = cursor.u32().unwrap() as usize;
    let oracle_size = cursor.u32().unwrap() as usize;
    let bytes = cursor.take(size).unwrap();
    let oracle = std::str::from_utf8(cursor.take(oracle_size).unwrap()).unwrap();
    (kind, version, bytes, oracle)
}

#[test]
fn conditional_sections_match_independent_scalar_oracles_and_exact_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut scalars) = (0, 0);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, oracle) = next_case(&mut cursor);
        let fields = decode(kind, version, bytes).unwrap();
        scalars += super::node_units::assert_oracle(&fields, oracle);
        assert_eq!(encode_fields(&fields, LIMITS.maximum_bytes).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, scalars), (57, 941));
}

#[test]
fn every_conditional_section_prefix_and_trailing_byte_is_rejected() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(kind, version, &bytes[..length]).is_err(),
                "section {kind} version {version} prefix {length}"
            );
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(decode(kind, version, &trailing).is_err());
    }
}

#[test]
fn invalid_dynamic_array_shapes_and_unsupported_direct_payloads_fail() {
    for bytes in [
        vec![1, 0, 0, 0, 1, 0, 0, 0],
        [1_u32, 24, 1, 1, u32::MAX]
            .into_iter()
            .flat_map(u32::to_le_bytes)
            .collect(),
    ] {
        assert!(decode(0, 19000, &bytes).is_err());
    }
    let shrink: Vec<u8> = [0_i32, 1, 0, 0]
        .into_iter()
        .flat_map(i32::to_le_bytes)
        .collect();
    let error = decode(1, 19000, &shrink).unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Unsupported);
}

#[test]
fn invalid_and_zero_width_counts_cannot_panic_or_publish() {
    let negative = (-1_i32).to_le_bytes();
    for minimum in [0, 2] {
        let mut reader = ModelDecoder::new(&negative, ArchiveVersion::new(19000), LIMITS).unwrap();
        let error = reader.signed_count("invalid", minimum).unwrap_err();
        assert_eq!(reader.finish().unwrap_err(), error);
    }
    let mut reader = ModelDecoder::new(&[0, 0], ArchiveVersion::new(19000), LIMITS).unwrap();
    let error = reader.count_with_minimum("invalid", 0).unwrap_err();
    assert_eq!(reader.finish().unwrap_err(), error);
}

#[test]
fn required_component_null_is_a_typed_poisoning_failure() {
    let mut reader = ModelDecoder::new(&[0, 0], ArchiveVersion::new(19000), LIMITS).unwrap();
    let error = reader
        .required_object(
            "unit_component",
            &mut ObjectReader::new(),
            ModelObjectRole::Declared(&["moUnitComponent_c"]),
        )
        .unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Invalid);
    assert_eq!(reader.finish().unwrap_err(), error);
}
