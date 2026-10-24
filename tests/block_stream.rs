//! Clause 9.3 carriers checked against native and independently encoded vectors.

use bessemer::FailureClass;
use bessemer::block_stream::{BlockStream, ChunkFraming, CompressedChunk};

const GUID: [u8; 16] = [
    0x23, 0x1d, 0xd5, 0x71, 0xda, 0x81, 0x48, 0xa2, 0xa8, 0x58, 0x98, 0xb2, 0x1b, 0x89, 0xef, 0x99,
];
const HELLO: &[u8] = &[
    0x78, 1, 0xcb, 0x48, 0xcd, 0xc9, 0xc9, 7, 0, 6, 0x2c, 2, 0x15,
];
const WORLD: &[u8] = &[
    0x78, 1, 0x2b, 0xcf, 0x2f, 0xca, 0x49, 1, 0, 6, 0xa6, 2, 0x29,
];
const NATIVE: &[u8] = include_bytes!("fixtures/blocks/blank-part-partition.bin");

fn synthetic(framing: ChunkFraming, blocks: &[&[u8]]) -> Vec<u8> {
    let mut body = GUID.to_vec();
    for compressed in blocks {
        body.extend_from_slice(&5_u32.to_le_bytes());
        body.extend_from_slice(&(compressed.len() as u32).to_le_bytes());
        body.extend_from_slice(compressed);
    }
    body.extend_from_slice(&[0; 8]);
    if framing == ChunkFraming::GuidOnly {
        return body;
    }
    let mut encoded = (body.len() as u32).to_le_bytes().to_vec();
    encoded.extend_from_slice(&body);
    encoded
}

#[test]
fn native_partition_matches_python_bytes_and_reencodes_exactly() {
    let carrier = BlockStream::from_bytes(NATIVE, ChunkFraming::LengthPrefixed, 718).unwrap();
    assert_eq!(carrier.chunks().len(), 2);
    assert_eq!(
        carrier.chunks()[0].payload(),
        include_bytes!("fixtures/blocks/blank-part-chunk-0.bin")
    );
    assert_eq!(
        carrier.chunks()[1].payload(),
        include_bytes!("fixtures/blocks/blank-part-chunk-1.bin")
    );
    assert_eq!(carrier.to_bytes().unwrap(), NATIVE);
    assert_eq!(
        BlockStream::from_bytes(NATIVE, ChunkFraming::LengthPrefixed, 717)
            .unwrap_err()
            .class,
        FailureClass::Unsupported
    );
}

#[test]
fn multiple_blocks_and_chunks_work_in_both_framings() {
    for framing in [ChunkFraming::LengthPrefixed, ChunkFraming::GuidOnly] {
        let first = synthetic(framing, &[HELLO, WORLD]);
        let mut bytes = first.clone();
        bytes.extend_from_slice(&synthetic(framing, &[WORLD]));
        let carrier = BlockStream::from_bytes(&bytes, framing, 15).unwrap();
        assert_eq!(carrier.chunks()[0].payload(), b"helloworld");
        assert_eq!(carrier.chunks()[1].payload(), b"world");
        assert_eq!(carrier.to_bytes().unwrap(), bytes);
    }
}

#[test]
fn edits_preserve_other_chunks_and_are_isolated() {
    let source = BlockStream::from_bytes(NATIVE, ChunkFraming::LengthPrefixed, 2_100_000).unwrap();
    let mut edited = source.clone();
    let replacement = vec![42; 1_048_577];
    edited.replace_chunk(0, replacement.clone()).unwrap();
    let output = edited.to_bytes().unwrap();
    let second_start = 4 + u32::from_le_bytes(NATIVE[..4].try_into().unwrap()) as usize;
    assert!(output.ends_with(&NATIVE[second_start..]));
    let reopened =
        BlockStream::from_bytes(&output, ChunkFraming::LengthPrefixed, 2_100_000).unwrap();
    assert_eq!(reopened.chunks()[0].payload(), replacement);
    assert_eq!(reopened.chunks()[1].payload(), source.chunks()[1].payload());
    assert_eq!(source.to_bytes().unwrap(), NATIVE);
    assert_eq!(edited.to_bytes().unwrap(), output);
}

