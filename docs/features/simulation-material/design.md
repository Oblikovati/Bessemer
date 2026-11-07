# Material operation boundaries

The Rust implementation is still pending. Use the existing normalized
`SimulationPersistVersion` and immutable persistence field/edit infrastructure.
The public profile covers the direct study-manager Material body. The distinct
AWDocument carrier needs its own evidence and must not be selected by basename.

Provide standalone exact-EOF reading and explicit complete-prefix reading,
matching the reference's two boundaries. Prefix reading returns the consumed
body length. Bound its input window by the body-byte limit without requiring
the complete enclosing carrier to fit that limit. An accepted result must own
only the body; following bytes stay outside its reconstruction and coverage.

`SimulationPersistText::source_parent()` exposes its entire retained parent.
Trimming only the Material snapshot's top-level bytes would leave following
carrier bytes reachable through text fields. Rebind every text to the exact
owned body after the prefix endpoint is known, preserving all original frame
offsets and bytes. Existing standalone load behavior must remain unchanged.
Measure the extra body copy before changing shared storage.

Expose byte, physical-field, declared-entry and text-byte limits. The grammar
has no tagged embedded objects. For associated property tables, validate the
declared nonnegative count even when presence is zero, but consume pairs only
when presence and count are nonzero. A byte-availability check for nonexistent
pairs would reject valid count-only records. Global and delegated tables always
consume the counted interleaved pairs. Keep declared-entry budgeting distinct
from the number of pairs actually present on the wire.

Bug #141 established the text boundary with the unchanged actual parser.
Null, allocated empty, odd-byte and even UTF-16 frames remain distinct raw
values. Odd frames have no decoded view; unpaired surrogate units survive.
No parity restriction or Unicode normalization belongs in this codec.

Construction must reparse the complete supplied shape. Isolated edits retain
the source and replacement, verify unchanged prefix/suffix bytes, and return
ordered physical field history with explicit completion, diagnostics, coverage
and achieved pattern error. Prefix and standalone check modes must remain
distinguishable. Geometric/material-model validity and native runtime repairs
are separate from stored-body validation.

Before publication, compare every native candidate's acceptance outcome,
complete accepted bytes and fields, plus independently generated edited fields
and history. Preserve all failed alternatives in the denominator. Require exact
and one-below limits, proper-prefix/trailing rejection for standalone bodies,
prefix suffix-isolation tests, input ownership, deterministic concurrency and
two fresh-process complete exports. Full local and platform gates remain required.
