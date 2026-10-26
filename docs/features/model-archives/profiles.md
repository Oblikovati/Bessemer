# Complete ordinary profiles

Feature #44 now reads complete ordinary and reused profiles, including the
documented 3D-profile alias. The ordinary branch composes sketch, chains,
geometry, contours, placement and suffix with one shared object map. The reused
branch retains its source-profile identity, chooser, selection handle and contour
data. A reused source is required and cannot be null. Unknown new trim or
profile-reference bodies still fail the containing operation.

Sketch chains retain compact/extended DWORD arrays, handle masks, raw words,
pairs and trim references. Contour objects and choosers preserve cyclic profile
references and duplicate/order-sensitive collections. Direct serializers allocate
no enclosing object slots. Masks are checked against their declared object kinds;
the negative tests use typed MFC identities to distinguish these masks from
unrelated inherited fields with similar names.

The existing kernel reader now returns a typed suppressed/serialized state to
callers that need it; its earlier wrapper and wire fields remain unchanged.
Profile geometry reads the extra-body collection only after a serialized primary
body, including when that collection is empty. Extra bodies retain their signed
indices and may independently be suppressed or framed. No body-validity guess,
raw kernel fallback or invented extra count is used.

The reference-array element size must be zero or a multiple of four. Nonzero
records retain capacity, growth, count and every payload bit as aligned u32
groups. These words receive no invented identifier semantics. Size/count products,
byte extents and aggregate entries are checked before iteration. An empty array
with a very large aligned element size remains valid without allocation. Two
eight-byte rows charge six entries (two rows plus four words): six passes and
five fails.

The independent packet has 180 cases and 11,072 scalar expectations across four
versions. It covers ordinary/reused profiles, both profile class names, all
supported chain/chooser masks, contour cycles, suppressed/framed geometry and
empty/mixed extra-body collections. Every exact reconstruction, truncated prefix
and trailing-byte check passes. An edit matches independently assembled bytes,
reparses and preserves the original. Every case survives source overwrite;
parallel writes preserve the complete owned graph and both kernel frames.

Forty-eight framed cases require explicit schema context; twenty-four suppressed
geometry cases complete without it. The primary and extra typed kernel records
are independently checked at identifiers -7 and -11. Sixteen null reused targets,
112 invalid mask mutations and 64 unrelated-class mutations reject. Dynamic count,
alignment and extent failures have focused checks. Four pure Python projection
tests cover raw word groups, bad extents, aliases/cycles and suppression metadata.
The scalar projection converts reference bytes independently to words and omits
only derived sketch metadata and kernel data already covered by the separate
typed kernel oracle; primary and extra suppression values remain included.

All thirteen focused tests, all 276 model tests, all-target Clippy and all 55
Python tests passed. Three fixtures reproduce in two fresh reference processes.
Two final native runs agree at 899/906 across 835 document paths, adding indices
25, 272, 295, 358, 758 and 761. The remaining first stops are four `Fillet_c` and
three `moCosmeticThread_c` bodies. Later dependencies may still appear; complete
corpus acceptance remains failed.

Nine edition 0.358 clauses and source/test/fixture/log hashes are retained in
`profile-verification.json`. Public operation/history contracts, configuration
diagnostics, native application/geometry acceptance and the platform matrix
remain open. The full PR gate belongs to the earlier geometric/kernel checkpoint
and must rerun before push. Humanizer was unavailable; prose remains for review.
