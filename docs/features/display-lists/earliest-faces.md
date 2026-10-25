# Earliest face and scene layouts

Feature #28 now admits display versions 4600–19000, excluding the uncounted-window
interval `0x1FAE`–`0x1FCE`. Full documented-schema parity, earlier contexts and
unresolved object bodies remain unfinished. Archive verification does not
establish native application acceptance or geometry validity.

Four documented conditions control independent stored layouts:

| Layout | Stored condition |
| --- | --- |
| Face `unknown_58` count and integer pairs | `version >= 0x1203` |
| Scene `flags` word | `version > 0x1211` |
| Component-property font records | 24 below `0x12A3`, then 25 below `0x12CF` |
| Polyline `unknown_10` word | `version > 0x12B0` |

Absent members remain absent. Construction and edits reject both missing required
fields and fields that the declared version does not store. Face-pair allocation
uses the same aggregate entry budget as the rest of the display archive.

The owned corpus has 22 complete part/assembly roots and 32 property/holder graph
roots. The approved sibling consumes each result to EOF. The 54 roots provide
20,574 independent field expectations and forced byte reconstruction. Positive
face cases store two pairs per face; polylines contain two complete points.
Property graphs retain shared aliases and exact font values, including signaling
NaN payloads. All cases declare native file context 19000.

The generator observes complete reference-reader spans, asserts their expected
primitive shape, and removes only version-absent members. It independently
encodes the polyline and embedded property records. Entry budgets combine the
reviewed parent counts with the actual new point and face-pair counts; object
counts come from the independent reader. Existing frozen vectors remain unchanged.

Eight integration tests compare every expected field, force reconstruction,
check all four boundaries, reject cross-version shapes, enforce exact and one-less
budgets, reject every prefix of seven complete roots, reject an impossible face
pair count, mutate caller input and compare eight concurrent edits with independent
edited bytes. Independent version-4 part and assembly documents have exact 4600
markers and an unrelated stream. Their tests resolve document context, edit,
reopen, preserve the unrelated stream and restore the authentic original file.

Two isolated Rust processes reconstructed all 54 roots and both complete owned
documents exactly, including independent context traces. Reports and source
hashes are in `earliest-face-*.json`. The prior 102 component roots and 88 property
roots also regenerated unchanged and passed their two-process checks.

On Windows x64 with Rust 1.99.0, the native display-stream audit retained all 1,086
accepted streams and 1,849,563 independent field comparisons in two processes.
Its manifest SHA-256 remained
`94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.
The local `cargo xtask pr` gate passed, including all 184 documentation tests.

```text
cargo test --test display_earliest_faces
cargo build --example rebuild_display_archives --example rebuild_display_documents
python -B -X utf8 docs/features/display-lists/verify_earliest_faces.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
python -B -X utf8 docs/features/display-lists/verify_earliest_component_documents.py target/pinned-1.99.0/debug/examples/rebuild_display_documents.exe earliest-face
```

The source conditions are in the approved sibling's `dl_tess.py`, `dl_light.py`
and `dl_doc.py`. Their complete hashes are retained in the fixture manifests.
The humanizer skill was unavailable; prose remains subject to maintainer review.
