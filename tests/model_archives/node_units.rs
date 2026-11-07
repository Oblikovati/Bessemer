use std::sync::Arc;

use super::header_imports::ModelHeaderLoadPlan;
use super::{
    ModelDecoder, ModelField, ModelFieldLimits, ModelFieldValue, ModelObjectRole, encode_fields,
    node,
};
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;
use crate::{Document, FailureClass, Result};

const UNITS: &[u8] = include_bytes!("../fixtures/model-archives/unit-boundaries.bin");
const NODES: &[u8] = include_bytes!("../fixtures/model-archives/node-boundaries.bin");
const NATIVE_UNITS: &[u8] = include_bytes!("../fixtures/model-archives/native-units.bin");
const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 10000,
    maximum_fields: 2000,
    maximum_entries: 1000,
    maximum_depth: 32,
    maximum_objects: 128,
};

struct OwnedCase<'a> {
    version: ArchiveVersion,
    bytes: &'a [u8],
    oracle: &'a str,
}

fn owned_cases(input: &[u8]) -> Vec<OwnedCase<'_>> {
    let mut cursor = ArchiveCursor::new(input);
    let mut cases = Vec::new();
    while cursor.remaining() != 0 {
        let version = ArchiveVersion::new(cursor.u32().unwrap());
        let size = cursor.u32().unwrap() as usize;
        let oracle_size = cursor.u32().unwrap() as usize;
        let bytes = cursor.take(size).unwrap();
        let oracle = std::str::from_utf8(cursor.take(oracle_size).unwrap()).unwrap();
        cases.push(OwnedCase {
            version,
            bytes,
            oracle,
        });
    }
    cases
}

fn decode_unit(
    input: &[u8],
    version: ArchiveVersion,
    mut map: ObjectReader,
    label: &str,
) -> Result<Vec<ModelField>> {
    let mut decoder = ModelDecoder::new(input, version, LIMITS)?;
    decoder.object(label, &mut map, ModelObjectRole::Any)?;
    decoder.finish()
}

fn decode_fresh_unit(input: &[u8], version: ArchiveVersion) -> Result<Vec<ModelField>> {
    let map = ObjectReader::new();
    decode_unit(input, version, map, "unit")
}

fn expected_node_flags(oracle: &str) -> Option<u32> {
    oracle
        .lines()
        .find_map(|line| line.strip_prefix("root.unknown_2c=i:"))
        .map(|word| word.parse().unwrap())
}

pub(super) fn scalar_value(value: &ModelFieldValue) -> Option<String> {
    match value {
        ModelFieldValue::U8(v) => Some(format!("i:{v}")),
        ModelFieldValue::U16(v) => Some(format!("i:{v}")),
        ModelFieldValue::I16(v) => Some(format!("i:{v}")),
        ModelFieldValue::I32(v) => Some(format!("i:{v}")),
        ModelFieldValue::U32(v) => Some(format!("i:{v}")),
        ModelFieldValue::U64(v) => Some(format!("i:{v}")),
        ModelFieldValue::F32(v) => Some(format!(
            "f:{:016x}",
            (f32::from_bits(v.bits()) as f64).to_bits()
        )),
        ModelFieldValue::F64(v) => Some(format!("f:{:016x}", v.bits())),
        ModelFieldValue::Text(v) => Some(oracle_text(v)),
        _ => None,
    }
}

fn oracle_text(value: &crate::archive_codec::ArchiveString) -> String {
    let bytes: String = value
        .text()
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    format!("s:{bytes}")
}

pub(super) fn assert_oracle(fields: &[ModelField], expected: &str) -> usize {
    let mut checked = 0;
    for line in expected.lines() {
        let (path, value) = line.split_once('=').unwrap();
        let field = fields
            .iter()
            .find(|field| field.path.as_ref() == path)
            .unwrap_or_else(|| panic!("missing expected scalar field {path}"));
        assert_eq!(scalar_value(&field.value).as_deref(), Some(value), "{path}");
        checked += 1;
    }
    checked
}

fn native_configuration() -> Vec<u8> {
    let document = Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
    document.read_stream("Contents/Config-0").unwrap()
}

fn native_root_imports() -> ObjectReader {
    let header = Arc::new(super::header_load_plan::native_header());
    let mut map = super::header_load_plan::root_map();
    let plan = ModelHeaderLoadPlan::new(header, ArchiveVersion::new(19000), 64).unwrap();
    let mut reader = ModelDecoder::new(&[], ArchiveVersion::new(19000), LIMITS).unwrap();
    let _imports = plan.append(&mut reader, &mut map).unwrap();
    assert!(reader.finish().unwrap().is_empty());
    map
}

fn native_units_seed() -> ObjectReader {
    let configuration = native_configuration();
    let mut map = native_root_imports();
    map.object(&mut ArchiveCursor::new(&configuration[26..44]))
        .unwrap();
    map.object(&mut ArchiveCursor::new(&configuration[82..108]))
        .unwrap();
    map
}

#[test]
fn all_owned_unit_classes_and_version_boundaries_match_independent_fields_and_bytes() {
    let cases = owned_cases(UNITS);
    assert_eq!(cases.len(), 524);
    let mut checked = 0;
    for case in cases {
        let fields = decode_unit(case.bytes, case.version, ObjectReader::new(), "unit").unwrap();
        checked += assert_oracle(&fields, case.oracle);
        assert_eq!(
            encode_fields(&fields, 10000).unwrap(),
            case.bytes,
            "version {}",
            case.version.value()
        );
    }
    assert_eq!(checked, 5385);
}

