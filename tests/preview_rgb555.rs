//! Complete RGB555 words and document operations against independent literal archives.
#[path = "support/rgb555_cases.rs"]
mod cases;
use bessemer::{Document, FailureClass, preview::DibPreview};
use cases::{Rgb555Case, rgb555_cases};
const LIMIT: usize = 70_000;

#[test]
fn rgb555_literals_reconstruct_all_words_and_unused_high_bits() {
    let mut full_domains = 0;
    for case in rgb555_cases() {
        let source = DibPreview::from_bytes(case.original, case.words.len()).unwrap();
        assert_eq!(source.decode_bgr555_words(LIMIT).unwrap(), case.words);
        assert_eq!(source.to_bytes().unwrap(), case.original);
        assert_rgb555_reconstruction(&source, &case);
        if case.words.len() == 65536 {
            assert_eq!(case.words, (0..=u16::MAX).collect::<Vec<u16>>());
            full_domains += 1;
        }
    }
    assert_eq!(full_domains, 8);
}

fn assert_rgb555_reconstruction(source: &DibPreview, case: &Rgb555Case<'_>) {
    let rebuilt = DibPreview::new(source.contents().clone(), LIMIT).unwrap();
    assert_eq!(rebuilt.to_bytes().unwrap(), case.original);
    let equivalent = source
        .with_contents(source.contents().clone(), LIMIT)
        .unwrap();
    assert_eq!(equivalent.to_bytes().unwrap(), case.original);
    let noop = source.with_bgr555_words(&case.words, LIMIT).unwrap();
    assert_eq!(noop.to_bytes().unwrap(), case.original);
    assert_eq!(source.contents().header.x_pixels_per_metre, -1234);
    assert_eq!(source.contents().header.colors_important, u32::MAX);
    assert_eq!(
        source.decode_pixels(LIMIT).unwrap_err().class,
        FailureClass::Unsupported
    );
}

fn edited_rgb555_bytes(case: &Rgb555Case<'_>) -> Vec<u8> {
    let source = DibPreview::from_bytes(case.original, LIMIT).unwrap();
    let mut supplied = case.edited_words.clone();
    let edited = source.with_bgr555_words(&supplied, LIMIT).unwrap();
    supplied.fill(0);
    assert_eq!(edited.to_bytes().unwrap(), case.edited);
    let fresh = DibPreview::from_bytes(case.edited, LIMIT).unwrap();
    assert_eq!(fresh.decode_bgr555_words(LIMIT).unwrap(), case.edited_words);
    let rebuilt = DibPreview::new(fresh.contents().clone(), LIMIT).unwrap();
    let reversed = rebuilt.with_bgr555_words(&case.words, LIMIT).unwrap();
    assert_eq!(reversed.to_bytes().unwrap(), case.original);
    assert_eq!(source.to_bytes().unwrap(), case.original);
    edited.to_bytes().unwrap()
}

#[test]
fn rgb555_pixel_changes_match_literal_bits_and_reverse_from_fresh_owned_contents() {
    for case in rgb555_cases() {
        edited_rgb555_bytes(&case);
    }
}

#[test]
fn every_rgb555_preview_replacement_reopens_and_reverses_the_complete_document() {
    for case in rgb555_cases() {
        assert_rgb555_document(&case);
    }
}

fn assert_rgb555_document(case: &Rgb555Case<'_>) {
    let mut original = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    original
        .replace_stream("Preview", case.original.to_vec())
        .unwrap();
    let bytes = original.to_bytes().unwrap();
    let mut changed = Document::from_bytes(&bytes).unwrap();
    changed
        .replace_stream("Preview", edited_rgb555_bytes(case))
        .unwrap();
    let mut fresh = Document::from_bytes(&changed.to_bytes().unwrap()).unwrap();
    assert_rgb555_other_streams(&original, &fresh);
    let preview = DibPreview::from_bytes(&fresh.read_stream("Preview").unwrap(), LIMIT).unwrap();
    let reversed = preview.with_bgr555_words(&case.words, LIMIT).unwrap();
    fresh
        .replace_stream("Preview", reversed.to_bytes().unwrap())
        .unwrap();
    assert_eq!(fresh.to_bytes().unwrap(), bytes);
    assert_eq!(original.to_bytes().unwrap(), bytes);
}

fn assert_rgb555_other_streams(original: &Document, fresh: &Document) {
    for stream in original.streams().filter(|stream| stream.name != "Preview") {
        assert_eq!(
            original.read_stream(stream.name).unwrap(),
            fresh.read_stream(stream.name).unwrap()
        );
    }
}

