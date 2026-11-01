# Auxiliary triangle pass

Specification section14.506 and C-888 bind style204, style208 and all four
style20C RGBA components to one local pass. The group is serialized only at
version840008 and above. Earlier missing values are not initializer defaults.

The caller requires a nonzero204 and widened binary32 alpha at least the stored
binary64 threshold0.99. Its threshold bytes are `ae47e17a14aeef3f`. With masked
comparison exceptions, unordered alpha bypasses the request. Adjacent binary32
patterns `3f7d70a3` and `3f7d70a4` fall below and above that threshold. The helper
argument for per-group RGB is true only for208 equal exactly1; another nonzero
selector does not select it.

A helper request is separate from triangle calls. The complete helper also
requires a nonnull core2520 object, plot word170 different from51, a nonzero
group count, a nonnull group array and a renderer type outside3 through5. The
group count is loaded as signed32 for iteration: a high-bit count can pass the
nonzero outer check while producing no iterations. Positive-count groups are
visited in order with stride184 bytes; only positive signed triangle counts
reach the named Triangles slot at58. The two payload identities are forwarded
without a triangle-validity check.

Without per-group RGB, calls use the supplied color. With it, each admitted
group supplies RGB from its color at group+C. Group alpha is ignored. Renderer
alpha comes from the supplied color and is capped at1. This cap is modeled only
inside the caller's admitted alpha domain; native MINSS behavior for other
inputs is not generalized. Original stored colors remain authoritative.

The concrete vtables are file-backed: CDrGL at1802DB188 and CDrEDW3Plot at
1802D8FD0. Their58 slots point to the exact named Triangles exports. Verification
checks both raw and virtual section bounds, slot bytes and target exports.
Both complete caller/helper bodies, constants, guards and loops are retained.
The two inherited C-887 COMISD qualifier captures stay byte-checked. One exact
`add eax,-3` instruction is recorded in unsigned display form for GNU; its
bytes remain `83c0fd` and every other operand remains strict.

All160 original stored sets produce no request in this caller. The sole nonzero
enable has alpha0.5; the other159 enable words are zero. Those frequencies do
not restrict field values or establish that another pass never renders them.
Fixtures retain all425 owned and276 native contexts, including missing groups:
224 owned and160 native contexts contain this group. Independent Source cases
cover86,632 requests over1,547 alpha patterns, signed enable classes and
exact-one selector classes. All three original gate tests and the entire
current catalog test module remain in the targeted denominator.

`auxiliary_triangle_controls()` reads the canonical parent/style snapshot.
`SimulationAuxiliaryTriangleControls::plan_masked_exceptions` takes explicit
runtime facts, group snapshots and a maximum-group budget. It composes the
request with the documented rejection gates and ordered positive-count filter.
Opaque payload tokens are forwarded unchanged, including zero; they are never
dereferenced or promoted into validated geometry.

Published plans own their calls, original controls and runtime context. Each
call retains its group index and color origin. Completion distinguishes empty
plans from call requests; the stage distinguishes caller bypass, helper
rejection and admitted setup. Coverage records the raw count, inspected count,
budget and exact request-pattern error. Native execution, payload validity and
scene traversal exclusions accompany every successful plan.

Budget overruns and missing required group snapshots return typed errors without
partial successful output. Unused groups are not required when earlier gates
reject the helper. A high-bit count admits setup when other checks pass but
inspects no groups, matching the signed iteration test. Original source groups
and archive bytes remain unchanged.

Independent helper fixtures cover6,628 scenarios using the reviewed native
pointer offsets and stride, rather than the Rust iterator representation. They
include renderer exclusions, signed/high-bit counts, missing data, budget
boundaries, positive/skipped groups, raw group colors and opaque payload tokens.
All7 Rust checks passed, including native projections and eight-worker replay.
Clippy passed with warnings denied. The strict Source suite passed16 checks;
the full suite passed165. Two fresh processes matched93,961 contexts byte for
byte against independent expectations. The [operation proof](aux-triangle-operation-verification.json)
records revisions, log hashes, exclusions and publication gates still pending.
