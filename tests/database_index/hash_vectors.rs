use super::*;

fn key_file(key: &[u8], hash: u32) -> Vec<u8> {
    let extent = 8192;
    let mut raw = initialized_empty(extent);
    raw.resize(3 * extent, 0);
    raw[extent + 40..extent + 44].copy_from_slice(&1u32.to_be_bytes());
    write_word(&mut raw, extent + 52, 2);
    raw[2 * extent..2 * extent + 2].copy_from_slice(&12u16.to_be_bytes());
    let cell = 2 * extent + 12;
    raw[cell..cell + 4].copy_from_slice(&hash.to_be_bytes());
    raw[cell + 4..cell + 8].copy_from_slice(&(key.len() as u32).to_be_bytes());
    raw[cell + 26..cell + 26 + key.len()].copy_from_slice(key);
    raw
}

#[test]
fn complete_files_check_all_unsigned_hash_and_2048_byte_prefix_boundaries() {
    let compressed = include_bytes!("../fixtures/database-index/hash-vectors.bin.gz");
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(compressed.as_slice())
        .take(100_000)
        .read_to_end(&mut raw)
        .unwrap();
    let mut cursor = raw.as_slice();
    assert_eq!(take(&mut cursor, 4), b"BHV1");
    assert_eq!(u32::from_le_bytes(word(&mut cursor)), 34);
    for _ in 0..34 {
        let length = u32::from_le_bytes(word(&mut cursor)) as usize;
        let hash = u32::from_le_bytes(word(&mut cursor));
        let key = take(&mut cursor, length);
        let file = key_file(key, hash);
        let graph = HashFileGraph::from_bytes(&file, context(file.len(), 8192)).unwrap();
        assert_eq!(graph.bucket(2).unwrap().cells()[0].key(), key);
        assert_eq!(graph.to_bytes(), file);
    }
    assert!(cursor.is_empty());
}
