# Changelog

## [Unreleased]

### Added

- Added typed SolidWorks biography archive reading, construction, record editing, and writing, with retained environment observations and explicit repeated-string expansion and abbreviation. (#12)
- Added typed SolidWorks header archive reading, construction, graph editing, and writing, preserving shared external references, unknown fields, embedded version overrides, and exact bounding-box bits. (#10)
- Added typed standalone SolidWorks version-history reading, graph editing, and writing, preserving shared date-code objects, unknown fields, and equivalent-edit byte identity. (#8)
- Added a Linux x86_64 ThreadSanitizer gate that instrumented the library, dependencies, and standard library in the full CI gate. (#8)
- Added compressed-block carrier decoding and chunk replacement with explicit framing, expansion limits, zlib integrity checks, and preservation of untouched chunk encodings. (#4)
- Added SolidWorks version-4 container reading, checked stream extraction, and stream replacement that preserved untouched compressed bytes. (#1)

### Fixed

- Preserved the original file when a stream replacement matched its original content, including after an intermediate edit. (#5)
