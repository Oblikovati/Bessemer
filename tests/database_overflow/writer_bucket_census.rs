use super::*;
#[path = "writer_census_documents.rs"]
mod native_documents;
use std::collections::BTreeSet;

struct NativeBucketCase {
    ordinal: u32,
    index: u32,
    expected: BucketCase,
}

fn census_bytes() -> Vec<u8> {
    let compressed = include_bytes!("../fixtures/database-overflow/native-buckets.bin.gz");
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(compressed.as_slice())
        .take(30_575_145)
        .read_to_end(&mut raw)
        .unwrap();
    assert_eq!(raw.len(), 30_575_144);
    assert_eq!(&raw[..8], b"BCW1\xff\x03\0\0");
    raw
}

fn native_bucket_case(cursor: &mut &[u8]) -> NativeBucketCase {
    let ordinal = u32::from_le_bytes(word(cursor));
    let index = u32::from_le_bytes(word(cursor));
    let length = u32::from_le_bytes(word(cursor)) as usize;
    let expected = decode_bucket(take(cursor, length));
    assert!(!expected.rejected);
    NativeBucketCase {
        ordinal,
        index,
        expected,
    }
}

fn check_native_bucket(case: &NativeBucketCase) -> (usize, usize) {
    let expected = &case.expected;
    let source = parse_bucket(expected).unwrap_or_else(|error| {
        panic!(
            "native stream {} bucket {}: {error}",
            case.ordinal, case.index
        )
    });
    check_cells(expected, &source);
    let mut inline = 0;
    let mut overflow = 0;
    for cell in &expected.cells {
        check_bucket_edit(expected, &source, cell);
        if cell.first == 0 {
            inline += 1;
        } else {
            overflow += 1;
        }
    }
    assert_eq!(source.to_bytes(), expected.raw);
    (inline, overflow)
}

#[test]
fn every_native_mixed_bucket_cell_matches_complete_images_edits_and_fresh_reversals() {
    let raw = census_bytes();
    let mut cursor = &raw[8..];
    let mut streams = BTreeSet::new();
    let (mut inline, mut overflow) = (0, 0);
    for _ in 0..1023 {
        let case = native_bucket_case(&mut cursor);
        streams.insert(case.ordinal);
        let counts = check_native_bucket(&case);
        inline += counts.0;
        overflow += counts.1;
    }
    assert!(cursor.is_empty());
    assert_eq!((streams.len(), inline, overflow), (440, 8702, 5140));
}
