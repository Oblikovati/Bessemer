use super::*;
use bessemer::FailureClass;
use std::io::Write;
const START: &[u8] = b"BESSEMER_EMPTY_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_EMPTY_END";

#[test]
fn every_registered_empty_container_completes_roundtrips_and_reports_missing_entities() {
    for row in empty_cases() {
        let mut document = Document::from_bytes(row.raw).unwrap();
        assert_eq!(document.key(), row.key);
        assert_eq!(document.version(), 4);
        assert_eq!(document.streams().count(), 0);
        assert_eq!(
            document.read_stream("missing").unwrap_err().class,
            FailureClass::Missing
        );
        assert_eq!(
            document
                .replace_stream("missing", vec![7])
                .unwrap_err()
                .class,
            FailureClass::Missing
        );
        assert_eq!(document.to_bytes().unwrap(), row.raw);
    }
}
#[test]
fn registered_empty_frames_reject_all_prefixes_bad_signatures_and_malformed_end_fields() {
    for row in empty_cases() {
        for length in 0..row.raw.len() {
            assert!(Document::from_bytes(&row.raw[..length]).is_err());
        }
        check_bad_fields(&row);
    }
}
fn check_bad_fields(row: &EmptyReference<'_>) {
    for offset in [7, 8, 12, 14, 16, 18, 20, 24, 28] {
        let mut bad = row.raw.to_vec();
        bad[offset] ^= 1;
        assert!(Document::from_bytes(&bad).is_err(), "{}:{offset}", row.key);
    }
    let mut unknown = row.raw.to_vec();
    unknown[..4].copy_from_slice(&0u32.to_be_bytes());
    let error = Document::from_bytes(&unknown).unwrap_err();
    assert_eq!(error.class, FailureClass::Unsupported);
    assert!(error.cause.contains("0x00000000"));
}
fn with_filler(row: &EmptyReference<'_>, before: usize, after: usize, at_end: bool) -> Vec<u8> {
    let byte = (0..=u8::MAX).find(|byte| !row.end.contains(byte)).unwrap();
    let mut raw = row.raw[..8].to_vec();
    raw.resize(8 + before, byte);
    let mut end = row.raw[8..].to_vec();
    let start = if at_end { before } else { 0 };
    end[12..16].copy_from_slice(&((before - start) as u32).to_le_bytes());
    end[16..20].copy_from_slice(&(start as u32).to_le_bytes());
    raw.extend_from_slice(&end);
    raw.resize(raw.len() + after, byte);
    raw
}
fn filler_cases(row: &EmptyReference<'_>) -> Vec<Vec<u8>> {
    let mut cases = Vec::new();
    for before in [0, 1, 255] {
        append_trailing_cases(&mut cases, row, before);
    }
    cases
}
fn append_trailing_cases(cases: &mut Vec<Vec<u8>>, row: &EmptyReference<'_>, before: usize) {
    for after in [0, 1, 255] {
        for at_end in [false, true] {
            cases.push(with_filler(row, before, after, at_end));
        }
    }
}
#[test]
fn all_18000_registered_empty_filler_layouts_retain_every_uninterpreted_byte() {
    let mut count = 0;
    for row in empty_cases() {
        for raw in filler_cases(&row) {
            assert_eq!(Document::from_bytes(&raw).unwrap().to_bytes().unwrap(), raw);
            count += 1;
        }
    }
    assert_eq!(count, 18000);
}
#[test]
fn unregistered_nonempty_recovery_remains_compatible_and_empty_inputs_are_isolated() {
    for raw in FILES {
        let mut unknown = raw.to_vec();
        unknown[..4].copy_from_slice(&0u32.to_be_bytes());
        let document = Document::from_bytes(&unknown).unwrap();
        assert!(document.streams().next().is_some());
        assert_eq!(document.to_bytes().unwrap(), unknown);
    }
    for row in empty_cases() {
        let mut caller = row.raw.to_vec();
        let document = Document::from_bytes(&caller).unwrap();
        caller.fill(0);
        drop(caller);
        assert_eq!(document.to_bytes().unwrap(), row.raw);
    }
}
#[test]
fn registered_empty_reads_are_thread_independent_and_match_two_fresh_processes() {
    let expected = projection();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8).map(|_| scope.spawn(projection)).collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
    let worker = "api::empty_replay_worker";
    assert_eq!(crate::process::transcript(worker, START, END), expected);
    assert_eq!(crate::process::transcript(worker, START, END), expected);
}
fn projection() -> Vec<u8> {
    let mut result = Vec::new();
    for row in empty_cases() {
        assert_eq!(
            Document::from_bytes(row.raw).unwrap().to_bytes().unwrap(),
            row.raw
        );
        result.extend_from_slice(row.raw);
        append_filler_projection(&mut result, &row);
    }
    result
}
fn append_filler_projection(result: &mut Vec<u8>, row: &EmptyReference<'_>) {
    for raw in filler_cases(row) {
        let document = Document::from_bytes(&raw).unwrap();
        let encoded = document.to_bytes().unwrap();
        assert_eq!(encoded, raw);
        result.extend_from_slice(&(encoded.len() as u32).to_le_bytes());
        result.extend_from_slice(&encoded);
    }
}
#[test]
#[ignore = "The parent runs this complete independent wire-checking worker twice"]
fn empty_replay_worker() {
    let transcript = projection();
    let mut output = std::io::stdout().lock();
    output.write_all(START).unwrap();
    output.write_all(&transcript).unwrap();
    output.write_all(END).unwrap();
}