#[test]
fn equivalent_replacements_and_reverts_keep_original_encoding() {
    let mut carrier = BlockStream::from_bytes(NATIVE, ChunkFraming::LengthPrefixed, 1024).unwrap();
    let original = carrier.chunks()[0].payload().to_vec();
    carrier.replace_chunk(0, original.clone()).unwrap();
    assert_eq!(carrier.to_bytes().unwrap(), NATIVE);
    carrier.replace_chunk(0, b"changed".to_vec()).unwrap();
    carrier.replace_chunk(0, original).unwrap();
    assert_eq!(carrier.to_bytes().unwrap(), NATIVE);
}

#[test]
fn failed_edits_preserve_the_source() {
    let mut carrier = BlockStream::from_bytes(NATIVE, ChunkFraming::LengthPrefixed, 718).unwrap();
    for (index, payload) in [(2, vec![1]), (0, vec![]), (0, vec![1; 308])] {
        assert!(carrier.replace_chunk(index, payload).is_err());
        assert_eq!(carrier.to_bytes().unwrap(), NATIVE);
    }
}

#[test]
fn malformed_headers_sizes_checksums_and_terminators_are_rejected() {
    let valid = synthetic(ChunkFraming::LengthPrefixed, &[HELLO]);
    for (offset, value) in [
        (0, 255),
        (4, 0),
        (20, 0),
        (22, 0x20),
        (24, 0),
        (40, 0),
        (41, 1),
    ] {
        let mut bytes = valid.clone();
        bytes[offset] = value;
        assert!(
            BlockStream::from_bytes(&bytes, ChunkFraming::LengthPrefixed, 2_000_000).is_err(),
            "offset {offset}"
        );
    }
    for end in 0..valid.len() {
        assert!(BlockStream::from_bytes(&valid[..end], ChunkFraming::LengthPrefixed, 100).is_err());
    }
}

#[test]
fn framing_is_explicit_and_trailing_bytes_are_not_ignored() {
    let mut bytes = synthetic(ChunkFraming::GuidOnly, &[HELLO]);
    assert!(BlockStream::from_bytes(&bytes, ChunkFraming::LengthPrefixed, 100).is_err());
    bytes.push(0);
    assert!(BlockStream::from_bytes(&bytes, ChunkFraming::GuidOnly, 100).is_err());
    let empty = synthetic(ChunkFraming::GuidOnly, &[]);
    assert!(BlockStream::from_bytes(&empty, ChunkFraming::GuidOnly, 100).is_err());
}

#[test]
fn checksum_failure_is_distinct_from_budget_and_framing_errors() {
    let mut bytes = synthetic(ChunkFraming::GuidOnly, &[HELLO]);
    let checksum_byte = bytes.len() - 9;
    bytes[checksum_byte] ^= 1;
    let error = BlockStream::from_bytes(&bytes, ChunkFraming::GuidOnly, 100).unwrap_err();
    assert_eq!(error.class, FailureClass::Integrity);
    assert!(error.entity.contains("chunk 0"));
    assert!(error.cause.contains("expected"));
}

#[test]
fn block_types_are_send_and_sync() {
    fn assert_traits<T: Send + Sync>() {}
    assert_traits::<BlockStream>();
    assert_traits::<CompressedChunk>();
    assert_traits::<ChunkFraming>();
}

#[test]
fn zlib_trailing_bytes_and_missing_checksums_are_rejected() {
    for compressed in [&HELLO[..HELLO.len() - 1], &[HELLO, &[0]].concat()[..]] {
        let bytes = synthetic(ChunkFraming::GuidOnly, &[compressed]);
        assert_eq!(
            BlockStream::from_bytes(&bytes, ChunkFraming::GuidOnly, 5)
                .unwrap_err()
                .class,
            FailureClass::Integrity
        );
    }
}

#[test]
fn concurrent_carrier_operations_match_serial_results() {
    let source = BlockStream::from_bytes(NATIVE, ChunkFraming::LengthPrefixed, 2048).unwrap();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    let mut local = source.clone();
                    local
                        .replace_chunk(0, b"isolated replacement".to_vec())
                        .unwrap();
                    local.to_bytes().unwrap()
                })
            })
            .collect();
        let outputs: Vec<_> = workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .collect();
        assert!(outputs.windows(2).all(|pair| pair[0] == pair[1]));
    });
    assert_eq!(source.to_bytes().unwrap(), NATIVE);
}
