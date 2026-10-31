# Auxiliary names and explicit runtime workspace projections

Tracking: [#121](https://github.com/Oblikovati/Bessemer/issues/121).
Behavior profile: `simulation-auxiliary-runtime-v1`.

The implementation follows approved `../inventor-interface` clauses
14.118–14.119, retained in `schema-snapshot.md`. Actual copied
`simulationauxiliary` and `simulationauxruntime` modules supply reference
outputs. Native rows and trial slices come from actual complete CFB/MAS/GEN
readers with their real paired context. Reader functions and contexts are not
substituted or monkeypatched.

## Stored UTF-16 buffers

`SimulationAuxiliaryBodyRow::name_view` exposes the full 256-byte name array,
all 128 raw little-endian u16 units, signed vector count, opaque prefix and two
separate trailing words. The name begins at stored offset twenty for profile
284 and twenty-four for profile 288. Other row fields retain their existing
typed views and exact bytes.

The bounded prefix scans only units zero through 126, stopping before the first
NUL. Unit 127, the 128th unit, remains outside that scan even when it contains a
NUL. Unconsumed bytes include the encountered terminator and all residual data.
UTF-16 units preserve unpaired surrogates and split pairs without replacement.
They do not imply native MFC trimming or a final returned CString; native trim
policy, row-selection guards and CAD name-array provenance remain unassigned.
Trailing spaces are retained rather than passed through a Unicode trim rule.

`project_name_store` uses the complete supplied row as an explicit destination
image. It copies at most 127 caller code units, stopping at source NUL, then
writes one terminator. Every remaining name byte and all other row bytes survive.
The result includes exact check scope, fidelity, authentic no-op identity,
parent/result history, zero requested-pattern error and string/provenance
exclusions. This models the bounded store stage; it does not prove a native
producer updated a stored row in place. The inspected producer used a temporary
buffer and copied the full array, whose historical initial contents are unknown.

## Explicit normalized memory images

`SimulationAuxiliaryWorkspace` owns exactly 288 supplied bytes. Numeric slot
bits are never cast to a host pointer, dereferenced or used as a portable file
reference. Modern normalization copies the complete stored image and ignores
prior workspace input. Legacy normalization requires an explicit prior image
and copies only stored spans 0..8, 12..20 and 20..284 to normalized offsets zero,
sixteen and twenty-four. Prior workspace bytes 8..16 survive unchanged.
The stored legacy word 8..12 is not copied, but remains in the retained source
row and `legacy_word_discarded` field; its historical meaning is not inferred.

Normalization retains both input snapshots, GEN copy parents when known and
exact nonoverlapping span history for all 288 destination bytes. No legacy
slot is fabricated as zero. `SimulationGenArchive::auxiliary_trial_workspace`
captures the actual original 288-byte trial slice at auxiliary start plus
logical row index times 288, including bytes outside a legacy stored-row span
when present. It retains the real GEN revision and trial stream offset. It does
not substitute a 284-byte stored row or a seeded zero workspace.

## Allocation plans and supplied-result binding

`SimulationAuxiliaryLogicalIndex` normalizes a positive signed-i32 one-based
position independently of stored identifier bits. `allocation_plan` selects by
identifier equality. Selected count zero still requires the allocation-call
path and a zero-byte delegated read; `Some(0)` remains distinct from a skipped
`None` request. Selected counts must be 0..=0x1fffffff so four-byte read sizes
fit the supported signed-width profile. Mismatches skip count consumption,
including negative stored counts.

`project_pointer_binding` requires explicit u64 allocation-result bits for every
selected row, including count zero. It changes only memory bytes 8..16. A
mismatch preserves the entire image without requiring or consuming a result.
Completion distinguishes a skipped gate, a selected byte no-op and a selected
change. The complete source/result identities, plan, exact check mode/fidelity,
zero slot-pattern error and unexecuted-runtime diagnostics accompany the result.
No allocation, delegated read or file write executes. A supplied zero result
does not assert that a native zero-size allocator returned null.

`normalize_and_bind` composes both stages. It retains the original stored row,
prior workspace, normalization and binding histories, and final byte-span
parents. Selected slot bytes derive from the captured supplied result; every
other byte derives from its original stored/prior source. This is a memory-only
projection, not a native allocation or a completed GEN publication.

## Evidence and reproduction

Owned fixtures cover sixteen complete name images, 192 literal name-store
goldens, 112 row/prior-workspace pairs, 448 plans and 2,240 binding contexts.
They include zero counts, ID mismatches, signed count limits, absent results,
nonzero u64 patterns, surrogate splits, embedded NULs, ignored unit 128 and
preserved trailing spaces. Actual reference outputs agree with independent
byte builders before they are frozen.

All five installed arrays are retained: 1,280 name bytes comprise 146 prefix
bytes including their first NULs and 1,134 residual bytes, 565 nonzero. Replaying
each stored prefix with its actual destination tail reconstructs the complete
stored row. All five plans select zero-length requests. Four legacy prior
workspaces come from exact original trial-read slices. An explicitly synthetic
supplied result zero demonstrates selected slot replacement while retaining
every other normalized byte; no historical allocation result is inferred.
Populated positive-length native vectors remain unavailable.

```powershell
python -B docs/features/simulation-auxiliary-runtime/freeze_auxiliary.py --export
cargo test --test simulation_auxiliary_runtime --test simulation_body_ranges -- --include-ignored
python -B docs/features/simulation-auxiliary-runtime/freeze_auxiliary.py --export --verify-rust
cargo xtask pr
```

The second source run compares the complete independently emitted Rust native
name/replay/trial/normalization/plan/binding packet byte for byte. Baselines
require explicit `--write` and review; snapshot scripts run sequentially because
their recorder shares a temporary report name. Native inputs stay in ignored
`target/`; ordinary tests use owned bytes, including complete earlier GEN
fixtures to pin original trial slices under both stored profiles. Prior body
range tests pin the shared row-copy behavior. Source mutation, source drop,
failure isolation, Send/Sync and concurrent serial-byte comparisons are covered.

## Completed local verification

`cargo xtask pr` passed 2,991 tests with no failures and twenty intentionally
ignored corpus tests across 230 test summaries. The focused run passed all
fourteen auxiliary runtime tests and all twelve earlier body-range tests,
including their native cases. Two fresh processes repeated all fourteen new
tests and emitted identical 10,460-byte projection packets, matching the entire
actual-reader packet. All twelve indexed Simulation fixture packets matched
the tested working bytes.

The fresh runs took 0.4164914 and 0.151702 seconds including startup. Observed
working-set peaks were 12,394,496 and 8,998,912 bytes. Sampling around one
millisecond gives a lower bound and may miss the actual peak. This is the first
implementation, with no previous published release or statistical speed claim.
`complete-verification.json` records source/runtime/test hashes, exact corpus
coverage, replay and isolation checks, observations and exclusions. Required
cross-platform CI remains a separate merge gate.

## Acceptance limits

Native MFC trimming and returned names, CAD/name identity, historical temporary
buffers, discarded-word/residual/trailing-word meanings, allocator outcomes,
native I/O failure handling and vector element domains remain unassigned.
Complete auxiliary/descriptor semantics and application acceptance remain open.
These byte projections do not establish geometry, topology consistency,
self-intersection or tolerance validity. No stored/result bits are dereferenced,
and no native allocator, reader, producer or installed target was executed.

No dependencies, package versions or release tags changed. The maintainer
humanizer skill was unavailable; the prose pass remains for review.
