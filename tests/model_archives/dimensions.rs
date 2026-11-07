use super::ModelFieldValue;
use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelObjectRole, encode_fields};
use crate::Result;
use crate::archive_codec::{ArchiveClass, ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/dimension-cases.bin");

fn loaded_map(mode: u8) -> Result<ObjectReader> {
    let mut map = ObjectReader::new();
    let name = match mode {
        0 => return Ok(map),
        1 => "moGFR_c",
        2 => "sgSketch",
        _ => {
            return Err(super::invalid(format!(
                "fixture loaded mode {mode}; expected 0..=2"
            )));
        }
    };
    map.append_loaded_object(Some(ArchiveClass {
        name: name.into(),
        schema: 7,
    }))?;
    Ok(map)
}

fn decode(mode: u8, version: u32, bytes: &[u8]) -> Result<Vec<ModelField>> {
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(version),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )?;
    reader.object("object", &mut loaded_map(mode)?, ModelObjectRole::Any)?;
    reader.finish()
}

#[test]
fn dimension_payloads_and_handles_match_independent_fields_and_exact_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(mode, version, bytes).unwrap();
        fields += super::transform_rows::assert_fields(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (661, 35708));
}

#[test]
fn dimension_prefixes_and_trailing_bytes_reject_incomplete_publication() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(mode, version, &bytes[..length]).is_err(),
                "prefix {length}, mode {mode}"
            );
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(decode(mode, version, &trailing).is_err());
    }
}

#[test]
fn dimension_handle_loaded_context_is_required_and_class_checked() {
    let mut cursor = ArchiveCursor::new(CASES);
    let mut checked = 0;
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        if mode == 0 {
            continue;
        }
        assert!(decode(0, version, bytes).is_err());
        assert!(decode(3 - mode, version, bytes).is_err());
        checked += 1;
    }
    assert_eq!(checked, 160);
    assert!(loaded_map(255).err().unwrap().cause.contains("255"));
}

#[test]
fn dimension_fields_own_scalar_values_and_transforms_after_source_overwrite() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        let mut source = bytes.to_vec();
        let fields = decode(mode, version, &source).unwrap();
        source.fill(0xa5);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
    }
}

fn edited_source() -> Vec<ModelField> {
    let mut cursor = ArchiveCursor::new(CASES);
    for _ in 0..7 {
        next_case(&mut cursor);
    }
    let (mode, version, bytes, _) = next_case(&mut cursor);
    assert_eq!((mode, version), (0, 19000));
    decode(mode, version, bytes).unwrap()
}

#[test]
fn dimension_edit_matches_independent_expected_bytes_and_preserves_original() {
    let original = edited_source();
    let original_bytes = encode_fields(&original, 30000).unwrap();
    let mut fields = original.clone();
    let count = fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.reference_count")
        .unwrap();
    count.value = ModelFieldValue::I32(12345);
    let encoded = encode_fields(&fields, 30000).unwrap();
    assert_eq!(
        encoded,
        include_bytes!("../fixtures/model-archives/dimension-edited.bin")
    );
    assert_eq!(decode(0, 19000, &encoded).unwrap(), fields);
    assert_eq!(encode_fields(&original, 30000).unwrap(), original_bytes);
}

#[test]
fn dimension_handles_reject_null_required_internal_dimensions_and_sketches() {
    let mut cursor = ArchiveCursor::new(CASES);
    let mut checked = 0;
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        let fields = decode(mode, version, bytes).unwrap();
        for (index, _) in fields.iter().enumerate().filter(|(_, field)| {
            ["root.object.dimension", "root.object.sketch"].contains(&field.path.as_ref())
        }) {
            let mut prefix = encode_fields(&fields[..index], 30000).unwrap();
            prefix.extend_from_slice(&[0, 0]);
            let error = decode(mode, version, &prefix).unwrap_err();
            assert!(error.cause.contains("nonnull"));
            checked += 1;
        }
    }
    assert_eq!(checked, 96);
}
