# Packed DIB previews

`bessemer::preview::DibPreview` implements the packed DIB stream in
[clause 8.1](specification-8.md). This is DIB behavior version 1, tracked by #18.
It supports the two installed-corpus formats: eight-bit BI_RLE8 and 32-bit BI_RGB
BGRX. Other bit depths, compression modes and header types return typed
unsupported errors. PNG streams are a separate clause 8.2 parity task.

The immutable archive retains every BITMAPINFOHEADER field, every RGBQUAD byte,
the complete declared image bytes, absolute-run alignment bytes, and bytes after
the RLE end marker. The outer size is derived when rebuilding. Stored image size
must match the complete image payload, including for BGRX. Raw fourth bytes in
palette and BGRX entries are preserved without assuming alpha semantics.

RLE decoding follows Microsoft's [bitmap compression contract](https://learn.microsoft.com/windows/win32/gdi/bitmap-compression).
It checks encoded runs, absolute runs, word alignment, EOL, EOB and delta targets.
Missing EOB, out-of-bounds writes and invalid palette indices fail. RLE row order
is bottom-up; BGRX supports positive bottom-up and negative top-down heights,
following the [bitmap header contract](https://learn.microsoft.com/windows/win32/gdi/bitmap-header-types).
Decoded output always uses top-to-bottom rows. Positions that no RLE command
wrote are `None`, so callers can choose their own background policy.

Callers supply a maximum pixel count at decoding, construction, editing and raster
extraction. Header dimensions and budgets are checked before raster allocation.
Palette and payload lengths are checked against available bytes before copying.
These limits complement `Document::read_stream_limited` for decompression.

`with_contents` validates an owned edit before publication. Equivalent stored
contents and reverts preserve the original bytes. `new` forces reconstruction.
`with_indexed_pixels` replaces all pixels with deterministic RLE runs of at most
255 indices, EOL for every row, and EOB. It deliberately replaces the original
command encoding, including any unwritten positions and trailing bytes.
`with_bgrx_pixels` replaces all four bytes per pixel in the original row orientation.
Both methods retain the header observations and palette, and update image size.
They reject replacements whose count or format differs from the source.

## Usage

```rust
use bessemer::preview::DibPreview;

let preview = DibPreview::from_bytes(&stream_bytes, 16_000_000)?;
let pixels = preview.decode_pixels(16_000_000)?;
let mut contents = preview.contents().clone();
contents.palette[0].red = 128;
let changed = preview.with_contents(contents, 16_000_000)?;
let stream_bytes = changed.to_bytes()?;
```

`rebuild_dib INPUT OUTPUT RASTER` forces reconstruction and emits a verification
raster: two bytes per indexed pixel (written marker, index), or four BGRX bytes
per pixel. `set_preview_color SOURCE STREAM INDEX BLUE GREEN RED DESTINATION`
performs a complete document palette edit, preserving the entry's reserved byte.

## Verification

On Windows x64 with Rust 1.99.0 MSVC, 2026-10-24, forced reconstruction matched
all 1,129 detected packed DIB streams in the installed core-document corpus:
1,118 RLE8 and 11 BGRX streams, totalling 59,380,280 extracted bytes. All
1,668,498,826 decoded pixels matched Pillow 12.3.0; none of these native rasters
had unwritten positions. Synthetic vectors separately verify unwritten positions.
The SHA-256 of sorted relative path, NUL, stream name, NUL and raw stream bytes was
`890a765face0defb281645320496f02e0fdd293543405c4ab650fe21c651e70b`.

Owned vectors cover all RLE command forms, odd/even absolute lengths, explicit and
default palettes, early EOB with unwritten pixels, retained suffix bytes, both
BGRX row orientations, and reserved bytes. Their generator uses Python wire
primitives and manually specified pixel expectations, independently of Rust.
Generated raster tests cover widths around 255-run boundaries. Integration tests
exercise every truncated fixture, malformed command and header cases, allocation
limits, mutation, source isolation, equivalent edits/reverts, compile-time
Send/Sync assertions, and eight rounds of eight concurrent operations.

Whole-document tests use the owned native blank part. Palette and full pixel
replacements reopen through `Document`. A separate two-process palette edit was
checked with the sibling container reader: only the selected three colour bytes
changed in the extracted preview, every untouched compressed stream stayed
byte-identical, and both complete output files matched. Native application
reopening was not run.

The external corpus verifier is `verify_corpus.py`; run it from the repository
root with Pillow available and the path to the built `rebuild_dib` executable.
It enumerates the approved sibling's `swcover` corpus, forces Rust reconstruction,
and compares written pixels with Pillow. It makes a private executable copy so
concurrent Cargo builds cannot replace a running verifier.

The retained source is Edition 0.118 from the evolving sibling working tree at
`dd0537b3a142d8dda00ef846891efc736a6aac46`, read 2026-10-24. That commit alone does
not identify its uncommitted schema updates. No production dependencies were
added. The maintainer humanizer skill is unavailable; its prose pass remains for
review. Complete schema parity remains tracked by #2.
