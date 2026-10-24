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

## Verified named-view component

`DisplayVersion` normalizes exact `_DL_VERSION_<n>/DLUpdateStamp` markers once.
Signed negative names become their unsigned 32-bit values; malformed, missing
and conflicting markers fail explicitly. Model/file versions and add-in version
strings do not substitute for this context. Equal numeric marker aliases agree.

`NamedViews` reads, constructs, edits and writes the complete named-view body in
13.4.1. Its [reference snapshot](reference-named-views.md) records all conditional
fields. Inactive versions have no body, distinct from a present empty collection.
Typed records retain both compact transforms, CString encodings and prefix widths,
section planes, raw selection flags and names, and unknown fields and bitsets.
Version-gated fields must have the exact required shape before publication.

`DisplayViewLimits` bounds bytes, view records and the cumulative nested planes
and selection-name entries. Counts are checked before allocation. Encoded CString
sizes are checked without allocating a temporary string. Every prefix width and
both character encodings have exact-boundary tests. Regression #29 checks that
equivalent CString edits cannot use shorter canonical prefix sizes to return
longer preserved source bytes beyond the requested budget.

Sixty independent vectors cover every named-view version boundary, absent and
empty collections, noncanonical DWORD CString prefixes, wide and Latin-1 text,
embedded NULs, raw nonboolean flags, planes and selected names. Eleven focused
tests cover complete field expectations and trace output, every truncation,
malformed counts, all budgets, invalid shapes, equivalent edits, authentic reverts,
source isolation, Send/Sync and repeated concurrent edits against serial bytes.

On Windows x64, Rust 1.99.0 MSVC, complete reference parsing of all 1,086 native
part/assembly streams identified 1,086 named-view collections containing 99 views
and 30,432 bytes. All 41 distinct versioned collections force-reconstructed byte
for byte. Native collections and all 60 owned vectors also matched independent
reference field traces: text, array shapes, signed/unsigned field values and exact
floating-point bits. The trace covers fields exposed by the reference decoder;
raw transform presence bytes and discarded Matrix5 integers have separate complete
wire-vector coverage. [named-verification.json](named-verification.json) records
manifest and fixture identities. This is component evidence, not full Bessemer
display-list archive acceptance.

```text
python -B -X utf8 docs/features/display-lists/generate_named_vectors.py
cargo test --test display_named_views --test display_named_trace
cargo build --release --example rebuild_named_views
python -B -X utf8 docs/features/display-lists/verify_named_corpus.py target/pinned-1.99.0/release/examples/rebuild_named_views.exe
```

## Verified annotation-view component

`AnnotationViews` implements the complete annotation-view body in 13.4.3, using
its [captured serializer](reference-annotation-views.md). The outer version gate,
the per-view unknown flag and the collection-level trailing bitset have distinct
conditions. An active collection can have zero views and still require the bitset.
It is not the same layout as named views.

`AnnotationViewId` wraps the stored signed ID once without inventing a restricted
domain or uniqueness rule. Repeated IDs, raw shown/current flags, both compact
transforms, second-name and unknown strings, section-plane fields and bitsets
remain unchanged. Native suffixes and visibility flags are not reconstructed or
normalized. `DisplayViewLimits` bounds physical/rebuilt bytes, view records and
cumulative planes. The shared immutable view envelope retains equivalent bytes
and authentic reverts, including the #29 preserved-prefix budget check.

Twenty-seven independent vectors exercise each version boundary, absent and
present-empty bodies, collection tails, duplicate IDs, nonboolean flags, all
conditional fields, noncanonical CString prefixes, embedded NULs, wide text and
exact special floating-point bits. Seven focused tests cover complete field and
trace expectations, every truncation, malformed counts, invalid shapes, budgets,
clone/source isolation, authentic reverts, Send/Sync and concurrent edits.

On Windows x64, Rust 1.99.0 MSVC, 2026-10-25, all 1,086 native annotation collections
matched independent byte and field oracles: 3,260 views and 874,416 bytes. Every
one of the 597 distinct versioned bodies force-reconstructed byte for byte; the
reference field traces also matched all 27 owned vectors. The parent display-
stream manifest is the same as the transform and named-view corpus above.
[annotation-verification.json](annotation-verification.json) records fixture,
reference-snapshot and corpus identities. These are component checks; Bessemer's
complete display-list archive remains unimplemented.

```text
python -B -X utf8 docs/features/display-lists/generate_annotation_vectors.py
cargo test --test display_annotation_views --test display_annotation_trace
cargo build --release --example rebuild_annotation_views
python -B -X utf8 docs/features/display-lists/verify_annotation_corpus.py target/pinned-1.99.0/release/examples/rebuild_annotation_views.exe
```

The named and annotation verifiers share framing, scalar/transform tracing and
complete-reference-stream scanning. Each still invokes its own native serializer
and compares the exact semantic fields of that distinct body.

## Remaining implementation and integration gate

- Connect the parsed display-list version to explicit part/assembly context,
  native file version and cross-stream face component IDs.
- Reuse existing MFC strings, exact numeric values, class/object maps and face
  tessellation records. Preserve cycles, aliases and object-tag spelling; do not
  reset inherited state between nested serializers.
- Add typed user environments, splitters, line fonts, buckets and display items,
  then 3D views and complete top-level framing. Named and annotation bodies
  are verified above.
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
