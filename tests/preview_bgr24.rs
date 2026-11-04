//! Independently assembled BI_RGB24 rasters, stored padding and complete document operations.
#[path = "support/bgr24_cases.rs"]
mod cases;
use bessemer::{Document, FailureClass, preview::DibPreview};
use cases::BGR24_CASES;
const LIMIT: usize = 64;

#[test]
fn all_literal_bgr24_archives_decode() {
    assert_eq!(BGR24_CASES.len(), 64);
    for &(raw, _, raster) in BGR24_CASES {
        let source = DibPreview::from_bytes(raw, raster.len() / 3).unwrap();
        assert_eq!(
            source.decode_bgr24_pixels(LIMIT).unwrap(),
            raster.as_chunks::<3>().0
        );
        assert_eq!(source.to_bytes().unwrap(), raw);
        assert_eq!(source.contents().header.x_pixels_per_metre, -1234);
        assert_eq!(source.contents().header.colors_important, u32::MAX);
        assert_bgr24_reconstruction(&source, raw, raster);
    }
}

fn assert_bgr24_reconstruction(source: &DibPreview, raw: &[u8], raster: &[u8]) {
    let rebuilt = DibPreview::new(source.contents().clone(), LIMIT).unwrap();
    assert_eq!(rebuilt.to_bytes().unwrap(), raw);
    let equivalent = source
        .with_contents(source.contents().clone(), LIMIT)
        .unwrap();
    assert_eq!(equivalent.to_bytes().unwrap(), raw);
    let same_pixels = source
        .with_bgr24_pixels(raster.as_chunks::<3>().0, LIMIT)
        .unwrap();
    assert_eq!(same_pixels.to_bytes().unwrap(), raw);
    assert_eq!(
        source.decode_pixels(LIMIT).unwrap_err().class,
        FailureClass::Unsupported
    );
}

fn edited_bgr24_bytes(raw: &[u8], expected: &[u8], raster: &[u8]) -> Vec<u8> {
    let source = DibPreview::from_bytes(raw, LIMIT).unwrap();
    let replacement: Vec<[u8; 3]> = raster
        .as_chunks::<3>()
        .0
        .iter()
        .map(|pixel| pixel.map(|v| v ^ 0x80))
        .collect();
    let edited = source.with_bgr24_pixels(&replacement, LIMIT).unwrap();
    assert_eq!(edited.to_bytes().unwrap(), expected);
    let fresh = DibPreview::from_bytes(expected, LIMIT).unwrap();
    assert_eq!(fresh.decode_bgr24_pixels(LIMIT).unwrap(), replacement);
    let rebuilt = DibPreview::new(fresh.contents().clone(), LIMIT).unwrap();
    let reversed = rebuilt
        .with_bgr24_pixels(raster.as_chunks::<3>().0, LIMIT)
        .unwrap();
    assert_eq!(reversed.to_bytes().unwrap(), raw);
    assert_eq!(source.to_bytes().unwrap(), raw);
    edited.to_bytes().unwrap()
}

#[test]
fn pixel_edits_match_literal_bytes_and_reverse_after_fresh_reconstruction() {
    for &(raw, edited, raster) in BGR24_CASES {
        edited_bgr24_bytes(raw, edited, raster);
    }
}

#[test]
fn complete_bgr24_preview_replacement_reopens_and_reverses_the_owned_part() {
    for &(raw, edited, raster) in BGR24_CASES {
        assert_bgr24_document(raw, edited, raster);
    }
}

fn assert_bgr24_document(raw: &[u8], edited: &[u8], raster: &[u8]) {
    let mut original = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    original.replace_stream("Preview", raw.to_vec()).unwrap();
    let bytes = original.to_bytes().unwrap();
    let mut changed = Document::from_bytes(&bytes).unwrap();
    changed
        .replace_stream("Preview", edited_bgr24_bytes(raw, edited, raster))
        .unwrap();
    let mut fresh = Document::from_bytes(&changed.to_bytes().unwrap()).unwrap();
    assert_other_bgr24_streams(&original, &fresh);
    let preview = DibPreview::from_bytes(&fresh.read_stream("Preview").unwrap(), LIMIT).unwrap();
    let reversed = preview
        .with_bgr24_pixels(raster.as_chunks::<3>().0, LIMIT)
        .unwrap();
    fresh
        .replace_stream("Preview", reversed.to_bytes().unwrap())
        .unwrap();
    assert_eq!(fresh.to_bytes().unwrap(), bytes);
    assert_eq!(original.to_bytes().unwrap(), bytes);
}

fn assert_other_bgr24_streams(original: &Document, reopened: &Document) {
    for stream in original.streams().filter(|stream| stream.name != "Preview") {
        assert_eq!(
            original.read_stream(stream.name).unwrap(),
            reopened.read_stream(stream.name).unwrap()
        );
    }
}

#[test]
fn all_bgr24_prefixes_extra_bytes_and_inconsistent_explicit_extents_fail() {
    for &(raw, _, raster) in BGR24_CASES {
        for end in 0..raw.len() {
            assert!(DibPreview::from_bytes(&raw[..end], LIMIT).is_err());
        }
        assert_bgr24_extent_failures(raw, raster.len() / 3);
    }
}

