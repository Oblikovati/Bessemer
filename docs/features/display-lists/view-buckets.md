# Modern view buckets

Issue #28 remains open. This checkpoint implements version-19000 `uiViewBucket_c`,
its inline `gcBucket_c` bodies and `gcBucketEdgeData_c` objects reached through
generic thread-map keys. All serializers consume typed fields and require a
complete stream; unresolved new object classes remain unsupported.

The codec retains polygon flags, nonpositive empty-list counts, duplicate map
keys, MFC ReadCount framing, optional transforms, detailing-mode originals,
sub-bucket identities, component strings, thread maps, hatch information and
embedded display items. Polygon coordinates, line properties, type-dependent
arrays, arc records and persistent identifiers preserve their stored bits.

The reference restores its enclosing coordinate state before reading thread maps
and repair polygons. Modern complete roots start in 3D. The implementation passes
each list's dimension explicitly: the main lists use their bucket's 2D flag;
thread and repair lists use the restored 3D state. It keeps no global or
thread-local coordinate state. This statement covers the supported modern root
layout, not unimplemented serializers or earlier versions.

Four [independently encoded fixtures](bucket-vectors.json) cover 2D, 3D, empty and
edited buckets. They match 993 semantic fields, including expanded transforms,
and reproduce all stream bytes through forced construction. The populated 2D
fixture contains 3D thread and repair polygons, all payload-bearing polygon type
branches, an edge-data object, a self alias, a nested bucket, a detailing-mode
original and an independently encoded display-mode edit. Negative counts remain
raw fields rather than being normalized to zero.

Eight integration tests verify those fields and bytes, raw count/flag framing,
coordinate scopes, nonpositive empty collections, aliases and declared pointer
roles, unknown generic classes, exact aggregate entry/object/field/byte limits,
malformed counts, conditional shape changes, every new prefix, whole-document
edit/reopen/revert, source isolation and eight concurrent edits.

The [native audit](archive-audit.json) reconstructs 979 of 1,086 reference-readable
streams. It independently checks 218,275 reference-geometry fields, 157,014 scene
fields, 176,509 root display-item fields and 234,395 view-bucket fields. Two
processes produce identical reconstructed bytes, diagnostics and combined traces.
All previously accepted paths remain accepted, and the corpus SHA-256 remains
`94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.

The remaining first failures are four modern spotlight objects and 103
earlier-version streams. More unsupported bodies may emerge after those readers
are added. Full display-list parity, cross-stream validation, geometric validity
and native application acceptance remain unfinished. The fixtures establish wire
interoperability and do not claim valid CAD models.

```text
python -B -X utf8 docs/features/display-lists/generate_bucket_vectors.py
cargo test --test display_buckets
cargo build --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/audit_archive_corpus.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
cargo xtask pr
```

The manifest records the evolving reference source hashes. The maintainer-side
humanizer skill is unavailable; its prose pass remains for review.
