# Owned DIB vectors

These synthetic packed DIBs were assembled independently from wire primitives by
[generate_fixtures.py](../../../docs/features/dib-previews/generate_fixtures.py).
Regenerate them with:

```text
python docs/features/dib-previews/generate_fixtures.py
```

`rle8.bin` has encoded and absolute runs, odd-length padding, delta skips, EOL,
EOB and retained suffix bytes. Tests specify its top-down raster manually.
`default-palette.bin` has an implicit 256-entry palette and one unwritten pixel.
The two BGRX fixtures store the same pixels in opposite row orientations, with
nonzero fourth bytes and an optional optimization palette. The existing owned
blank part supplies the complete-document tests.
