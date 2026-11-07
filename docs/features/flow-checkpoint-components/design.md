# Checkpoint component contract

## Logical context and layout

The documented version-six parent graph supplies separate logical buffers.
Component bytes do not carry a common version-six prefix, physical record
identifier, compression envelope, or complete parent framing. The typed
reader requires this context explicitly rather than inferring it from a
physical record number or the viewer-query prefix.

Graph-one objects contain exactly seven raw 64-bit patterns. Graph-two and
auxiliary grids begin with three nonnegative signed dimensions and three
arbitrary signed members. Group lengths use the cyclic dimensions 1, 2, 0.
Each group stores all presence bytes before its present row bodies. Any nonzero
presence byte selects a row; values above one remain unchanged.

A row contains a nonnegative signed count and that many raw 32-bit indices.
Graph-two rows then have one slot per index; auxiliary rows have no slot stream.
Slot tags zero and one have no container body. Every other tag selects a numeric
container. Only tags zero, one, and two are canonical writer tags, so accepting
a tag above two must not imply that the documented writer produces it.

A numeric container has a nonnegative signed count N, followed by N raw 64-bit
patterns, N raw 32-bit patterns, and 2N further raw 64-bit patterns. Empty
containers are complete four-byte bodies. The trailing component contains
exactly three such containers. Floating-point encodings stay raw patterns;
NaN payloads, signed zeros, and other bit values are not normalized.

Chunk buffers require signed version zero and a positive signed chunk count,
followed by an unsigned 64-bit declared raw size. Each chunk has a nonnegative
signed encoded length and exactly that many opaque bytes. Empty chunks are
allowed. Complete framing neither decodes the chunk codec nor proves agreement
with the declared raw size.

## Bounds and publication

The public implementation must bound complete input bytes, dimension-derived
row positions, stored row indices, slots, containers, numeric patterns, chunks,
and encoded bytes before allocating or multiplying untrusted counts. Complete
EOF is required for every successful component. Negative counts, partial bodies,
extensions, and inconclusive required framing checks cannot yield success.

Published views must own their immutable component source and retain exact
field intervals and hierarchy. Existing-field edits must preserve structure,
unrelated bytes, source isolation, and complete provenance. Changing presence
or tag classes requires restructuring and is outside an existing-field edit.
Tests must distinguish reader permissiveness from writer canonicality.

Coverage must report complete component framing separately from the unresolved
parent record, engineering meanings, native runtime state, and codec validation.
The baseline sets complete-component framing to true and both complete-parent
coverage and engineering assignment to false. Chunk codec and decoded-size
verification also remain false. No aggregate document coverage is claimed.
