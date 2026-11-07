# Simulation ELF images and projections

Tracking: [#115](https://github.com/Oblikovati/Bessemer/issues/115).
Behavior profile: `simulation-elf-projections-v1`.

This implementation follows approved `../inventor-interface` clauses
14.107–14.109, retained in `schema-snapshot.md`. Reference checks import actual
copied CFB, transfer, ELF and scalar-record readers. Hashes and import origins
are recorded before temporary-source cleanup. Reader functions and their parsed
contexts are not replaced.

## Complete native initializer

`SimulationElfInitialArchive` reads, constructs, edits and writes the complete
observed 1,224-byte wrapped initializer. It validates the transfer word, fixed
244-byte record profile, 100-slot capacity, constructor marker, fixed words and
all 1,192 continuation zeros. Both independent constructor parameters preserve
all 32 bits. A later advertised update count is not an existing data-block count.
Construction uses the documented 61-word initializer equations.

Parameter IDs bind a typed A/B selection to one immutable image. Edits retain
every byte except the selected four-byte parameter and report completion,
identity derivation, exact check mode, coverage, zero requested-pattern error
and semantic exclusions. No-op edits retain identity and bytes. The
`SimulationElfInitialStudy` operation composes this edit through a complete CWR,
reopens the initializer and preserves unrelated logical streams and metadata.
Unchanged MAS/GEN identities are reused only after verifying their actual bytes.

## Selected index context

`SimulationElfIndexContext` preserves every source byte while validating copied
framing and the observed 244-byte/100-slot header/index context. It does not
establish whole populated-file acceptance. A typed selector is validated once
in 1..=100. Directory traversal checks the selected pair and every complete
visited record. A negative identifier terminates after its record is read;
zero skips; a positive ID requires an explicit external map. Empty pairs retain
their original first-record word and have no visit cursor.

Coverage explicitly distinguishes empty, declared-count exhaustion and negative
termination. Limits bound source bytes and records actually visited. A large
declared count can stop early on a negative ID; the implementation does not
allocate that declared count. Unvisited bytes, including opaque tails, remain
exact source bytes with no invented validation claim.

`SimulationElfMappingContext` normalizes an explicit caller map once. Every
entry must be zero or a one-based target within the supplied destination extent;
source and destination budgets are explicit. Copy results preserve the full
record and its original identifier. Writes follow native visit order and record
overwritten source parents. Unassigned destinations remain `None`, rather than
invented previous memory. Completion distinguishes empty copies from failures.
Results include selected-source coverage, copied-byte counts, source history,
zero copied-pattern error and acceptance exclusions.

## Header proposals and scalar records

`project_index_update` returns only the documented 1,220 header-body bytes at
wrapped stream offset 4. It checks marker/count arithmetic, compares the old
watermark as signed, clears prior directory entries and padding, inserts one
pair and advances the marker. The result explicitly reports
`HeaderOnlyNoDataRecords`, zero data records written and no completed populated
file. There is no API that silently applies this proposal as a completed ELF
file. Required record bytes and extents must be established separately.

`SimulationElfScalarRecord` retains a signed identifier and all sixty binary32
patterns in ten groups of six. NaNs, signed zero and unused cells stay exact.
Store-stage construction accepts resolved groups and fills omitted cells with
positive zero. It does not execute caller filtering, numeric calculation,
physical component assignment, unit conversion or native runtime gates.

## Verification

Owned fixtures include four complete constructors, six selected mapping cases,
all 100 header selectors and five scalar-store cases. Literal expected mapping
slots and independent constructor/store/header bytes are checked against the
actual copied readers. The overwrite case gives the final repeated identifier
a distinct payload. Header cases start with nonzero padding and a prior entry,
so failure to clear either cannot accidentally match the expected body.

The installed corpus contains one 1,224-byte ELF initializer in a complete CWR.
Its directory is empty. The actual reader reopens the independently defined
parameter-A edit and checks all six streams and directory metadata. Populated
traversal and mapping fixtures are synthetic; no installed populated ELF case
is claimed.

```powershell
cargo test --test simulation_elf
python -B docs/features/simulation-elf/freeze_elf.py --export
cargo test --test simulation_elf -- --include-ignored
python -B docs/features/simulation-elf/verify_initial_edits.py
cargo xtask pr
```

Run feature scripts sequentially because the snapshot recorder uses one
temporary report name. Baseline updates require explicit `--write` and review.
Tests cover every short initializer prefix, malformed words and continuation,
bad selectors/addresses/counts, visit/map/allocation limits, foreign identities,
source and mapping mutation, source dropping, store shapes, signed watermark
rules, marker overflow, Send/Sync and concurrent deterministic copies/edits.

The native verifier reuses the existing OUT stream-preservation helper with an
explicit ELF name. Its default OUT behavior was rechecked through actual
readers; fresh compatibility evidence is retained here while the original OUT
provenance remains unchanged.

The Windows PR gate passed 2,942 tests with 14 corpus-dependent tests ignored.
All fifteen focused tests passed with the installed initializer enabled. Two
fresh processes ran those fifteen tests and produced identical 3,366,912-byte
edited CWR files, compared byte for byte. `complete-verification.json` records
code/source hashes, limits, observations and exclusions. Memory observations
are sampled lower bounds, not guaranteed process peaks. There is no prior
published implementation for a release comparison.

## Acceptance limits

Whole populated-file lifecycle, unknown header roles, physical components,
units, frames and row associations remain unproven. Native arithmetic, caller
filtering, runtime gates and native-application acceptance are not executed.
No geometry, topology, self-intersection or tolerance-consistency pass is
implied by these stored-image operations. Header-only proposals are not valid
populated-file publications.

No dependencies, package versions or release tags changed. The maintainer
humanizer skill was unavailable; the prose pass remains for review.
