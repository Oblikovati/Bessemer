# Earliest light and appearance layouts

The light work admitted display versions 4500–19000, excluding the uncounted-window
interval `0x1FAE`–`0x1FCE`. Full documented-schema parity, earlier contexts and
unresolved object bodies remain unfinished. These are archive checks, separate
from native application acceptance and geometry validity.

The [earliest wall work](earliest-walls.md) later extended the lower bound to 4400.

The new conditions cover coupled light, material and assembly layouts:

| Stored members | Condition |
| --- | --- |
| Scene background brightness, rendering brightness and reflectivity | `version > 0x1194` |
| Decal underlying-appearance word | `version > 0x1197` |
| RV knurl height and angle | `version > 0x119E` |
| Second light enabled word, brightness and shadow settings | `version > 0x11A3` |
| Directional softness and spot/point rendering/fog settings | `version >= 0x11A4` |
| Weld-bead polygon lists | `version > 0x11A5` |
| RV dimple size | `version > 0x11AC` |
| Assembly hidden-body atom array | `version > 0x11AF` |
| Original decal ID | `version > 0x11BB` |
| Document detail mask | `version > 0x11ED` |
| Old-light ID | `version > 0x11F5` |

Before `0x11A4`, a light stores one `enabled_in_pv` value. Later layouts store that
first value as `enabled_in_pv_initial`, then another enabled word after the keep
flag. Both retain their raw values. The ambient, base-feature, directional, spot
and point classes all share the versioned base. Model-feature light serialization
uses another context and remains separate work.

The independent corpus has 44 complete boundary roots, 24 additional light-kind
roots, 12 populated or empty 2D/3D bucket roots, 14 populated decal roots and eight
property/holder graphs. All 102 roots are consumed to EOF by the approved sibling
and provide 37,709 independent field expectations. They retain raw floating bits,
property variants, aliases, noncanonical flags, Unicode and embedded NULs. Native
file context is explicitly 19000.

Every light kind is independently encoded before conversion. The converter
observes native reader spans and removes only version-absent members. Assembly
fixtures independently insert three hidden atoms: a negative value, zero and the
maximum signed word;
both assembly runtime classes are handled. Weld lists contain a polygon on each
side when stored. Earlier layouts omit the entire list, including its count word.
Populated decal cases exercise both older option words.

Nine integration tests compare every expected field and forced byte, each layout
boundary, cross-version construction failures, exact and one-less byte/field/
object/entry budgets, all prefixes of all five early light kinds, impossible
hidden-atom counts, caller source mutation and eight concurrent independent edits.
Two complete owned version-4 documents have exact version-4500 markers. Tests
resolve context, edit, reopen, preserve an unrelated stream and restore the
authentic original document.

Two isolated Rust processes reconstructed all 102 roots and both complete owned
documents exactly, with independent context traces. Fixture, budget, source and
process evidence is recorded in `earliest-light-*.json`.

On Windows x64 with Rust 1.99.0, the native audit retained all 1,086 accepted
display streams and 1,849,563 independent field comparisons in two processes.
The display-stream manifest SHA-256 remained
`94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.
The local `cargo xtask pr` gate passed, including all 184 documentation tests.

```text
cargo test --test display_earliest_lights
cargo build --example rebuild_display_archives --example rebuild_display_documents
python -B -X utf8 docs/features/display-lists/verify_earliest_lights.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
python -B -X utf8 docs/features/display-lists/verify_earliest_component_documents.py target/pinned-1.99.0/debug/examples/rebuild_display_documents.exe earliest-light
```

The source conditions are in the approved sibling's `displaylists.py`,
`dl_light.py`, `dl_tess.py` and `dl_ui.py`. Source-module hashes are retained in the
fixture manifests. The humanizer skill was unavailable; prose remains subject to
maintainer review.
