# Earliest component layouts

The component work admitted display versions 4800–19000, excluding the uncounted-window
interval `0x1FAE`–`0x1FCE`. Full schema parity, earlier contexts and unresolved
object bodies remain unfinished. Archive checks do not establish native
application acceptance or geometry validity.

The [earliest face work](earliest-faces.md) later extended the lower bound to 4600.

The new layouts retain field absence at each documented boundary:

| Field or layout | Stored condition |
| --- | --- |
| Component reference, spool reference and description | `version > 0x12C3` |
| Component-property embedded fonts | 25 below `0x12CF`, then 26 below `0x13EF` |
| Component suppression | `version > 0x12D0` |
| Component type | `version > 0x12D3` |
| Extra decal-face floats and word | `version > 0x12DB` |
| Polygon BOS flag | `version > 0x12DD` |
| Component dependency names | `version > 0x12DF` |
| Component equation dependencies | `version > 0x12E2` |
| Component folder flag | `version > 0x12E8` |

Construction and edits must match the declared version's stored shape. Missing
members are not synthesized. The existing decal codec already had its older
face-field condition; populated cases now exercise it in admitted complete roots.

The owned corpus contains 48 part/assembly roots, 32 shared property/holder graphs,
12 populated or empty 2D/3D bucket roots and 10 populated decal roots. All 102 roots
are independently encoded or converted from observed reference-reader spans.
The approved sibling consumes every result to EOF. They provide 36,978 independent
field expectations, including exact float bits, stored IDs, arrays and aliases.
Prior fixtures remain unchanged. Native file context is explicitly 19000.

Component conversion asserts the original empty dependency arrays before removing
their count words. Independent populated lists are then inserted on both sides of
the dependency boundaries where fields exist. Equation maps retain duplicate keys
and both populated and empty value arrays. An independently encoded ambient edit
checks that those arrays survive an unrelated change.

Ten integration tests check exact fields and forced bytes, each boundary, both
directions of cross-version construction, exact allocation limits and one-less
failures, all prefixes of seven complete component roots, impossible dependency
counts, source mutation, authentic reverts and eight concurrent independent edits.
The new bucket oracle retains signaling NaNs directly from archived point bytes,
avoiding Python's quieting float promotion. Wire payloads and Rust float behavior
are unchanged.

The property generator also reproduced #35's signaling-NaN trace failure under
Python's hardware promotion. It now preserves those payloads before tracing.
All 88 previously frozen property fixtures regenerated unchanged, including their
traces, and passed their two-process check. The 16 earlier scene fixtures also
retained exact independent reconstruction.

Two independently built version-4 documents carry exact version-4800 markers and
an unrelated stream. Tests resolve context from the document, make the independent
ambient edit, reopen it, check the unrelated stream and restore the authentic
original file. Two isolated Rust processes also reconstruct both complete files
with exact independent context traces. All 102 standalone roots match their
independent bytes in two processes. Reports and source hashes are retained in
`earliest-component-*.json`.

The native display-stream audit retained all 1,086 accepted streams and 1,849,563
independent field comparisons in two processes. Its manifest SHA-256 remained
`94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.
The local `cargo xtask pr` gate passed, including 184 documentation tests.

```text
cargo test --test display_earliest_components
cargo build --example rebuild_display_archives --example rebuild_display_documents
python -B -X utf8 docs/features/display-lists/verify_earliest_components.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
python -B -X utf8 docs/features/display-lists/verify_earliest_component_documents.py target/pinned-1.99.0/debug/examples/rebuild_display_documents.exe
```

The exact source conditions are in the approved sibling's `dl_tess.py`,
`dl_light.py` and `dl_ui.py`; their hashes are retained in the fixture manifests.
The humanizer skill was unavailable; prose remains subject to maintainer review.
