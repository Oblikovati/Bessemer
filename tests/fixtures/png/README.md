# Owned PNG vectors

These images were generated independently from PNG wire primitives by
[generate_fixtures.py](../../../docs/features/png-previews/generate_fixtures.py).
They contain no installed sample images. Regenerate them with:

```text
python docs/features/png-previews/generate_fixtures.py
```

`cases.txt` lists 120 combinations: fifteen legal color/depth pairs, two interlace
modes, and four image sizes. Every image has matching `.samples` and `.rgba` files
of big-endian u16 values derived from closed-form pixel expectations. The encoded
images mix all five filters, split zlib across IDATs, include an empty IDAT, and
retain safe and unsafe unknown ancillary chunks. Indexed cases include palettes
and transparency; grayscale and RGB cases include transparency keys.
`default-image-apng.png` adds one-frame animation controls to a valid base PNG;
tests retain these opaque chunks while decoding the default image.
