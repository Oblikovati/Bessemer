//! Strict archive extents, RLE bounds, explicit unsupported variants and immutable failures.
#[path = "support/preview.rs"]
mod support;
use bessemer::preview::*;
use support::*;

#[test]
fn every_truncation_and_trailing_outer_byte_fails() {
    for bytes in [RLE, TOP, BOTTOM, DEFAULT] {
        for end in 0..bytes.len() {
            assert!(DibPreview::from_bytes(&bytes[..end], LIMIT).is_err());
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(DibPreview::from_bytes(&trailing, LIMIT).is_err());
    }
}

#[test]
fn invalid_commands_counts_rows_indices_deltas_and_missing_eob_are_rejected() {
    let cases: &[&[u8]] = &[
        &[],
        &[1],
        &[1, 1],
        &[0, 0],
        &[0, 2],
        &[0, 3, 1, 2],
        &[6, 1, 0, 1],
        &[1, 5, 0, 1],
        &[0, 2, 6, 0, 0, 1],
        &[0, 2, 0, 4, 0, 1],
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        &[0, 3, 1, 2, 3],
    ];
    for bytes in cases {
        assert!(commands(5, 4, bytes).is_err(), "{bytes:?}");
    }
    assert!(commands(5, 4, &[0, 3, 1, 2, 3, 0, 0, 1]).is_ok());
    assert!(commands(5, 4, &[0, 4, 1, 2, 3, 4, 0, 1]).is_ok());
}

#[test]
fn dimensions_budgets_header_formats_palette_and_image_sizes_are_checked() {
    for (width, height) in [(0, 1), (-1, 1), (1, 0), (1, -1), (1, i32::MIN)] {
        assert!(commands(width, height, &[0, 1]).is_err());
    }
    assert!(DibPreview::from_bytes(RLE, 19).is_err());
    assert!(parse(RLE).decode_pixels(19).is_err());
    for field in ["size", "planes", "bits", "compression", "palette", "image"] {
        let mut contents = parse(RLE).contents().clone();
        match field {
            "size" => contents.header.header_size = 108,
            "planes" => contents.header.planes = 2,
            "bits" => contents.header.bit_count = 4,
            "compression" => contents.header.compression = 2,
            "palette" => contents.header.colors_used = 257,
            _ => contents.header.image_size = 1,
        }
        let error = DibPreview::new(contents, LIMIT).unwrap_err();
        let expected = if ["size", "bits", "compression"].contains(&field) {
            bessemer::FailureClass::Unsupported
        } else {
            bessemer::FailureClass::Invalid
        };
        assert_eq!(error.class, expected, "{field}");
    }
}

#[test]
fn malformed_owned_shapes_and_wrong_pixel_replacements_fail_without_source_edits() {
    let source = parse(RLE);
    let mut contents = source.contents().clone();
    contents.palette.pop();
    assert!(source.with_contents(contents, LIMIT).is_err());
    assert!(source.with_indexed_pixels(&[0; 19], LIMIT).is_err());
    assert!(source.with_indexed_pixels(&[5; 20], LIMIT).is_err());
    assert!(source.with_indexed_pixels(&[0; 20], 19).is_err());
    assert!(source.with_bgrx_pixels(&[[0; 4]; 20], LIMIT).is_err());
    assert!(parse(TOP).with_indexed_pixels(&[0; 4], LIMIT).is_err());
    let mut contents = parse(TOP).contents().clone();
    contents.pixel_bytes.pop();
    contents.header.image_size -= 1;
    assert!(DibPreview::new(contents, LIMIT).is_err());
    assert_eq!(source.to_bytes().unwrap(), RLE);
}

#[test]
fn byte_mutations_are_typed_failures_or_complete_rebuildable_archives() {
    for index in 0..RLE.len() {
        let mut bytes = RLE.to_vec();
        bytes[index] ^= 0xff;
        if let Ok(source) = DibPreview::from_bytes(&bytes, LIMIT) {
            assert_eq!(
                DibPreview::new(source.contents().clone(), LIMIT)
                    .unwrap()
                    .to_bytes()
                    .unwrap(),
                bytes
            );
            source.decode_pixels(LIMIT).unwrap();
        }
    }
}
