use super::direct_collection_tests::decode_collection;
use super::lightweight_section_tests::next_case;
use super::{ModelDecoder, ModelField, ModelFieldValue, ModelObjectRole, encode_fields};
use super::{sketch_dimensions, sketch_logical, sketch_patterns};
use crate::Result;
use crate::archive_codec::{ArchiveClass, ArchiveCursor, ObjectReader, ObjectRecord};

const CASES: &[u8] =
    include_bytes!("../fixtures/model-archives/sketch-dimension-pattern-cases.bin");
const ORIGINAL: &[u8] = include_bytes!("../fixtures/model-archives/sketch-pattern-original.bin");
const EDITED: &[u8] = include_bytes!("../fixtures/model-archives/sketch-pattern-edited.bin");
const LOADED: &[&str] = &[
    "moNotePattern_c",
    "sgLinearPattCntDim",
    "sgCircularPattCntDim",
];

fn seed(map: &mut ObjectReader, names: &[&str]) -> Result<()> {
    for name in names {
        map.append_loaded_object(Some(ArchiveClass {
            name: (*name).into(),
            schema: 7,
        }))?;
    }
    Ok(())
}

fn read_body(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, mode: u8) -> Result<()> {
    match mode {
        0 => reader.object("object", map, ModelObjectRole::Any),
        1 => sketch_dimensions::valued_collections(reader, map),
        2 => sketch_logical::collections(reader, map),
        3 => sketch_logical::isoparam_collection(reader, map),
        4 => sketch_patterns::linear_collection(reader, map),
        5 => sketch_patterns::following_collections(reader, map),
        6 => composition(reader, map),
        value => Err(super::invalid(format!(
            "dimension/pattern fixture mode {value}; expected 0..=6"
        ))),
    }
}

fn composition(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    sketch_dimensions::valued_collections(reader, map)?;
    sketch_logical::collections(reader, map)?;
    sketch_logical::isoparam_collection(reader, map)?;
    sketch_patterns::linear_collection(reader, map)?;
    sketch_patterns::following_collections(reader, map)
}

fn decode(mode: u8, version: u32, bytes: &[u8]) -> Result<Vec<ModelField>> {
    decode_collection(version, bytes, |reader, map| {
        if mode & 0x80 != 0 {
            seed(map, LOADED)?;
        }
        read_body(reader, map, mode & 0x7f)
    })
}

#[test]
fn sketch_dimension_and_pattern_records_match_independent_values_and_exact_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(mode, version, bytes).unwrap();
        fields += super::transform_rows::assert_fields(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (632, 19916));
}

#[test]
fn sketch_dimension_and_pattern_prefixes_and_trailing_bytes_reject_partial_results() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(mode, version, &bytes[..length]).is_err(),
                "mode {mode}, prefix {length}"
            );
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(decode(mode, version, &trailing).is_err());
    }
}

#[test]
fn pattern_constraint_edit_matches_independent_bytes_and_preserves_original() {
    let mut source = ORIGINAL.to_vec();
    let original = decode(0, 19000, &source).unwrap();
    source.fill(0xa5);
    let mut changed = original.clone();
    changed
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.x_scale")
        .unwrap()
        .value = ModelFieldValue::I32(12345);
    assert_eq!(encode_fields(&changed, 30000).unwrap(), EDITED);
    assert_eq!(decode(0, 19000, EDITED).unwrap(), changed);
    assert_eq!(encode_fields(&original, 30000).unwrap(), ORIGINAL);
}

#[test]
fn sketch_collection_word_counts_treat_ffff_as_literal_not_extended_count() {
    let bytes = [65535u16.to_le_bytes().as_slice(), &[0, 0, 0, 0]].concat();
    for mode in 1..=6 {
        let error = decode(mode, 19000, &bytes).unwrap_err();
        assert!(error.cause.contains("65535"));
    }
    assert!(decode(255, 19000, &[]).unwrap_err().cause.contains("127"));
}

#[test]
fn explicitly_loaded_pattern_targets_are_required_and_class_checked() {
    let mut cursor = ArchiveCursor::new(CASES);
    let mut checked = 0;
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        if mode & 0x80 == 0 {
            continue;
        }
        let fields = decode(mode, version, bytes).unwrap();
        if !fields.iter().any(is_loaded_reference) {
            continue;
        }
        assert!(decode(mode & 0x7f, version, bytes).is_err());
        let wrong = decode_collection(version, bytes, |reader, map| {
            seed(
                map,
                &[
                    "sgCircularPattCntDim",
                    "moNotePattern_c",
                    "sgLinearPattCntDim",
                ],
            )?;
            read_body(reader, map, mode & 0x7f)
        });
        assert!(wrong.is_err());
        checked += 1;
    }
    assert_eq!(checked, 72);
}

fn is_loaded_reference(field: &ModelField) -> bool {
    matches!(&field.value, ModelFieldValue::ObjectTag(tag)
        if matches!(&tag.record, ObjectRecord::Reference { index, .. } if index.value() <= 3))
}

#[test]
fn valued_dimension_groups_reject_null_and_unrelated_tagged_objects() {
    for replacement in [vec![0, 0], unrelated_node_tag()] {
        let bytes = [1u16.to_le_bytes().as_slice(), &replacement].concat();
        let error = decode(1, 19000, &bytes).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
    }
}

fn unrelated_node_tag() -> Vec<u8> {
    let name = b"moNodeName_c";
    [
        65535u16.to_le_bytes().as_slice(),
        &7u16.to_le_bytes(),
        &(name.len() as u16).to_le_bytes(),
        name,
    ]
    .concat()
}

#[test]
fn isoparametric_states_reject_unknown_widths_and_unbounded_counts() {
    let mut cursor = ArchiveCursor::new(CASES);
    let mut checked = 0;
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        if mode != 3 {
            continue;
        }
        let fields = decode(mode, version, bytes).unwrap();
        if !fields
            .iter()
            .any(|field| field.path.ends_with("segment_states.count"))
        {
            continue;
        }
        reject_state_mutations(mode, version, &fields);
        checked += 1;
    }
    assert_eq!(checked, 8);
}

fn reject_state_mutations(mode: u8, version: u32, original: &[ModelField]) {
    for (suffix, value) in [
        ("element_width", 1),
        ("element_width", 4),
        ("count", u32::MAX),
    ] {
        let mut fields = original.to_vec();
        fields
            .iter_mut()
            .find(|field| field.path.ends_with(&format!("segment_states.{suffix}")))
            .unwrap()
            .value = ModelFieldValue::U32(value);
        let bytes = encode_fields(&fields, 30000).unwrap();
        let error = decode(mode, version, &bytes).unwrap_err();
        assert_eq!(
            error.class,
            if suffix == "count" {
                crate::FailureClass::Invalid
            } else {
                crate::FailureClass::Unsupported
            }
        );
        assert!(error.cause.contains(&value.to_string()));
    }
}

#[test]
fn sketch_dimension_and_pattern_results_own_their_source() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        let mut source = bytes.to_vec();
        let fields = decode(mode, version, &source).unwrap();
        source.fill(0xa5);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
    }
}

#[test]
fn pattern_parallel_writes_preserve_shared_help_data_and_handles() {
    let fields = decode(0, 19000, ORIGINAL).unwrap();
    std::thread::scope(|scope| {
        let writers: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| encode_fields(&fields, 30000).unwrap()))
            .collect();
        for writer in writers {
            assert_eq!(writer.join().unwrap(), ORIGINAL);
        }
    });
}
