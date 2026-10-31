# Cosworks persistence framing

Feature issue: [#127](https://github.com/Oblikovati/Bessemer/issues/127).
Behavior profile: `cosworks-persistence-framing-v1`. The full local gate passed;
platform CI acceptance remains pending.

The approved reference is `../inventor-interface/solidworks/cosworkspersist.py`
and specification clauses 14.262–14.263, 14.329 and 14.332. These describe
bounded text framing, complete study-order lists and version carriers. They do
not establish complete Simulation object semantics or native save compatibility.

## Required behavior

Text records retain their four-byte little-endian byte count and complete
payload, including the final two zero bytes. Null count zero differs from an
allocated empty count two. Count one, missing headers, truncated payloads,
missing terminators and configured resource-limit violations fail explicitly.
Odd payload lengths remain readable as bytes without a UTF-16 view. Even
payloads expose raw code units, preserving embedded NULs and isolated surrogates.
Nonzero offsets preserve the original parent and consumed boundary.

Study-order streams contain a leading text, a raw four-byte count and exactly
that many text records, followed by EOF. Preserve duplicate and null items in
order. Check minimum remaining headers before allocating or iterating. The
leading name receives no active-study interpretation.

Primary `CosworksVersoin` decoding requires a normalized nonnegative i32
numeric context and an explicit full or partial mode. Apply the exact
800018/800019, 800076/800077 and 830031/830032 word boundaries. Retain every
consumed signed word, its raw bytes, ordered runtime owner updates and the
unconsumed suffix. Full mode requires EOF; partial mode preserves the remainder.
Runtime owner destinations are distinct from stored field offsets.

Legacy `CosworksVersoinNumber` supports the documented 128-byte profile:
six UTF-16LE ASCII digits, one zero code unit and 114 uninterpreted suffix bytes.
Other numeric syntaxes remain unsupported. Carrier availability is explicit:
a present primary wins, including when malformed; only an unavailable primary
permits legacy selection. Both unavailable select compatibility value 500070.
An empty carrier is present and malformed. Do not infer availability from
arbitrary storage errors or implement an unverified `wcstod` conversion.

## Read and write acceptance

Publish immutable owned records with exact framing reconstruction. Edits must
preserve untouched bytes and return completion, diagnostics, explicit validation
coverage, achieved byte error and complete source/replacement provenance.
Resource limits apply before copying replacement payloads. Authentic no-ops
retain identity. Successful and failed edits leave inputs isolated. No
cancellation API is introduced. This work does not change existing GUID-string framing, whose
even-length and four-field constraints describe a separate format.

Freeze complete approved reader modules and the exact clauses before producing
baselines. Verify hashes before decoding retained evidence: C-639 contains 262
version and 335 study-order payloads; C-709 covers 262 primary and 25 legacy
payloads. Those counts are reference claims awaiting Bessemer's own replay.
Use independent literal expectations for null, empty, odd-length, surrogate,
duplicate-item, signed-word and version-boundary cases. Reject every truncated
prefix and unexpected suffix where the declared mode requires EOF.

Compile-time assertions must cover all public types' Send/Sync contracts.
Concurrent reads and edits must equal serial results; two fresh processes must
produce byte-identical complete projections. Record reference hashes, fixture
hashes, toolchain, options, executed tests and exclusions. Require `cargo xtask pr`
locally and all five required CI checks before integration. Kernel geometry and
SolidWorks application acceptance remain separate from byte framing acceptance.

No dependency, package version or release change is planned. The maintainer
humanizer skill is unavailable; the prose pass remains for review.

## Reference replay checkpoint

`freeze_persistence.py` copies the complete approved Python source tree before
importing its unchanged `cosworkspersist` reader. Five checks passed: all 597
primitive/study payloads reconstructed across 25,988 bytes and 1,031 text spans;
all 262 primary carriers matched independent word boundaries and ordered owner
updates in both modes; all 25 legacy carriers preserved their full opaque
suffixes; and all 21 coexisting carrier pairs selected primary. Complete clause
and retained-inventory hashes accompany the source snapshot. A second run
without `--write` passed against the unchanged initial baselines.

These preflight results precede the complete Rust corpus comparison below.
The full local gate passed; platform CI remains required before integration.

## Rust implementation checkpoint

The working implementation reads and constructs raw counted text, complete
study-order streams, primary carriers with explicit full/partial context and
the observed legacy carrier. Availability-based selection preserves malformed
primary errors. Study text replacement checks the complete result and every
byte outside the replacement, retains both original inputs, and returns
ordered history for the leading text and every item. Authentic no-ops retain
their revision and text identities. Parsed study texts share one immutable
owned source allocation.

Thirty-one focused integration tests passed, covering literal bytes, all
three primary thresholds, signed words, raw suffixes, legacy rejection cases,
study counts, malformed prefixes, isolated edits and concurrent reuse. All
nineteen public types have compile-time Send/Sync assertions. The private
terminated-size overflow regression passed. Clippy passed with warnings denied;
the function/file size audit passed.

## Complete corpus comparison

The owned fixture contains 1,497 cases: null and empty text, every possible
leading opaque byte in odd frames, isolated surrogate units, nonzero offsets,
complete study lists, all version gates and exact edit splices. The native
packet contains 5,012 cases drawn from the hash-bound retained evidence,
including 262 primitive version texts, 335 complete study lists, 524 primary
reads across both modes, 25 legacy carriers, 21 primary-precedence pairs and
3,845 study-text edit/no-op contexts. Native payloads remain under `target/`;
only the owned synthetic fixture is committed.

Rust reconstructed every complete carrier and matched all text parents,
boundaries, raw unit views, words, modeled assignments, unconsumed/opaque bytes,
edited images and ordered history relations. The actual frozen reference
checked the complete 4,486,980-byte Rust projection packet, SHA-256
`004c89cb2624da2e2bf4e1030e1cc7af42e756ca04c094b3c99f4f1294e1e597`.
The six reference checks passed. Focused compatibility testing also passed all
six locator tests, including its explicitly injected export.

Two fresh native test processes each passed all 31 tests, including the injected
native export. Their complete packets equal each other and the reference-checked
Rust output byte for byte. Elapsed times were 1.2728379 and 0.5700576 seconds;
sampled working sets were 15,200,256 and 15,081,472 bytes. Startup is included,
the full local gate ran concurrently, and sampling provides lower bounds that
may miss actual peaks. This is the first implementation; no previous published
baseline or statistical speed claim is available.

```powershell
python -B docs/features/cosworks-persistence/freeze_persistence.py
$env:BESSEMER_PERSISTENCE_NATIVE_INPUT = Join-Path (Get-Location) 'target/cosworks-persistence-native.bin'
$env:BESSEMER_PERSISTENCE_PROJECTION_OUTPUT = Join-Path (Get-Location) 'target/cosworks-persistence-projections.bin'
cargo test --test simulation_persistence -- --include-ignored
python -B docs/features/cosworks-persistence/freeze_persistence.py --verify-rust
cargo xtask pr
```

The ignored export requires both injected paths. Baselines change only with
explicit `--write` during reviewed fixture preparation. Source checks run
sequentially against complete frozen modules. Native open errors, broader
legacy numeric syntax, CString conversion, field business meanings, fresh
whole-document extraction and SolidWorks read/save acceptance remain outside
this bare-stream framing profile. No installed target executed.

## Completed local gate

`cargo xtask pr` passed with 3,066 passed tests, zero failures and 25 ignored
exports across 235 test summaries. It ran formatting, workspace/all-target
Clippy with warnings denied, the workspace build and workspace tests. Explicit
focused runs separately covered the persistence and locator exports. All
twenty-three indexed Simulation/USD/persistence fixture packets equal their
tested bytes. `complete-verification.json` records revisions, source and fixture
hashes, options, checks, fresh-process observations and acceptance exclusions.
