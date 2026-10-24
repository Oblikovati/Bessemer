# Changelog

## [Unreleased]

### Added

- Added compressed-block carrier decoding and chunk replacement with explicit framing, expansion limits, zlib integrity checks, and preservation of untouched chunk encodings. (#4)
- Added SolidWorks version-4 container reading, checked stream extraction, and stream replacement that preserved untouched compressed bytes. (#1)

### Fixed

- Preserved the original file when a stream replacement matched its original content, including after an intermediate edit. (#5)
