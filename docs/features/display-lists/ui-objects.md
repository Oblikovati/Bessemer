# Shared line-style and layer-manager objects

Feature #28 now reads and reconstructs the documented `moLineStyle_c` and
`uiLayerMgr_c` bodies in generic CObject positions. Declared object roles still
reject an unrelated runtime class. Their presence in a generic scene list does
not establish native application or geometric validity.

Line styles retain both CString names, the full unsigned 16-bit segment count,
every stored double and the raw thick/thin byte. Layer maps retain ordered,
duplicate signed keys, empty and Unicode names, orphan attribute keys and raw
signed attribute values. The archive does not apply the native in-memory name
filter or map old nonzero attributes to 3; those transformations would lose bytes.

At version `0x1BE5`, layer counts change from MFC ReadCount to signed 32-bit
nonnegative counts. Earlier counts preserve their short, extended 32-bit and
extended 64-bit spellings. The 64-bit payload stores its low word first, unlike
the separate high-word-first archive formats. Physical minimum widths and the
shared aggregate budget are checked before iterating or publishing a result.

The owned fixtures contain 25 line-style roots and 36 layer-map roots with
87,905 independent field expectations. They cover parts and assemblies at 5000,
`0x1BE4`, `0x1BE5` and 19000; empty objects; independent edits; all count spellings;
and the full 65,535-segment line-style count. Exact double bits include signaling
NaNs, signed zero, infinities and a subnormal. Native file context is explicitly
19000 throughout.

Each new object replaces the independently named child in a complete scene's
CObject list. Floor selection then aliases that definition. The generator consumes
the modern source completely before applying disjoint replacements and validates
the final target stream with the reference reader. Generic aliases and
revision-scoped handles resolve to the same loaded definition. Scene oracles
normalize layer tuple entries to the codec's explicit key/value field paths.

Independent collection arithmetic pins exact limits for all 61 roots. The native
19000 part base has 134 entries from its existing inline-shaded fixture plus
nine scene entries; the assembly base has 65 entries, two 37-entry inline facets
and nine scene entries. UI objects then add their segment or map entries.
Historical bases reuse their reviewed complete-root arithmetic. Reference object
load counts are retained separately in `ui-object-budgets.bin`.

Eight focused integration tests compare every independent field and forced byte,
check exact and one-short limits, preserve count spelling, reject huge counts,
reject wrong declared roles, reject every small-root prefix and trailing bytes,
exercise concurrent isolated edits, reopen complete documents, preserve unrelated
streams and restore authentic bytes after reverting.

Run `cargo test --test display_ui_objects` and
`python -B -X utf8 docs/features/display-lists/verify_ui_objects.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe`.
The latter regenerates frozen roots and requires identical output from two Rust
processes. Full documented-schema parity remains unfinished.

The humanizer skill was unavailable; prose remains subject to maintainer review.

Validation on Windows x64 with Rust 1.99.0 passed all eight focused tests and
`cargo xtask pr`, including 179 documentation tests. All 61 new roots rebuilt
byte for byte in two processes, and all earlier root, bucket and file-context
process checks passed. The native audit retained 1,086 accepted documents and
1,849,563 independent field comparisons with unchanged corpus hash
`94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.
Cross-platform CI remains pending the complete feature PR.
