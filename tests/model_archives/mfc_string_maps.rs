use super::reader::ModelArchiveContext;
use super::{
    ModelDecoder, ModelField, ModelFieldLimits, ModelFieldValue, ModelObjectRole, encode_fields,
};
use crate::Result;
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;

const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 5000,
    maximum_fields: 2000,
    maximum_entries: 1000,
    maximum_depth: 32,
    maximum_objects: 20,
};

struct MapCase {
    kind: u8,
    bytes: &'static [u8],
    oracle: &'static str,
    original: &'static str,
    slots: usize,
}

fn cases() -> Vec<MapCase> {
    let mut cursor = ArchiveCursor::new(include_bytes!(
        "../fixtures/assembly-configurations/string-maps.bin"
    ));
    assert_eq!(cursor.take(4).unwrap(), b"MS64");
    let result = (0..cursor.u32().unwrap())
        .map(|_| next_case(&mut cursor))
        .collect();
    cursor.finish().unwrap();
    result
}

fn next_case(cursor: &mut ArchiveCursor<'static>) -> MapCase {
    let kind = cursor.u8().unwrap();
    let raw = cursor.u32().unwrap() as usize;
    let oracle = cursor.u32().unwrap() as usize;
    let original = cursor.u32().unwrap() as usize;
    let slots = cursor.u32().unwrap() as usize;
    MapCase {
        kind,
        bytes: cursor.take(raw).unwrap(),
        oracle: std::str::from_utf8(cursor.take(oracle).unwrap()).unwrap(),
        original: std::str::from_utf8(cursor.take(original).unwrap()).unwrap(),
        slots,
    }
}

fn decode(bytes: &[u8], limits: ModelFieldLimits) -> Result<(Vec<ModelField>, usize)> {
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(19000),
        limits,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )?;
    let mut map = ObjectReader::new();
    reader.object("object", &mut map, ModelObjectRole::Any)?;
    Ok((reader.finish()?, map.into_snapshot().slot_count()))
}

fn assert_original_inventory(bytes: &[u8], oracle: &str) {
    let mut end = 0;
    for row in oracle.lines() {
        let parts: Vec<_> = row.split('\t').collect();
        let start: usize = parts[0].parse().unwrap();
        let next: usize = parts[1].parse().unwrap();
        assert_eq!(start, end);
        let expected: Vec<u8> = parts[3]
            .as_bytes()
            .as_chunks::<2>()
            .0
            .iter()
            .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
            .collect();
        assert_eq!(&bytes[start..next], expected);
        end = next;
    }
    assert_eq!(end, bytes.len());
}

#[test]
fn every_map_prefix_width_code_unit_and_bend_caller_matches_the_actual_reference() {
    let records = cases();
    assert_eq!(records.len(), 28);
    assert_eq!(records.iter().filter(|case| case.kind == 1).count(), 1);
    for case in records {
        let (fields, slots) = decode(case.bytes, LIMITS).unwrap();
        assert_eq!(slots, case.slots);
        assert_original_inventory(case.bytes, case.original);
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
fn every_map_and_bend_prefix_and_appended_byte_fails_without_partial_success() {
    for case in cases() {
        for length in 0..case.bytes.len() {
            assert!(decode(&case.bytes[..length], LIMITS).is_err());
        }
        let mut trailing = case.bytes.to_vec();
        trailing.push(0);
        assert!(decode(&trailing, LIMITS).is_err());
    }
}

#[test]
fn map_code_unit_edits_match_independent_goldens_and_preserve_the_source() {
    for (original, expected, suffix, value) in [
        (
            include_bytes!("../fixtures/assembly-configurations/map-latin1.bin").as_slice(),
            include_bytes!("../fixtures/assembly-configurations/map-latin1-edited.bin").as_slice(),
            "latin1[0]",
            ModelFieldValue::U8(254),
        ),
        (
            include_bytes!("../fixtures/assembly-configurations/map-utf16.bin").as_slice(),
            include_bytes!("../fixtures/assembly-configurations/map-utf16-edited.bin").as_slice(),
            "utf16[0]",
            ModelFieldValue::U16(0xd801),
        ),
    ] {
        let baseline = decode(original, LIMITS).unwrap().0;
        let mut fields = baseline.clone();
        let path = format!("root.object.entries[0].key.map_text.{suffix}");
        fields
            .iter_mut()
            .find(|field| field.path.as_ref() == path)
            .unwrap()
            .value = value;
        assert_eq!(
            encode_fields(&fields, LIMITS.maximum_bytes).unwrap(),
            expected
        );
        assert_eq!(decode(expected, LIMITS).unwrap().0, fields);
        assert_eq!(
            encode_fields(&baseline, LIMITS.maximum_bytes).unwrap(),
            original
        );
    }
}

#[test]
fn unpaired_utf16_units_and_empty_qword_maps_are_preserved_as_complete_values() {
    let mut unpaired = false;
    let mut empty = false;
    for case in cases() {
        let fields = decode(case.bytes, LIMITS).unwrap().0;
        unpaired |= fields.iter().any(|field| {
            field.path.contains(".map_text.utf16[") && field.value == ModelFieldValue::U16(0xd800)
        });
        empty |= fields.iter().any(|field| {
            field.path.ends_with("entries.count.count_qword")
                && field.value == ModelFieldValue::U64(0)
        });
    }
    assert!(unpaired && empty);
}

#[test]
fn huge_map_and_string_counts_and_global_entry_budgets_fail_before_allocating() {
    let raw = include_bytes!("../fixtures/assembly-configurations/map-latin1.bin");
    let mut cursor = ArchiveCursor::new(raw);
    ObjectReader::new().object(&mut cursor).unwrap();
    let header = raw.len() - cursor.remaining();
    let mut count = raw[..header].to_vec();
    count.extend_from_slice(&u16::MAX.to_le_bytes());
    count.extend_from_slice(&u32::MAX.to_le_bytes());
    count.extend_from_slice(&u64::MAX.to_le_bytes());
    assert!(decode(&count, LIMITS).is_err());
    let mut text = raw[..header].to_vec();
    text.extend_from_slice(&1_u16.to_le_bytes());
    text.extend_from_slice(&[255, 255, 255]);
    text.extend_from_slice(&u32::MAX.to_le_bytes());
    text.extend_from_slice(&u64::MAX.to_le_bytes());
    assert!(decode(&text, LIMITS).is_err());
    let mut limited = LIMITS;
    limited.maximum_entries = 2;
    assert!(decode(raw, limited).is_err());
}

#[test]
fn map_codecs_reject_unselected_or_older_contexts_and_keep_the_operation_failed() {
    let raw = include_bytes!("../fixtures/assembly-configurations/map-latin1.bin");
    for (version, context) in [
        (19000, ModelArchiveContext::Unspecified),
        (0x27cb, ModelArchiveContext::OrdinaryPartExternalXml),
    ] {
        let mut reader =
            ModelDecoder::new_in_context(raw, ArchiveVersion::new(version), LIMITS, context)
                .unwrap();
        let mut map = ObjectReader::new();
        let error = reader
            .object("object", &mut map, ModelObjectRole::Any)
            .unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Unsupported);
        let Err(final_error) = reader.finish() else {
            panic!("expected a poisoned map operation");
        };
        assert_eq!(error, final_error);
    }
}
