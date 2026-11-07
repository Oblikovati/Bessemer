# Earliest body-property layouts

Feature #28 now admits display versions 4300–19000, excluding the uncounted-window
interval `0x1FAE`–`0x1FCE`. Full documented-schema parity and earlier contexts
remain unfinished. Non-null legacy wall-material bodies remain explicitly
unsupported. Archive checks do not establish native application acceptance or
geometry validity.

The new conditions cover body-property representation and independent state words:

| Layout | Stored condition |
| --- | --- |
| Part `unknown_168` and assembly `unknown_370` words | `version >= 0x10F0` |
| Body-property map values | Direct visual-property pointers below `0x10F6`; `uoBodyPropInfo_c` pointers afterward |
| Component `is_virtual_comp` word | `version > 0x110D` |
| Table OLE-list selector and optional unsigned number array | `version > 0x1115` |

An old map stores each key and its visual-property pointer directly. It does not
store a wrapper CObject or wrapper flags. The flat field path ends in
`body_prop_infos[i].value.viz_prop`; there is no synthetic `value` object tag.
Later maps retain the actual wrapper object and its property fields. Construction
and edits must match the declared version in both directions.

The 26 complete owned part/assembly roots contain direct visual aliases, duplicate
keys, null map entries, wrapped maps, populated OLE arrays and independent ambient
edits. They provide 8,303 independent field expectations and forced byte
reconstruction. File context is explicitly 19000. Every root is consumed to EOF
by the approved sibling reader.

The generator observes complete object boundaries, removes one wrapper's class
and object slots for old layouts, and independently encodes the direct map.
Surviving later class and object references are relocated through the existing
MFC tag adapter. Scene name, merged-list and floor-selection aliases prove that
later references retain identity. Loaded-object counts decrease by one; duplicate
map keys still count as separate entries. Null-only cases retain one real map
entry without creating an object.

Nine integration tests check every expected field, aliases and nulls, the absence
of synthetic objects, each version condition, cross-version construction failures,
exact and one-less budgets, all prefixes of nine complete roots, an impossible OLE
count, caller source mutation and eight concurrent independent edits. Complete
owned version-4 part and assembly documents carry exact version-4300 markers.
Tests resolve their context, edit, reopen, preserve an unrelated stream and restore
the authentic original bytes.

Two isolated Rust processes reconstruct all 26 roots and both complete owned
documents exactly, including independent context traces. Fixture, budget, source
and process evidence is recorded in `earliest-body-*.json`.

On Windows x64 with Rust 1.99.0, the native audit retained all 1,086 accepted
display streams and 1,849,563 independent field comparisons in two processes.
The display-stream manifest SHA-256 remained
`94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.
The local `cargo xtask pr` gate passed, including all 184 documentation tests.
The prior 42 wall roots and the shared-font fixture also regenerated unchanged;
the wall roots passed their two-process reconstruction check.

```text
cargo test --test display_earliest_bodies
cargo build --example rebuild_display_archives --example rebuild_display_documents
python -B -X utf8 docs/features/display-lists/verify_earliest_bodies.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
python -B -X utf8 docs/features/display-lists/verify_earliest_component_documents.py target/pinned-1.99.0/debug/examples/rebuild_display_documents.exe earliest-body
```

The source conditions are in the approved sibling's `dl_tess.py` and `dl_doc.py`;
complete module hashes are retained in the fixture manifests. The humanizer skill
was unavailable; prose remains subject to maintainer review.
