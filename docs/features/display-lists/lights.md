# Modern DisplayLists lights

Issue #28 remains open. This checkpoint adds complete version-19000 spotlight and
point-light bodies to the existing ambient, base-feature and directional light
readers. It covers all five light runtime classes used by DisplayLists'
`serializeOldLights` context. Model-feature light serialization has a different
base layout and remains separate work; the stream bytes do not identify that
context by themselves.

Both new classes reuse the existing light base and property codec. Position,
target, cone, exponent, falloff, radius, fog and rendering values retain their
stored bits. Counts with their high bit set retain the reference's completed-empty
property-list semantics. Unknown property types carry no value, matching the
documented switch. The library does not infer physical light constraints from
these archive fields.

Four [independently encoded fixtures](light-vectors.json) cover spotlights,
point lights, empty properties and a spotlight exponent edit. They match 215
semantic fields, including all property variants, supplementary Unicode,
noncanonical flags, signed zero, infinities and NaNs. The edited fixture was
encoded from wire primitives rather than produced by the Rust writer.

Seven integration tests verify those fields and exact forced reconstruction,
raw property and flag values, empty-count spellings, malformed counts and variant
shapes, every new light prefix, exact aggregate entry/field/byte budgets,
whole-document edit/reopen/revert, untouched streams, source ownership and eight
concurrent edits.

The [native audit](archive-audit.json) reconstructs 983 of 1,086 reference-readable
streams: every version-19000 stream in this corpus. It independently checks 56,876
light fields alongside 220,518 reference-geometry fields, 158,633 scene fields,
176,601 root display-item fields and 234,395 view-bucket fields. Both processes
produce identical bytes, diagnostics and traces, and no previously accepted paths
were lost. The corpus SHA-256 remains
`94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.

The remaining native failures are five version-7000 streams, 23 version-9000
streams, 52 version-10000 streams, one version-11000 stream and 22 version-18000
streams. All are explicitly unsupported. Unexercised schema branches,
cross-stream validation, geometric validity and native application acceptance
also remain required work. Passing this corpus subset does not establish full
display-list or model-schema parity.

```text
python -B -X utf8 docs/features/display-lists/generate_light_vectors.py
cargo test --test display_lights
cargo build --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/audit_archive_corpus.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
cargo xtask pr
```

The fixture manifest records the evolving reference source hashes. The
maintainer-side humanizer skill is unavailable; its prose pass remains for review.
