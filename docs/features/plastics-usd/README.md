# Lossless Plastics USD records and explicit writer layout

Tracking: [#123](https://github.com/Oblikovati/Bessemer/issues/123).
Behavior profile: `plastics-usd-records-v1`.

This profile follows approved `../inventor-interface` clauses 14.187–14.198
and 14.201, retained in `schema-snapshot.md`. The complete copied `plasticsusd`
module supplies reference framing and layout outputs. Independent literal
headers, code-unit payloads and writer sequences supply expected values before
those outputs are frozen. Reader functions are not substituted or monkeypatched.

## Complete counted records

`UsdArchive::from_bytes` frames every supplied byte using explicit caller
record and per-payload limits. An empty stream completes successfully. A
truncated record fails without publishing a partial archive. `#EOF` text does
not stop framing; following records remain visible. Revision-bound
`UsdRecordId` values reject foreign revisions even when their source bytes are
identical.

`UsdStringRecord` retains the complete original length header, character width,
u64 declared count, raw payload and source offsets. Single-byte payloads remain
opaque because the native source code page depends on runtime context. UTF-16
access exposes raw u16 units, including counted NULs, isolated surrogates and
surrogate pairs. It does not replace invalid units, trim values or infer text
termination.

The decoder retains noncanonical byte/word/dword/qword length forms. After the
Unicode marker, a word count `0xfffe` is a count rather than another marker.
Escaped u64 counts are checked for multiplication, addressability and payload
limits before slicing or allocating a payload. Impossible huge counts fail
without narrowing to u32. Existing decoded `ArchiveString` behavior remains
unchanged: its text/code-page and count contract cannot represent these opaque
values faithfully. The new parser reuses the existing checked archive cursor.

`from_units` and `from_narrow_bytes` construct shortest documented provider
prefixes from explicit caller payloads. `from_bytes` accepts exactly one
complete encoded record while preserving its original prefix width.
`UsdArchive::from_records` constructs complete streams from those exact record
images, enforcing caller limits before reserving output storage.

## Isolated edits and history

`edit_record` replaces one complete encoded record. It validates complete
output framing and compares every unchanged record byte for byte before
publication. The result retains the original archive, supplied replacement,
complete output, source/result identities, exact check scope, ordered copied
or derived relationships and zero requested-pattern error. Derived records
retain both the original position and the complete supplied replacement parent.
An exact byte no-op retains the original archive and record identities.

This is a portable byte operation. It does not execute a native save, code-page
conversion or loader. Captured values remain valid after caller/source drop,
returned-byte mutation, failed follow-up operations and independent edits.

## Explicit Unicode writer grammar

`writer_layout` checks the complete documented `75 + 10 * adapter_count`
Unicode record sequence. It validates every required separator, label, empty
record and final `#EOF` marker. Narrow values, missing or extra records, changed
fixed literals and a NUL-extended EOF fail this explicit grammar check even
when their raw record framing is complete.

The projection retains its complete source and typed record identities for
the host, volume serial, every ten-record adapter group, five customer values
and twelve labeled profile values. Coverage records every checked record/byte
and all `56 + 3 * adapter_count` fixed literal records. Variable values remain
unparsed and untrimmed. This checks the writer structure; native loader
dispatch, historical profiles, code-page conversion and value-domain acceptance
remain separate.

```rust
use bessemer::plastics::{UsdArchive, UsdLimits, UsdStringRecord};
let limits = UsdLimits {
    maximum_records: 1_000,
    maximum_payload_bytes: 1_000_000,
};
let record = UsdStringRecord::from_units(&[0x41, 0, 0xd800], 6)?;
let archive = UsdArchive::from_records(&[record], limits)?;
let replacement = UsdStringRecord::from_narrow_bytes(&[0x80, 0], 2)?;
let edited = archive.edit_record(&archive.record_id(0)?, &replacement)?;
assert_eq!(archive.record_count(), edited.archive.record_count());
# Ok::<(), bessemer::BessemerError>(())
```

## Evidence and reproduction

Owned fixtures cover 49 canonical/noncanonical record forms, four complete
writer layouts with zero, one, two and seven adapters, and fifty literal
complete-stream edit goldens including an exact no-op. All 400 records in the
four layouts are individually changed: 254 fixed records must fail the grammar
check, while 146 variable records remain accepted without content parsing.
Focused guards cover small-record prefixes, large headers/terminal truncation,
payload/count limits, u64 overflow, source ownership, foreign identities and
concurrent record edits and layout projections. Compile-time Send/Sync
assertions cover every new public type.

The native-derived 88-byte separator prefix is checked against its hash-bound
evidence JSON and the actual reader. It remains a prefix model, not a complete
installed file fixture. No complete native USD fixture is established.

The complete independently emitted Rust record/layout/edit packet matches
5,732,860 actual-reader bytes. It contains every original record header and
payload, writer-layout identity/label projection, complete edited stream and
resulting record boundary. Inputs and expected edits are synthetic; this does
not claim native writer execution or full-file native compatibility.

```powershell
python -B docs/features/plastics-usd/freeze_usd.py
$env:BESSEMER_USD_PROJECTION_OUTPUT = Join-Path (Get-Location) 'target/plastics-usd-projections.bin'
cargo test --test plastics_usd -- --include-ignored
python -B docs/features/plastics-usd/freeze_usd.py --verify-rust
cargo xtask pr
```

The ignored export test requires the explicitly injected output path. Normal
tests use owned bytes without filesystem writes. Baselines require explicit
`--write` and review. Snapshot scripts run sequentially because their recorder
shares a temporary report name. Reports retain actual reader origins, hashes,
independent fixture identities and acceptance scope.

## Completed local verification

The final `cargo xtask pr` gate passed 3,013 tests with no failures and
twenty-two intentionally ignored corpus/export tests across 232 summaries.
The focused run passed all fifteen USD integration tests including the injected
export, and both private count/offset guard tests passed. Two fresh processes
repeated all fifteen integration tests and emitted identical 5,732,860-byte
packets matching the actual reader. All seventeen indexed Simulation/USD
fixture packets matched tested bytes.

Fresh runs took 0.1104573 and 0.0863342 seconds including startup, with observed
working-set peaks of 7,540,736 and 23,031,808 bytes. No local gate was running
during these observations. Sampling around one millisecond gives a lower bound
and may miss actual peaks. This is the first implementation, with no previous
published release or statistical speed claim. `complete-verification.json`
records source/runtime/test hashes, complete owned-case scope, the native-derived
prefix limitation, history/ownership/concurrency checks and observations.
Required cross-platform CI remains a separate merge gate.

## Acceptance limits

This profile is the Plastics diagnostic archive, without an assumed relation
to another product's similarly named USD format. Whole-project native
validity, loader state/reset/dispatch, source code pages and conversion,
host/adapter/customer/system value semantics, native file creation and native
application acceptance remain open. Geometry, topology consistency,
self-intersection and tolerance validity are outside these record operations.
No installed target, diagnostic collector or host inventory command ran.

No dependencies, package versions or release tags changed. The maintainer
humanizer skill was unavailable; the prose pass remains for review.
