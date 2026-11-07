//! Complete BI_RGB1/4/8 storage and pixel replacement against independent literals.
#[path = "support/indexed_rgb_cases.rs"]
mod cases;
use bessemer::{
    Document, FailureClass,
    preview::{DibPixels, DibPreview},
};
use cases::{IndexedRgbCase, indexed_rgb_cases};
const LIMIT: usize = 1024;

#[test]
fn uncompressed_indexed_archives_are_supported() {
    assert_eq!(indexed_rgb_cases().len(), 867);
    for case in indexed_rgb_cases() {
        let source = DibPreview::from_bytes(case.original, case.indices.len()).unwrap();
        assert_indexed_raster(&source, case.indices);
        assert_eq!(source.to_bytes().unwrap(), case.original);
        assert_eq!(case.original.len(), case.edited.len());
        assert_eq!(case.indices.len(), case.edited_indices.len());
        let rebuilt = DibPreview::new(source.contents().clone(), LIMIT).unwrap();
        assert_eq!(rebuilt.to_bytes().unwrap(), case.original);
        let noop = source.with_indexed_pixels(case.indices, LIMIT).unwrap();
        assert_eq!(noop.to_bytes().unwrap(), case.original);
    }
}

fn assert_indexed_raster(preview: &DibPreview, indices: &[u8]) {
    let expected: Vec<Option<u8>> = indices.iter().copied().map(Some).collect();
    assert_eq!(
        preview.decode_pixels(LIMIT).unwrap(),
        DibPixels::Indexed(expected)
    );
}

fn assert_indexed_edit(case: &IndexedRgbCase<'_>) -> Vec<u8> {
    let source = DibPreview::from_bytes(case.original, LIMIT).unwrap();
    let edited = source
        .with_indexed_pixels(case.edited_indices, LIMIT)
        .unwrap();
    assert_eq!(edited.to_bytes().unwrap(), case.edited);
    let fresh = DibPreview::from_bytes(case.edited, LIMIT).unwrap();
    assert_indexed_raster(&fresh, case.edited_indices);
    let rebuilt = DibPreview::new(fresh.contents().clone(), LIMIT).unwrap();
    let reversed = rebuilt.with_indexed_pixels(case.indices, LIMIT).unwrap();
    assert_eq!(reversed.to_bytes().unwrap(), case.original);
    assert_eq!(source.to_bytes().unwrap(), case.original);
    edited.to_bytes().unwrap()
}

#[test]
fn every_indexed_edit_preserves_unused_bits_padding_and_fresh_reversal() {
    for case in indexed_rgb_cases() {
        assert_indexed_edit(&case);
    }
}

#[test]
fn complete_indexed_documents_reopen_preserve_other_streams_and_reverse_fresh() {
    for case in indexed_rgb_cases() {
        assert_indexed_document(&case);
    }
}

fn assert_indexed_document(case: &IndexedRgbCase<'_>) {
    let mut original = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    original
        .replace_stream("Preview", case.original.to_vec())
        .unwrap();
    let bytes = original.to_bytes().unwrap();
    let mut changed = Document::from_bytes(&bytes).unwrap();
    changed
        .replace_stream("Preview", assert_indexed_edit(case))
        .unwrap();
    let mut fresh = Document::from_bytes(&changed.to_bytes().unwrap()).unwrap();
    assert_indexed_other_streams(&original, &fresh);
    let preview = DibPreview::from_bytes(&fresh.read_stream("Preview").unwrap(), LIMIT).unwrap();
    let reversed = preview.with_indexed_pixels(case.indices, LIMIT).unwrap();
    fresh
        .replace_stream("Preview", reversed.to_bytes().unwrap())
        .unwrap();
    assert_eq!(fresh.to_bytes().unwrap(), bytes);
    assert_eq!(original.to_bytes().unwrap(), bytes);
}

fn assert_indexed_other_streams(original: &Document, fresh: &Document) {
    for stream in original.streams().filter(|stream| stream.name != "Preview") {
        assert_eq!(
            original.read_stream(stream.name).unwrap(),
            fresh.read_stream(stream.name).unwrap()
        );
    }
}