#[test]
fn native_units_table_reconstructs_all_fields_with_the_exact_inherited_map() {
    let version = ArchiveVersion::new(19000);
    let fields = decode_unit(NATIVE_UNITS, version, native_units_seed(), "units").unwrap();
    let expected = include_str!("../fixtures/model-archives/native-units.txt");
    assert_eq!(assert_oracle(&fields, expected), 234);
    assert_eq!(encode_fields(&fields, 1487).unwrap(), NATIVE_UNITS);
    assert!(decode_unit(NATIVE_UNITS, version, ObjectReader::new(), "units").is_err());
}

#[test]
fn node_fields_keep_id_flags_and_version_gated_omissions() {
    let cases = owned_cases(NODES);
    assert_eq!(cases.len(), 7);
    let mut checked = 0;
    for case in cases {
        let mut reader = ModelDecoder::new(case.bytes, case.version, LIMITS).unwrap();
        let prefix = node::lightweight(&mut reader, &mut ObjectReader::new()).unwrap();
        assert_eq!((prefix.flags_28, prefix.id), (0x80000001, -7));
        assert_eq!(prefix.unknown_2c, expected_node_flags(case.oracle));
        let fields = reader.finish().unwrap();
        checked += assert_oracle(&fields, case.oracle);
        assert_eq!(encode_fields(&fields, 10000).unwrap(), case.bytes);
    }
    assert_eq!(checked, 36);
}

#[test]
fn native_node_prefix_reopens_after_typed_root_header_imports() {
    let configuration = native_configuration();
    let bytes = &configuration[26..78];
    let mut reader = ModelDecoder::new(bytes, ArchiveVersion::new(19000), LIMITS).unwrap();
    let prefix = node::lightweight(&mut reader, &mut native_root_imports()).unwrap();
    assert_eq!(
        (prefix.flags_28, prefix.id, prefix.unknown_2c),
        (0x40000000, 0, Some(0x8100))
    );
    assert_eq!(encode_fields(&reader.finish().unwrap(), 52).unwrap(), bytes);
}

#[test]
fn native_unit_prefixes_never_publish_incomplete_fields() {
    for end in 0..NATIVE_UNITS.len() {
        assert!(
            decode_unit(
                &NATIVE_UNITS[..end],
                ArchiveVersion::new(19000),
                native_units_seed(),
                "units"
            )
            .is_err(),
            "prefix {end}"
        );
    }
}

#[test]
fn native_node_prefixes_never_publish_incomplete_fields() {
    let configuration = native_configuration();
    for end in 26..78 {
        let mut reader =
            ModelDecoder::new(&configuration[26..end], ArchiveVersion::new(19000), LIMITS).unwrap();
        assert!(node::lightweight(&mut reader, &mut native_root_imports()).is_err());
        assert!(reader.finish().is_err());
    }
}

#[test]
fn unsupported_versions_and_unknown_class_bodies_remain_typed_failures() {
    let mut reader = ModelDecoder::new(&[], ArchiveVersion::new(0xc7c), LIMITS).unwrap();
    let error = node::lightweight(&mut reader, &mut ObjectReader::new()).unwrap_err();
    assert_eq!(error.class, FailureClass::Unsupported);
    assert_eq!(reader.finish().unwrap_err(), error);
    let bytes = b"\xff\xff\x01\x00\x07\x00Unknown";
    let error = decode_unit(
        bytes,
        ArchiveVersion::new(19000),
        ObjectReader::new(),
        "unit",
    )
    .unwrap_err();
    assert_eq!(error.class, FailureClass::Unsupported);
    assert!(error.cause.contains("Unknown"));
}

#[test]
fn short_and_wide_zero_counts_remain_distinct_and_impossible_counts_fail() {
    let tag = b"\xff\xff\x01\x00\x0e\x00moUnitsTable_c";
    for count in [&[0, 0][..], &[0xff, 0xff, 0, 0, 0, 0]] {
        let bytes = [tag.as_slice(), count].concat();
        let fields = decode_fresh_unit(&bytes, ArchiveVersion::new(19000)).unwrap();
        assert_eq!(encode_fields(&fields, 10000).unwrap(), bytes);
    }
    let bad = [tag.as_slice(), &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff]].concat();
    assert!(decode_fresh_unit(&bad, ArchiveVersion::new(19000)).is_err());
    let old = [tag.as_slice(), &[0, 0]].concat();
    let error = decode_fresh_unit(&old, ArchiveVersion::new(0x68)).unwrap_err();
    assert_eq!(error.class, FailureClass::Unsupported);
}

#[test]
fn native_unit_fields_own_inputs_and_parallel_parsers_use_independent_maps() {
    let mut input = NATIVE_UNITS.to_vec();
    let version = ArchiveVersion::new(19000);
    let fields = decode_unit(&input, version, native_units_seed(), "units").unwrap();
    input.fill(0);
    assert_eq!(encode_fields(&fields, 1487).unwrap(), NATIVE_UNITS);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..4)
            .map(|_| {
                scope.spawn(|| {
                    decode_unit(NATIVE_UNITS, version, native_units_seed(), "units").unwrap()
                })
            })
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), fields);
        }
    });
}

#[test]
fn node_name_role_mismatch_poisons_the_entire_node_read() {
    let tag = b"\xff\xff\x01\x00\x0e\x00moUnitsTable_c";
    let mut reader = ModelDecoder::new(tag, ArchiveVersion::new(19000), LIMITS).unwrap();
    let error = node::lightweight(&mut reader, &mut ObjectReader::new()).unwrap_err();
    assert_eq!(error.class, FailureClass::Invalid);
    assert!(error.cause.contains("moNodeName_c"));
    assert_eq!(reader.finish().unwrap_err(), error);
}
