# Uncompressed indexed DIB previews

Feature #209 implements BI_RGB at 1, 4 and 8 bits per pixel under SolidWorks
clause 8.1 and its referenced Windows DIB contract. This is DIB behavior version 3;
the earlier RLE8, BGR24 and BGRX results retain their existing encodings.
There is no new public type or pixel method.

`DibPreview::from_bytes` and `new` validate dimensions, the complete raster extent,
palette length and every referenced index. A zero palette count selects 2, 16 or
256 entries according to bit depth. Explicit shorter palettes are accepted only
when every pixel references an existing entry. Rows have four-byte alignment;
positive heights are bottom-up and negative heights top-down. Zero uncompressed
image sizes derive the exact extent from dimensions without normalizing the header.

`decode_pixels` returns `DibPixels::Indexed`, with `Some(index)` at every position.
The `None` representation remains specific to unwritten RLE8 positions.
`with_indexed_pixels` accepts top-to-bottom indices and preserves the stored row
orientation, palette bytes, metadata, padding and unused low bits in the final
packed byte. Equivalent edits and fresh reversals reproduce the literal archive.
Wrong counts, out-of-range indices, malformed extents and exceeded budgets return
typed errors before an edited preview is published.

`generate_fixtures.py` assembles 864 original/edited pairs independently of Rust.
The matrix crosses all three bit depths, eighteen widths from 1 through 257, both
orientations, one/two-row heights, default/short palettes and explicit/zero image
sizes. Every bit depth includes all four padding sizes. Widths surround byte, nibble, DWORD and 256-pixel boundaries. Each edit flips
the low bit of every index. The generator independently checks that the whole
archive differs by exactly that many bits, ruling out extra padding or metadata
changes when combined with the native pixel comparison.
Three additional one-pixel archives verify the minimum one-entry palette at each
bit depth, including unused-bit preservation and no-op replacement.

`verify_windows_oracle.ps1` wraps each packed DIB in a BMP envelope and checks
all 137,811 RGB pixels across 1,731 images using installed System.Drawing/GDI+.
It rejects an intentionally wrong expected index, checks packet EOF and totals,
and records the decoder identity and aggregate RGB digest in `windows-oracle.json`.
This verifies pixel interpretation; literal byte comparisons separately prove
preservation of padding, unused bits and reserved bytes.

The Rust integration tests exercise complete document replacement, unchanged
streams, fresh reversal, prefix/trailing-byte rejection, malformed indices and
palettes, exact budgets, source isolation and eight-worker output determinism.
The shipped `rebuild_dib` example needs no new format dispatch: its existing
indexed raster output uses two bytes per pixel, the written marker and index.
`verify_example.py` compares every original/edited archive and raster in two fresh
processes using a temporary directory.

Full local and exact-head remote gates remain required before integration.
These owned fixtures and Windows checks do not establish a native SolidWorks
population for the new forms, SolidWorks application opening, geometric validity
or complete Corliss/schema parity. RLE4, other bit depths, masks and extended
headers remain explicit unsupported forms. No dependency, package version or
release tag changed. Humanizer is unavailable; maintainer prose review remains pending.

Primary references: [palette sizes and row alignment](https://learn.microsoft.com/en-us/windows/win32/gdi/bitmap-header-types),
[packed indices, orientation and omitted sizes](https://learn.microsoft.com/en-us/windows/win32/wmdm/-bitmapinfoheader).
