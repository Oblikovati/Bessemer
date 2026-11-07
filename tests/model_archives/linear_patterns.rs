use super::annotation_view_tests::LIMITS;
use super::direct_collection_tests::decode_limited;
use super::lightweight_section_tests::next_case;
use super::{ModelField, ModelFieldValue, ModelObjectRole, encode_fields};
use crate::archive_codec::ArchiveCursor;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/linear-pattern-cases.bin");
const ORIGINAL: &[u8] = include_bytes!("../fixtures/model-archives/linear-pattern-original.bin");
const EDITED: &[u8] = include_bytes!("../fixtures/model-archives/linear-pattern-edited.bin");

#[test]
fn pattern_direction_accepts_only_the_documented_line_family() {
    for (name, accepted) in [
        ("moLineRef_w", true),
        ("moTempAxisRef_w", true),
        ("moSurfaceRef_w", false),
        ("moLineRef_w_suffix", false),
    ] {
        let result = decode_limited(19000, &[1, 0], LIMITS, |reader, map| {
            map.append_loaded_object(Some(crate::archive_codec::ArchiveClass {
                name: name.into(),
                schema: 1,
            }))?;
            reader.object(
                "line",
                map,
                ModelObjectRole::Declared(super::geometric_references::LINE_REFERENCE_CLASSES),
            )
        });
        assert_eq!(result.is_ok(), accepted, "{name}: {result:?}");
    }
}

fn decode(mode: u8, version: u32, bytes: &[u8]) -> crate::Result<Vec<ModelField>> {
    decode_limited(version, bytes, LIMITS, |reader, map| {
        if mode == 0 {
            return reader.object("object", map, ModelObjectRole::Any);
        }
        reader.scope("object", |reader| match mode {
            1 => super::pattern_shape::read(reader, map),
            2 => super::pattern_variation::increment(reader, map),
            3 => super::pattern_variation::original(reader, map),
            4 => super::pattern_variation::read(reader, map),
            5 => super::linear_patterns::collections(reader, map),
            _ => Err(super::invalid(format!(
                "pattern mode {mode}; expected 0..=5"
            ))),
        })
    })
}

#[test]
fn complete_patterns_match_independent_fields_and_exact_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut values) = (0, 0);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, oracle) = next_case(&mut cursor);
        let fields = decode(mode, version, bytes).unwrap();
        values += super::transform_rows::assert_fields(&fields, oracle);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, values), (714, 132479));
}

#[test]
fn pattern_prefixes_and_trailing_bytes_never_publish_partial_results() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(mode, version, &bytes[..length]).is_err(),
                "mode {mode}, prefix {length}"
            );
        }
        assert!(decode(mode, version, &[bytes, &[0]].concat()).is_err());
    }
}

#[test]
fn pattern_edit_matches_independent_bytes_without_changing_the_original() {
    let fields = decode(0, 19000, ORIGINAL).unwrap();
    let mut changed = fields.clone();
    changed
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.tail.word_1120")
        .unwrap()
        .value = ModelFieldValue::I32(12345);
    assert_eq!(encode_fields(&changed, 30000).unwrap(), EDITED);
    assert_eq!(decode(0, 19000, EDITED).unwrap(), changed);
    assert_eq!(encode_fields(&fields, 30000).unwrap(), ORIGINAL);
}

#[test]
fn patterns_survive_source_overwrite_and_parallel_writes() {
    let mut source = ORIGINAL.to_vec();
    let fields = decode(0, 19000, &source).unwrap();
    source.fill(0xa5);
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
fn pattern_signed_dynamic_counts_reject_negative_and_impossible_extents() {
    let fields = decode(0, 19000, ORIGINAL).unwrap();
    let mut checked = 0;
    for (index, field) in fields.iter().enumerate().filter(|(_, field)| {
        field.path.ends_with(".count") && matches!(field.value, ModelFieldValue::I32(_))
    }) {
        for value in [i32::MIN, i32::MAX] {
            let mut changed = fields.clone();
            changed[index].value = ModelFieldValue::I32(value);
            assert!(
                decode(0, 19000, &encode_fields(&changed, 30000).unwrap()).is_err(),
                "{}",
                field.path
            );
        }
        checked += 1;
    }
    assert_eq!(checked, 22);
}

#[test]
fn pattern_collections_and_variations_charge_independent_entry_budgets() {
    let mut cursor = ArchiveCursor::new(CASES);
    let mut checked = 0;
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        let (body, required): (super::sketch_geometry::DirectReader, usize) = match mode {
            4 => (super::pattern_variation::read, 15),
            5 => (super::linear_patterns::collections, 6),
            _ => continue,
        };
        let fields = decode(mode, version, bytes).unwrap();
        if !fields.iter().any(|field| {
            matches!(
                field.value,
                ModelFieldValue::I32(2) | ModelFieldValue::U32(2)
            )
        }) {
            continue;
        }
        for maximum_entries in [required - 1, required] {
            let limits = super::ModelFieldLimits {
                maximum_entries,
                ..LIMITS
            };
            let result = decode_limited(version, bytes, limits, body);
            assert_eq!(
                result.is_ok(),
                maximum_entries == required,
                "mode {mode}: {result:?}"
            );
        }
        checked += 1;
    }
    assert_eq!(checked, 28);
}

#[test]
fn pattern_unsigned_extents_and_map_counts_reject_impossible_lengths() {
    let fields = decode(0, 19000, ORIGINAL).unwrap();
    for path in [
        "root.object.collections.map_838.count.count_dword",
        "root.object.collections.map_868.count.count_dword",
        "root.object.tail.indices_928.count",
    ] {
        let index = fields
            .iter()
            .position(|field| field.path.as_ref() == path)
            .unwrap();
        let mut changed = fields.clone();
        changed[index].value = ModelFieldValue::U32(u32::MAX);
        assert!(
            decode(0, 19000, &encode_fields(&changed, 30000).unwrap()).is_err(),
            "{path}"
        );
    }
}
