# Display versions 7000 and 9000

Issue #28 remains open. The complete-stream codec now accepts the selected
version-7000 and version-9000 contexts alongside the existing supported versions,
with explicit file-version context 19000. Unlisted intermediate versions remain
unsupported rather than being inferred from nearby sample layouts.

At version 7000, named, spatial and annotation view collections are absent from
the wire. Their published fields contain `None`, preserving the distinction from
a present empty collection. The environment tail integer, body string and unit
triple are absent too. Both new contexts omit the scene tooltip, later material
integer and grouped component-folder tail. Parts use the earlier decal-list
framing. The codecs retain all remaining raw values without supplying missing
wire fields from defaults.

Older display items now honor the table, simple-text and pipe-spline gates.
Version 7000 annotation starts contain only their two base integers. Version 9000
has annotation subrecords whose minimum size is 24 bytes, before the later string
was introduced. Annotation-end records retain their integer at both versions.
Older view-bucket bodies were unsupported at this checkpoint; the subsequent
[legacy bucket work](legacy-buckets.md) implements the four older supported snapshots.

Seven [independently encoded fixtures](early-shaded.json) cover shaded parts and
assemblies at both versions, an independently encoded material edit and unshaded
roots. The version-9000 unshaded fixture ends with five minimal annotation
subrecords, pinning their physical count bound. The fixtures include textured
materials, inline facets, shared identities, all wall shapes and populated
version-9000 spatial views. They match 2,371 independently decoded scalar fields
and reconstruct every byte through forced construction.

Seven tests verify those oracles, explicit collection presence, early item
fields, exact entry/object/field/byte limits, every prefix of both unshaded roots,
malformed counts, shape changes between contexts, container replacement/reopen/
revert, source ownership and eight concurrent edits. Fixture context is supplied
explicitly; these checks do not establish cross-stream model validity or native
application acceptance.

The [native audit](archive-audit.json) now reconstructs all **1,086 of 1,086**
reference-readable part/assembly streams, including the final five version-7000
and 23 version-9000 samples. Both processes produce identical reconstructed
bytes, diagnostics and combined traces, and no previously accepted path was lost.
Independent field counts are 238,667 reference-geometry, 191,971 scene, 268,294
root display-item, 234,395 view-bucket, 63,310 light and 852,926 material fields:
1,849,563 comparisons in total. Corpus SHA-256 remains
`94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.

This completes the native corpus reconstruction gate. It does not complete the
documented schemas. Other bucket contexts, populated decals/configurations and other
unexercised bodies, other version contexts, whole-document context resolution,
ordered validation/history, full model archives, geometric validity and native
application acceptance remain required work. Integration still requires the
remaining schema, validation and CI gates for #28.

The shared fixture builders were extended without changing the earlier owned
scene or version-10000/11000 baselines. Their regeneration matched the recorded
bytes and manifests unchanged. Fixture framing was extracted into a shared test
module in a separate structural commit before adding the new tests.

Local verification used Windows x64 and the pinned Rust 1.99.0 MSVC toolchain:

```text
python -B -X utf8 docs/features/display-lists/generate_early_shaded.py
cargo test --test display_early_shaded
cargo build --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/audit_archive_corpus.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
cargo xtask pr
```

The manifest records the evolving reference source hashes. The maintainer-side
humanizer skill is unavailable; its prose pass remains for review.
