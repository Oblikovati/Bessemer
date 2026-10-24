# PNG previews

`bessemer::preview::PngPreview` implements the static PNG datastream referenced by
[clause 8.2](specification-8.md). This is PNG behavior version 1, tracked by #20.
The implementation follows the [PNG second edition](https://www.w3.org/TR/2003/REC-PNG-20031110/),
the external standard referenced by the SolidWorks schema.

The reader checks the signature, every chunk length and CRC, IHDR fields, critical
chunk order, contiguous IDAT framing, required palettes, standard ancillary field
syntax and ordering, zlib completion and Adler checks, exact scanline lengths,
filter codes, and palette-index bounds. Unknown critical chunks and APNG animation
chunks fail explicitly. No partial image is published after a failed check.

All fifteen legal color/depth combinations are supported: grayscale, RGB, indexed,
grayscale-alpha and RGBA, including packed samples and 16-bit channels. Both
noninterlaced and Adam7 images support all five row filters. Decoded rasters retain
unscaled channel samples and original palette indices. `rgba16` expands palette
and transparency values and scales channels to 16 bits. It does not apply gamma,
chromaticity or ICC color management. ICC profile contents and registered language
semantics remain opaque; their PNG framing and compressed payloads are checked.

`PngLimits` bounds pixels before raster allocation, physical chunk count, and the
cumulative expansion of image scanlines and compressed ancillary content. Input
stream size can be bounded separately with `Document::read_stream_limited`.
The archive retains every raw chunk payload and its order, including unknown
ancillary content, text encodings, compressed representation and unused row bits.
Valid chunk lengths and CRCs are derived when rebuilding. This implementation
requires complete zlib consumption and rejects extra bytes in compressed streams.

Published archives and rasters are immutable and Send/Sync. `new` reconstructs an
archive from stored chunks; `from_raster` constructs one from exact typed samples
and a palette. `with_contents` validates an owned chunk edit. Equivalent stored
edits and authentic full reverts preserve original bytes. Unknown unsafe chunks
cannot be retained across critical changes; unknown chunks retain their required
critical region or side of IDAT. Duplicate identical payloads are matched in order.

`with_samples` preserves the source color model, sample depth, dimensions, palette
and interlace mode. Changed pixels use deterministic unfiltered scanlines and one
IDAT chunk. Unknown unsafe chunks, palette histograms and timestamps are removed
from this edited image. Other valid metadata is retained in its original region.
Identical samples preserve the original encoding and metadata. Restoring the
complete original stored contents restores the original datastream.

## Usage

```rust
use bessemer::preview::{PngColor, PngHeader, PngLimits, PngPreview, PngRaster};

let limits = PngLimits {
    maximum_pixels: 16_000_000,
    maximum_inflated_bytes: 256 * 1024 * 1024,
    maximum_chunks: 100_000,
};
let raster = PngRaster {
    header: PngHeader {
        width: 2, height: 1, bit_depth: 8,
        color: PngColor::Rgba, interlaced: false,
    },
    samples: vec![255, 0, 0, 255, 0, 0, 255, 255],
};
let preview = PngPreview::from_raster(raster, Vec::new(), limits)?;
let bytes = preview.to_bytes()?;
```

`rebuild_png INPUT OUTPUT SAMPLES RGBA16` forces chunk reconstruction and emits
big-endian u16 sample oracles. `set_png_sample SOURCE STREAM SAMPLE_INDEX VALUE
DESTINATION` edits one exact sample and replaces its stream through `Document`.

## Verification

On Windows x64, Rust 1.99.0 MSVC, 2026-10-24, all 1,156 PNG streams detected in the
installed core-document corpus reconstructed byte for byte: 29,589,271 extracted
bytes. Every one of their 355,123,200 pixels matched Pillow 12.3.0 as exact expanded
RGBA16 values. Native cases comprised 1,121 RGBA8 and 35 RGB8 noninterlaced images.
The SHA-256 of sorted relative path, NUL, stream name, NUL and raw bytes was
`a89470ca2068bc0f68868b1f0e1290ca34e45aed832ec302acf475b856fcc02e`.

The 120 owned vectors cover every legal color/depth pair, both interlace modes,
all filters, and dimensions 9×9, 1×1, 2×3 and 4×1. Python wire primitives generate
them from closed-form channel values, with separate exact sample and RGBA16
oracles. All 120 were independently accepted by Pillow: grayscale16 samples matched
exactly, and the other formats matched normalized RGBA high bytes. Exact low-byte
checks for all 16-bit formats use the closed-form oracles.

Integration tests cover reconstruction, typed construction, edits and authentic
reverts, standard metadata, malformed framing/CRC/zlib/filter cases, expansion
budgets, unsupported variants, palette domains, unknown-chunk copy rules, duplicate
payload regions, source isolation, and eight rounds of eight concurrent edits
compared with serial output. Whole-document tests use the owned native blank part.
A separate two-process sample edit matched Pillow's expected pixels, preserved
every untouched compressed stream byte, and produced identical complete files.
Native application reopening and color-profile rendering were not run.

`verify_corpus.py` repeats the native oracle with the approved sibling reader and
Pillow; pass the built `rebuild_png` executable path from the repository root. The
verifier copies that executable into its temporary directory to isolate it from
later builds. `generate_fixtures.py` deterministically rebuilds the owned vectors.

The evaluated `png` 0.18.1 dependency required an additional `miniz_oxide` version,
which the repository's duplicate-version ban rejects. The implementation instead
reuses existing `flate2` and the checked compressed-block inflater. No dependency,
version or compression backend changed. `cargo deny --all-features check` and
`cargo audit` passed. The source snapshot retains Edition 0.121 from the evolving
sibling working tree at `dd0537b3a142d8dda00ef846891efc736a6aac46`; that commit alone
does not identify its uncommitted schema updates. The maintainer humanizer skill is
unavailable; its prose pass remains for review. Broader schema parity remains #2.
