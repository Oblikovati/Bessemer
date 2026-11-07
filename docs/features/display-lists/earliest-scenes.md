# Earliest scene layouts

The scene work admitted display versions 4850–19000, excluding the uncounted-window
interval `0x1FAE`–`0x1FCE`. Earlier contexts and unresolved object bodies remain
explicitly unsupported. Full documented-schema parity remains unfinished.

The [earliest component work](earliest-components.md) later extended the lower bound to 4800.

The part tessellation word `is_tessed_for_exploded_view` is stored only above
`0x12FC`; the `pwSceneData_c` word `bg_fit_to_sw_graphics_area` only above `0x1310`.
Missing words remain absent fields, rather than synthesized defaults. Construction
and edits must match the declared version's field shape in both directions.

Sixteen complete owned part/assembly roots bracket both gates and the previous
5000 lower bound. Their 5,078 independent field expectations extend the earlier
root suite from 512 roots and 156,403 expectations to 528 roots and 161,481.
Prior fixtures remain unchanged. The generator observes the approved native
reader's spans, removes only the version-absent words, then requires the complete
reference reader to consume the result to EOF. The assembly case contains inline
part tessellation and populated RV appearances. Native file context remains 19000.

The new oracle widens signaling f32 NaN payloads through integer bit placement.
Python's ordinary hardware promotion quiets them, which would incorrectly change
the expected archived payload. This adjustment affects new trace expectations;
wire bytes are retained and the Rust codec's exact float behavior is unchanged.

Integration tests check every field, forced reconstruction, exact entry budgets,
both sides of both gates, cross-version shape rejection, every truncated prefix,
independent ambient edits, reopening, source mutation and eight concurrent edits.
Two isolated Rust processes also reconstructed all 16 roots byte for byte against
the independent fixtures. Source hashes and process results are recorded in
`earliest-scene-part.json`, `earliest-scene-assembly.json` and
`earliest-scene-verification.json`.

The native corpus gate retained all 1,086 accepted streams and 1,849,563 independent
field comparisons in two processes. Its display-stream manifest remained
`94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.
The local `cargo xtask pr` gate passed, including 184 documentation tests.

```text
cargo test --test display_earlier_range --test display_archive --test display_version_range --test display_middle_range
cargo build --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/generate_earliest_scene_roots.py
python -B -X utf8 docs/features/display-lists/verify_earliest_scene_roots.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
```

The source conditions are in the approved sibling's `dl_tess.py` and `dl_light.py`.
Their complete hashes are retained in the fixture manifests; the evolving sibling
working tree is not identified by its Git commit alone. No native application
reopening or cross-platform CI is claimed by these archive checks. The humanizer
skill was unavailable; prose remains subject to maintainer review.
