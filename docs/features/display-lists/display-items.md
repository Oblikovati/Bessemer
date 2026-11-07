# Modern display items

Issue #28 remains open. This checkpoint implements the version-19000
`SerializeElement` payload switch and connects it to each root view's display-item
list. The caller supplies display version, file version and document kind; other
version contexts remain unsupported by the complete-stream reader.

All 32 payload-bearing item kinds have typed fields: points, curves, lines,
polylines, polygons, text, fonts, pens, hyperlinks, symbols, tables, property
records, XML markers, annotation records, pipe splines and the remaining scalar
items. Recursive symbols use the operation's existing scope and entry limits.
Both MFC ReadCount spellings are retained.

The reference switch consumes no payload for its default arm, including unknown
item tags. Its `size_of_element` word is a retained observation rather than a byte
length. Bessemer follows these rules, preserving tags and raw size words without
using them to skip bytes. Every accepted archive must still consume the complete
stream and reproduce its bytes through forced construction.

The [independently encoded fixtures](item-vectors.json) each contain 48 top-level
items, including payload-free tags. They cover both text-width branches, 3D
transforms, nested and empty symbol lists, optional table identifiers, signed
zero, infinity, noncanonical flags, supplementary Unicode and an independently
encoded point-style edit. Three fixtures match 1,265 semantic scalar fields;
compact-transform fields are expanded for comparison. Width buffers retain each
four-byte word. Text width length follows the approved reader's decoded-character
count, including supplementary characters.

Six integration tests check those fields, exact forced rebuilds, untouched
document streams, edit/reopen/revert, every prefix through the inserted view,
invalid counts and conditional shapes, exact entry/field/byte budgets, source
ownership and eight concurrent edits.

The [native audit](archive-audit.json) now reconstructs 961 of 1,086 complete
reference-readable streams. It checks 215,049 reference-geometry fields, 157,014
scene fields and 174,936 display-item fields against the independent reader.
Both processes produced identical reconstructed bytes, diagnostics and semantic
traces, with no previously accepted paths lost. The unchanged corpus SHA-256 is
`94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.

The remaining first failures are 18 modern view buckets, four modern spotlights
and 103 earlier-version streams. Later failures may emerge after those bodies
are implemented. Cross-stream validity, geometric validity and native application
acceptance remain separate unfinished work; these checks establish archive
interoperability only.

```text
python -B -X utf8 docs/features/display-lists/generate_item_vectors.py
cargo test --test display_items
cargo build --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/audit_archive_corpus.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
cargo xtask pr
```

The source hashes in the fixture manifest identify the evolving reference files.
The maintainer-side humanizer skill is unavailable; its prose pass remains for
review.
