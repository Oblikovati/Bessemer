# Auxiliary body fields and exclusive body-type queries

Tracking: [#120](https://github.com/Oblikovati/Bessemer/issues/120).
Behavior profile: `simulation-body-ranges-v1`.

The implementation follows approved `../inventor-interface` clauses
14.116–14.117, retained in `schema-snapshot.md`. Actual complete copied CFB,
MAS and GEN readers provide native input context. The actual copied
`simulationbodytype` reader supplies runtime-query outputs. No full runtime
descriptor file grammar is inferred from its native object offsets.

## Bounded auxiliary reference

The published `fields` function lives in
`research/simulation_aux_body_ranges_evidence.py`, whose top-level code also
inspects native modules and writes evidence. The verifier compiles only that
unchanged function AST and the original standard-library imports from the
complete frozen source file. It records the original filename/module origin,
complete file hash and exact function-source hash. Top-level native inspection
is not executed. This is a bounded function comparison, not a claim that the
complete native research verifier or installed target ran. Reader functions
and their contexts are not replaced or monkeypatched.

## Stored rows and isolated edits

`SimulationAuxiliaryBodyProfile` normalizes the 284/288-byte stored profiles.
`SimulationAuxiliaryBodyRow` owns every original byte and interprets only body-ID
bits and signed L/H fields. Bounds start at offset twelve for stored 284-byte
rows and sixteen for 288-byte rows. The original identifier/vector-selection
role remains intact. Stored rows are never automatically normalized into a
different byte profile.

`SimulationGenArchive::auxiliary_body_row` snapshots a complete actual row and
retains its GEN revision and physical row index as copy parents. Standalone
rows have no inferred GEN parent. Exact serialization and typed three-field
edits preserve every other byte; edit outcomes include authentic no-op identity,
parent/result history, fidelity, coverage, zero requested-pattern error and
semantic exclusions. These are standalone row edits: changing an identifier
does not automatically reframe a GEN vector or publish a modified GEN file.

Field views preserve mathematical L-1/H-1 in i64, including values outside i32.
`descriptor_proposal` skips zero/zero and otherwise checks both decrements fit
native i32 before proposing three runtime inputs. It neither executes the
fresh-list/study-mode builder guards nor assigns the remaining row or descriptor
fields. Every proposal retains its complete source/revision, exact field scope
and conditional-runtime exclusions. A raw row with i32::MIN remains readable
and writable; its overflowing runtime proposal fails explicitly.

## Exclusive runtime query

`SimulationBodyTypeInputs` contains only caller-supplied body bits, signed start
and exclusive upper bound. `SimulationBodyQueryMesh::project_body_type` reuses
the immutable whole node/element snapshot and its actual GEN copy parent.
Negative bounds return flag -1 before callback/traversal. Otherwise a supplied
i16 callback result is required even in mode one. Values other than -1 return
sixteen before scan-only bound/reference checks.

With callback -1, fixed modes return their inspected numeric flags. Mode five
returns -1 here, unlike the node-mark query's initial flag two. Mode three scans
exactly `[start, upper)`: equal/reversed bounds scan no elements, and safe
nonempty bounds must fit the supplied element block. The maximum signed
endpoint is rejected only on this scan path. Nonzero activity and full u32
body-ID equality select elements. Zero references are skipped; unsafe selected
IDs/counts are rejected. The first active node association ends the query even
when its family has no assigned flag. Later references are outside that
explicit early-stop scope, not silently claimed as checked.

Results distinguish negative guards, callback overrides, fixed dispatch,
completed scans without an association and completed first associations. They
retain exact visited-element history, source node/element/reference parents,
callback fields, check mode, counters, zero flag-pattern error and exclusions.
`file_descriptor_binding` remains false. This function returns the type flag;
it does not write marks or return the node-mark routine's constant zero.
Its traversal uses source node/element budgets; the mark-write budget does
not limit this read-only operation.

## Endpoint and corpus evidence

All five installed stored intervals partition 25,106 elements exactly once,
with matching element body-selection IDs throughout. Both stored profiles and
all three complete paired images are covered. The five full-span exclusive
queries and five conditional mapped-bound queries match actual reader flags,
callback fields and ordered visits. Their descriptor values are explicit
contexts, not recovered historical runtime objects.

The same proposed L-1/H-1 values have different consumer meanings. The node-mark
routine treats them inclusively and can visit stored ordinal H; the body-type
query treats the upper bound exclusively and omits H. Tests preserve both
conventions without adjustment, including a stored singleton 1..1 that proposes
0,0: node marking can visit the first record while the type query scans none.

Owned fixtures include 144 complete auxiliary rows and literal edit goldens,
3,357 accepted query contexts and 612 independently rejected contexts. They
cover callback/guard precedence, fixed-mode differences, the last stored
reference slot, unknown first-family termination, unvisited invalid tails,
empty/negative/exclusive bounds and full signed/unsigned ID patterns. The prior
selection suite also pins shared reference validation unchanged.

```powershell
python -B docs/features/simulation-body-ranges/freeze_ranges.py --export
cargo test --test simulation_body_ranges --test simulation_selection -- --include-ignored
python -B docs/features/simulation-body-ranges/freeze_ranges.py --export --verify-rust
cargo xtask pr
```

The second source run compares all emitted Rust native fields, complete edited
rows, proposed inputs and query results byte for byte. Baseline replacement
requires explicit `--write` and review. Source snapshot scripts run sequentially
because they share a temporary report name. Native input stays in ignored
`target/`; normal tests use owned bytes. Tests also cover source/returned-byte
mutation, source drop, failure isolation, Send/Sync and concurrent serial-byte
comparisons. Compressed fixtures have explicit binary Git attributes.

## Completed verification

The full local PR gate passed 2,978 tests with zero failures and nineteen
explicitly ignored corpus checks. All twelve body-range tests and eleven prior
selection compatibility tests passed with corpus checks enabled. Two fresh
processes each ran the full body-range workload and produced byte-identical
23,412-byte native projection packets, matching the actual query reader and
unchanged bounded field helper. All ten Simulation fixture packets matched
their staged Git bytes exactly.

The two processes took 0.093 and 0.065 seconds including startup. Observed
working-set peaks were 18,731,008 and 6,569,984 bytes; one-millisecond sampling
can miss the actual peak, especially for short processes. There is no prior
published implementation for comparison, and these observations make no
statistical speed claim. `complete-verification.json` records revisions,
runtime/test/reader hashes, bounded helper scope, check modes and exclusions.
Required Linux ThreadSanitizer CI remains separate from the Windows checks.

## Acceptance limits

Complete auxiliary semantics and descriptor serialization, runtime builder/list
provenance, historical bounds, CAD-body mapping, complete flag/API enum
conversion and native callback/application behavior remain unproven. Geometry,
topology consistency, intersections and tolerances are not established by these
field bindings or runtime integer queries. No complete native research verifier,
decompiler, callback or installed runtime target was executed.

No dependencies, package versions or release tags changed. The maintainer
humanizer skill was unavailable; the prose pass remains for review.
