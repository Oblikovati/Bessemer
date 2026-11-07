//! Independent DIB header, palette, row-orientation and RLE opcode expectations.
#[path = "support/preview.rs"]
mod support;
use bessemer::preview::*;
use support::*;

#[test]
fn all_owned_archives_reconstruct_every_byte_and_preserve_metadata() {
    for bytes in [RLE, TOP, BOTTOM, DEFAULT] {
        let source = parse(bytes);
        assert_eq!(source.to_bytes().unwrap(), bytes);
        assert_eq!(
            DibPreview::new(source.contents().clone(), LIMIT)
                .unwrap()
                .to_bytes()
                .unwrap(),
            bytes
        );
        assert_eq!(source.contents().header.x_pixels_per_metre, -1234);
        assert_eq!(source.contents().header.colors_important, u32::MAX);
        assert_eq!(
            source
                .with_contents(source.contents().clone(), LIMIT)
                .unwrap()
                .to_bytes()
                .unwrap(),
            bytes
        );
    }
    assert_eq!(
        parse(RLE).contents().palette[0],
        PaletteEntry {
            blue: 0,
            green: 1,
            red: 2,
            reserved: 128
        }
    );
    assert_eq!(parse(DEFAULT).contents().palette.len(), 256);
}

#[test]
fn encoded_absolute_delta_eol_eob_and_unwritten_pixels_match_manual_oracle() {
    let expected = vec![
        Some(4),
        Some(3),
        Some(2),
        Some(1),
        Some(0),
        None,
        Some(2),
        Some(2),
        Some(2),
        None,
        None,
        None,
        None,
        None,
        None,
        Some(1),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
    ];
    assert_eq!(
        parse(RLE).decode_pixels(LIMIT).unwrap(),
        DibPixels::Indexed(expected)
    );
    assert_eq!(
        parse(DEFAULT).decode_pixels(LIMIT).unwrap(),
        DibPixels::Indexed(vec![None])
    );
    assert_eq!(&parse(RLE).contents().pixel_bytes[30..], &[0xa5, 0x5a]);
}

#[test]
fn both_bgrx_orientations_preserve_fourth_bytes_and_roundtrip_replacements() {
    let expected: Vec<[u8; 4]> = (0..16).collect::<Vec<u8>>().as_chunks::<4>().0.to_vec();
    for bytes in [TOP, BOTTOM] {
        let source = parse(bytes);
        assert_eq!(
            source.decode_pixels(LIMIT).unwrap(),
            DibPixels::Bgrx(expected.clone())
        );
        assert_eq!(
            source
                .with_bgrx_pixels(&expected, LIMIT)
                .unwrap()
                .to_bytes()
                .unwrap(),
            bytes
        );
        let replacement = vec![[0xaa, 0xbb, 0xcc, 0xdd]; 4];
        let changed = source.with_bgrx_pixels(&replacement, LIMIT).unwrap();
        assert_eq!(
            parse(&changed.to_bytes().unwrap())
                .decode_pixels(LIMIT)
                .unwrap(),
            DibPixels::Bgrx(replacement)
        );
        assert_eq!(source.to_bytes().unwrap(), bytes);
    }
}

#[test]
fn indexed_replacement_uses_deterministic_bounded_runs_and_top_down_order() {
    let source = commands(257, 2, &[0, 1]).unwrap();
    let mut indices = vec![4; 514];
    indices[0] = 3;
    let changed = source.with_indexed_pixels(&indices, LIMIT).unwrap();
    assert_eq!(&changed.contents().pixel_bytes[..6], &[255, 4, 2, 4, 0, 0]);
    assert_eq!(
        changed.decode_pixels(LIMIT).unwrap(),
        DibPixels::Indexed(indices.iter().map(|index| Some(*index)).collect())
    );
    assert_eq!(
        changed.contents().header.image_size as usize,
        changed.contents().pixel_bytes.len()
    );
    assert_eq!(
        parse(&changed.to_bytes().unwrap()).contents(),
        changed.contents()
    );
}

#[test]
fn generated_rasters_cover_run_boundaries_and_repeatable_encodings() {
    for width in [1, 2, 3, 254, 255, 256, 257, 511] {
        let source = commands(width, 3, &[0, 1]).unwrap();
        let indices: Vec<u8> = (0..width * 3)
            .map(|i| ((i * 7 + i / 13) % 5) as u8)
            .collect();
        let edited = source.with_indexed_pixels(&indices, LIMIT).unwrap();
        assert_eq!(
            edited.decode_pixels(LIMIT).unwrap(),
            DibPixels::Indexed(indices.into_iter().map(Some).collect())
        );
        assert_eq!(
            parse(&edited.to_bytes().unwrap()).to_bytes().unwrap(),
            edited.to_bytes().unwrap()
        );
    }
}
