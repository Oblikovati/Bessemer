# Changelog

## [Unreleased]

### Added

- Added lossless display-list transform and view codecs, plus part/assembly archive reading, validated edits and writing for display versions 7000, 9000, 10000, 11000 and 18000–19000 with shared object identities, inline facets, external face references, all seven reference-geometry kinds, versioned appearance and scene layouts, display-item payloads, versioned view buckets, graphics-reference graphs, markup-ink grids, per-configuration data and all five old-light classes. Complete display-list schema support remained under development. (#28)

- Added embedded SolidWorks Inspection project XML reading, construction and writing, with preserved unknown content and a complete drawing Job Number edit example. (#26)

- Added lossless namespace-aware XML metadata reading, construction, section-scoped property views, immutable text and attribute edits, and writing for all 17 documented stream families, with bounded parsing and preserved Unicode and keyword framing. (#22)

- Added static PNG preview reading, typed raster construction, sample editing, and writing, with exact chunks, CRC and zlib checks, all color/depth pairs, Adam7, bounded metadata expansion, and ancillary copy rules. (#20)
- Added packed DIB preview reading, construction, palette and pixel editing, and writing, with checked RLE8 and BGRX decoding, retained raw fields, explicit unwritten pixels, and allocation limits. (#18)
- Added typed face tessellation directory, block, and facet reading, construction, editing, and writing, preserving version-dependent fields, raw arrays, exact floating-point bits, and component links. (#16)
- Added typed Parasolid binary transmit partition and delta reading, construction, field editing, and writing, with explicit base schemas, both byte orders, layout edits, pointer flags, and exact floating-point bits. (#14)
- Added typed SolidWorks biography archive reading, construction, record editing, and writing, with retained environment observations and explicit repeated-string expansion and abbreviation. (#12)
- Added typed SolidWorks header archive reading, construction, graph editing, and writing, preserving shared external references, unknown fields, embedded version overrides, and exact bounding-box bits. (#10)
- Added typed standalone SolidWorks version-history reading, graph editing, and writing, preserving shared date-code objects, unknown fields, and equivalent-edit byte identity. (#8)
- Added a Linux x86_64 ThreadSanitizer gate that instrumented the library, dependencies, and standard library in the full CI gate. (#8)
- Added compressed-block carrier decoding and chunk replacement with explicit framing, expansion limits, zlib integrity checks, and preservation of untouched chunk encodings. (#4)
- Added SolidWorks version-4 container reading, checked stream extraction, and stream replacement that preserved untouched compressed bytes. (#1)

### Fixed

- Preserved the original file when a stream replacement matched its original content, including after an intermediate edit. (#5)
