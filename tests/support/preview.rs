// Integration crates exercise distinct subsets of the same independently assembled vectors.
#![allow(dead_code)]
use bessemer::preview::*;
pub const RLE: &[u8] = include_bytes!("../fixtures/preview/rle8.bin");
pub const TOP: &[u8] = include_bytes!("../fixtures/preview/bgrx-top.bin");
pub const BOTTOM: &[u8] = include_bytes!("../fixtures/preview/bgrx-bottom.bin");
pub const DEFAULT: &[u8] = include_bytes!("../fixtures/preview/default-palette.bin");
pub const LIMIT: usize = 1_000_000;

pub fn parse(bytes: &[u8]) -> DibPreview {
    DibPreview::from_bytes(bytes, LIMIT).unwrap()
}

pub fn commands(width: i32, height: i32, bytes: &[u8]) -> bessemer::Result<DibPreview> {
    let mut contents = parse(RLE).contents().clone();
    contents.header.width = width;
    contents.header.height = height;
    contents.header.image_size = bytes.len() as u32;
    contents.pixel_bytes = bytes.to_vec();
    DibPreview::new(contents, LIMIT)
}
