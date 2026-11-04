# 24-bit packed DIB previews

Feature #207 adds complete BI_RGB24 storage and pixel edits under SolidWorks
clause 8.1 and its referenced Windows DIB contract. The extension uses DIB
behavior version 2; existing RLE8/BGRX results retain version 1 behavior.
No package version or release tag changed.

`DibPreview::from_bytes`, `new`, `with_contents` and `to_bytes` preserve the full
header, optional optimization palette, pixel bytes and nonzero row padding.
Positive heights store bottom-up rows; negative heights store top-down rows.
Rows occupy a multiple of four bytes. Explicit image lengths must equal that
complete raster extent. A zero declaration derives the extent from checked
dimensions and remains zero after edits and fresh reversals.

`decode_bgr24_pixels(maximum_pixels)` returns top-to-bottom `[blue, green, red]`
triples. `with_bgr24_pixels(pixels, maximum_pixels)` edits only those triples,
leaving padding and all other stored observations unchanged. Both enforce the
pixel budget. `decode_pixels` retains its existing `Indexed`/`Bgrx` result type
and returns a typed unsupported error directing BGR24 callers to the dedicated
method. This avoids inventing a fourth byte or breaking enum matches.

```rust
# fn example(preview: &bessemer::preview::DibPreview) -> bessemer::Result<()> {
let mut pixels = preview.decode_bgr24_pixels(16_000_000)?;
pixels[0] = [0, 128, 255];
let edited = preview.with_bgr24_pixels(&pixels, 16_000_000)?;
let bytes = edited.to_bytes()?;
# Ok(())
# }
```

`rebuild_dib INPUT OUTPUT RASTER` forces archive reconstruction and emits three
bytes per BGR24 pixel. Its previous RLE8/BGRX raster encodings remain unchanged.

`generate_fixtures.py` assembles 64 original/edited pairs independently of Rust:
widths one through four, one/two-row heights, both orientations, zero/two-entry optimization palettes,
explicit/omitted sizes, raw metadata and all four possible padding widths.
The native Windows oracle checked all 480 RGB pixels across those 128 images.
The shared verifier also reran the eight Bug #208 cases with its original defaults.
Literal whole-stream comparisons independently check padding and reserved bytes;
the native decoder does not establish their preservation.

The Rust tests cover reconstruction, no-op edits, literal changed bytes, fresh
reversals, malformed prefixes/extents, dimensions, palettes, unsupported formats,
budgets, isolation, concurrency and complete Preview replacement in the owned
part document. `verify_example.py` checks the shipped executable's output against
the literal archives and rasters using a temporary directory.

These are owned-format and Windows decoder checks. They do not establish an
installed SolidWorks BGR24 population, SolidWorks application opening, geometric
acceptance, or full schema parity. Full local and exact-head remote gates remain
separate requirements before integration.

Primary references: [DIB color tables and alignment](https://learn.microsoft.com/en-us/windows/win32/gdi/bitmap-header-types),
[BGR triples and omitted lengths](https://learn.microsoft.com/en-us/windows/win32/wmdm/-bitmapinfoheader).
