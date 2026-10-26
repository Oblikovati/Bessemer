use super::annotation_view_tests::decode;
use super::lightweight_section_tests::next_case;
use super::{ModelFieldValue, encode_fields};
use crate::archive_codec::ArchiveCursor;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/explosion-cases.bin");
const REQUIRED: &[&str] = &[
    "root.object.body",
    "root.object.body_478",
    "root.object.step",
];

#[test]
fn explosion_records_match_independent_fields_transforms_and_exact_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (_, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(version, bytes).unwrap();
        fields += super::transform_rows::assert_fields(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (44, 1882));
}

#[test]
fn explosion_prefixes_and_trailing_bytes_reject_incomplete_results() {
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
fn explosion_edits_own_values_and_reparse_without_mutating_sources() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (_, version, bytes, _) = next_case(&mut cursor);
    let mut source = bytes.to_vec();
    let original = decode(version, &source).unwrap();
    source.fill(0xa5);
    let mut changed = original.clone();
    let field = changed
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.raw_360")
        .unwrap();
    field.value = ModelFieldValue::I32(12345);
    let encoded = encode_fields(&changed, 30000).unwrap();
    assert_eq!(
        encoded,
        include_bytes!("../fixtures/model-archives/explosion-edited.bin")
    );
    assert_eq!(decode(version, &encoded).unwrap(), changed);
    assert_eq!(encode_fields(&original, 30000).unwrap(), bytes);
}

#[test]
fn explosion_owned_fields_are_send_sync_and_parallel_writes_are_deterministic() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<super::ModelField>();
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        let fields = decode(version, bytes).unwrap();
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

#[test]
fn explosion_required_bodies_and_steps_reject_null_and_unrelated_classes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let mut checked = 0;
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        let fields = decode(version, bytes).unwrap();
        for (index, _) in fields
            .iter()
            .enumerate()
            .filter(|(_, field)| REQUIRED.contains(&field.path.as_ref()))
        {
            reject_required_reference(version, &fields[..index]);
            checked += 1;
        }
    }
    assert_eq!(checked, 12);
}

fn reject_required_reference(version: u32, prefix: &[super::ModelField]) {
    let prefix = encode_fields(prefix, 30000).unwrap();
    let null = [prefix.as_slice(), &[0, 0]].concat();
    assert!(
        decode(version, &null)
            .unwrap_err()
            .cause
            .contains("nonnull")
    );
    let unrelated = [prefix.as_slice(), b"\xff\xff\x01\x00\x0c\x00moNodeName_c"].concat();
    assert!(
        decode(version, &unrelated)
            .unwrap_err()
            .cause
            .contains("moNodeName_c")
    );
}

#[test]
fn explosion_dword_counts_reject_impossible_cardinality_before_iteration() {
    let mut cursor = ArchiveCursor::new(CASES);
    let mut checked = 0;
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        let Some(encoded) = oversized_dword_count(decode(version, bytes).unwrap()) else {
            continue;
        };
        let error = decode(version, &encoded).unwrap_err();
        assert!(error.cause.contains("65534"));
        checked += 1;
    }
    assert_eq!(checked, 16);
}

fn oversized_dword_count(mut fields: Vec<super::ModelField>) -> Option<Vec<u8>> {
    let count = fields
        .iter_mut()
        .find(|field| field.path.ends_with("raw_array_3a0.count.count_word"))?;
    count.value = ModelFieldValue::U16(65534);
    Some(encode_fields(&fields, 30000).unwrap())
}
