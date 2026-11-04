//! Regression vectors for omitted BI_RGB image lengths, independently decoded by GDI+.
#[path = "support/preview.rs"]
mod support;
use bessemer::{
    Document, FailureClass,
    preview::{DibPixels, DibPreview},
};
use support::{LIMIT, RLE, parse};
const CASES: &[(&[u8], &[u8])] = &[
    (
        include_bytes!("fixtures/preview-zero-size/w1-bottom-p0.bin"),
        include_bytes!("fixtures/preview-zero-size/w1-bottom-p0.bgrx"),
    ),
    (
        include_bytes!("fixtures/preview-zero-size/w1-bottom-p2.bin"),
        include_bytes!("fixtures/preview-zero-size/w1-bottom-p2.bgrx"),
    ),
    (
        include_bytes!("fixtures/preview-zero-size/w1-top-p0.bin"),
        include_bytes!("fixtures/preview-zero-size/w1-top-p0.bgrx"),
    ),
    (
        include_bytes!("fixtures/preview-zero-size/w1-top-p2.bin"),
        include_bytes!("fixtures/preview-zero-size/w1-top-p2.bgrx"),
    ),
    (
        include_bytes!("fixtures/preview-zero-size/w3-bottom-p0.bin"),
        include_bytes!("fixtures/preview-zero-size/w3-bottom-p0.bgrx"),
    ),
    (
        include_bytes!("fixtures/preview-zero-size/w3-bottom-p2.bin"),
        include_bytes!("fixtures/preview-zero-size/w3-bottom-p2.bgrx"),
    ),
    (
        include_bytes!("fixtures/preview-zero-size/w3-top-p0.bin"),
        include_bytes!("fixtures/preview-zero-size/w3-top-p0.bgrx"),
    ),
    (
        include_bytes!("fixtures/preview-zero-size/w3-top-p2.bin"),
        include_bytes!("fixtures/preview-zero-size/w3-top-p2.bgrx"),
    ),
];

#[test]
fn omitted_lengths_reconstruct_headers_palettes_and_literal_top_down_rasters() {
    for &(packed, raster) in CASES {
        let source = parse(packed);
        let expected = raster.as_chunks::<4>().0.to_vec();
        assert_eq!(source.contents().header.image_size, 0);
        assert_eq!(
            source.decode_pixels(LIMIT).unwrap(),
            DibPixels::Bgrx(expected)
        );
        assert_eq!(source.to_bytes().unwrap(), packed);
        assert_zero_reconstruction(&source, packed);
        assert_zero_reversal(&source, packed, raster);
    }
}

fn assert_zero_reconstruction(source: &DibPreview, packed: &[u8]) {
    let rebuilt = DibPreview::new(source.contents().clone(), LIMIT).unwrap();
    let equivalent = source
        .with_contents(source.contents().clone(), LIMIT)
        .unwrap();
    assert_eq!(rebuilt.to_bytes().unwrap(), packed);
    assert_eq!(equivalent.to_bytes().unwrap(), packed);
}

fn edited_zero_bytes(packed: &[u8], raster: &[u8]) -> Vec<u8> {
    let source = parse(packed);
    let replacement = vec![[0xa1, 0xb2, 0xc3, 0xd4]; raster.len() / 4];
    let edited = source.with_bgrx_pixels(&replacement, LIMIT).unwrap();
    let bytes = edited.to_bytes().unwrap();
    let fresh = DibPreview::new(parse(&bytes).contents().clone(), LIMIT).unwrap();
    assert_eq!(fresh.contents().header.image_size, 0);
    assert_eq!(
        fresh.decode_pixels(LIMIT).unwrap(),
        DibPixels::Bgrx(replacement)
    );
    assert_eq!(
        &bytes[..bytes.len() - raster.len()],
        &packed[..packed.len() - raster.len()]
    );
    assert_zero_reversal(&fresh, packed, raster);
    assert_eq!(source.to_bytes().unwrap(), packed);
    bytes
}

fn assert_zero_reversal(fresh: &DibPreview, packed: &[u8], raster: &[u8]) {
    let reversed = fresh
        .with_bgrx_pixels(raster.as_chunks::<4>().0, LIMIT)
        .unwrap();
    assert_eq!(reversed.to_bytes().unwrap(), packed);
}

