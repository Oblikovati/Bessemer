# Shaded display versions 10000 and 11000

Issue #28 remains open. Complete-stream reading, construction and isolated edits
now support display versions 10000 and 11000, in addition to 18000–19000, with
explicit file-version context 19000. The two new versions are selected snapshots,
not an inferred supported range between 10000 and 18000.

The implementation handles the older field omissions and class tails:

| Record | Older wire behavior |
| --- | --- |
| Root | No original font scale, envelope options or ink-view dimensions. Configuration count is present for assemblies but absent for parts. |
| Material | No PLM ID, second texture or DSPBR presence word. Textures omit the C-thread integer. Graphic mesh color is present at 11000, absent at 10000. |
| Part tessellation | No configuration ID/name, graphics manager, bounding box, reference collector or thread-display byte. |
| Body and face | The body tail integer appears at 11000. Both versions omit face-string framing and later persistent-ID fields. |
| Viewer component | Configuration-description and fixed-state tails differ between the snapshots. Neither has the later graphics, reference-geometry, tessellation-number or routing tails. |
| Scene | No HDR exposure, tone-map setting or field of view. |
| Display items | No table 3D-view string, nested symbol list or later annotation strings/ID. A minimal annotation subrecord is 25 bytes. |

Older view-bucket bodies were unsupported at this checkpoint; the subsequent
[legacy bucket work](legacy-buckets.md) implements them for the four supported
older snapshots. Populated configuration,
decal and other previously unresolved bodies remain explicit failures; the reader
does not skip or retain opaque remainders to claim completion.

Six [independently assembled roots](old-shaded.json) cover both document kinds at
both versions, an independently encoded material edit and an unshaded root whose
final annotation contains three minimal subrecords. The shaded fixtures include
inline facets from the independent clause-12 wire builder, a textured material,
all three scene-wall shapes, shared material references and a self-owner assembly
cycle. They match 1,978 material, scene, light, item and component scalar fields.
Every complete stream reconstructs exactly through forced construction.

Seven integration tests cover those oracles, conditional field presence, old
annotation sizes, raw values, exact aggregate entry/object/field/byte budgets,
unresolved bucket rejection, owner identity, every prefix of an older shaded
part, malformed counts, source ownership, container replacement/reopen/revert and
eight concurrent edits. Container replacement uses explicit fixture context;
cross-stream model validity and native application acceptance are not established.

The [native audit](archive-audit.json) reconstructs 1,058 of 1,086 streams, including
all 52 version-10000 and the single version-11000 sample. Both processes produce
identical bytes, diagnostics and combined traces, with no previously accepted
paths lost. Independent checks cover 238,667 reference-geometry, 183,897 scene,
203,106 root display-item, 234,395 view-bucket, 61,742 light and 833,169 material
fields. Corpus SHA-256 remains
`94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.

The remaining native failures are five version-7000 and 23 version-9000 streams.
Those layouts, other bucket contexts, unexercised schema branches, other file contexts,
whole-document context resolution and validation, full model archives, geometric
validity and native application acceptance remain required work. The corpus count
does not establish complete schema parity.

Local validation used Windows x64 and Rust 1.99.0 MSVC. The relevant commands are:

```text
python -B -X utf8 docs/features/display-lists/generate_old_shaded.py
cargo test --test display_old_shaded
cargo build --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/audit_archive_corpus.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
cargo xtask pr
```

The manifest records the evolving reference source hashes. The maintainer-side
humanizer skill is unavailable; its prose pass remains for review.
