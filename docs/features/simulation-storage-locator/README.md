# Exact UTF-16 Simulation logical storage lookup

Tracking: [#126](https://github.com/Oblikovati/Bessemer/issues/126).
Behavior profile: `simulation-storage-locator-v1`.

This feature follows approved `../inventor-interface` clause 14.100, retained in
`schema-snapshot.md`. The complete copied `simulationstorage` module supplies
reference outputs. Independent literal prefix/suffix builders supply expected
components before freezing. Reader functions are not substituted or monkeypatched.

`SimulationCwrLogicalName::from_units` captures raw UTF-16 code units and
normalizes the last ASCII-dot boundary once. The reproduced profile requires a
NUL-free input containing a dot. Empty/no-dot inputs and embedded NULs return
typed errors; they are not predictions of native failure behavior. Isolated
surrogates and noncharacters remain raw rather than replaced or rejected.

`project_locator` takes the complete original prefix before the final dot,
appends exact literal `.CWR`, and retains the entire original suffix as the
stream name. It searches across the whole input, without stripping directories,
changing case, trimming whitespace, normalizing paths or applying a suffix table.
For example, `a.b\file` projects to storage `a.CWR` and stream `b\file`.
An empty prefix or stream suffix remains a completed lexical result.

The result owns the original name and both exact component arrays. It includes
parent code-unit ranges, complete lexical completion/check scope, checked input
length, storage/stream mode patterns `0x10`, storage format zero and zero
requested-pattern error. Filesystem/COM execution, native name acceptance and
payload decoding remain explicitly false. A projected stream name can require
separate native validation; the lexical result does not establish it.

The normalized name and returned components are immutable snapshots. Caller
input mutation, source drop and mutation of a separately cloned returned array
do not alter earlier values. The last-dot boundary is reused without reparsing
or a dependency on the host operating system's path rules.

Owned fixtures contain 256 complete parent/storage/stream/mode goldens. They
cross empty and dotted prefixes, Windows/forward-slash paths, casing, spaces,
Unicode supplementary characters, unpaired high/low surrogates and noncharacters.
Thirteen excluded input shapes match actual reader guards. Compile-time
Send/Sync assertions cover all four public types; eight threads repeatedly
compare complete source/component units against serial output.

```powershell
python -B docs/features/simulation-storage-locator/freeze_locator.py
$env:BESSEMER_STORAGE_LOCATOR_PROJECTION_OUTPUT = Join-Path (Get-Location) 'target/simulation-storage-locator-projections.bin'
cargo test --test simulation_storage_locator -- --include-ignored
python -B docs/features/simulation-storage-locator/freeze_locator.py --verify-rust
cargo xtask pr
```

The ignored export requires its explicitly injected path. Normal tests use
owned code units without filesystem writes. When including EGR compatibility
exports, also set that feature's `BESSEMER_EGR_PROJECTION_OUTPUT` path. Baselines
require explicit `--write` and review. Source scripts run sequentially and retain
the exact complete reader origin/hash, code-unit fixture identity and clause.

Native COM case matching, stream-name legality, existence-probe wildcard effects,
filesystem resolution, no-dot/NUL behavior and actual storage/payload operations
remain outside this lexical profile. No installed target or native I/O executed.
This does not broaden EGR's separate constructor-bound flat-file transport or
infer a CWR payload grammar from a suffix.

## Completed local verification

`cargo xtask pr` passed with 3,035 passed tests, zero failures and 24 ignored
exports across 234 test summaries. Explicit focused runs passed all six locator
tests, including its export, and all eighteen EGR compatibility tests. Two fresh
processes ran all six locator tests and produced identical 19,400-byte packets;
the actual frozen reference checked every byte (SHA-256
`d8d257cc62b70827c445b59889a6fac4c40f62998931818bbb86b6a259c79922`).
All twenty-two indexed Simulation/USD fixture packets equal their tested bytes.
The reference verifier passed all four checks. Full observations, source hashes,
coverage and exclusions are recorded in `complete-verification.json`.

Fresh-process elapsed times were 0.228402 and 0.0603424 seconds, with sampled
working sets of 5,722,112 and 2,912,256 bytes. These include startup while the
full gate ran concurrently; sampling provides lower bounds and may miss peaks.
There is no prior published locator implementation or statistical speed claim.
Required platform CI and SolidWorks application acceptance remain separate.

No dependencies, package versions or release tags changed. The maintainer
humanizer skill was unavailable; the prose pass remains for review.
