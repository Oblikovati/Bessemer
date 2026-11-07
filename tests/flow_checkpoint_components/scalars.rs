use super::*;
pub(super) fn unsigned(raw: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(raw[offset..offset + 4].try_into().unwrap())
}
pub(super) fn wide(raw: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(raw[offset..offset + 8].try_into().unwrap())
}
pub(super) fn words(raw: &[u8], start: usize, values: &[u32]) -> usize {
    for (ordinal, value) in values.iter().enumerate() {
        assert_eq!(unsigned(raw, start + ordinal * 4), *value);
    }
    start + values.len() * 4
}
pub(super) fn wides(raw: &[u8], start: usize, values: &[u64]) -> usize {
    for (ordinal, value) in values.iter().enumerate() {
        assert_eq!(wide(raw, start + ordinal * 8), *value);
    }
    start + values.len() * 8
}
pub(super) fn container(raw: &[u8], expected: &ContainerProjection) {
    assert_eq!(unsigned(raw, expected.start) as usize, expected.count);
    assert_eq!(expected.first.len(), expected.count);
    assert_eq!(expected.indices.len(), expected.count);
    assert_eq!(expected.second.len(), expected.count * 2);
    assert_eq!(expected.end - expected.start, 4 + expected.count * 28);
    let position = wides(raw, expected.start + 4, &expected.first);
    let position = words(raw, position, &expected.indices);
    assert_eq!(wides(raw, position, &expected.second), expected.end);
}
pub(super) fn chunks(raw: &[u8], expected: &ChunkProjection) {
    assert_eq!(expected.version, 0);
    assert_eq!(unsigned(raw, 0), 0);
    assert_eq!(unsigned(raw, 4) as usize, expected.count);
    assert_eq!(wide(raw, 8), expected.declared);
    assert_eq!(expected.count, expected.chunks.len());
    assert!(expected.count > 0);
    assert!(!expected.codec_decoded && !expected.decoded_size_verified);
    let mut position = 16;
    for (ordinal, chunk) in expected.chunks.iter().enumerate() {
        assert_eq!(chunk.ordinal, ordinal);
        assert_eq!(unsigned(raw, position) as usize, chunk.stored);
        assert_eq!(chunk.stored, chunk.encoded.len());
        position += 4;
        assert_eq!(raw[position..position + chunk.stored], chunk.encoded);
        position += chunk.stored;
    }
    assert_eq!(position, raw.len());
}
