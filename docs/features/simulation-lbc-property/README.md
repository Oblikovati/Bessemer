# Simulation LBC Property framing

Feature issue: [#128](https://github.com/Oblikovati/Bessemer/issues/128).
Behavior profile: `simulation-lbc-property-v1`. Complete corpus comparison,
focused tests and the full local gate passed; platform CI remains pending.

The approved reference is `../inventor-interface/solidworks/cosworkspersist.py`
and specification clauses 14.330, 14.334 and 14.335. This feature covers the
binary `LBCManager/Property` carrier. Other managers, load collections and
CosmosXpress ancestry require their own documented profiles.

## Context and field order

Require an explicit normalized nonnegative signed-i32 version for the reader
profile. The nine documented sizes are 52, 56, 60, 64, 192, 204, 208, 212 and
216 bytes. Exercise both sides of all eight version thresholds. Preserve the
original field order, including runtime destination +0x494 before +0x484.
The variable array uses ordered count overrides to 14, 15, 16 and 17 words;
counts are not stored in the payload. Runtime destinations label unknown
members and must remain distinct from physical stored offsets.

The installed writer profile is an explicit alternative context: exactly
54 words and 216 bytes in its independently recovered order. It contains no
version word and does not establish a runtime version. Do not use this profile
as an automatic fallback after a version-selected reader fails.

All four-byte patterns retain their signed-i32 view and exact source bytes.
Construction must use the selected profile's exact field count. Same-width
field edits must retain the original profile, complete source and replacement
provenance, ordered history, explicit checks and achieved pattern error.
Publish immutable owned results with revision-bound field identities and
authentic no-op identity preservation.

## Native evidence boundary

Retained C-710 evidence contains 356 streams and 46,568 bytes, with 390 supplied
candidate contexts: 338 successful attempts and 52 failures. Eighteen streams
have no successful retained candidate. Preserve those negative results; do not
choose a different version from payload length. Independent failure assertions
must precede any exception-catching in the verifier so an unexpected rejection
of a successful fixture cannot become an accepted negative case.

C-711 independently records the installed writer's complete member order and
eight modern payload identities. Bind it to its exact C-710 parent hash, verify
every payload hash, and reconstruct all eight in writer order. These are
retained extraction and static-recovery records; no fresh native extraction or
installed writer execution is planned.

Freeze complete approved reader modules and exact clauses. Use independently
specified literal layouts and signed-word patterns at every version threshold,
then compare all retained contexts and full edited images with the actual
frozen reader. Include every truncated prefix, extra-byte cases, foreign IDs,
source/drop/returned-buffer isolation, compile-time Send/Sync, concurrent reuse
and two fresh-process byte comparisons. Record options, hashes, toolchain,
executed checks and exclusions.

Require `cargo xtask pr` and all five platform checks before integration. Keep
one integration PR open and merge it when green. Field business meanings,
historical mismatch causes, geometry and native application read/save acceptance
remain separate from this framing profile. No dependency, package version or
release change is planned. The maintainer humanizer skill is unavailable; the
prose pass remains for review.

## Retained reference checkpoint

Four checks passed against the complete actual frozen reader. All 356 native
payload hashes and 390 retained candidate outcomes matched: 338 accepted,
52 rejected and eighteen unmatched streams. Seventy-two independent signed-word
cases checked the eighteen boundary contexts against literal member orders.
The independent writer order matched all 54 members, its C-710 parent hash
matched exactly, and all eight modern payloads reconstructed byte for byte.
Complete clauses and reader snapshot hashes are retained alongside the reports.
These preflight results are followed by the complete Rust comparison below.
Platform integration gates remain pending.

## Rust implementation checkpoint

The implementation requires an explicit typed reader-version or fixed-writer
profile, preserves all original words and runtime destinations, and constructs
the selected exact layout. Field edits retain the complete source, supplied
replacement value, selected profile and ordered history. Authentic no-ops retain
the original revision and field IDs.

Ten focused tests passed: all threshold sizes and signed patterns; literal
54-member writer order; every truncated prefix and extra bytes; every field's
changed/no-op edit; foreign/out-of-bounds identities; caller/drop/returned-value
isolation; compile-time Send/Sync for all fourteen public types; and eight
threads repeating complete serial edit images and all field relations. Clippy
with warnings denied and the function/file size audit passed. Fresh-process
checks and the full local gate also passed. Platform CI remains required before
integration.

## Complete corpus comparison

The owned fixture covers 6,962 contexts: 95 complete reader/writer images,
6,860 changed/no-op field edits and seven fixed-writer size failures. It uses
all eighteen threshold contexts, the explicit writer profile, signed limits
and nonuniform per-word patterns. Each edit has an independent literal
four-byte splice and full ordered history expectation.

The native packet covers 22,710 contexts: all 390 retained reader candidates,
eight independently identified fixed-writer images and 22,312 complete field
edit/no-op contexts. All 52 recorded candidate failures remain typed errors,
including the eighteen streams without a successful supplied candidate. No
version is inferred from size. Fixed-writer expectations come from the
independent C-711 member order, without supplying an invented native version;
reader expectations use the complete actual frozen `cosworkspersist` module.

Rust reconstructed every accepted image and matched all words, both stored and
runtime offsets, explicit profiles, byte-preserving edits, history and coverage.
The verifier compared every byte of the 63,853,680-byte Rust export, SHA-256
`89c839298a4fb2cd368300a2370492fa93a5fdfdd8440f0a35b6547bc6c01482`.
All five reference checks passed, including a run without `--write` against
unchanged baselines. All 31 persistence compatibility tests also passed with
their export path supplied. Native payloads remain untracked under `target/`;
only the synthetic fixture is committed.

```powershell
python -B docs/features/simulation-lbc-property/freeze_lbc_property.py
$env:BESSEMER_LBC_PROPERTY_NATIVE_INPUT = Join-Path (Get-Location) 'target/simulation-lbc-property-native.bin'
$env:BESSEMER_LBC_PROPERTY_PROJECTION_OUTPUT = Join-Path (Get-Location) 'target/simulation-lbc-property-projections.bin'
cargo test --test simulation_lbc_property -- --include-ignored
python -B docs/features/simulation-lbc-property/freeze_lbc_property.py --verify-rust
cargo xtask pr
```

The ignored export requires both injected paths. Baseline creation/replacement
requires explicit `--write` and review. Source checks run sequentially against
complete frozen modules. These results establish bare-stream framing and
preservation, with the historical mismatches and application acceptance
exclusions retained above.

Two fresh native test processes each passed all ten tests with the native
export enabled. Their complete packets equal each other and the verified Rust
output byte for byte. Elapsed times were 0.7718302 and 0.691968 seconds; sampled
working sets were 140,734,464 and 149,102,592 bytes. Startup is included, the
full local gate ran concurrently, and sampling gives lower bounds that may
miss actual peaks. There is no prior published implementation or statistical
speed claim. All twenty-four indexed Simulation/USD/persistence fixture packets
equal their tested bytes.

## Completed local gate

`cargo xtask pr` passed with 3,075 passed tests, zero failures and 26 ignored
exports across 236 test summaries. It checked formatting, ran workspace and
all-target Clippy with warnings denied, built the workspace and ran workspace
tests. Explicit focused runs covered all ten LBC tests and all 31 persistence
compatibility tests with both exports enabled. `complete-verification.json`
records source and fixture identities, options, checks, fresh-process
observations and acceptance exclusions. No dependencies, package versions or
release tags changed.
