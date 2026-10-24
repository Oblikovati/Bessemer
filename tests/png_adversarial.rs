//! PNG framing, CRC, zlib, ordering, metadata budgets and unknown-chunk edit boundaries.
#[path = "support/png.rs"]
mod support;
use bessemer::preview::*;
use support::*;

#[test]
fn every_truncation_and_appended_outer_bytes_fail() {
    for end in 0..BASE.len() {
        assert!(PngPreview::from_bytes(&BASE[..end], LIMITS).is_err());
    }
    let mut bytes = BASE.to_vec();
    bytes.push(0);
    assert!(PngPreview::from_bytes(&bytes, LIMITS).is_err());
    let mut bytes = BASE.to_vec();
    bytes[29] ^= 1;
    assert_eq!(
        PngPreview::from_bytes(&bytes, LIMITS).unwrap_err().class,
        bessemer::FailureClass::Integrity
    );
    assert!(ChunkType::new(*b"abca").is_err());
    assert!(ChunkType::new(*b"a0Aa").is_err());
}

#[test]
fn required_chunks_order_and_duplicate_or_unknown_critical_cases_fail() {
    let source = parse(BASE);
    for kind in [*b"IHDR", *b"IDAT", *b"IEND"] {
        let mut contents = source.contents().clone();
        contents.chunks.retain(|chunk| chunk.kind.bytes() != kind);
        assert!(PngPreview::new(contents, LIMITS).is_err(), "{kind:?}");
    }
    for kind in [*b"IHDR", *b"IEND", *b"tRNS", *b"gAMA", *b"sRGB"] {
        let mut contents = source.contents().clone();
        before_image(&mut contents, chunk(kind, &[]));
        assert!(PngPreview::new(contents, LIMITS).is_err());
    }
    {
        let mut contents = source.contents().clone();
        before_image(&mut contents, chunk(*b"ZZZZ", &[]));
        assert_eq!(
            PngPreview::new(contents, LIMITS).unwrap_err().class,
            bessemer::FailureClass::Unsupported
        );
    }
    let mut contents = source.contents().clone();
    let first = contents
        .chunks
        .iter()
        .position(|chunk| chunk.kind.bytes() == *b"IDAT")
        .unwrap();
    contents.chunks.insert(first + 1, chunk(*b"vpAg", b"break"));
    assert!(PngPreview::new(contents, LIMITS).is_err());
}

#[test]
fn dimensions_depths_methods_and_budgets_are_checked_before_raster_allocation() {
    for (offset, value) in [(3, 0), (7, 0), (8, 3), (9, 1), (10, 1), (11, 1), (12, 2)] {
        let mut contents = parse(BASE).contents().clone();
        contents.chunks[0].payload[offset] = value;
        assert!(PngPreview::new(contents, LIMITS).is_err(), "{offset}");
    }
    let mut contents = parse(BASE).contents().clone();
    contents.chunks[0].payload[..8].fill(0x7f);
    let limits = PngLimits {
        maximum_pixels: usize::MAX,
        maximum_inflated_bytes: usize::MAX,
        ..LIMITS
    };
    assert!(PngPreview::new(contents, limits).is_err());
    for limits in [
        PngLimits {
            maximum_pixels: 80,
            ..LIMITS
        },
        PngLimits {
            maximum_inflated_bytes: 0,
            ..LIMITS
        },
        PngLimits {
            maximum_chunks: 1,
            ..LIMITS
        },
    ] {
        assert!(PngPreview::from_bytes(BASE, limits).is_err());
    }
}

#[test]
fn invalid_ancillary_payloads_are_not_silently_ignored() {
    for (kind, payload) in [
        (*b"gAMA", vec![0; 4]),
        (*b"sRGB", vec![4]),
        (*b"sBIT", vec![0; 4]),
        (*b"pHYs", vec![2; 9]),
        (*b"tIME", vec![0; 7]),
        (*b"bKGD", vec![0; 5]),
        (*b"hIST", vec![0; 4]),
        (*b"tRNS", vec![0; 6]),
        (*b"tEXt", b"bad  keyword\0x".to_vec()),
        (*b"zTXt", b"Name\0\x01bad".to_vec()),
        (*b"iTXt", b"Name\0\x02\0\0\0bad".to_vec()),
        (*b"iCCP", b"Name\0\0bad".to_vec()),
        (*b"sPLT", b"Name\0\x04".to_vec()),
    ] {
        assert!(with_metadata(kind, &payload).is_err(), "{kind:?}");
    }
    let payload = [b"Name\0\0".as_slice(), &compressed(&vec![b'x'; 2000])].concat();
    let mut contents = parse(BASE).contents().clone();
    before_image(&mut contents, chunk(*b"zTXt", &payload));
    assert!(
        PngPreview::new(
            contents,
            PngLimits {
                maximum_inflated_bytes: 500,
                ..LIMITS
            }
        )
        .is_err()
    );
}