#[test]
fn fresh_pixel_edits_and_reversals_preserve_zero_declarations_and_source_isolation() {
    for &(packed, raster) in CASES {
        edited_zero_bytes(packed, raster);
    }
}

#[test]
fn malformed_extents_nonzero_mismatches_and_raster_budgets_remain_errors() {
    for &(packed, raster) in CASES {
        assert_zero_extent_rejections(packed, raster.len() / 4);
        let source = parse(packed);
        let mut contents = source.contents().clone();
        contents.header.image_size = raster.len() as u32 + 1;
        assert!(source.with_contents(contents, LIMIT).is_err());
        assert!(source.with_bgrx_pixels(&[], LIMIT).is_err());
        assert_eq!(source.to_bytes().unwrap(), packed);
    }
    let mut rle = RLE.to_vec();
    rle[24..28].fill(0);
    assert!(DibPreview::from_bytes(&rle, LIMIT).is_err());
}

fn assert_zero_extent_rejections(packed: &[u8], pixels: usize) {
    for end in 0..packed.len() {
        assert!(DibPreview::from_bytes(&packed[..end], LIMIT).is_err());
    }
    let mut trailing = packed.to_vec();
    trailing.push(0x7f);
    trailing[..4].copy_from_slice(&((packed.len() - 3) as u32).to_le_bytes());
    assert!(DibPreview::from_bytes(&trailing, LIMIT).is_err());
    let error = DibPreview::from_bytes(packed, pixels - 1).unwrap_err();
    assert_eq!(error.class, FailureClass::Invalid);
    assert!(parse(packed).decode_pixels(pixels - 1).is_err());
    let mut contents = parse(packed).contents().clone();
    contents.pixel_bytes.pop();
    assert!(DibPreview::new(contents, LIMIT).is_err());
}

#[test]
fn zero_length_preview_replacement_reopens_every_stream_and_reverses_fresh() {
    for &(packed, raster) in CASES {
        verify_zero_document(packed, raster);
    }
}

fn verify_zero_document(packed: &[u8], raster: &[u8]) {
    let mut original = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    original.replace_stream("Preview", packed.to_vec()).unwrap();
    let original_bytes = original.to_bytes().unwrap();
    let mut changed = Document::from_bytes(&original_bytes).unwrap();
    changed
        .replace_stream("Preview", edited_zero_bytes(packed, raster))
        .unwrap();
    let mut fresh = Document::from_bytes(&changed.to_bytes().unwrap()).unwrap();
    assert_other_zero_document_streams(&original, &fresh);
    reverse_zero_document(&mut fresh, raster);
    assert_eq!(fresh.to_bytes().unwrap(), original_bytes);
    assert_eq!(original.to_bytes().unwrap(), original_bytes);
}

fn assert_other_zero_document_streams(original: &Document, fresh: &Document) {
    for stream in original.streams().filter(|stream| stream.name != "Preview") {
        assert_eq!(
            fresh.read_stream(stream.name).unwrap(),
            original.read_stream(stream.name).unwrap()
        );
    }
}

fn reverse_zero_document(fresh: &mut Document, raster: &[u8]) {
    let preview = parse(&fresh.read_stream("Preview").unwrap());
    let reversed = DibPreview::new(preview.contents().clone(), LIMIT)
        .unwrap()
        .with_bgrx_pixels(raster.as_chunks::<4>().0, LIMIT)
        .unwrap();
    fresh
        .replace_stream("Preview", reversed.to_bytes().unwrap())
        .unwrap();
}

fn all_zero_edits() -> Vec<Vec<u8>> {
    CASES
        .iter()
        .map(|&(packed, raster)| edited_zero_bytes(packed, raster))
        .collect()
}

#[test]
fn concurrent_zero_size_decodes_and_edits_are_repeatable() {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<DibPreview>();
    let workers: Vec<_> = (0..8).map(|_| std::thread::spawn(all_zero_edits)).collect();
    let expected = all_zero_edits();
    for worker in workers {
        assert_eq!(worker.join().unwrap(), expected);
    }
}
