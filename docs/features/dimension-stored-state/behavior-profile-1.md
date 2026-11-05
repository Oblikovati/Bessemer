# Dimension stored-state behavior profile 1

`dimension-stored-state-v1` adds archive-backed projection and selected stored
value edits for specification clauses 11.436 and 11.437. Existing raw field
names, byte layouts, accepted parser versions and complete field-edit behavior
remain unchanged. There is no file migration or package version bump.

The mask's bit 1 is the stored bend-angle enforcement flag. Bits 2 through 4
store the fillet reference-point change number, including zero. The unresolved
projection is `mask & 0xFFFFFFE1`. The adjacent signed I32 is the redundant
identifier; its old `unknown_5c` field remains unchanged. Every signed value is
accepted. Native constructor defaults and increment transitions do not narrow
stored values.

Projection requires the existing decoder's adjacent same-scope signed dimension
words and a handle owned by the selected archive revision. It handles direct,
derived and nested dimension bases through that typed signature. A short class
whitelist would exclude supported derived bodies, so it is not used. Other mask
fields cannot satisfy the complete signature in a validated archive.

Edits validate the source revision before building replacement fields and use
the existing complete archive edit operation. They preserve unselected fields,
raw unknown bits, optional text, aliases and full ordered history. An invalid
counter request fails before publication; it does not clamp or wrap. Completed
no-ops retain identity. Changed results are immutable, isolated revisions.

Acceptance uses independent complete owned stream literals and original-reader
primitive partitions, signed limits, every valid stored counter, all remaining
u8 counter requests, exact full images, unselected fields and revision-bound
history. Configuration edits also reopen a whole document, preserve unrelated
streams and reverse byte for byte. The complete retained 906-case gate preserves
configuration and feature carriers; feature counts bind by identity and hash to
all original per-stream C-1923 promoted identifier counts. Historical accounting
values are not migrated by that comparison.

Compile-time Send/Sync checks, shared-snapshot successful and failed edits,
serial/eight-worker full projections and two fresh process exports check result
isolation and determinism. These file-operation checks are separate from native
application, geometry and whole-format acceptance. Local repository validation
and exact-publication-head platform checks remain independent gates.

The humanizer skill is unavailable; its prose pass remains for maintainer review.
