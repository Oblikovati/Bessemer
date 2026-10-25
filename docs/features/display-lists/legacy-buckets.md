# Buckets at the older supported snapshots

Issue #28 remains open. View buckets now read, construct, edit and write at display
versions 7000, 9000, 10000 and 11000, with explicit file-version context 19000.
These are the selected older snapshots already supported by the complete root
reader. This checkpoint does not enable untested intermediate version contexts.

The older serializers use float32 polygon coordinates and separate signed
integer flags rather than the later float64 coordinates and packed flag word.
Those integers retain their raw noncanonical values. The comparison trace derives
the reference flags separately, including implicit visible/printable layers for
positive polygons and the different early-return behavior of zero/negative
point counts. No synthesized flag word is inserted into the stored archive.

Component string-map keys are 16-bit at versions 7000/9000 and signed 32-bit at
10000/11000. Both MFC count spellings, duplicate keys and strings remain intact.
Thread-map edge objects use their earlier node fields and omit the later
persistent IDs and flags. Detailing-mode originals, model display-item tails and
later state fields are absent. The surface-cut field is absent at 7000; hatch-set
information appears at 11000 among these snapshots.

Main polygon lists use the bucket's 2D/3D setting. Thread and repair lists use the
restored enclosing 3D state. Dimensions are passed explicitly, with no global or
thread-local state. The existing line-property, type-array, object-map and
coordinate-budget machinery is shared across the old and modern layouts.

Ten [independently encoded complete roots](legacy-buckets.json) cover all four
versions in both dimensions, an independently encoded smooth-flag edit and an
empty bucket. They match 1,751 semantic fields, including interpreted flags and
signaling NaN coordinates, and reproduce every byte through forced construction.
They also contain heterogeneous thread keys, a self alias, a nested empty bucket,
nonpositive polygon/list/type counts and noncanonical boolean values.

Eight tests cover those oracles, raw flag values and closed-form flag masks,
coordinate scopes, key widths, omitted fields, exact entry/object/field/byte
budgets, every prefix of a populated older root, malformed counts, changed
coordinate dimensions, alias identity, completed-empty behavior, whole-container
replacement/reopen/revert, source ownership and eight concurrent edits. The
existing incomplete-body regression now expects an invalid body instead of an
unsupported dialect; positive fixtures establish that the dialect is implemented.

Modern owned bucket regeneration produced the same bytes, traces and manifest.
The [native audit](archive-audit.json) still reconstructs all 1,086 reference-readable
part/assembly streams exactly in both processes and checks the same 1,849,563
semantic fields. The native corpus has no buckets at these four older versions,
so the new layout coverage comes from the independent owned fixtures rather than
a larger native sample count. Corpus SHA-256 remains
`94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.

Remaining version/caller contexts, populated decals/configurations and other
unexercised bodies, context resolution, ordered validation/history, model
archives, geometric validity and native application acceptance remain required
work. Wire reconstruction does not establish valid CAD models or full schema
parity. Integration still requires the remaining #28 gates and cross-platform CI.

Local verification used Windows x64 and Rust 1.99.0 MSVC:

```text
python -B -X utf8 docs/features/display-lists/generate_legacy_buckets.py
cargo test --test display_legacy_buckets
cargo test --test display_buckets --test display_old_shaded
cargo build --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/audit_archive_corpus.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
cargo xtask pr
```

The fixture manifest records the evolving reference source hashes. The
maintainer-side humanizer skill is unavailable; its prose pass remains for review.
