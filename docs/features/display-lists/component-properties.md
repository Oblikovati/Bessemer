# Component-property object graphs

Feature #28 now reads and reconstructs the complete registered
`moComponentProperty` and `moComponentPropertyHolder` bodies in generic CObject
positions within admitted display versions 4850–19000. Earlier archive contexts
remain unsupported. The property pointer has a declared `moComponentProperty`
role; a generic pointer in the holder may be null, an alias, a nested object or a
cycle. These archive checks do not establish application or geometric validity.

Component properties contain 26 embedded font records below `0x13EF`, 30 through
`0x23B2` and 31 afterward. Each reuses the existing CString, line-width and signed
end-cap reader. Entity visibility retains two unsigned words, the raw byte,
unsigned flags, layer word and line width. `all_views` remains a raw byte.
`added_for_envelop` appears at `0x147C`; `empty_prop` appears above `0x3738`.
No native in-memory defaulting or flag normalization changes stored values.

Holders retain their generic pointer, component-name CString, typed property
pointer, ordered unsigned occurrence values and two signed tail words. Occurrence
arrays use `su_CArchive::ReadCount`: a word, or `0xFFFF` followed by a dword. A
following `0xFFFFFFFF` is a large count, not the start of a 64-bit extension.
The older layer manager's distinct MFC 64-bit count remains separate. Existing
object arrays now share their unchanged 16/32-bit count reader with occurrences;
that extraction was committed separately after the archive, light and scene tests.

The independent fixtures contain 40 property roots and 48 holder roots split by
document kind, with 40,702 field expectations. They bracket every admitted
property boundary and include null holders, empty extended occurrence arrays,
noncanonical raw flags, signaling float NaNs, nested line styles, owner cycles,
shared property definitions and independent edits. Each font slot and occurrence
contributes to the shared archive entry budget; aliases do not reload properties
or charge their embedded fonts twice. Reference object-load counts and independent
collection arithmetic pin exact and one-short limits.

The reference's nested font `weight` record is normalized to the existing flat
width field paths before comparison. Semantic expectations and forced bytes are
both checked. The shared UI integration suite now covers 149 roots and 128,607
field expectations. It retains the earlier 61 line-style/layer-map vectors.

Eight component-specific tests cover all body boundaries, raw widths and flags,
shared graph identities, null and declared roles, physical occurrence bounds,
16/32-bit count spelling, independent concurrent edits, every selected prefix,
trailing bytes, whole-document reopening, unrelated stream preservation and
authentic byte reverts. Run `cargo test --test display_ui_objects` and
`python -B -X utf8 docs/features/display-lists/verify_component_properties.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe`.
The process verifier regenerates all 88 roots and requires identical forced
reconstruction from two Rust processes. Native file context is 19000 throughout.

Full clause 13 parity, operation history and whole-document cross-stream
validation remain unfinished. The humanizer skill was unavailable; prose remains
subject to maintainer review.

Validation on Windows x64 with Rust 1.99.0 passed all 16 shared UI integration
tests and `cargo xtask pr`, including 179 documentation tests. All 88 new roots
rebuilt byte for byte in two processes; every earlier root, UI-object, bucket and
file-context process check also passed. The native audit retained 1,086 accepted
documents and 1,849,563 independent field comparisons with unchanged corpus hash
`94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.
Cross-platform CI remains pending the complete feature PR.