#[test]
fn rgb555_prefixes_extra_bytes_and_mismatched_extents_never_publish_partial_pixels() {
    for case in rgb555_cases() {
        for end in 0..case.original.len() {
            assert!(DibPreview::from_bytes(&case.original[..end], LIMIT).is_err());
        }
        assert_rgb555_extent_failures(&case);
    }
}

fn assert_rgb555_extent_failures(case: &Rgb555Case<'_>) {
    let mut trailing = case.original.to_vec();
    trailing.push(0xff);
    assert!(DibPreview::from_bytes(&trailing, LIMIT).is_err());
    trailing[..4].copy_from_slice(&((case.original.len() - 3) as u32).to_le_bytes());
    assert!(DibPreview::from_bytes(&trailing, LIMIT).is_err());
    let source = DibPreview::from_bytes(case.original, LIMIT).unwrap();
    let mut contents = source.contents().clone();
    contents.header.image_size = contents.pixel_bytes.len() as u32 + 1;
    assert!(source.with_contents(contents, LIMIT).is_err());
    let mut contents = source.contents().clone();
    contents.pixel_bytes.pop();
    assert!(source.with_contents(contents, LIMIT).is_err());
    assert_eq!(source.to_bytes().unwrap(), case.original);
}

#[test]
fn rgb555_wrong_counts_budgets_and_formats_leave_sources_isolated() {
    for case in rgb555_cases() {
        let source = DibPreview::from_bytes(case.original, LIMIT).unwrap();
        assert!(DibPreview::from_bytes(case.original, case.words.len() - 1).is_err());
        assert!(source.decode_bgr555_words(case.words.len() - 1).is_err());
        assert!(source.with_bgr555_words(&[], LIMIT).is_err());
        assert!(
            source
                .with_bgr555_words(&case.words, case.words.len() - 1)
                .is_err()
        );
        assert!(source.with_indexed_pixels(&[0], LIMIT).is_err());
        assert!(source.with_bgr24_pixels(&[[0; 3]], LIMIT).is_err());
        assert!(source.with_bgrx_pixels(&[[0; 4]], LIMIT).is_err());
        assert_eq!(source.to_bytes().unwrap(), case.original);
    }
    assert_rgb555_extraction_rejects_other_formats();
}

fn assert_rgb555_extraction_rejects_other_formats() {
    let formats: &[&[u8]] = &[
        include_bytes!("fixtures/preview/rle8.bin"),
        include_bytes!("fixtures/preview/bgrx-top.bin"),
        include_bytes!("fixtures/preview-bgr24/w1-top-p0-z0.bin"),
        include_bytes!("fixtures/preview-indexed-rgb/one-entry-1.bin"),
    ];
    for bytes in formats {
        let source = DibPreview::from_bytes(bytes, LIMIT).unwrap();
        assert_eq!(
            source.decode_bgr555_words(LIMIT).unwrap_err().class,
            FailureClass::Unsupported
        );
        assert_eq!(
            source.with_bgr555_words(&[0], LIMIT).unwrap_err().class,
            FailureClass::Unsupported
        );
        assert_eq!(source.to_bytes().unwrap(), *bytes);
    }
}

#[test]
fn rgb555_header_dimensions_palettes_and_unproved_masks_remain_checked() {
    for case in rgb555_cases() {
        let source = DibPreview::from_bytes(case.original, LIMIT).unwrap();
        assert_rgb555_shape_failures(&source);
        let mut contents = source.contents().clone();
        contents.header.colors_used += 1;
        assert!(source.with_contents(contents, LIMIT).is_err());
        for compression in [1, 2, 3] {
            let mut contents = source.contents().clone();
            contents.header.compression = compression;
            assert_eq!(
                source.with_contents(contents, LIMIT).unwrap_err().class,
                FailureClass::Unsupported
            );
        }
    }
}

fn assert_rgb555_shape_failures(source: &DibPreview) {
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

fn all_rgb555_edits() -> Vec<Vec<u8>> {
    rgb555_cases().iter().map(edited_rgb555_bytes).collect()
}

#[test]
fn concurrent_rgb555_outputs_match_every_single_threaded_archive() {
    let workers: Vec<_> = (0..8)
        .map(|_| std::thread::spawn(all_rgb555_edits))
        .collect();
    let expected = all_rgb555_edits();
    for worker in workers {
        assert_eq!(worker.join().unwrap(), expected);
    }
}
