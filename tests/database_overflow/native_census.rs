use super::packet::{decode_pages, take, word};
use super::*;
use std::io::Read;
#[path = "native_documents.rs"]
mod native_documents;

struct NativeChainCase {
    ordinal: u32,
    first: NonZeroU64,
    payload: OwnedOverflowCase,
}

fn unpack_census() -> Vec<u8> {
    let compressed = include_bytes!("../fixtures/database-overflow/native-census.bin.gz");
    let reader = flate2::read::GzDecoder::new(compressed.as_slice());
    let mut raw = Vec::new();
    reader.take(23_174_344).read_to_end(&mut raw).unwrap();
    assert_eq!(raw.len(), 23_174_343);
    assert_eq!(&raw[..8], b"BOV1\x14\x14\0\0");
    raw
}

fn native_record(mut cursor: &[u8]) -> NativeChainCase {
    let ordinal = u32::from_le_bytes(word(&mut cursor));
    let extent = u32::from_le_bytes(word(&mut cursor)) as usize;
    let key_length = u32::from_le_bytes(word(&mut cursor)) as usize;
    let value_length = u64::from_le_bytes(word(&mut cursor)) as usize;
    let first = identity(u64::from_le_bytes(word(&mut cursor)));
    let count = u32::from_le_bytes(word(&mut cursor)) as usize;
    let pages = decode_pages(&mut cursor, count, extent);
    NativeChainCase {
        ordinal,
        first,
        payload: remaining_payload(cursor, extent, pages, key_length, value_length),
    }
}

fn remaining_payload(
    mut cursor: &[u8],
    extent: usize,
    pages: Vec<OverflowPageImage>,
    key_length: usize,
    value_length: usize,
) -> OwnedOverflowCase {
    let key = take(&mut cursor, key_length).to_vec();
    let value = take(&mut cursor, value_length).to_vec();
    assert!(cursor.is_empty());
    OwnedOverflowCase {
        extent,
        pages,
        key,
        value,
        wrapped: false,
    }
}

fn read_native(case: &NativeChainCase, pages: &[OverflowPageImage]) -> OverflowPayload {
    let expected = &case.payload;
    OverflowPayload::from_pages(
        pages,
        case.first,
        expected.key.len() as u32,
        expected.value.len() as u64,
        context(expected),
    )
    .unwrap_or_else(|error| {
        panic!(
            "native stream {} first {}: {error}",
            case.ordinal, case.first
        )
    })
}

fn assert_native_edits(case: &NativeChainCase) {
    let expected = &case.payload;
    let source = read_native(case, &expected.pages);
    assert_eq!(source.key(), expected.key, "native stream {}", case.ordinal);
    assert_eq!(
        source.value(),
        expected.value,
        "native stream {}",
        case.ordinal
    );
    let replacement: Vec<_> = expected.value.iter().map(|byte| byte ^ 0x5a).collect();
    let edit = source.edit_value(&replacement).unwrap();
    assert_eq!(
        edit.result().pages(),
        expected_pages(expected, &replacement)
    );
    let fresh = read_native(case, edit.result().pages());
    assert_eq!(fresh.edit_value(&expected.value).unwrap().result(), &source);
    assert_eq!(source.pages(), expected_pages(expected, &expected.value));
    assert_coverage(expected, &source);
    check_history(&edit);
}

#[test]
fn every_frozen_native_overflow_cell_matches_expected_spans_edits_and_reverses() {
    let raw = unpack_census();
    let mut cursor = &raw[8..];
    let mut multipage = 0;
    for _ in 0..5140 {
        let length = u32::from_le_bytes(word(&mut cursor)) as usize;
        let case = native_record(take(&mut cursor, length));
        multipage += usize::from(case.payload.pages.len() > 1);
        assert_native_edits(&case);
    }
    assert!(cursor.is_empty());
    assert_eq!(multipage, 1);
}