#[test]
fn every_indexed_prefix_and_trailing_byte_is_rejected() {
    for case in indexed_rgb_cases() {
        for end in 0..case.original.len() {
            assert!(DibPreview::from_bytes(&case.original[..end], LIMIT).is_err());
        }
        let mut trailing = case.original.to_vec();
        trailing.push(0xff);
        assert!(DibPreview::from_bytes(&trailing, LIMIT).is_err());
        trailing[..4].copy_from_slice(&((case.original.len() - 3) as u32).to_le_bytes());
        assert!(DibPreview::from_bytes(&trailing, LIMIT).is_err());
    }
}

#[test]
fn invalid_palette_indices_sizes_replacements_and_budgets_leave_sources_isolated() {
    for case in indexed_rgb_cases() {
        let source = DibPreview::from_bytes(case.original, LIMIT).unwrap();
        assert!(DibPreview::from_bytes(case.original, case.indices.len() - 1).is_err());
        assert!(source.decode_pixels(case.indices.len() - 1).is_err());
        assert!(source.with_indexed_pixels(&[], LIMIT).is_err());
        assert!(
            source
                .with_indexed_pixels(case.indices, case.indices.len() - 1)
                .is_err()
        );
        assert_indexed_palette_failures(&source, case.indices.len());
        assert_indexed_extent_failures(&source);
        assert_eq!(source.to_bytes().unwrap(), case.original);
    }
}

fn assert_indexed_palette_failures(source: &DibPreview, count: usize) {
    let mut contents = source.contents().clone();
    contents.header.colors_used = (1 << contents.header.bit_count) + 1;
    assert!(source.with_contents(contents, LIMIT).is_err());
    let mut contents = source.contents().clone();
    contents.header.colors_used = 1;
    contents.palette.truncate(1);
    contents.pixel_bytes[0] = 0xff;
    assert_eq!(
        source.with_contents(contents, LIMIT).unwrap_err().class,
        FailureClass::Invalid
    );
    let invalid = vec![u8::MAX; count];
    if source.contents().palette.len() < 256 {
        assert!(source.with_indexed_pixels(&invalid, LIMIT).is_err());
    }
}

fn assert_indexed_extent_failures(source: &DibPreview) {
    let mut contents = source.contents().clone();
    contents.header.image_size = contents.pixel_bytes.len() as u32 + 1;
    assert!(source.with_contents(contents, LIMIT).is_err());
    let mut contents = source.contents().clone();
    contents.pixel_bytes.pop();
    assert!(source.with_contents(contents, LIMIT).is_err());
    assert_eq!(
        source.decode_bgr24_pixels(LIMIT).unwrap_err().class,
        FailureClass::Unsupported
    );
    assert!(source.with_bgr24_pixels(&[[0; 3]], LIMIT).is_err());
    assert!(source.with_bgrx_pixels(&[[0; 4]], LIMIT).is_err());
}

#[test]
fn indexed_shape_and_compression_errors_remain_typed() {
    for case in indexed_rgb_cases() {
        let source = DibPreview::from_bytes(case.original, LIMIT).unwrap();
        assert_indexed_shape_failures(&source);
        let mut contents = source.contents().clone();
        contents.header.compression = 3;
        assert_eq!(
            source.with_contents(contents, LIMIT).unwrap_err().class,
            FailureClass::Unsupported
        );
    }
}

fn assert_indexed_shape_failures(source: &DibPreview) {
    for (width, height) in [(0, 1), (-1, 1), (1, 0), (1, i32::MIN), (i32::MAX, i32::MAX)] {
        let mut contents = source.contents().clone();
        contents.header.width = width;
        contents.header.height = height;
        assert_eq!(
            DibPreview::new(contents, usize::MAX).unwrap_err().class,
            FailureClass::Invalid
        );
    }
    let mut contents = source.contents().clone();
    contents.header.planes = 2;
    assert!(source.with_contents(contents, LIMIT).is_err());
}

fn all_indexed_edits() -> Vec<Vec<u8>> {
    indexed_rgb_cases()
        .iter()
        .map(assert_indexed_edit)
        .collect()
}

#[test]
fn concurrent_indexed_results_equal_all_single_threaded_bytes() {
    let workers: Vec<_> = (0..8)
        .map(|_| std::thread::spawn(all_indexed_edits))
        .collect();
    let expected = all_indexed_edits();
    for worker in workers {
        assert_eq!(worker.join().unwrap(), expected);
    }
}
