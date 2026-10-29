# Direct sketch points, lines and text

Feature #44 now reads complete direct point, line and text collections. These
records do not allocate MFC map slots for the geometry itself; their tagged
references share the operation's existing map. Collection categories and masks
remain stored fields. Prototype selection does not normalize their bytes.

Point coordinates use the stored 2D/3D bit. Minimal-owner and jet states remain
explicitly unsupported. Line endpoints retain short or escaped signed indices,
while the entity's flag word selects its optional vector. The existing handle
reader now uses the same escaped-index primitive; its owned fixtures remain
unchanged.

Text resolves its tagged dimension-text target through typed object identity.
The reader finds the target's stored fields in the current operation, including
aliases to an earlier target. It adds no cache or global state. Missing or
wrong-typed required facts fail; imported class identity alone cannot establish
a body field. Lookup failure poisons the decoder before publication. Present
curve text now uses the [documented kernel continuation](../alignment-groups/README.md);
compressed records require explicit kernel-schema context, and malformed or
unimplemented kernel layouts still fail.

The following measurements record the original Feature #44 checkpoint. Current
complete-sketch and native-corpus evidence is linked from the alignment feature.
The independent packet contains 308 cases and 12,378 scalar expectations.
Cases cover 2D/3D and high-bit masks, escaped endpoint extremes, optional vectors,
category masks, shared text aliases and compositions sharing one map across all
three collections. Every case reconstructs exactly. Every truncated prefix and
trailing byte fails. A line edit matches separately assembled, reference-checked
bytes, reparses to its changed fields and leaves the source unchanged. Source
overwrite and four-writer checks cover every case.

Six collection tests and three referenced-field tests pass, along with all 191
model tests, all-target Clippy and all 50 Python tests. Four new wire fixtures
reproduced exactly in fresh processes. The native corpus reproduced all 906
previous outcomes. Results are bound in `direct-sketch-verification.json`.
No partial `sgSketch` root reader
is exposed. Hatches, arcs, ellipses, conics, splines, surfaces, dimension groups,
patterns and the sketch tail still need composition. Native acceptance remains
866/906; the complete corpus gate still fails.

These are structural archive checks, not sketch solving or geometry validation.
Public model operations and their full result/history contracts remain pending.
The full PR gate belongs to the earlier geometric/kernel checkpoint and must
rerun before push. Humanizer was unavailable; prose remains for maintainer review.
