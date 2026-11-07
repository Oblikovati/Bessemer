# Simulation selection and body node-mark queries

Tracking: [#119](https://github.com/Oblikovati/Bessemer/issues/119).
Behavior profile: `simulation-selection-v1`.

The implementation follows approved `../inventor-interface` clauses
14.114–14.115, retained in `schema-snapshot.md`. Actual copied
`simulationselection` and `simulationbodynodes` readers provide reference
projections. Native blocks come from actual complete CFB/MAS/GEN readers and
their real paired context. Reader functions and contexts are not substituted.

## Selection predicates

`SimulationElementRecord::selection` exposes the unsigned word at offset 28 as
a native body-selection ID and retains its family byte. The reference reader's
`selection_key` alias has the same stored value. No CAD-body, assembly-component,
material or persistent geometry mapping is assigned.

`SimulationElementFamilyMask` preserves a complete u32 caller pattern, including
signed masks represented by their bits. `valid_for_mask` applies only the named
helper: bit one accepts family four, bits two and sixteen both accept family
eight, and bit four accepts family two. Other bits have no independent accepting
branch. Zero rejects in this helper; all bits set still rejects other families.
Activity, node count and stored classification code do not control it.

`solid_averaging_scalar_gate` separately requires nonzero activity, family four
and count ten. Zero mask bypasses the helper. Zero signed caller key disables
equality selection; other keys compare against the zero-extended u16, so -1
does not match 65535. `solid_averaging_exclusion_gate` models zero scalar key
with the supplied set enabled: membership rejects. Rust's typed i32 ordered
set supplies valid caller keys without reproducing native tree state.

## Complete supplied-context queries

`SimulationBodyQueryMesh` owns whole 52-byte node and 140-byte element blocks.
`SimulationGenArchive::body_query_mesh` supplies a caller using the existing
complete paired GEN reader.
The snapshot retains that GEN revision as a copy parent; loose supplied blocks
have no inferred GEN parent. Explicit limits bound source records and actual
write visits. Selected counts and node references are checked during each
query; malformed unvisited references are not promoted into an unrelated
whole-mesh rejection.

`SimulationBodyQueryContext` normalizes full body-ID bits, signed mode, supplied
i16 callback result and safe inclusive range. Mode one bypasses callback/range
processing and directly writes body-ID bits to active nodes. Modes two through
five use the supplied element range, activity and body-ID equality without a
family restriction. Other modes scan all elements and require family four.
Empty ranges retain caller marks; nonempty ranges must be in bounds. The
unsafe maximum signed endpoint is rejected even for an empty range.

Element paths skip node ID zero and reject negative or out-of-range selected
IDs. Only active destinations are written. Their mark is the signed body ID
when greater than one, otherwise one; mode one does not apply that clamp.
Unknown untouched marks remain `None`, and supplied prior marks remain intact.
The native return value stays zero even when writes occurred.

Callback values other than -1 override the type flag with sixteen. Without that
override, only an unresolved range flag can resolve at its first successful
active-node write. Families four, eight/three and two yield one, two and four.
An unknown first family still resolves the flag at -1 and blocks later
inference. Fallback unknown modes do not perform that inference. No callback,
native pointer guard, averaging or historical session is executed/recovered.

The immutable result retains source blocks/revision, normalized context,
captured initial marks, final marks and every ordered write. History includes
node and element/reference parents, duplicate destinations, overwritten prior
patterns and written patterns. Completed empty and completed writes are
distinct. Exact check scope, counters, zero achieved mark-pattern error and
ownership/runtime/geometry exclusions accompany the result. Failure publishes
no partial marks and leaves the source and caller marks unchanged.

## Evidence and reproduction

Owned fixtures cover 88,800 scalar/set cross-product cases and 9,472 family/mask
pairs, with independent numeric expectations and actual reader comparisons.
The 1,117 whole-block query cases cover all mode branches, supplied callback
extremes, body bit patterns, optional prior values, empty/inactive meshes,
duplicate writes, and first selected records that produce no successful mark.
Independent complete histories agree with actual reader marks, node visits,
type flags, callback fields and zero return values.

All 25,106 installed records are checked with complete paired context. Eleven
explicit native-corpus queries include all five documented mode-three body
selections, three family-four fallback scans and three mode-one scans. The five
body selections retain 209,204 write visits and 45,398 per-query marked nodes;
those counts include overlap and are not a union coverage total. Composite
body selections overlap on 76 nodes each, with a union of 10,553 nodes.

```powershell
python -B docs/features/simulation-selection/freeze_selection.py --export
cargo test --test simulation_selection --test simulation_elements -- --include-ignored
python -B docs/features/simulation-selection/freeze_selection.py --export --verify-rust
cargo xtask pr
```

The second source run compares every emitted Rust native predicate and complete
query mark/history packet byte for byte. Run snapshot scripts sequentially;
their recorder shares a temporary report name. Baseline replacement requires
explicit `--write` and review. Native inputs stay in ignored `target/`; normal
tests use owned bytes. The earlier element fixtures also pin the shared solid
record gate unchanged. Boundary, failure-isolation, Send/Sync and concurrent
serial-byte comparisons are included in the Rust tests.

## Completed verification

The full local PR gate passed 2,967 tests with zero failures and eighteen
explicitly ignored corpus checks. All eleven selection tests and all eight
prior element compatibility tests passed with corpus checks enabled. Two fresh
processes each ran the complete selection workload and produced byte-identical
15,890,868-byte native predicate/query/history packets, matching the actual
frozen readers. `complete-verification.json` records revisions, hashes, limits,
check scopes, native counts and exclusions.

The two selection processes took 0.284 and 0.285 seconds including startup.
Observed working-set peaks were 65,236,992 and 64,720,896 bytes; approximately
one-millisecond sampling can miss the actual peak. There is no prior published
implementation for comparison, and these observations make no statistical
speed claim. Required Linux ThreadSanitizer CI remains separate from the
checks executed on this Windows host.

## Acceptance limits

The native body-selection domain has no established CAD/assembly ownership
conversion. Complete type-flag/API enum conversion, other callback operations,
historical session state and application acceptance remain unproven. Native
tree/pointer corruption, allocation/HRESULT behavior and unsafe reference
accesses are not reproduced. Geometry, topology consistency, self-intersection
and tolerance validity are not established by these integer mark projections.

No dependencies, package versions or release tags changed. The maintainer
humanizer skill was unavailable; the prose pass remains for review.