fn assert_bgr24_extent_failures(raw: &[u8], count: usize) {
    let mut trailing = raw.to_vec();
    trailing.push(0xff);
    assert!(DibPreview::from_bytes(&trailing, LIMIT).is_err());
    trailing[..4].copy_from_slice(&((raw.len() - 3) as u32).to_le_bytes());
    assert!(DibPreview::from_bytes(&trailing, LIMIT).is_err());
    assert!(DibPreview::from_bytes(raw, count - 1).is_err());
    let source = DibPreview::from_bytes(raw, LIMIT).unwrap();
    let mut contents = source.contents().clone();
    contents.header.image_size = contents.pixel_bytes.len() as u32 + 1;
    assert!(source.with_contents(contents, LIMIT).is_err());
    let mut contents = source.contents().clone();
    contents.pixel_bytes.pop();
    assert!(source.with_contents(contents, LIMIT).is_err());
    assert!(source.decode_bgr24_pixels(count - 1).is_err());
    assert_eq!(source.to_bytes().unwrap(), raw);
}

#[test]
fn wrong_replacement_formats_counts_and_budgets_leave_sources_unchanged() {
    for &(raw, _, raster) in BGR24_CASES {
        let source = DibPreview::from_bytes(raw, LIMIT).unwrap();
        assert!(source.with_bgr24_pixels(&[], LIMIT).is_err());
        assert!(
            source
                .with_bgr24_pixels(raster.as_chunks::<3>().0, raster.len() / 3 - 1)
                .is_err()
        );
        assert!(source.with_bgrx_pixels(&[[0; 4]; 2], LIMIT).is_err());
        assert!(source.with_indexed_pixels(&[0; 2], LIMIT).is_err());
        assert_eq!(source.to_bytes().unwrap(), raw);
    }
    assert_old_format_bgr24_rejections();
}

fn assert_old_format_bgr24_rejections() {
    for bytes in [
        include_bytes!("fixtures/preview/bgrx-top.bin").as_slice(),
        include_bytes!("fixtures/preview/rle8.bin").as_slice(),
    ] {
        let source = DibPreview::from_bytes(bytes, LIMIT).unwrap();
        assert_eq!(
            source.decode_bgr24_pixels(LIMIT).unwrap_err().class,
            FailureClass::Unsupported
        );
        assert_eq!(
            source
                .with_bgr24_pixels(&[[0; 3]; 4], LIMIT)
                .unwrap_err()
                .class,
            FailureClass::Unsupported
        );
        assert_eq!(source.to_bytes().unwrap(), bytes);
    }
}

fn all_bgr24_edits() -> Vec<Vec<u8>> {
    BGR24_CASES
        .iter()
        .map(|&(raw, edited, raster)| edited_bgr24_bytes(raw, edited, raster))
        .collect()
}

#[test]
fn invalid_dimensions_palettes_and_unsupported_encodings_are_explicit() {
    for &(raw, _, _) in BGR24_CASES {
        let source = DibPreview::from_bytes(raw, LIMIT).unwrap();
        assert_bgr24_header_rejections(&source);
        assert_bgr24_palette_rejections(&source);
        assert_bgr24_format_rejections(&source);
        assert_bgr24_fixed_header_rejections(&source);
        assert_eq!(source.to_bytes().unwrap(), raw);
    }
}

fn assert_bgr24_header_rejections(source: &DibPreview) {
    for (width, height) in [(0, 2), (-1, 2), (1, 0), (1, i32::MIN), (i32::MAX, i32::MAX)] {
        let mut contents = source.contents().clone();
        contents.header.width = width;
        contents.header.height = height;
        assert_eq!(
            DibPreview::new(contents, usize::MAX).unwrap_err().class,
            FailureClass::Invalid
        );
    }
}

fn assert_bgr24_palette_rejections(source: &DibPreview) {
    let mut contents = source.contents().clone();
    contents.header.colors_used = u32::MAX;
    assert!(source.with_contents(contents, LIMIT).is_err());
    let mut contents = source.contents().clone();
    contents.palette.push(bessemer::preview::PaletteEntry {
        blue: 1,
        green: 2,
        red: 3,
        reserved: 4,
    });
    assert!(source.with_contents(contents, LIMIT).is_err());
}

fn assert_bgr24_format_rejections(source: &DibPreview) {
    for (bits, compression) in [(24, 1), (24, 2), (24, 3), (16, 0)] {
        let mut contents = source.contents().clone();
        contents.header.bit_count = bits;
        contents.header.compression = compression;
        assert_eq!(
            source.with_contents(contents, LIMIT).unwrap_err().class,
            FailureClass::Unsupported
        );
    }
}

fn assert_bgr24_fixed_header_rejections(source: &DibPreview) {
    let mut contents = source.contents().clone();
    contents.header.planes = 2;
    assert_eq!(
        source.with_contents(contents, LIMIT).unwrap_err().class,
        FailureClass::Invalid
    );
    let mut contents = source.contents().clone();
    contents.header.header_size = 108;
    assert_eq!(
        source.with_contents(contents, LIMIT).unwrap_err().class,
        FailureClass::Unsupported
    );
}

#[test]
fn concurrent_bgr24_outputs_match_single_threaded_bytes() {
    let workers: Vec<_> = (0..8)
        .map(|_| std::thread::spawn(all_bgr24_edits))
        .collect();
    let expected = all_bgr24_edits();
    for worker in workers {
        assert_eq!(worker.join().unwrap(), expected);
    }
}
