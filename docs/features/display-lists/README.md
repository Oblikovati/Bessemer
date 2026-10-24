# Display-list archives

Development for #28 is on `codex/display-lists`. It is not ready for integration.
The goal remains complete clause 13 part and assembly archive reading, writing,
construction and isolated edits, with all documented branches and all 1,086 native
streams verified. No prefix-only archive API or opaque remainder fallback will
be merged. Drawings and unresolved layouts must fail explicitly.

## Verified foundation

`DisplayTransform` implements both distinct overloads in
[clause 11.6](specification-11.6.md). Compact restore retains its raw matrix
presence byte, optional matrix, translation, scale, raw flags and flag-selected
perspective. Fixed extraction retains all 120 bytes as typed stored values,
including the two raw word pairs. Neither performs native in-memory corrections
or claims geometry validity. `StoredDouble` retains NaNs, signed zero and all
other bit patterns. Validated results are immutable; owned content edits are
checked before publication. Matrix5 payload storage is owned and clone-isolated.

The independent generator produces 1,025 complete vectors: all 256 flags with
presence bytes 0, 1, 2 and 255, plus the fixed form. Values exercise signed zero,
infinities, quiet/signaling NaN payloads, a subnormal and the largest finite double.
Tests check all wire fields, forced reconstruction, every truncation, trailing
bytes, overload confusion, invalid flags/payload combinations, source isolation,
Send/Sync and concurrent deterministic encodings.

On Windows x64 with Rust 1.99.0 MSVC, the reference reader completed all 1,086 core
part/assembly display-list streams. Instrumented transform boundaries identified
20,209 compact and 6,277 fixed transforms (1,804,378 bytes). Bessemer forced byte-
identical reconstruction of all 3,467 distinct transform encodings. The SHA-256
of sorted corpus-relative path, NUL and display-list payload was
`94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.
This proves transform codec fidelity, not Bessemer's complete-stream support.

```text
python -X utf8 docs/features/display-lists/generate_transform_vectors.py
cargo test --test display_transforms
cargo build --release --example rebuild_display_transforms
python -X utf8 docs/features/display-lists/verify_transform_corpus.py target/pinned-1.99.0/release/examples/rebuild_display_transforms.exe
```

The verifier imports the approved sibling reader, observes each serializer's exact
input span during complete reference parsing, and deduplicates by overload plus
exact bytes. Its private executable copy prevents build collisions. Native sample
bytes are read in place and are not redistributed. The owned wire vectors use
independent Python struct primitives and require no installed application.

## Remaining implementation and integration gate

- Parse display-list version names once as unsigned u32, and take explicit part or
  assembly context, native file version and cross-stream face component IDs.
- Reuse existing MFC strings, exact numeric values, class/object maps and face
  tessellation records. Preserve cycles, aliases and object-tag spelling; do not
  reset inherited state between nested serializers.
- Add typed user environments, splitters, line fonts, buckets and display items,
  then named, 3D and annotation views and complete top-level framing.
- Add lights, visual properties, scenes, part/assembly shaded data, decals,
  textured planes, units, ink views and configuration tails. Unknown-meaning
  fields remain explicit raw typed values; unresolved bodies are unsupported.
- Complete the immutable archive graph, validated owned construction/edits,
  revision-scoped identities, authentic reverts, ordered coverage and history.
- Run independent complete-field and exact forced-byte oracles for all 1,086
  streams, owned version/degeneracy vectors, malformed input/budgets, whole-
  document isolation, process/concurrency comparisons and all five CI checks.
- Update the public README and final CHANGELOG entry, review this evidence, create
  one PR, merge when green and remove the finished branch. Native application
  acceptance stays separate from archive validity and geometric validity.

[Clause 13](specification-13.md) is a working-tree snapshot from the approved
`../inventor-interface` repository, whose reference commit is
`dd0537b3a142d8dda00ef846891efc736a6aac46`; its documentation is evolving. Corliss's
typed decoded fields and isolated edits inform the public API design. The
maintainer-side humanizer skill is unavailable; its prose pass remains for review.
