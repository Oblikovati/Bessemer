# Legend read-time zero assignments

Feature166's C857 evidence binds specification14.477 to the complete installed
legend reader. GNU decoding and an independent Ghidra capture agree on all
181 instructions and the complete 747-byte body. The named export, complete
decompilation hash, zero-register setup, version branches and store operands
remain pinned. No target code executes.

Within the explicit-field path, versions700006..840007 assign zero to C0, C4 and
C8 without reading those words from the stream. Versions840008..840009 read C0
and assign zero only to C4/C8. Versions840010 and later read all three fields and
perform none of these forced-zero assignments. The raw-class conversion before
700006 is excluded from this policy.

Derived assignments have no wire offsets. Structural absence stays absent;
zeros are not inserted into the original payload. Other skipped fields,
including D4/D8 and E8/FC/EC/100, receive no inferred defaults. Constructor and
prior-object state remain separate from these explicitly bound stores.

All 276 original native interpretations are retained. Of these, 116 have three
derived zero assignments and160 have none:348 assignments over absent fields,
with zero additional byte credit. All eight rejected candidates, prefix-only
scope and original version-selection qualifications remain unchanged. Owned
replay retains all425 parents and distinguishes the20 unsupported raw-class
contexts from completed empty assignment maps in later explicit-field contexts.

The [verification record](legend-resets-verification.json) records targeted and
full replay. The remaining feature166 obligations and publication gates remain
separate from this local policy.

The targeted freeze passed 19 checks in 5.940 seconds. Strict replay passed the
same 19 checks in 7.209 seconds and exited successfully after snapshot recording
and cleanup. These runs include the two original reset tests, the owned
reader-routing baseline and eight original catalog/style/color tests. Full
integrated replay and publication acceptance remain separate gates.

The typed operation uses behavior version `C857-explicit-reader-resets-v1`.
`SimulationPersistVersion::legend_read_zero_assignments` and both complete parent
views return immutable plans with ordered assignments, completion, original
version history, diagnostics and exact local store coverage. A completed empty
plan establishes no constructor or prior-object state. Raw-class conversion
returns a typed unsupported error rather than an empty plan.

The operation fixtures cover 4,137 version cases, 557 complete owned parents
(including 132 later-profile wire mutations) and all 276 native parents. Stored
C0/C4/C8 presence and raw bits are checked independently of derived assignments.
Concurrent replay compares eight workers with serial results. The operation
does not execute the native reader, validate a native pointer or emulate its
remaining reads.

Humanizer is unavailable in this checkout; maintainer prose review remains pending.
