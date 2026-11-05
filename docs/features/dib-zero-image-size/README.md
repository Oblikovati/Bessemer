# Omitted uncompressed DIB image sizes

Bug #208 corrects the existing BGRX reader. A complete uncompressed image with
`biSizeImage = 0` was rejected because the decoder consumed zero pixel bytes.
The reader now derives the exact extent from checked dimensions. Explicit image
sizes must still match the raster; RLE8 still requires its declared byte extent.
Edits retain the zero declaration, palette, metadata and raw fourth pixel bytes.

SolidWorks clause 8.1 references the Windows DIB header. Its observed RLE8 table
uses explicit image lengths. Microsoft's [BITMAPINFOHEADER contract](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfoheader)
permits an omitted image length for uncompressed RGB. For supported 32-bit packed
pixels, the dimension-derived extent is `width * abs(height) * 4`. This does not
admit other bit depths, compression methods, masks or headers.

`generate_fixtures.py` independently assembles eight literal archives and expected
top-to-bottom BGRX rasters using Python wire primitives. Widths one and three,
both height signs and zero/two-entry optimization palettes cover 32 pixels.
`verify_windows_oracle.ps1` adds a BMP envelope without changing the DIB, then
checks every RGB pixel with installed Windows System.Drawing/GDI+.
`windows-oracle.json` records that successful comparison and decoder identity.
The native decoder does not supply an oracle for the unused fourth bytes; Rust
tests compare those bytes directly with the literal fixtures.

`tests/preview_rgb_zero_size.rs` checks forced reconstruction, equivalent edits,
pixel replacement, fresh reversal, exact extents, budgets, malformed payloads,
source isolation, concurrent output and complete Preview stream replacement in
an owned part document. This is owned-format evidence, not evidence of a native
SolidWorks zero-size population or acceptance by the SolidWorks application.

The pre-fix one-pixel expected-success probe exited 101 against revision
`9136b07a7b1dc463e22b3888203baf45597ccc7b`. Native GDI+ decoded that image to
RGB 86/52/18, matching its literal BGR bytes. Full local and remote gates must
include the fix before integration; earlier green gates do not cover it.
