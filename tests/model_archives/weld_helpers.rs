use super::lightweight_section_tests::next_case;
use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelFieldLimits, ModelObjectRole, encode_fields};
use super::{datum_helpers, tolerance_helpers, weld_helpers};
use crate::Result;
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/weld-helper-cases.bin");
const NATIVE: &[u8] = include_bytes!("../fixtures/model-archives/native-weld-helpers.bin");
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
    read_case(kind, &mut reader, &mut map)?;
    reader.finish()
}

fn read_case(kind: u8, reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    match kind {
        0 => weld_helpers::weld(reader),
        1 => weld_helpers::gtol(reader, map),
        2 => datum_helpers::read(reader),
        3 => tolerance_helpers::read(reader),
        4 => reader.object(
            "object",
            map,
            ModelObjectRole::Declared(&["moGTolDataHelper_c"]),
        ),
        5 => weld_helpers::read(reader, map),
        _ => panic!("unexpected weld fixture kind {kind}"),
    }
}

#[test]
fn weld_gtol_datum_and_tolerance_cases_match_independent_fields_and_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(kind, version, bytes).unwrap();
        fields += super::node_units::assert_oracle(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 10000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (81, 6093));
}

#[test]
fn native_weld_and_gtol_helpers_match_every_scalar_and_stored_byte() {
    let fields = decode(5, 19000, NATIVE).unwrap();
    let oracle = include_str!("../fixtures/model-archives/native-weld-helpers.txt");
    assert_eq!(super::node_units::assert_oracle(&fields, oracle), 296);
    assert_eq!(encode_fields(&fields, 10000).unwrap(), NATIVE);
}

#[test]
fn every_weld_case_prefix_and_trailing_byte_rejects_incomplete_publication() {
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
fn helper_nested_counts_fail_closed_instead_of_allocating_unbounded_sequences() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, _) = next_case(&mut cursor);
        if kind != 1 && kind != 3 {
            continue;
        }
        let fields = decode(kind, version, bytes).unwrap();
        reject_invalid_counts(kind, version, bytes, &fields);
    }
}

fn reject_invalid_counts(kind: u8, version: u32, bytes: &[u8], fields: &[ModelField]) {
    for (index, field) in fields.iter().enumerate() {
        if !field.path.ends_with("_count") {
            continue;
        }
        let offset = encode_fields(&fields[..index], 10000).unwrap().len();
        for count in [-1_i32, i32::MAX] {
            let mut altered = bytes.to_vec();
            altered[offset..offset + 4].copy_from_slice(&count.to_le_bytes());
            assert!(
                decode(kind, version, &altered).is_err(),
                "{} count {count}",
                field.path
            );
        }
    }
}
