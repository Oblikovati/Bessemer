# Earliest wall and appearance layouts

Feature #28 now admits display versions 4400–19000, excluding the uncounted-window
interval `0x1FAE`–`0x1FCE`. Full documented-schema parity, earlier contexts and
unresolved object bodies remain unfinished. Archive verification does not
establish native application acceptance or geometry validity.

Four conditions control the new stored layouts:

| Layout | Stored condition |
| --- | --- |
| RV axis, shader strings, specular/roughness and bump fields | `version >= 0x1165` |
| RV refraction roughness, finish shader and appearance directions/translations | `version > 0x1179` |
| Face `unknown_68` word | `version >= 0x117D` |
| Legacy wall material pointer before visibility/reflectivity | `version < 0x117F` |

At `0x117F`, walls instead store their visual-property pointer and trailing word
after visibility and reflectivity. Earlier null material pointers are retained
as null CObject tags. All three wall kinds use the same versioned base while
retaining the geometry fields stored for each kind. No material or geometry conversion is
inferred from the native reader's in-memory fixups.

The reference leaves `moPWMaterialProperties_c` bodies unresolved. Non-null
instances return `Unsupported`, with the offending class in the cause, before
an archive is published. An independently encoded negative fixture is rejected
by both the approved sibling and Bessemer. This is a documented exclusion, not
evidence that non-null old wall materials have been implemented.

The positive corpus has 22 complete part/assembly roots and 20 shared property/
holder graphs. The approved sibling consumes all 42 roots to EOF. They supply
15,404 independent field expectations and exact forced reconstruction. The
generator observes reference-reader spans, asserts their primitive shape and
removes only version-absent fields. Old walls receive an independently encoded
null pointer; only the previous visual-pointer tags and the new-layout trailing
word are removed, preserving global object numbering. File context is 19000.

Nine integration tests check every expected field, all four boundaries, every
wall kind and null pointer, cross-version construction, exact and one-less
byte/field/object/entry budgets, all prefixes of nine complete roots, the
unresolved-body failure, caller input mutation and eight concurrent edits against
independent edited bytes. Complete owned version-4 part and assembly documents
carry exact version-4400 markers. Tests resolve document context, edit, reopen,
retain an unrelated stream and restore authentic original bytes.

Two isolated Rust processes reconstruct all 42 roots and both complete owned
documents exactly, including independent context traces. Fixture, source, budget,
negative-case and process evidence is recorded in `earliest-wall-*.json`.

On Windows x64 with Rust 1.99.0, the native audit retained all 1,086 accepted
display streams and 1,849,563 independent field comparisons in two processes.
The display-stream manifest SHA-256 remained
`94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.
The prior 102 light roots also regenerated unchanged and passed their two-process
check.
The local `cargo xtask pr` gate passed, including all 184 documentation tests.

```text
cargo test --test display_earliest_walls
cargo build --example rebuild_display_archives --example rebuild_display_documents
python -B -X utf8 docs/features/display-lists/verify_earliest_walls.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
python -B -X utf8 docs/features/display-lists/verify_earliest_component_documents.py target/pinned-1.99.0/debug/examples/rebuild_display_documents.exe earliest-wall
```

The source conditions are in the approved sibling's `dl_light.py` and
`dl_tess.py`; complete module hashes are retained in the fixture manifests.
The humanizer skill was unavailable; prose remains subject to maintainer review.
