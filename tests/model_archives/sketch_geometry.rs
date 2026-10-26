use super::lightweight_section_tests::next_case;
use super::{ModelDecoder, ModelField, ModelFieldValue, encode_fields, sketch_geometry};
use crate::Result;
use crate::archive_codec::{ArchiveCursor, ObjectReader};

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/sketch-geometry-cases.bin");
const ORIGINAL: &[u8] = include_bytes!("../fixtures/model-archives/sketch-line-original.bin");
const EDITED: &[u8] = include_bytes!("../fixtures/model-archives/sketch-line-edited.bin");

fn decode(mode: u8, version: u32, bytes: &[u8]) -> Result<Vec<ModelField>> {
    super::direct_collection_tests::decode_collection(version, bytes, |reader, map| {
        read_mode(reader, map, mode)
    })
}

fn read_mode(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, mode: u8) -> Result<()> {
    match mode {
        0 => sketch_geometry::points(reader, map),
        1 => sketch_geometry::lines(reader, map),
        2 => sketch_geometry::texts(reader, map),
        3 => {
            sketch_geometry::points(reader, map)?;
            sketch_geometry::lines(reader, map)?;
            sketch_geometry::texts(reader, map)
        }
        value => Err(super::invalid(format!(
            "collection fixture mode {value}; expected 0..=3"
        ))),
    }
}

#[test]
fn direct_point_line_text_collections_match_independent_fields_and_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(mode, version, bytes).unwrap();
        fields += super::transform_rows::assert_fields(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (308, 12378));
}

#[test]
fn direct_sketch_collection_prefixes_and_trailing_bytes_never_publish() {
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
fn direct_line_edit_matches_independent_bytes_and_preserves_original() {
    let mut source = ORIGINAL.to_vec();
    let original = decode(1, 19000, &source).unwrap();
    source.fill(0xa5);
    let mut changed = original.clone();
    let field = changed
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.lines[0].unknown_e8")
        .unwrap();
    field.value = ModelFieldValue::U16(12345);
    assert_eq!(encode_fields(&changed, 30000).unwrap(), EDITED);
    assert_eq!(decode(1, 19000, EDITED).unwrap(), changed);
    assert_eq!(encode_fields(&original, 30000).unwrap(), ORIGINAL);
}

#[test]
fn direct_sketch_unsupported_point_and_kernel_text_states_fail_explicitly() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (_, version, bytes, _) = next_case(&mut cursor);
    for (path, value) in [("mask", 0x8020), ("has_jet", 65535)] {
        let mut fields = decode(0, version, bytes).unwrap();
        let path = format!("root.points[0].{path}");
        fields
            .iter_mut()
            .find(|field| field.path.as_ref() == path)
            .unwrap()
            .value = ModelFieldValue::U16(value);
        let error = decode(0, version, &encode_fields(&fields, 30000).unwrap()).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Unsupported);
    }
    let kernel = include_bytes!("../fixtures/model-archives/sketch-text-kernel-unsupported.bin");
    let error = decode(2, 19000, kernel).unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Unsupported);
    assert!(decode(255, 19000, &[]).unwrap_err().cause.contains("255"));
}

#[test]
fn direct_sketch_counts_reject_negative_and_impossible_lengths() {
    for mode in 0..3 {
        for count in [-1i32, i32::MAX] {
            assert!(decode(mode, 19000, &count.to_le_bytes()).is_err());
        }
    }
}

#[test]
fn direct_sketch_collection_results_own_input_and_parallel_writes_agree() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        let mut source = bytes.to_vec();
        let fields = decode(mode, version, &source).unwrap();
        source.fill(0xa5);
        std::thread::scope(|scope| {
            let writers: Vec<_> = (0..4)
                .map(|_| scope.spawn(|| encode_fields(&fields, 30000).unwrap()))
                .collect();
            for writer in writers {
                assert_eq!(writer.join().unwrap(), bytes);
            }
        });
    }
}
