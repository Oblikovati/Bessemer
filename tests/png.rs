//! Closed-form PNG vectors: all color/depth pairs, filters, interlace passes and exact samples.
#[path = "support/png.rs"]
mod support;
use bessemer::preview::*;
use support::*;

#[test]
fn every_owned_vector_rebuilds_exactly_and_matches_independent_sample_oracles() {
    for name in include_str!("fixtures/png/cases.txt").lines() {
        let bytes = fixture(name, "png");
        let source = parse(&bytes);
        assert_eq!(
            PngPreview::new(source.contents().clone(), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            bytes,
            "{name}"
        );
        let samples: Vec<u8> = source
            .raster()
            .samples
            .iter()
            .flat_map(|value| value.to_be_bytes())
            .collect();
        assert_eq!(samples, fixture(name, "samples"), "{name}");
        let rgba: Vec<u8> = source
            .rgba16()
            .iter()
            .flatten()
            .flat_map(|value| value.to_be_bytes())
            .collect();
        assert_eq!(rgba, fixture(name, "rgba"), "{name}");
        assert_eq!(
            source
                .with_samples(source.raster().samples.clone(), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            bytes
        );
    }
}

#[test]
fn typed_raster_construction_supports_every_color_depth_and_interlace_combination() {
    for name in include_str!("fixtures/png/cases.txt").lines() {
        let source = parse(&fixture(name, "png"));
        let palette = source
            .contents()
            .chunks
            .iter()
            .find(|chunk| chunk.kind.bytes() == *b"PLTE")
            .map(|chunk| chunk.payload.as_chunks::<3>().0.to_vec())
            .unwrap_or_default();
        let constructed =
            PngPreview::from_raster(source.raster().clone(), palette, LIMITS).unwrap();
        assert_eq!(constructed.raster(), source.raster());
        assert_eq!(
            parse(&constructed.to_bytes().unwrap()).raster(),
            source.raster()
        );
    }
}

#[test]
fn duplicate_unknown_payloads_keep_their_individual_before_and_after_image_regions() {
    let source = parse(BASE);
    let mut contents = source.contents().clone();
    let duplicate = contents.chunks[1].clone();
    let end = contents.chunks.len() - 1;
    contents.chunks.insert(end, duplicate);
    let source = PngPreview::new(contents, LIMITS).unwrap();
    let mut contents = source.contents().clone();
    before_image(&mut contents, chunk(*b"tEXt", b"Title\0Updated"));
    assert!(source.with_contents(contents, LIMITS).is_ok());
}

#[test]
fn every_zlib_byte_boundary_can_be_an_idat_boundary_including_the_adler_checksum() {
    let source = parse(BASE);
    let compressed: Vec<u8> = source
        .contents()
        .chunks
        .iter()
        .filter(|chunk| chunk.kind.bytes() == *b"IDAT")
        .flat_map(|chunk| chunk.payload.iter().copied())
        .collect();
    let mut contents = source.contents().clone();
    contents
        .chunks
        .retain(|chunk| chunk.kind.bytes() != *b"IDAT");
    let position = contents.chunks.len() - 2;
    contents.chunks.splice(
        position..position,
        compressed.iter().map(|byte| chunk(*b"IDAT", &[*byte])),
    );
    let segmented = PngPreview::new(contents, LIMITS).unwrap();
    assert_eq!(segmented.raster(), source.raster());
    assert_eq!(
        parse(&segmented.to_bytes().unwrap()).raster(),
        source.raster()
    );
}

#[test]
fn animation_extensions_preserve_the_default_png_image_without_claiming_frame_decoding() {
    let bytes = fixture("default-image-apng", "png");
    let source = parse(&bytes);
    assert_eq!(source.raster(), parse(BASE).raster());
    assert_eq!(
        PngPreview::new(source.contents().clone(), LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        bytes
    );
    let mut samples = source.raster().samples.clone();
    samples[0] ^= 1;
    let edited = source.with_samples(samples, LIMITS).unwrap();
    assert!(
        !edited
            .contents()
            .chunks
            .iter()
            .any(|chunk| [*b"acTL", *b"fcTL", *b"fdAT"].contains(&chunk.kind.bytes()))
    );
    assert_eq!(
        edited
            .with_contents(source.contents().clone(), LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        bytes
    );
}

#[test]
fn every_format_encodes_edited_samples_without_losing_depth_or_interlace() {
    for name in include_str!("fixtures/png/cases.txt").lines() {
        let bytes = fixture(name, "png");
        let source = parse(&bytes);
        let mut samples = source.raster().samples.clone();
        samples[0] ^= 1;
        let changed = source.with_samples(samples.clone(), LIMITS).unwrap();
        assert_eq!(changed.raster().header, source.raster().header);
        assert_eq!(
            parse(&changed.to_bytes().unwrap()).raster().samples,
            samples,
            "{name}"
        );
        assert!(
            changed
                .contents()
                .chunks
                .iter()
                .any(|chunk| chunk.kind.bytes() == *b"vpAg")
        );
        assert!(
            !changed
                .contents()
                .chunks
                .iter()
                .any(|chunk| chunk.kind.bytes() == *b"vpAG")
        );
        assert_eq!(source.to_bytes().unwrap(), bytes);
        assert_eq!(
            changed
                .with_contents(source.contents().clone(), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            bytes
        );
    }
}

#[test]
fn complete_stored_content_reverts_restore_original_bytes_after_ancillary_edits() {
    let source = parse(BASE);
    let original = source.contents().clone();
    let mut contents = original.clone();
    before_image(&mut contents, chunk(*b"tEXt", b"Title\0Edited"));
    let edited = source.with_contents(contents, LIMITS).unwrap();
    assert_eq!(
        edited
            .with_contents(original, LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        BASE
    );
}

#[test]
fn rgb_transparency_compares_original_samples_before_scaling() {
    let source = parse(&fixture("c2-d16-i1-9x9", "png"));
    let mut contents = source.contents().clone();
    let key: Vec<u8> = source.raster().samples[..3]
        .iter()
        .flat_map(|value| value.to_be_bytes())
        .collect();
    contents
        .chunks
        .iter_mut()
        .find(|chunk| chunk.kind.bytes() == *b"tRNS")
        .unwrap()
        .payload = key;
    let changed = PngPreview::new(contents, LIMITS).unwrap();
    assert_eq!(changed.rgba16()[0][3], 0);
    assert_eq!(changed.rgba16()[1][3], u16::MAX);
}

#[test]
fn standard_ancillary_fields_and_bounded_compressed_text_are_retained() {
    for (kind, payload) in [
        (*b"cHRM", vec![0; 32]),
        (*b"gAMA", 100u32.to_be_bytes().to_vec()),
        (*b"sBIT", vec![8; 4]),
        (*b"sRGB", vec![3]),
        (*b"bKGD", vec![0; 6]),
        (*b"pHYs", vec![0; 9]),
        (*b"tIME", vec![7, 234, 10, 24, 23, 59, 60]),
        (*b"tEXt", b"Title\0Caf\xe9".to_vec()),
        (*b"sPLT", b"Palette\0\x08\x01\x02\x03\x04\x00\x01".to_vec()),
    ] {
        let source = with_metadata(kind, &payload).unwrap();
        assert!(
            source
                .contents()
                .chunks
                .iter()
                .any(|chunk| chunk.kind.bytes() == kind && chunk.payload == payload)
        );
        assert_eq!(
            parse(&source.to_bytes().unwrap()).contents(),
            source.contents()
        );
    }
    for kind in [*b"zTXt", *b"iCCP"] {
        let payload = [
            b"Title\0\0".as_slice(),
            &compressed(b"Example profile or text"),
        ]
        .concat();
        assert!(with_metadata(kind, &payload).is_ok());
    }
    for flag in [0, 1] {
        let text = if flag == 0 {
            "λ".as_bytes().to_vec()
        } else {
            compressed("λ".as_bytes())
        };
        let payload = [
            b"Title\0".as_slice(),
            &[flag, 0],
            b"en-GB\0",
            "Τίτλος".as_bytes(),
            b"\0",
            &text,
        ]
        .concat();
        assert!(with_metadata(*b"iTXt", &payload).is_ok());
    }
}

#[test]
fn indexed_metadata_preserves_palette_frequencies_and_partial_alpha() {
    let source = parse(&fixture("c3-d2-i0-9x9", "png"));
    let mut contents = source.contents().clone();
    before_image(&mut contents, chunk(*b"hIST", &[0; 8]));
    before_image(&mut contents, chunk(*b"bKGD", &[3]));
    let source = PngPreview::new(contents, LIMITS).unwrap();
    let mut samples = source.raster().samples.clone();
    samples[0] = 1;
    let edited = source.with_samples(samples, LIMITS).unwrap();
    assert!(
        !edited
            .contents()
            .chunks
            .iter()
            .any(|chunk| chunk.kind.bytes() == *b"hIST")
    );
    assert_eq!(edited.rgba16()[0][3], 37 * 257);
}
