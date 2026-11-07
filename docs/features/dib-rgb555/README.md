# RGB555 packed DIB previews

Feature #210 adds 16-bit BI_RGB reading and raw-word replacement under SolidWorks
clause 8.1 and its referenced Windows DIB contract. This is DIB behavior version 4;
the earlier formats retain their existing output. The common row-layout extraction
was committed separately and passed unchanged owned fixtures before format admission.

`decode_bgr555_words(maximum_pixels)` returns top-to-bottom `u16` values.
The low five bits are blue, the next five green, and the next five red; the high
bit is retained without alpha semantics. `with_bgr555_words(words, maximum_pixels)`
replaces every stored word, including a caller-selected high bit, while preserving
row padding, palette bytes and header observations. No rendered-color conversion
occurs in these APIs. The existing `DibPixels` enum remains unchanged;
`decode_pixels` directs RGB555 callers to the dedicated word method with a typed error.

Both height signs, odd/even widths, optional optimization palettes and explicit
or zero image sizes are supported. Zero sizes derive an exact extent from checked
dimensions and remain zero through edits and fresh reversals. Every raw `u16`
value is retained. Other compression modes, masks and extended headers remain
explicit unsupported forms.

```rust
# fn example(preview: &bessemer::preview::DibPreview) -> bessemer::Result<()> {
let mut words = preview.decode_bgr555_words(16_000_000)?;
words[0] ^= 0x8000;
let edited = preview.with_bgr555_words(&words, 16_000_000)?;
let bytes = edited.to_bytes()?;
# Ok(())
# }
```

`rebuild_dib INPUT OUTPUT RASTER` emits two little-endian bytes per RGB555 word,
including its high bit. Existing indexed, BGR24 and BGRX raster encodings stay
unchanged. Run the shared packet verifier with `--family rgb555` to check the
RGB555 caller; its default still checks the indexed RGB corpus.

`generate_fixtures.py` independently assembles 136 original/edited pairs. The 128
small cases cross eight widths, one/two-row heights, both orientations, optional
palettes and explicit/zero sizes. Eight 256-by-256 cases each contain all 65,536
raw words. Each edit flips one bit in every color channel and the unused high bit;
the generator checks that the whole archive differs by exactly four bits per pixel.
Original and edited literals retain padding and metadata independently of Rust.

Installed Windows System.Drawing/GDI+ checked all 1,053,904 pixels across 272 images.
The native oracle verified every five-bit channel level, rejected an intentionally
wrong color word and confirmed that toggling the unused high bit does not change
displayed RGB. Bit replication in the verifier describes that observed Windows
display conversion; the production API preserves raw words. The preliminary
channel probe's decoded bitmap view was 32-bit, which does not change the input
DIB's stored 16-bit header or establish a different wire encoding.

Rust tests cover literal word/byte reconstruction, no-op edits, fresh reversal,
input mutation after publication, all proper prefixes, malformed extents,
dimensions/palettes/compression, budgets, source isolation, eight-worker output
determinism and whole-document reopening with every other stream unchanged.
The earlier unsupported-depth mutation now uses two bits: 16-bit BI_RGB is admitted
by this feature and has its own complete positive corpus. Earlier fixture bytes
and their expected decoded values were not changed.

Full local and exact-head remote gates remain separate requirements before
integration. These owned-format and Windows decoder results do not establish a
native SolidWorks RGB555 population, SolidWorks application opening, geometric
validity or complete Corliss/schema parity. No dependency, package version or
release tag changed. Humanizer is unavailable; maintainer prose review remains pending.

Primary reference: [Windows RGB555 storage](https://learn.microsoft.com/en-us/windows/win32/wmdm/-bitmapinfoheader).
