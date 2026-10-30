# Signed node-group runs and shared references

Tracking: [#122](https://github.com/Oblikovati/Bessemer/issues/122).
Behavior profile: `simulation-node-groups-v1`.

This feature follows approved `../inventor-interface` clause 14.120, retained
in `schema-snapshot.md`. Complete copied `simulationmesh` and
`simulationbodynodes` modules provide reference outputs. Complete actual
CFB/MAS/GEN readers provide the installed stored blocks with their paired
context. Reader functions and context are not replaced or monkeypatched.

## Stored tags and diagnostic runs

The existing `SimulationNodeView::group_tag` reads the signed i16 at node
offset twenty-two. Its u16 representation remains available by casting the
signed value, and the full 52-byte record remains in the retained GEN image.
Negative patterns are sign-extended; stored `0xffff` means signed minus one,
which does not compare equal to an element body-selection value 65535.

`SimulationGenArchive::node_group_runs` partitions all stored node records into
contiguous equal-tag runs. Runs include inactive nodes, preserve separated
repetitions and continue across the native 16,384-record allocation boundary.
Each run includes its signed tag, inclusive one-based first/last ordinals and
typed first/last node identities bound to the retained complete GEN revision.

The result retains the immutable source image, parent identity, exact check
scope, successful empty/populated completion, visited node/tag-byte counts,
zero tag-pattern error and explicit diagnostic limits. Every stored node is
visited and covered exactly once; every node within a run has the recorded
tag. The operation leaves all input bytes and existing node identities intact.

These diagnostic runs do not emulate the producer's element-driven traversal,
prior cursor or body-transition logic. They do not establish exclusive CAD body
ownership. Existing `project_body_nodes` queries continue to use element IDs,
reference slots and node activity without applying a node-tag filter.

## Evidence and reproduction

Six independent owned blocks contain 81,933 nodes. They cover all 65,536 word
patterns, zero and inactive nodes, separated repeated tags, empty completion
and a negative-tag run crossing the allocation-chunk boundary. An independent
ordinal grouping oracle agrees with the complete actual reader before fixture
bytes are frozen. Malformed record extents are rejected by the reference
reader and by complete GEN construction without publishing partial images.

All 45,246 installed node records and their 90,492 tag bytes are compared.
Each composite fixture contains tag one at ordinals 1..5333 and tag two at
5334..10553. The benchmark contains one tag-one run at 1..24140. Five explicit
mode-three body queries retain every selected ordinal and every cross-tag
ordinal. Each composite body-two query selects 5,296 nodes, including 76 tagged
one. The two fixtures therefore retain 152 cross-tag selections.

The Rust projection packet contains every activity/signed/raw-tag view, every
run, and every selected and cross-tag ordinal from those five queries. Its
725,356 bytes match the actual readers byte for byte. A separate synthetic
test proves that signed minus-one and another positive tag do not exclude
nodes referenced by a body-65535 element. Node-tag diagnostics do not change
the earlier body-query implementation.

```powershell
python -B docs/features/simulation-node-groups/freeze_groups.py --export
cargo test --test simulation_node_groups --test simulation_selection -- --include-ignored
python -B docs/features/simulation-node-groups/freeze_groups.py --export --verify-rust
cargo xtask pr
```

Baselines require explicit `--write` and review. Snapshot scripts run
sequentially because their recorder shares a temporary report name. Installed
inputs and projection exports remain in ignored `target/`; normal tests own
their synthetic bytes. Tests retain complete source images after source drop,
returned-byte mutation, failed construction and independent coordinate edits.
Compile-time Send/Sync assertions and parallel full-byte comparisons cover the
new public types and concurrent read projections.

## Completed local verification

`cargo xtask pr` passed 2,997 tests with no failures and twenty-one
intentionally ignored corpus tests across 231 test summaries. The focused run
passed all seven node-group tests and all eleven existing selection tests,
including their native cases. Two fresh processes repeated all seven new tests
and produced identical complete 725,356-byte packets matching the actual
readers. All thirteen indexed Simulation fixture packets matched tested bytes.

Fresh runs took 0.6504698 and 0.3826152 seconds including startup, with observed
working-set peaks of 43,302,912 and 42,467,328 bytes. The full local gate was
running concurrently. Sampling around one millisecond gives a lower bound and
may miss the actual peak; this first implementation has no previous published
release or statistical speed claim. `complete-verification.json` records
source/runtime/test hashes, complete native and owned scope, isolation and
parallel replay, observations and exclusions. Cross-platform CI is a separate
required merge gate.

## Acceptance limits

The complete tag domain and CAD mapping remain unassigned beyond the observed
signed load. Diagnostic runs do not execute or complete the native producer's
element/cursor traversal. Cross-tag selections are preserved as shared
references, without a healing or relabeling rule. Geometry, topology
consistency, self-intersection, tolerance validity and SolidWorks application
acceptance remain separate. No installed target was executed.

No dependencies, package versions or release tags changed. The maintainer
humanizer skill was unavailable; the prose pass remains for review.
