// Integration crates exercise distinct portions of the same independent wire corpus.
#![allow(dead_code)]
use bessemer::preview::*;
pub const LIMITS: PngLimits = PngLimits {
    maximum_pixels: 1_000_000,
    maximum_inflated_bytes: 16 * 1024 * 1024,
    maximum_chunks: 10_000,
};
pub const BASE: &[u8] = include_bytes!("../fixtures/png/c6-d8-i0-9x9.png");

pub fn fixture(name: &str, extension: &str) -> Vec<u8> {
    std::fs::read(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/png")
            .join(format!("{name}.{extension}")),
    )
    .unwrap()
}
pub fn parse(bytes: &[u8]) -> PngPreview {
    PngPreview::from_bytes(bytes, LIMITS).unwrap()
}
pub fn chunk(kind: [u8; 4], payload: &[u8]) -> PngChunk {
    PngChunk {
        kind: ChunkType::new(kind).unwrap(),
        payload: payload.to_vec(),
    }
}

pub fn before_image(contents: &mut PngContents, chunk: PngChunk) {
    let position = contents
        .chunks
        .iter()
        .position(|chunk| chunk.kind.bytes() == *b"IDAT")
        .unwrap();
    contents.chunks.insert(position, chunk);
}
pub fn compressed(bytes: &[u8]) -> Vec<u8> {
    use std::io::Write;
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::fast());
    encoder.write_all(bytes).unwrap();
    encoder.finish().unwrap()
}
pub fn with_metadata(kind: [u8; 4], payload: &[u8]) -> bessemer::Result<PngPreview> {
    let mut contents = parse(BASE).contents().clone();
    before_image(&mut contents, chunk(kind, payload));
    PngPreview::new(contents, LIMITS)
}
