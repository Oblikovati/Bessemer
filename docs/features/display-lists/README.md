# Display-list archive interoperability

The implementation on `codex/display-lists` reads, constructs, edits and writes
complete part and assembly `Contents/DisplayLists` archives for the resolved
clause 13 grammar. It retains typed stored values, shared object graphs and exact
wire framing. The [acceptance review](acceptance-review.md) maps #28's requirements
to current evidence. Final integration requires all five CI checks on the PR head.

This feature is one part of Bessemer. Complete document model archives and other
stream families remain work under #2; geometric interpretation and native
application acceptance have separate requirements.

## Supported operations

- `DisplayArchive::from_bytes` reads through EOF with explicit unsigned display
  version, part/assembly kind, native file version and external component IDs.
- `DisplayArchiveContext::from_document` resolves normalized display markers and
  optional face-directory declarations. Native file and tessellation contexts
  remain explicit application inputs.
- `DisplayArchive::new` forces typed encoding and complete reparsing. It validates
  field types, paths, wire order and the shared MFC map without original-byte shortcuts.
- `with_contents` validates an isolated owned edit. Equivalent edits and authentic
  reverts retain their source bytes. Revision-bound object handles reject foreign
  snapshots, even when stored map indices agree.
- `edit_with_history` returns the archive, completion, diagnostics, exact check
  coverage, stored-field fidelity and structural field history together.
- `check_external_face_links` checks supplied directory/block links and requested
  facet positions. Its borrowed results retain exact archive identities and
  explicit checked/unreferenced counts.

Published archives own their storage, are immutable through their public borrowing
API, and are `Send + Sync`. Builders and object maps are operation-local. Byte,
field, object, nesting and cumulative entry limits apply before publication.

## Current evidence

| Scope | Recorded result |
|---|---|
| Native part/assembly streams | All 1,086 reconstruct exactly in two processes; 1,849,563 independent reference fields |
| Owned complete roots | 11,313 roots in 135 groups reconstruct in two processes |
| Registered runtime classes | All 27 reference registrations have runtime dispatchers and owned full-root witnesses |
| Version comparisons | 362 of 391 observed both outcomes; all remaining 29 have checked caller or negative-case reviews |
| Native complete documents | All 1,086 reconstruct in two processes with resolved contexts |
| Native external links | 170 documents, 193 blocks, 1,258 declarations and 35,242 requested facets checked independently |
| Drawing VBLists | All 20 native streams and containers reconstruct; 33 shaded models and 1,383 independent fields |

The [executed predicate audit](version-predicate-audit.md) and
[review verification](predicate-review-verification.json) bind reader, runtime,
adapter and executable inputs to hashes. Missing decoders, absent witnesses,
unreviewed comparisons, changed outcomes and stale evidence fail their checks.
Branch observations establish execution coverage; field oracles and edit tests
remain distinct evidence.

The [current schema snapshot](current-schema-snapshot.json) preserves edition
0.273 alongside the earlier [0.128 snapshot](specification-13.md). Clause 13.2–13.8
layout text remains unchanged between those captures. The newer snapshot includes
the drawing VBLists clauses 13.9–13.11.

## Defined limits

Unsigned display versions include zero and high signed-marker aliases. The
uncounted open-window interval `0x1FAE`–`0x1FCE` returns typed unsupported errors.
Inline object-backed facets in `0xA25`–`0xC7B` remain unresolved; empty and external
facet cases in that interval retain complete operations.

Non-null legacy PW wall materials, non-null `moSketchBitmap_c` textured-plane
bodies and unresolved `moFacetedData_c` graphics bodies fail explicitly. Their
supported null-pointer forms preserve complete surrounding layouts. Unknown-meaning
stored fields retain their typed raw values. Successful archive validation grants
no topology, intersection, tolerance or geometric-validity claim.

Drawing `Contents/DisplayLists` remains unresolved. The separate
[drawing VBLists codec](../drawing-buckets/README.md) implements complete modern
streams from unsigned model version `0x27CC`, including shaded part/assembly
models and trailing direct lists. It resolves `_MO_VERSION_` context independently
of display markers and publishes no opaque shaded continuation.

## Verification and integration

`cargo test` runs the repository tests. `cargo xtask pr` checks formatting, Clippy
with warnings denied, builds and tests. CI runs that PR gate and full gates on
Linux x86_64, Windows x64, macOS x86_64 and macOS arm64; Linux also runs the
ThreadSanitizer tests. The final PR head needs all five checks before integration
into the standing interop candidate branch. No version bump or release is included.

Native and owned verifier commands remain with their focused evidence:
[document contexts](document-context.md), [external links](external-face-links.md),
[edit outcomes](edit-history.md), [named-view integration](named-branch-layouts.md)
and [early PW scene regressions](early-pw-scenes.md).

Historical findings and checkpoint counts are retained verbatim in
[view/initial-archive checkpoints](view-codec-checkpoints.md) and
[archive-layout checkpoints](archive-layout-checkpoints.md), with their original
reports and immutable fixtures. The humanizer skill was unavailable; new prose
remains subject to maintainer review.
