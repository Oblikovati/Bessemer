# Simulation counted scalar framing

Feature issue: [#129](https://github.com/Oblikovati/Bessemer/issues/129).
Behavior profile: `simulation-persistent-scalars-v1`. Complete corpus comparison,
focused tests and the full local gate passed; platform CI remains pending.

The approved references are the complete `cosworkspersist.py` module and
`research/cosworks_study_versions.py` in `../inventor-interface/solidworks`.
Specification clauses 14.267, 14.271, 14.272, 14.276 and 14.444 describe the
selected bare-stream profiles and their runtime interpretation boundaries.

## Profiles and limits

StudyVersionNumbers is a signed little-endian i32 count followed by exactly
that many signed i32 values. A completed empty array is four zero bytes.
Preserve duplicate values, signed bit patterns and order. Reject negative or
over-limit counts, missing values and trailing bytes before decoding entries.

UniqueIds is a signed i32 used count, signed i32 allocation capacity and exactly
used-count signed i64 keys. Require nonnegative used count, capacity at least
used count, the caller's used-entry limit and exact EOF. Capacity is retained
metadata; it never controls allocation or payload consumption. An empty table
can preserve capacity i32::MAX in its eight-byte header. Preserve duplicate
keys, nonzero high words, signed extrema and original order.

Require explicit profile selection and entry limits. The existing persistence
array reader defaults to 10,000 entries while the newer standalone study-array
reader defaults to 100,000; Bessemer must compare both under the same supplied
limit. These resource defaults do not establish native runtime policy.

## Read and write acceptance

Use immutable owned archives and typed revision-bound field IDs. Construct
complete valid headers and payloads from explicitly typed values. Edits to an
entry or allocation capacity must preserve the selected profile and every
untouched byte, retain original source/replacement parents, and return complete
ordered history, explicit check scope, diagnostics and achieved pattern error.
Changing the used/count field requires construction of the matching complete
payload; a count word must never publish an inconsistent table.

Authentic no-ops retain identities. Compile-time Send/Sync, concurrent serial
equivalence and caller/source/drop/returned-value isolation are required.
Version-to-release mapping, absent-stream fallback, per-study version selection,
native last-key caching, one-based key mapping and CAD reference resolution are
separate operations; this framing work supplies none of their missing context.

## Reference and corpus gates

Use current C-824 evidence: 51 version arrays, 82 values and 532 bytes. The
current evidence stores raw bytes and values inside each row's `result`; do not
assume the superseded C-643 row shape still exists at that path. Compare both
actual frozen readers and the retained hashes and ordered values.

C-648/C-652 provide 51 UniqueIds tables, 443 keys and 3,952 bytes. Verify both
counts, every raw payload hash and the independently retained adjacent word
pairs before reconstructing complete inputs. Add independent synthetic signed
limits, duplicate/high-word keys, zero-used/large-capacity tables and every
malformed header/length/resource boundary. Compare complete edited images and
all history relations, then compare two fresh processes byte for byte.

Freeze complete reader modules and exact clauses; retain reference identities,
options, executed tests and exclusions. Run `cargo xtask pr` and require all
five platform gates before integration. Keep one integration PR open and merge
when green. No dependency, package version or release change is planned. The
maintainer humanizer skill is unavailable; the prose pass remains for review.

## Reference preflight checkpoint

Seven checks passed against complete frozen copies of both actual readers.
All 51 current C-824 arrays reconstructed exactly, preserving 82 values across
532 bytes. Both array readers matched under an explicit 10,000-entry limit.
All 51 UniqueIds tables reconstructed exactly, preserving 443 keys across
3,952 bytes and matching every retained adjacent word pair. The C-648 inventory
hash matched its independent C-652 checkpoint.

Independent checks passed for empty arrays, duplicate/signed-limit values,
nonzero and negative high words, zero-used tables with capacity i32::MAX,
malformed prefixes/counts/capacities, surplus bytes and reduced resource limits.
A default run matched all initial baselines without `--write`. Complete reader,
clause and current inventory identities are retained alongside the reports.
The Python file/function size audit passed. These checks establish the reference
boundary; they do not yet verify a Bessemer implementation.

## Rust implementation checkpoint

Explicit CountedI32 and UniqueIdsI64 profiles retain complete raw headers and
payloads. Typed stored fields include used count, optional allocation capacity
and every ordered entry. Construction derives the count from the supplied
complete payload. Same-width entry/capacity edits preserve all untouched bytes,
retain source and replacement parents, and return ordered history for headers
and entries. Identical count edits are authentic no-ops; changed counts require
construction of a matching complete payload.

Thirteen focused tests passed, covering signed limits and duplicate values, complete
literal bytes, empty/maximum-capacity framing, malformed prefixes and limits,
entry/capacity/count behavior, foreign IDs, caller/drop/returned-value isolation,
compile-time Send/Sync for all fifteen public types, and eight threads repeating
complete serial edits and history. Two private tests passed for extent overflow
and count/limit normalization before allocation. Workspace/all-target Clippy
passed with warnings denied; the function/file size audit passed. Fresh-process
checks and the full local gate also passed. Platform CI remains required before
integration.

## Complete corpus comparison

The owned packet contains 2,592 contexts: 278 complete reads, 1,786 accepted
field edits/no-ops, 483 rejected edits and 45 rejected reads. Cases include
explicit 10,000- and 100,000-entry limits, signed extrema, duplicates, high-word
keys, metadata-only maximum capacity, every representative truncated prefix,
surplus bytes and invalid count/capacity changes.

The native packet covers all 102 retained carriers and 1,254 accepted complete
field edits/no-ops, with 153 rejected count/capacity edits. It preserves every
original count/header, ordered value, capacity and adjacent key word. Native
payloads remain untracked under `target/`; only the owned fixture is committed.
Both actual frozen readers check every accepted i32 image under the same
explicit limit. The complete frozen persistence reader checks every accepted
i64 table. Every edit has an independent literal splice and ordered history
expectation covering both headers and entries.

All 10,934,848 bytes of the Rust projection matched the reference packet,
SHA-256 `9291b550782dac2de1bf7e5642b396cb52f3d93e404c7ed3b00d18c179138757`.
All eight reference checks passed against unchanged baselines on the default
run. One earlier pin attempt stopped before import when an unrelated upstream
file changed during copying; a fresh complete snapshot passed. The guard was
retained. All ten LBC Property compatibility tests also passed with their
explicit native export enabled.

```powershell
python -B docs/features/simulation-persistent-scalars/freeze_scalars.py
$env:BESSEMER_PERSISTENT_SCALARS_NATIVE_INPUT = Join-Path (Get-Location) 'target/simulation-persistent-scalars-native.bin'
$env:BESSEMER_PERSISTENT_SCALARS_PROJECTION_OUTPUT = Join-Path (Get-Location) 'target/simulation-persistent-scalars-projections.bin'
cargo test --test simulation_persistent_scalars -- --include-ignored
python -B docs/features/simulation-persistent-scalars/freeze_scalars.py --verify-rust
cargo xtask pr
```

The ignored export requires both injected paths. Baselines require explicit
`--write` and review. Source scripts run sequentially against complete frozen
modules. No native allocator, mapper, writer or application executes. These
results establish bare-stream framing and exact preservation; runtime version
policy, key mapping and native application acceptance remain separate.

Two fresh native test processes each passed all thirteen tests with the native
export enabled. Their complete packets equal each other and the Rust output
checked by the reference, byte for byte. Elapsed times were 1.2383677 and 0.9308386
seconds; sampled working sets were 40,566,784 and 44,105,728 bytes. Startup is
included, the full local gate ran concurrently, and sampling gives lower bounds
that may miss actual peaks. This is the first implementation; there is no prior
published baseline or statistical speed claim. All twenty-five indexed
Simulation/USD/persistence fixture packets equal their tested bytes.

## Completed local gate

`cargo xtask pr` passed with 3,089 passed tests, zero failures and 27 ignored
exports across 237 test summaries. It checked formatting, ran workspace and
all-target Clippy with warnings denied, built the workspace and ran workspace
tests. Explicit focused runs covered all thirteen scalar tests, both private
overflow tests and all ten LBC compatibility tests with both exports enabled.
`complete-verification.json` records revisions, source and fixture hashes,
options, checks, fresh-process observations and acceptance exclusions. No
dependencies, package versions or release tags changed.
