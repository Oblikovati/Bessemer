# Simulation string and scalar metadata

Tracking: [#116](https://github.com/Oblikovati/Bessemer/issues/116).
Behavior profile: `simulation-metadata-v1`.

The implementation follows approved `../inventor-interface` clauses
14.110–14.111, retained in `schema-snapshot.md`. Actual copied CFB,
`simulationguids` and `simulationscalars` readers supply reference projections.
Hashes and import origins are recorded before temporary-source cleanup.
Reader functions and their parsed contexts are not substituted.

## Four stored strings

`SimulationGuidArchive` preserves the four bare byte-counted fields of the
optional root stream `TimeStampGUIDs`. It assigns no GUID syntax or field roles.
Null has count zero and no content; allocated empty has count two and a zero
terminator. Nonzero lengths count bytes, including the final two-byte NUL, in
the supported even range 2..=0x100000. This is the portable even UTF-16 profile;
the inspected native reader itself has no even-length guard.

Raw UTF-16 units preserve supplementary pairs, unpaired surrogates, embedded
NULs and arbitrary stored text. A separate caller-visible view stops before the
first NUL. Stored content is never replaced with that visible prefix. An MFC
CString adapter is unsuitable here because both framing and surrogate/NUL
requirements differ; existing raw-unit conventions are used instead.

All bytes after the fourth field are retained as an unassigned tail. Coverage
explicitly reports `IncompleteUnassignedTail`, rather than complete stream
acceptance. Stored-field edits preserve other frames and that tail and return
all four identity derivations, exact check scope, fidelity, requested-pattern
error and semantic exclusions. Existing-stream CWR GUID publication requires
complete four-field framing and refuses unassigned tails.

`SimulationGuidWriterProjection` models the inspected caller profile separately
from stored framing. All four caller strings must be NUL-free. Null or empty
third/fourth strings suppress creation; the first two may be null or allocated
empty. The creation gate precedes paired-reader length validation, so an unused
oversized first input can be suppressed. Created output obeys the paired
reader limits. The result proposes stream bytes and executes no COM storage
creation or native allocation path.

## Bare scalar words

`SimulationScalarKind` normalizes exact `TimeStamp` and `RunState` names.
`SimulationScalarArchive` reads, constructs, edits and writes exactly four bare
little-endian bytes, with signed and unsigned views. An invented transfer
envelope is rejected.

`TimeStamp` has the inspected model-document `GetUpdateStamp` binding. No clock
unit, epoch, increment rule or cross-session uniqueness is inferred. Current
build metadata does not identify every historical producer. `RunState` retains
arbitrary stored patterns with an unassigned complete enum; the inspected
callers' constant one is not named success, failure or a solver return code.

The supplied-stamp helper projection returns RunState one and the exact signed
update-stamp word. It does not execute command-line conversion, application
state predicates, the native accessor or native storage I/O.

## Complete existing-stream edits

`SimulationMetadataArchive` reads optional known metadata once inside a complete
compound carrier. Scalar edits and complete GUID-field edits preserve unrelated
streams and metadata. Cached component identities are reused only after their
actual resulting bytes or absence have been checked. No-op edits preserve
original carrier bytes and component identities.

`apply_helper_stamp` requires both scalar roots before producing any result.
It applies the inspected order, RunState then TimeStamp, and composes all-entry
carrier history through the intermediate revision. Both component scalar
derivations, coverage and exclusions remain in the result. Missing streams
fail explicitly; this API does not hide unsupported CFB structural insertion
behind a successful operation. Standalone string-writer projection likewise
does not claim to create a missing root stream.

## Verification

Owned fixtures cover seven stored-string cases, every field's independent edit
golden, ten caller-writer cases, fourteen scalar patterns and seven supplied
helper stamps. Cases distinguish null/empty, raw/caller-visible NUL handling,
unpaired/supplementary units, unknown tails, the inclusive length limit and the
writer's pre/post-gate checks. Independent complete CFB carriers cover existing
string edits, scalar edits, helper application and normal-to-mini allocation.

All three installed CWR carriers lack `TimeStampGUIDs`. Populated GUID coverage
is synthetic. Native evidence contains three TimeStamp streams and one RunState
stream, sixteen bytes in total. Actual readers reopen all four native scalar
edits and all six publishable synthetic GUID carrier edits and compare every
unrelated stream and directory metadata byte within the compound codec's
documented allocation-field exclusions. The unknown-tail case is not published.

```powershell
cargo test --test simulation_metadata
python -B docs/features/simulation-metadata/freeze_metadata.py --export
cargo test --test simulation_metadata -- --include-ignored
python -B docs/features/simulation-metadata/verify_scalar_edits.py
python -B docs/features/simulation-metadata/verify_guid_edits.py
cargo xtask pr
```

Run scripts sequentially because the snapshot recorder uses one temporary
report name. Baseline changes require explicit `--write` and review. Tests cover
short/truncated input, malformed lengths/terminators, limits, unknown tails,
foreign IDs, no-op/revert behavior, source mutation, Send/Sync, concurrent edits,
all-parent helper history and complete carrier reopening/preservation.

## Completed verification

The local PR gate passed with 2,950 tests, zero failures and sixteen explicitly
ignored corpus checks. All ten focused tests passed with corpus checks enabled.
Two fresh processes each ran that complete focused workload and produced
byte-identical scalar edit bundles (18,808,888 bytes) and synthetic GUID edit
bundles (1,396,816 bytes). `complete-verification.json` records revisions,
reader/runtime/test hashes, check scopes and exclusions.

The two focused processes took 0.316 and 0.153 seconds including startup.
Observed working-set peaks were 103,247,872 and 111,837,184 bytes; sampling at
approximately one millisecond can miss the actual peak. There is no previous
published implementation for comparison, and these observations make no
statistical speed claim. Required Linux ThreadSanitizer CI remains separate
from the checks executed on this Windows host.

## Acceptance limits

No populated native GUID example or GUID field role is known. Other producer
versions, odd byte-count semantics, native allocation/HRESULT failure behavior,
COM/native runtime and application acceptance remain unproven. Complete state
enums and historical producer semantics remain unassigned. These operations do
not establish geometry, topology, self-intersection or tolerance consistency.

No dependencies, package versions or release tags changed. The maintainer
humanizer skill was unavailable; the prose pass remains for review.
