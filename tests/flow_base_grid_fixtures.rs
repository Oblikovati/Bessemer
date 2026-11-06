//! Pin exact boundary bits and zero-dimension behavior before introducing a typed reader.
use std::io::{Cursor, Read};

#[test]
fn all_56_reference_views_rebuild_exact_raw_and_compressed_payloads() {
    let mut packet = Vec::new();
    flate2::read::GzDecoder::new(&include_bytes!("fixtures/flow-base-grid/grid.bin.gz")[..])
        .take(100_001)
        .read_to_end(&mut packet)
        .unwrap();
    let mut cursor = Cursor::new(packet.as_slice());
    assert_eq!(read_array::<4>(&mut cursor), *b"FBG1");
    assert_eq!(read_word(&mut cursor), 56);
    for _ in 0..56 {
        check_case(&mut cursor);
    }
    assert_eq!(cursor.position(), packet.len() as u64);
}

fn check_case(cursor: &mut Cursor<&[u8]>) {
    let name = String::from_utf8(read_blob(cursor)).unwrap();
    let compressed = read_array::<1>(cursor)[0] != 0;
    let stored = read_blob(cursor);
    let decoded = read_blob(cursor);
    let dimensions: [i32; 3] = std::array::from_fn(|_| i32::from_le_bytes(read_array(cursor)));
    let bounds: [[i32; 2]; 3] =
        std::array::from_fn(|_| std::array::from_fn(|_| i32::from_le_bytes(read_array(cursor))));
    let count = u128::from_le_bytes(read_array(cursor));
    assert_eq!(bounds, dimensions.map(|n| [0, n - 1]), "{name}");
    assert_eq!(count, dimensions.into_iter().map(|n| n as u128).product());
    let mut rebuilt: Vec<u8> = dimensions.into_iter().flat_map(i32::to_le_bytes).collect();
    for dimension in dimensions {
        append_boundaries(cursor, dimension, &mut rebuilt);
    }
    assert_eq!(rebuilt, decoded, "{name}");
    check_transport(compressed, &stored, &decoded);
}

fn append_boundaries(cursor: &mut Cursor<&[u8]>, dimension: i32, rebuilt: &mut Vec<u8>) {
    let start = read_word(cursor) as usize;
    let end = read_word(cursor) as usize;
    let count = read_word(cursor);
    assert!(dimension >= 0);
    assert_eq!(i64::from(count), i64::from(dimension) + 1);
    assert_eq!(start, rebuilt.len());
    assert_eq!(end - start, count as usize * 8);
    for _ in 0..count {
        let bits = u64::from_le_bytes(read_array(cursor));
        assert_eq!(f64::from_bits(bits).to_bits(), bits);
        rebuilt.extend_from_slice(&bits.to_le_bytes());
    }
    assert_eq!(end, rebuilt.len());
}

fn check_transport(compressed: bool, stored: &[u8], decoded: &[u8]) {
    if !compressed {
        assert_eq!(stored, decoded);
        return;
    }
    let mut inflated = Vec::new();
    flate2::read::ZlibDecoder::new(stored)
        .take(10_001)
        .read_to_end(&mut inflated)
        .unwrap();
    assert_eq!(inflated, decoded);
}

fn read_array<const N: usize>(cursor: &mut Cursor<&[u8]>) -> [u8; N] {
    let mut bytes = [0; N];
    cursor.read_exact(&mut bytes).unwrap();
    bytes
}

fn read_word(cursor: &mut Cursor<&[u8]>) -> u32 {
    u32::from_le_bytes(read_array(cursor))
}

fn read_blob(cursor: &mut Cursor<&[u8]>) -> Vec<u8> {
    let mut bytes = vec![0; read_word(cursor) as usize];
    cursor.read_exact(&mut bytes).unwrap();
    bytes
}
