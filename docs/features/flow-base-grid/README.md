# Flow base grid

Issue: #241. Reference: `../inventor-interface/solidworks/flowbasegrid.py`,
specification section 14.41 and the native base-grid corpus.

Record 101 has three signed dimensions followed by three boundary arrays,
each with dimension plus one binary64 words. It has no version word. Zero
dimensions retain one boundary word and produce an empty default tree range.
The baseline preserves every bit, including signed zero and nonfinite values.

The owned fixture contains 56 views: the native CPT payload and all 27 triples
drawn from dimensions zero, one and three, each in raw and zlib transport.
Source file hashes and the native boundary arrays are checked before freezing.
The Rust baseline independently rebuilds all decoded bytes and verifies the
mathematical grid count, default tree bounds, array spans and raw word bits.
Regeneration uses a temporary directory and never overwrites the owned fixture.

`FlowCarrier::record_101_grid` reads an exact revision-bound occurrence under
explicit raw/zlib context and independent decoded-byte and total-word bounds.
It checks signed nonnegative dimensions and the full expected extent before
publishing borrowed boundary views. There is no allocation per boundary word.
Views retain decoded offsets, original word bits, and binary64 projections.
The default tree bounds are available as `FlowTreeBounds`; a zero dimension
maps to the empty range `[0,-1]`. The mathematical root count uses `u128`.

`replace_boundary_word` edits an existing word on a copy, writes the complete
carrier and reparses the selected grid before publication. The edit retains
immutable source/result checkpoints and all physical history relations. Exact
no-ops preserve the original stored stream and revision identity. Fresh raw
reversals restore wire bytes; fresh zlib reversals restore decoded bytes,
without requiring a canonical compressed representation.

Dimensions and cardinalities cannot be changed through this word editor.
Serialized coverage does not validate units, boundary monotonicity, model
geometry, native refinement, runtime object indexing or block-vector ordering.
Compression context remains explicit. No native application acceptance is claimed.

Validation covers all 452 stored boundary words across the frozen views, each
with an exact no-op, changed payload and fresh reversal. It also checks every
stored prefix, extra bytes, independent allocation limits, negative or impossible
dimensions, malformed zlib, foreign/stale identities and edit index bounds.
Eight workers edit a shared immutable source after its caller input is mutated.
A measured compressed-growth case must fail under the original physical limit.
Two fresh processes produce identical output, checked against the frozen Source
views and independent whole-carrier patches; noncanonical zlib output is checked
by its exact decoded bytes, checksum and stream termination.