#[test]
fn unknown_chunk_copy_rules_reject_unsafe_image_edits_and_region_moves() {
    let source = parse(BASE);
    let mut contents = source.contents().clone();
    let image = contents
        .chunks
        .iter_mut()
        .find(|chunk| chunk.kind.bytes() == *b"IDAT")
        .unwrap();
    image.payload.push(0);
    assert!(
        source
            .with_contents(contents, LIMITS)
            .unwrap_err()
            .cause
            .contains("unsafe unknown")
    );
    let mut contents = source.contents().clone();
    let safe = contents.chunks.remove(1);
    let end = contents.chunks.len() - 1;
    contents.chunks.insert(end, safe);
    assert!(
        source
            .with_contents(contents, LIMITS)
            .unwrap_err()
            .cause
            .contains("moved unknown")
    );
    assert!(source.with_samples(vec![0; 1], LIMITS).is_err());
    assert!(
        source
            .with_samples(vec![256; source.raster().samples.len()], LIMITS)
            .is_err()
    );
    assert_eq!(source.to_bytes().unwrap(), BASE);
}

#[test]
fn zlib_completeness_adler_filter_and_exact_scanline_extent_are_required() {
    let mut invalid_filter = vec![0; 333];
    invalid_filter[0] = 5;
    let valid = compressed(&[0; 333]);
    let mut bad_adler = valid.clone();
    *bad_adler.last_mut().unwrap() ^= 1;
    let cases = [
        compressed(&[0; 332]),
        compressed(&[0; 334]),
        compressed(&invalid_filter),
        valid[..valid.len() - 1].to_vec(),
        bad_adler,
        Vec::new(),
        compressed(&[0; 10000]),
    ];
    for bytes in cases {
        let mut contents = parse(BASE).contents().clone();
        contents
            .chunks
            .retain(|chunk| chunk.kind.bytes() != *b"IDAT");
        let end = contents.chunks.len() - 2;
        contents.chunks.insert(end, chunk(*b"IDAT", &bytes));
        assert!(PngPreview::new(contents, LIMITS).is_err());
    }
}

#[test]
fn compressed_metadata_uses_a_shared_budget_and_checks_utf8_and_completeness() {
    let mut contents = parse(BASE).contents().clone();
    let payload = [b"Name\0\0".as_slice(), &compressed(&[b'x'; 60])].concat();
    before_image(&mut contents, chunk(*b"zTXt", &payload));
    before_image(&mut contents, chunk(*b"zTXt", &payload));
    assert!(
        PngPreview::new(
            contents,
            PngLimits {
                maximum_inflated_bytes: 433,
                ..LIMITS
            }
        )
        .is_err()
    );
    for payload in [
        b"Name\0\0\0\0\0\xff".as_slice(),
        b"Name\0\x01\0\0\0bad",
        b" Name\0\0\0\0\0text",
    ] {
        assert!(with_metadata(*b"iTXt", payload).is_err());
    }
    let mut payload = [b"Name\0\0".as_slice(), &compressed(b"text")].concat();
    payload.push(0);
    assert!(with_metadata(*b"zTXt", &payload).is_err());
}

#[test]
fn invalid_palette_constructor_and_sample_domains_fail_explicitly() {
    let source = parse(&fixture("c3-d1-i0-9x9", "png"));
    assert!(PngPreview::from_raster(source.raster().clone(), vec![], LIMITS).is_err());
    assert!(PngPreview::from_raster(source.raster().clone(), vec![[0; 3]; 257], LIMITS).is_err());
    assert!(PngPreview::from_raster(source.raster().clone(), vec![[0; 3]; 1], LIMITS).is_err());
    let source = parse(&fixture("c0-d1-i0-9x9", "png"));
    assert!(PngPreview::from_raster(source.raster().clone(), vec![[0; 3]; 2], LIMITS).is_err());
}
