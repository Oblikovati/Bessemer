use super::direct_collection_tests::decode_collection;
use super::lightweight_section_tests::next_case;
use super::{ModelDecoder, ModelField, ModelFieldValue, ModelObjectRole, encode_fields};
use super::{
    sketch_archive as archive, sketch_constraints as constraints,
    sketch_intersections as intersections,
};
use super::{sketch_managers as managers, sketch_offsets as offsets, sketch_tail as tail};
use crate::Result;
use crate::archive_codec::{ArchiveCursor, ObjectReader};

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/sketch-tail-cases.bin");
const ORIGINAL: &[u8] = include_bytes!("../fixtures/model-archives/sketch-tail-original.bin");
const EDITED: &[u8] = include_bytes!("../fixtures/model-archives/sketch-tail-edited.bin");
const READERS: &[super::sketch_geometry::DirectReader] = &[
    object,
    constraints::points,
    constraints::constraint,
    constraints::logical_group,
    constraints::collections,
    offsets::collections,
    intersections::collections,
    state,
    tail::visual,
    tail::local_relations,
    tail::coincidences,
    silhouettes,
    managers::blocks,
    managers::loops,
    managers::slots,
    final_ordinary,
    archive::after_patterns,
    archive::ordinary,
    dynamic_words,
    intersections::intersection,
    offsets::sketch_offset,
    offsets::offset,
    intersections::styled,
    managers::handle_data,
];

fn object(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.object("object", map, ModelObjectRole::Any)
}

fn state(reader: &mut ModelDecoder<'_>, _: &mut ObjectReader) -> Result<()> {
    tail::state(reader)
}

fn silhouettes(reader: &mut ModelDecoder<'_>, _: &mut ObjectReader) -> Result<()> {
    tail::silhouettes(reader)
}

fn final_ordinary(reader: &mut ModelDecoder<'_>, _: &mut ObjectReader) -> Result<()> {
    tail::final_ordinary(reader)
}

fn dynamic_words(reader: &mut ModelDecoder<'_>, _: &mut ObjectReader) -> Result<()> {
    intersections::dynamic_words(reader)
}

fn decode(mode: u8, version: u32, bytes: &[u8]) -> Result<Vec<ModelField>> {
    let read = READERS.get(usize::from(mode)).ok_or_else(|| {
        super::invalid(format!("sketch tail fixture mode {mode}; expected 0..=23"))
    })?;
    decode_collection(version, bytes, read)
}

#[test]
fn complete_sketch_tails_and_roots_match_independent_values_and_exact_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(mode, version, bytes).unwrap();
        fields += super::transform_rows::assert_fields(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (1038, 33645));
}

#[test]
fn sketch_tail_and_root_prefixes_and_trailing_bytes_cannot_publish_partial_results() {
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
        assert!(decode(mode, version, &trailing).is_err(), "mode {mode}");
    }
}

#[test]
fn complete_sketch_edit_matches_independent_bytes_and_leaves_original_isolated() {
    let original = decode(0, 19000, ORIGINAL).unwrap();
    let mut changed = original.clone();
    changed
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.final_fields.word_134")
        .unwrap()
        .value = ModelFieldValue::I32(12345);
    assert_eq!(encode_fields(&changed, 30000).unwrap(), EDITED);
    assert_eq!(decode(0, 19000, EDITED).unwrap(), changed);
    assert_eq!(encode_fields(&original, 30000).unwrap(), ORIGINAL);
}

