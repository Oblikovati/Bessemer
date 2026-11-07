use super::annotation_view_tests::decode;
use super::lightweight_section_tests::next_case;
use super::{ModelFieldValue, encode_fields};
use crate::archive_codec::ArchiveCursor;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/sketch-reference-cases.bin");
const BAD_STATES: &[(&str, ModelFieldValue, crate::FailureClass)] = &[
    (
        "root.object.mode",
        ModelFieldValue::I16(1),
        crate::FailureClass::Invalid,
    ),
    (
        "root.object.entity.minimal",
        ModelFieldValue::U16(65535),
        crate::FailureClass::Unsupported,
    ),
];

#[test]
fn sketch_supporting_records_match_independent_values_and_exact_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (_, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(version, bytes).unwrap();
        fields += super::transform_rows::assert_fields(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (259, 2960));
}

#[test]
fn sketch_supporting_prefixes_and_trailing_bytes_reject_incomplete_results() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(version, &bytes[..length]).is_err(),
                "prefix {length}"
            );
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(decode(version, &trailing).is_err());
    }
}

#[test]
fn sketch_reference_mode_and_minimal_owner_states_reject_unsupported_shapes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let mut checked = 0;
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        let fields = decode(version, bytes).unwrap();
        checked += reject_changed_states(version, fields);
    }
    assert_eq!(checked, 48);
}

fn reject_changed_states(version: u32, fields: Vec<super::ModelField>) -> usize {
    for (path, value, class) in BAD_STATES {
        let mut changed = fields.clone();
        let Some(field) = changed
            .iter_mut()
            .find(|field| field.path.as_ref() == *path)
        else {
            continue;
        };
        field.value = value.clone();
        let bytes = encode_fields(&changed, 30000).unwrap();
        let error = decode(version, &bytes).unwrap_err();
        assert_eq!(error.class, *class);
        assert!(error.cause.contains("expected"));
        return 1;
    }
    0
}

#[test]
fn escaped_handle_edit_matches_independent_wire_and_preserves_source() {
    let mut cursor = ArchiveCursor::new(CASES);
    for _ in 0..77 {
        next_case(&mut cursor);
    }
    let (_, version, bytes, _) = next_case(&mut cursor);
    let original = decode(version, bytes).unwrap();
    let mut changed = original.clone();
    let field = changed
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.reference_id")
        .unwrap();
    field.value = ModelFieldValue::I32(12345);
    let edited = encode_fields(&changed, 30000).unwrap();
    let expected = include_bytes!("../fixtures/model-archives/sketch-handle-edited.bin");
    assert_eq!(edited, expected);
    assert_eq!(decode(version, &edited).unwrap(), changed);
    assert_eq!(encode_fields(&original, 30000).unwrap(), bytes);
}

#[test]
fn sketch_supporting_records_own_sources_and_parallel_encoding_is_deterministic() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        let mut source = bytes.to_vec();
        let fields = decode(version, &source).unwrap();
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
