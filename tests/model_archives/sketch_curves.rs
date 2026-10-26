use super::direct_collection_tests::{decode_collection, decode_limited};
use super::lightweight_section_tests::next_case;
use super::{
    ModelDecoder, ModelField, ModelFieldValue, encode_fields, sketch_curves, sketch_surfaces,
};
use crate::Result;
use crate::archive_codec::{ArchiveCursor, ObjectReader};

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/sketch-curve-cases.bin");
const ORIGINAL: &[u8] = include_bytes!("../fixtures/model-archives/sketch-arc-original.bin");
const EDITED: &[u8] = include_bytes!("../fixtures/model-archives/sketch-arc-edited.bin");
const BAD_COUNTS: &[&str] = &[
    "index_arrays[0].count",
    "index_arrays[3].count",
    "contour_count",
];

fn read_mode(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, mode: u8) -> Result<()> {
    match mode {
        0 => sketch_curves::hatches(reader, map),
        1 => sketch_curves::arcs(reader, map),
        2 => sketch_curves::ellipses(reader, map),
        3 => sketch_curves::conics(reader, map),
        4 => sketch_surfaces::read(reader, map),
        5 => composition(reader, map),
        value => Err(super::invalid(format!(
            "curve fixture mode {value}; expected 0..=5"
        ))),
    }
}

fn composition(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    sketch_curves::hatches(reader, map)?;
    sketch_curves::arcs(reader, map)?;
    sketch_curves::ellipses(reader, map)?;
    sketch_curves::conics(reader, map)?;
    sketch_surfaces::read(reader, map)
}

fn decode(mode: u8, version: u32, bytes: &[u8]) -> Result<Vec<ModelField>> {
    decode_collection(version, bytes, |reader, map| read_mode(reader, map, mode))
}

#[test]
fn sketch_curves_and_surfaces_match_independent_values_and_exact_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(mode, version, bytes).unwrap();
        fields += super::transform_rows::assert_fields(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (1254, 46844));
}

#[test]
fn sketch_curve_and_surface_prefixes_and_trailing_bytes_fail_publication() {
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
fn sketch_arc_edit_matches_independent_wire_without_mutating_original() {
    let original = decode(1, 19000, ORIGINAL).unwrap();
    let mut changed = original.clone();
    let field = changed
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.arcs[0].stored_category_mask")
        .unwrap();
    field.value = ModelFieldValue::U16(12345);
    assert_eq!(encode_fields(&changed, 30000).unwrap(), EDITED);
    assert_eq!(decode(1, 19000, EDITED).unwrap(), changed);
    assert_eq!(encode_fields(&original, 30000).unwrap(), ORIGINAL);
}

#[test]
fn sketch_curve_counts_reject_negative_and_impossible_cardinalities() {
    for mode in 0..5 {
        for count in [-1i32, i32::MAX] {
            assert!(decode(mode, 19000, &count.to_le_bytes()).is_err());
        }
    }
    assert!(decode(255, 19000, &[]).unwrap_err().cause.contains("255"));
}

fn populated_hatch() -> (u32, Vec<u8>) {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        if mode == 0
            && decode(mode, version, bytes)
                .unwrap()
                .iter()
                .any(|field| field.path.as_ref() == "root.hatches[0].dword_array.items[2]")
        {
            return (version, bytes.to_vec());
        }
    }
    panic!("missing populated hatch fixture");
}

#[test]
fn hatch_parallel_arrays_and_containers_obey_the_exact_aggregate_budget() {
    let (version, bytes) = populated_hatch();
    assert!(hatch_budget(version, &bytes, 15).is_ok());
    assert!(hatch_budget(version, &bytes, 14).is_err());
}

fn hatch_budget(version: u32, bytes: &[u8], maximum_entries: usize) -> Result<Vec<ModelField>> {
    let limits = super::ModelFieldLimits {
        maximum_entries,
        ..super::annotation_view_tests::LIMITS
    };
    decode_limited(version, bytes, limits, sketch_curves::hatches)
}

#[test]
fn hatch_nested_lengths_fail_before_iterating_arrays_or_contours() {
    let (version, bytes) = populated_hatch();
    let original = decode(0, version, &bytes).unwrap();
    for path in BAD_COUNTS {
        for value in [-1, i32::MAX] {
            let mut fields = original.clone();
            let name = format!("root.hatches[0].{path}");
            fields
                .iter_mut()
                .find(|field| field.path.as_ref() == name)
                .unwrap()
                .value = ModelFieldValue::I32(value);
            let error = decode(0, version, &encode_fields(&fields, 30000).unwrap()).unwrap_err();
            assert_eq!(error.class, crate::FailureClass::Invalid);
        }
    }
}

#[test]
fn hatch_tagged_container_counts_cannot_exceed_remaining_bytes() {
    let (version, bytes) = populated_hatch();
    for member in ["object_list", "dword_array"] {
        let mut fields = decode(0, version, &bytes).unwrap();
        let path = format!("root.hatches[0].{member}.items.count.count_word");
        fields
            .iter_mut()
            .find(|field| field.path.as_ref() == path)
            .unwrap()
            .value = ModelFieldValue::U16(65534);
        let error = decode(0, version, &encode_fields(&fields, 30000).unwrap()).unwrap_err();
        assert!(error.cause.contains("65534"));
    }
}

#[test]
fn sketch_surface_entity_flags_are_send_sync_and_encoding_owns_the_source() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<super::sketch_entities::SketchEntityFlags>();
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
fn populated_hatch_parallel_writes_keep_cyclic_references_and_unsigned_words() {
    let (version, bytes) = populated_hatch();
    let fields = decode(0, version, &bytes).unwrap();
    std::thread::scope(|scope| {
        let writers: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| encode_fields(&fields, 30000).unwrap()))
            .collect();
        for writer in writers {
            assert_eq!(writer.join().unwrap(), bytes);
        }
    });
}