#[test]
fn complete_sketch_tails_and_roots_own_their_source() {
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
fn complete_sketch_parallel_writes_preserve_shared_objects_and_duplicate_keys() {
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

#[test]
fn unknown_silhouette_offset_and_dynamic_array_forms_stop_explicitly() {
    assert_eq!(
        tail_error(11, 19000, &[1, 0, 0, 0]).class,
        crate::FailureClass::Unsupported
    );
    for width in [1u32, 2, 3, 6, u32::MAX] {
        let error = tail_error(18, 19000, &width.to_le_bytes());
        assert!(error.cause.contains(&width.to_string()));
    }
    assert!(tail_error(3, 0x46cb, &[]).cause.contains("18123"));
    assert!(tail_error(255, 19000, &[]).cause.contains("255"));
}

fn tail_error(mode: u8, version: u32, bytes: &[u8]) -> crate::BessemerError {
    decode(mode, version, bytes).unwrap_err()
}

#[test]
fn manager_and_nested_array_counts_fail_before_iteration() {
    let original = decode(0, 19000, ORIGINAL).unwrap();
    let mut checked = 0;
    for (index, field) in original.iter().enumerate() {
        if !field.path.ends_with(".count") {
            continue;
        }
        let bad = match field.value {
            ModelFieldValue::I32(_) => ModelFieldValue::I32(i32::MAX),
            ModelFieldValue::U32(_) => ModelFieldValue::U32(u32::MAX),
            _ => continue,
        };
        let mut changed = original.clone();
        changed[index].value = bad;
        assert!(
            decode(0, 19000, &encode_fields(&changed, 30000).unwrap()).is_err(),
            "{}",
            field.path
        );
        checked += 1;
    }
    assert_eq!(checked, 16);
    assert_eq!(encode_fields(&original, 30000).unwrap(), ORIGINAL);
}

#[test]
fn sketch_external_entity_references_accept_only_the_documented_wrapper_family() {
    use crate::archive_codec::ArchiveClass;
    for name in [
        "moRefWrapper_c",
        "moSketchExtRef_w",
        "moTempAxisRef_w",
        "moNodeName_c",
    ] {
        let bytes = [1u16.to_le_bytes().as_slice(), &[0; 7]].concat();
        let result = decode_collection(19000, &bytes, |reader, map| {
            map.append_loaded_object(Some(ArchiveClass {
                name: name.into(),
                schema: 1,
            }))?;
            super::sketch_references::external_entity(reader, map)
        });
        assert_eq!(result.is_ok(), name != "moNodeName_c", "{name}");
        if let Ok(fields) = result {
            assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
        }
    }
}

#[test]
fn dynamic_word_arrays_charge_the_independent_total_and_bound_empty_wide_arrays() {
    use super::annotation_view_tests::LIMITS;
    use super::direct_collection_tests::decode_limited;
    let bytes: Vec<u8> = [8u32, 0, 0, 2, 1, 2, 3, 4]
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect();
    for maximum_entries in [5, 6] {
        let limits = super::ModelFieldLimits {
            maximum_entries,
            ..LIMITS
        };
        assert_eq!(
            decode_limited(19000, &bytes, limits, dynamic_words).is_ok(),
            maximum_entries == 6
        );
    }
    let empty: Vec<u8> = [u32::MAX - 3, 1, 2, 0]
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect();
    assert_eq!(
        encode_fields(&decode(18, 19000, &empty).unwrap(), 30000).unwrap(),
        empty
    );
}

#[test]
fn unsupported_offset_kernel_branches_never_publish_their_typed_prefix() {
    let original = decode(0, 19000, ORIGINAL).unwrap();
    for kind in [8, 9] {
        let mut changed = original.clone();
        changed
            .iter_mut()
            .find(|field| field.path.ends_with("offsets[0].kind_4c"))
            .unwrap()
            .value = ModelFieldValue::U16(kind);
        let error = decode(0, 19000, &encode_fields(&changed, 30000).unwrap()).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Unsupported);
        assert!(error.cause.contains(&kind.to_string()));
    }
    assert_eq!(encode_fields(&original, 30000).unwrap(), ORIGINAL);
}

#[test]
fn offset_parallel_arrays_charge_all_four_independently_counted_groups() {
    use super::annotation_view_tests::LIMITS;
    use super::direct_collection_tests::decode_limited;
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        if mode != 21 || version != 19000 || bytes[..2] != 3u16.to_le_bytes() {
            continue;
        }
        for maximum_entries in [11, 12] {
            let limits = super::ModelFieldLimits {
                maximum_entries,
                ..LIMITS
            };
            assert_eq!(
                decode_limited(version, bytes, limits, offsets::offset).is_ok(),
                maximum_entries == 12
            );
        }
        return;
    }
    panic!("expected independently assembled populated offset fixture");
}
