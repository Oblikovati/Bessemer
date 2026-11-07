use super::reader::ModelArchiveContext;
use super::{
    ModelDecoder, ModelField, ModelFieldLimits, ModelFieldValue, ModelObjectRole, encode_fields,
};
use crate::Result;
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;

const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 100_000,
    maximum_fields: 30_000,
    maximum_entries: 30_000,
    maximum_depth: 128,
    maximum_objects: 512,
};

struct FormingCase {
    version: u32,
    style: u8,
    bytes: &'static [u8],
    oracle: &'static str,
    slots: usize,
}

fn cases() -> Vec<FormingCase> {
    let mut cursor = ArchiveCursor::new(include_bytes!("../fixtures/forming-tools/instances.bin"));
    assert_eq!(cursor.take(4).unwrap(), b"FI01");
    let result = (0..cursor.u32().unwrap())
        .map(|_| next_case(&mut cursor))
        .collect();
    cursor.finish().unwrap();
    result
}

fn next_case(cursor: &mut ArchiveCursor<'static>) -> FormingCase {
    let version = cursor.u32().unwrap();
    let style = cursor.u8().unwrap();
    let raw_length = cursor.u32().unwrap() as usize;
    let oracle_length = cursor.u32().unwrap() as usize;
    let slots = cursor.u32().unwrap() as usize;
    let bytes = cursor.take(raw_length).unwrap();
    let oracle = std::str::from_utf8(cursor.take(oracle_length).unwrap()).unwrap();
    FormingCase {
        version,
        style,
        bytes,
        oracle,
        slots,
    }
}

fn decode(bytes: &[u8], version: u32) -> Result<(Vec<ModelField>, usize)> {
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(version),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )?;
    let mut map = ObjectReader::new();
    reader.object("object", &mut map, ModelObjectRole::Any)?;
    Ok((reader.finish()?, map.into_snapshot().slot_count()))
}

#[test]
fn complete_instances_and_declared_representation_callers_match_all_source_primitives() {
    let records = cases();
    assert_eq!(records.len(), 26);
    for case in records {
        let (fields, slots) = decode(case.bytes, case.version).unwrap();
        assert_eq!(slots, case.slots);
        super::manager_oracle_tests::assert_reference_fields(
            &fields,
            case.bytes,
            case.oracle,
            case.oracle.lines().count(),
        );
        assert_eq!(
            encode_fields(&fields, LIMITS.maximum_bytes).unwrap(),
            case.bytes
        );
    }
}

#[test]
fn every_direct_instance_prefix_and_appended_byte_rejects_incomplete_publication() {
    for case in cases().into_iter().filter(|case| case.style != 6) {
        for length in 0..case.bytes.len() {
            assert!(
                decode(&case.bytes[..length], case.version).is_err(),
                "v{} style{} prefix{}",
                case.version,
                case.style,
                length
            );
        }
        let mut trailing = case.bytes.to_vec();
        trailing.push(0);
        assert!(decode(&trailing, case.version).is_err());
        assert!(decode(case.bytes, 0x27cb).is_err());
    }
}

#[test]
fn independent_instance_word_edit_preserves_all_other_fields_and_reverts() {
    let original = include_bytes!("../fixtures/forming-tools/instance-19000-1.bin");
    let expected = include_bytes!("../fixtures/forming-tools/edited-instance.bin");
    let baseline = decode(original, 19000).unwrap().0;
    let mut fields = baseline.clone();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.payload.raw_9d8")
        .unwrap()
        .value = ModelFieldValue::I32(123456);
    assert_eq!(
        encode_fields(&fields, LIMITS.maximum_bytes).unwrap(),
        expected
    );
    assert_eq!(decode(expected, 19000).unwrap().0, fields);
    assert_eq!(
        encode_fields(&baseline, LIMITS.maximum_bytes).unwrap(),
        original
    );
}

#[test]
fn required_profile_and_declared_cached_profile_roles_remain_load_bearing() {
    let original = include_bytes!("../fixtures/forming-tools/instance-19000-1.bin");
    let fields = decode(original, 19000).unwrap().0;
    let profile = fields
        .iter()
        .position(|field| field.path.as_ref() == "root.object.payload.profile_8f8")
        .unwrap();
    let offset = encode_fields(&fields[..profile], LIMITS.maximum_bytes)
        .unwrap()
        .len();
    for tag in [0_u16, 2] {
        let mut malformed = original.to_vec();
        malformed[offset..offset + 2].copy_from_slice(&tag.to_le_bytes());
        assert!(decode(&malformed, 19000).is_err());
    }
}
