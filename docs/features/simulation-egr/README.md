# Conditional EGR tables, existing-field edits and solver projections

Tracking: [#124](https://github.com/Oblikovati/Bessemer/issues/124).
Behavior profile: `simulation-egr-flat-v1`.

This profile follows approved `../inventor-interface` clauses 14.514–14.522,
retained in `schema-snapshot.md`. Complete copied `simulationegr` and
`simulationelementnames` modules supply reference outputs. Independent literal
images, field edits, name mappings and integer-line builders supply expected
results before freezing. Reader functions are not substituted or monkeypatched.

## Explicit complete flat profile

`SimulationEgrArchive::from_bytes` requires the caller-established
`ConstructorBoundFlatCFile` profile and a maximum logical capacity. It reads
from byte zero without inventing or stripping a transfer prefix. It validates
the complete 128-byte-header record equations, zero through thirty-two loaded
fields, positive widths totaling at most 128 bytes, descriptor/directory
alignment, exact file endpoint, unique live mappings, logical high-water word,
deleted-record links and the complete live/deleted physical partition.

Logical capacity, live rows, deleted records, allocated physical records and
unallocated capacity remain separate. Zero directory slots are holes. Deleted
records retain their complete bytes and signed next-link word without receiving
a live solver-row interpretation. Chain cycles, collisions, invalid endpoints,
incomplete partitions and inconsistent high-water values fail before publication.

The archive retains every original header word, padding byte, descriptor tag,
alignment byte, row tail and deleted record. Tags remain raw signed words;
values other than one are accepted. A completed image with no live rows remains
distinct from failure and can retain deleted or unallocated capacity. Coverage
states the exact checked profile and complete byte/row counts. Complete EGR
semantics and native cache/save behavior remain unassigned.

`parts` and `from_parts` reconstruct complete supplied regions exactly, including
their original alignment bytes. Header bounds and region lengths are checked
before reserving the concatenated output. No missing padding or allocation
state is fabricated. New input images receive distinct revisions even when
their bytes match; clones retain the original revision.

## Typed live rows and isolated field edits

Physical record IDs, live logical-row IDs and inclusive field-range IDs are
normalized once and bound to one complete revision. Foreign identities cannot
alias equal-byte images. A checked hole returns `None` from `live_row_id`;
requesting an otherwise valid field range on a hole returns a typed `Missing`
error. Invalid field bounds remain `Invalid`, without a native cache fallback.

`field_range` captures exact loaded-width bytes and their complete file parent.
`edit_field_range` requires a replacement of exactly that width. It validates
complete resulting framing and every byte outside the selected span before
returning an immutable result. The result retains source and replacement
parents, source/result range identities, every allocated physical record's
copied/derived relationship, exact coverage and zero requested-pattern error.
An authentic byte no-op retains all original identities. The operation does
not insert rows, change allocation state or execute a native save.

## Conditional row and integer EGROUP views

The row interpretation requires exactly thirty-two loaded four-byte fields.
`conditional_row` establishes this from actual table metadata and retains the
real file and live-row parent. Other valid positive-width profiles remain
available as raw fields; requesting this interpretation returns `Unsupported`.
Standalone row images require an explicit normalized loaded-width profile and
do not invent a file parent.

Only the first ten signed words receive this interpretation: group identifier,
solver type code and eight original-order integer parameters. The remaining
88 bytes remain raw. Parameter units and physical meanings are not assigned.
The solver lookup preserves all thirty-three known ASCII names, a distinct
matched-null-source state for code zero, and unmatched/no-write states for
other codes. None of this assigns a solver-name field to GEN records.

`project_destination` takes an owned immutable destination. Known names require
room for their terminal NUL and preserve every suffix byte. Unmatched codes
retain the entire destination and projected native return zero. Code zero
returns explicit incomplete-copy completion with no fabricated output, return
or achieved-pattern metric. The native helper's unchecked copy does not run.

`project_egroup` uses a normalized positive signed32 logical ordinal and supplied
signed group offset. A stored identifier mismatch completes a skip. Selected
rows preserve native 32-bit addition wrapping and swap the final two integer
parameters only in the output projection. Known names produce the exact
logical ASCII `EGROUP` line ending in LF. Unknown/null name states remain
explicitly incomplete, with no invented buffer text or line. Native output
functions and a complete solver-file transport are not modeled.

`descriptor_writer_projection` separately proposes tag-one/original-width
pairs and retains original tags/alignment. It leaves the file unchanged and
does not project header reuse, directory writes, output errors or a complete
native save.

## Evidence and reproduction

Fourteen owned tables cover dense/permuted/sparse maps, nontrivial deleted
chains, only-deleted records, unallocated capacity, a 33-slot directory,
sixteen/seventeen-field alignment, shifted mixed widths, whole-row fields and
zero-field/minimal images. All 3,992 existing-live-field intervals match actual
reader selections and independently patched complete-file goldens. Each also
checks authentic no-op identity and complete physical-record history.

The name matrix covers 183 supplied contexts: 150 accepted projections and
33 insufficient-destination errors, including all names, preserved tails,
byte no-ops, zero and signed extremes. The EGROUP matrix covers 936 contexts:
396 complete known-name lines, 468 identifier skips and 72 incomplete name
states. Tests check original row bytes, integer permutation and signed wrapping.

A fresh actual-CFB survey retains complete stream names and hashes for the
three established CWR carriers. None has a named EGR stream. This is zero
native EGR fixtures within that scope, without a claim of global absence.
Synthetic images and static native bindings do not become native file fixtures.

The independently emitted Rust packet matches all 3,693,682 actual-reader
bytes across metadata, descriptors, field edits, names and EGROUP results.
Boundary, ownership, source-drop, foreign-identity and eight-thread repeated
edit/line tests accompany compile-time Send/Sync assertions for all thirty new
public types. Normal tests use owned bytes without filesystem writes.

```powershell
python -B docs/features/simulation-egr/freeze_egr.py
$env:BESSEMER_EGR_PROJECTION_OUTPUT = Join-Path (Get-Location) 'target/simulation-egr-projections.bin'
cargo test --test simulation_egr -- --include-ignored
python -B docs/features/simulation-egr/freeze_egr.py --verify-rust
cargo xtask pr
```

The ignored export requires its explicitly injected path. When including the
USD compatibility export, also set `BESSEMER_USD_PROJECTION_OUTPUT` as documented
in that feature. Baselines require explicit `--write` and review. Source scripts
run sequentially and retain complete copied-module origins/hashes, independent
fixture identities and census scope. Native input carriers remain outside the
repository; their census is read-only.

## Completed local verification

`cargo xtask pr` passed 3,030 tests with no failures and twenty-three
intentionally ignored corpus/export tests across 233 summaries. The focused
run passed all eighteen EGR tests and all fifteen USD compatibility tests,
including both injected exports. Two fresh processes repeated all eighteen EGR
tests and produced identical complete 3,693,682-byte packets matching the
actual readers. All twenty-one indexed Simulation/USD fixture packets matched
the tested bytes.

Fresh runs took 0.3194354 and 0.1118973 seconds including startup, with observed
working-set peaks of 17,575,936 and 18,022,400 bytes. The full local gate was
running concurrently. Sampling around one millisecond gives a lower bound and
may miss actual peaks. This first implementation has no prior published EGR
release or statistical speed claim. `complete-verification.json` records exact
reader/runtime/test hashes, complete owned-case and native-census scope,
history/ownership/concurrency checks and observations. Cross-platform CI is a
separate required merge gate.

## Acceptance limits

This is the explicitly established flat positive-width profile, without a
general suffix-based dispatch or invented transport. Complete EGR semantics,
unassigned words/tags/values, native cache/error behavior, missing-row runtime
returns, complete native saves and native EGR file compatibility remain open.
The conditional solver-name bridge is EGR-specific; GEN binding remains
unproven. Geometry, topology consistency, self-intersection, tolerance validity
and SolidWorks application acceptance are separate. No installed target ran.

No dependencies, package versions or release tags changed. The maintainer
humanizer skill was unavailable; the prose pass remains for review.
