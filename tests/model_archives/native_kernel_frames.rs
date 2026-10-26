use super::kernel_frame_tests::{decode, kernel};
use crate::archive_codec::ArchiveCursor;

#[test]
#[ignore = "requires explicit kernel schema and target/model-native-kernel-frames.bin"]
fn all_native_kernel_frames_match_reference_nodes_values_and_forced_encoding() {
    let context = super::native_corpus_tests::corpus_kernel_context().unwrap();
    let bytes = std::fs::read("target/model-native-kernel-frames.bin").unwrap();
    let mut cursor = ArchiveCursor::new(&bytes);
    assert_eq!(cursor.take(4).unwrap(), b"BKR1");
    assert_eq!(cursor.u32().unwrap(), 5);
    let (mut records, mut fields) = (0, 0);
    for _ in 0..5 {
        let (record_count, field_count) = check_frame(&mut cursor, &context);
        records += record_count;
        fields += field_count;
    }
    assert_eq!((records, fields), (236, 8351));
    assert_eq!(cursor.remaining(), 0);
}

fn check_frame(
    cursor: &mut ArchiveCursor<'_>,
    context: &super::kernel_frames::CompressedKernelContext,
) -> (usize, usize) {
    let size = cursor.u32().unwrap() as usize;
    let records = cursor.u32().unwrap() as usize;
    let oracle_size = cursor.u32().unwrap() as usize;
    let raw = cursor.take(size).unwrap();
    let oracle = std::str::from_utf8(cursor.take(oracle_size).unwrap()).unwrap();
    let decoded = decode(raw, Some(context.clone()), 1).unwrap();
    let frame = kernel(&decoded);
    assert_eq!(frame.record_count(), records);
    assert_eq!(frame.to_bytes().unwrap(), raw[1..]);
    let rows = super::kernel_rows::rows(frame);
    for line in oracle.lines() {
        let (path, expected) = line.split_once('=').unwrap();
        assert_eq!(rows.get(path).map(String::as_str), Some(expected), "{path}");
    }
    (records, oracle.lines().count())
}
