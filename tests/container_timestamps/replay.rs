use super::*;
use std::io::{Read, Write};
const START: &[u8] = b"BESSEMER_ZIP_TIME_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_ZIP_TIME_END";

#[test]
fn immutable_timestamp_snapshots_isolate_input_mutation_failures_and_eight_workers() {
    for (fixture, raw) in NATIVE.iter().enumerate() {
        let mut caller = raw.to_vec();
        let source = ZipEntryTimestamps::from_bytes(&caller, caller.len()).unwrap();
        caller.fill(0);
        drop(caller);
        check_workers(&source);
        assert_eq!(source.as_bytes(), NATIVE[fixture]);
    }
}
fn mixed_requests(source: &ZipEntryTimestamps) -> Vec<Vec<u8>> {
    let id = source.entry_named("PreviewPNG").unwrap().id();
    let foreign = ZipEntryTimestamps::from_bytes(source.as_bytes(), source.as_bytes().len())
        .unwrap()
        .entry_named("PreviewPNG")
        .unwrap()
        .id();
    (0..32)
        .map(|bits| {
            let edit = source.replace_timestamp(&id, packed_request(bits)).unwrap();
            assert!(
                source
                    .replace_timestamp(&foreign, packed_request(bits))
                    .is_err()
            );
            check_history(&edit);
            edit.result().as_bytes().to_vec()
        })
        .collect()
}
fn check_workers(source: &ZipEntryTimestamps) {
    let expected = mixed_requests(source);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| mixed_requests(source)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
}
#[test]
fn two_fresh_timestamp_processes_match_complete_source_verified_arithmetic_and_edited_wires() {
    let first = projection();
    assert_eq!(first, projection());
    let mut raw = Vec::new();
    flate2::read::ZlibDecoder::new(first.as_slice())
        .take(64_000_001)
        .read_to_end(&mut raw)
        .unwrap();
    assert!(!raw.is_empty() && raw.len() <= 64_000_000);
}
fn projection() -> Vec<u8> {
    crate::process::transcript("api::replay::timestamp_replay_worker", START, END)
}
#[test]
#[ignore = "The parent invokes this independent Source-verifying worker twice"]
fn timestamp_replay_worker() {
    let mut transcript =
        flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    for row in requests() {
        let actual = ZipTimestampRequest {
            calendar_words: row.calendar,
            packed_override: row.override_word,
        }
        .packed_word();
        assert_eq!(actual, row.expected);
        transcript.write_all(&actual.to_le_bytes()).unwrap();
    }
    for row in natives() {
        append_native(&mut transcript, &row);
    }
    let compressed = transcript.finish().unwrap();
    let mut output = std::io::stdout().lock();
    output.write_all(START).unwrap();
    output.write_all(&compressed).unwrap();
    output.write_all(END).unwrap();
}
fn append_native(transcript: &mut flate2::write::ZlibEncoder<Vec<u8>>, row: &NativeTimestamp) {
    let source = source_for(row);
    check_native_entry(&source, row);
    let id = source.entry_named(&row.name).unwrap().id();
    let bits = !row.packed;
    let edit = source.replace_timestamp(&id, packed_request(bits)).unwrap();
    let expected = expected_wire(row, bits);
    assert_eq!(edit.result().as_bytes(), expected);
    check_history(&edit);
    transcript
        .write_all(&(expected.len() as u64).to_le_bytes())
        .unwrap();
    transcript.write_all(edit.result().as_bytes()).unwrap();
}
