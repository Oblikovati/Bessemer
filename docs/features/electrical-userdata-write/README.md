# Existing userdata text-cell writes (#238)

`ElectricalArchive::edit_userdata_text` commits a unique-key TEXT assignment to
an existing `use_data0`–`use_data19` cell on an isolated SQLite/carrier copy.
It retains complete stored-database checks, typed coverage, logical cell/member
history and an exact immutable source checkpoint. Overall schema parity remains
incomplete.

```rust,ignore
let slot = ElectricalUserdataSlot::new(3)?;
let key = ElectricalUserdataKey { object_id: 26, object_no: -1 };
let edit = archive.edit_userdata_text(&member, key, slot, "new text", limits)?;
let output = edit.result().to_bytes();
let original = edit.source().to_bytes();
```

## Implemented contract

The signed key is compared by SQLite without ID-gate normalization. A missing
or ambiguous match fails; the writer does not choose a first duplicate. A checked
slot supplies the only generated column suffix, while keys and text are bound
parameters. Numeric affinity that changes the requested TEXT type fails exact
result verification rather than being accepted as a provider conversion.

Every operation checks the complete source image and schema, rejects triggers,
hidden/generated columns and virtual/shadow tables, and validates all foreign
keys. It snapshots every ordinary table and application header metadata before
writing. An operation-local defensive memory connection enforces foreign keys
and bounds page growth before the isolated transaction is committed.

The serialized output is reopened independently. Full image integrity, schema
definitions/root pages, application ID, user/schema versions, encoding, page size,
auto-vacuum mode, foreign keys and every stored table are checked. Exactly one
requested TEXT cell may differ; all other values, columns, row counts and selected
scan order must remain equal. Unsupported cascades or constraint failures publish
no result. Views retain their schema definitions; their computed rows are outside
the stored-data comparison.

The existing SQLite limits bound each parse/query, with a shared output budget
across complete source/result snapshot validation. Page growth is bounded by the
image byte limit. Output accounting does not claim peak engine/allocator memory.
Unchanged TEXT skips mutation and retains exact original bytes. Changes may
rewrite the complete SQLite image; fresh opposite edits need not restore its
physical encoding. `source()` retains the exact carrier checkpoint, including
the original database and compressed representation.

The result composes the existing ZIP edit/history with before/after cell values,
key, slot and source/result occurrence identities. Encoded unrelated members
remain unchanged. Native recordset conversions, migrations, application loading,
EWG internals and geometry are explicitly unverified.

## API validation

Fifteen owned fixtures cover ordinary and WITHOUT ROWID tables, TEXT-key affinity,
NULL/blob replacement, exact no-ops, page growth, duplicate keys, numeric target
affinity, CHECK/foreign-key constraints, cascades, triggers and generated/virtual
tables. Independent Python SQLite expectations retain all ordinary-table values.
Focused Rust tests also cover absent keys, stale revisions, malicious-looking
bound strings, slot/value/VM/page/output limits, exact budget boundaries and
eight-worker byte agreement with a serial result.

The explicit native census passed all 680 original key/slot edits and 680 exact
no-ops in 100.34 seconds. Each result reopened, retained every other stored value,
passed complete checks and preserved encoded unrelated ZIP payloads and the source
checkpoint. Eight first/last-slot native operations also run in the regular gate.
Two fresh processes produced identical owned/representative-native results and
typed errors. These are verification timings, not a comparison with an earlier
release; no earlier userdata writer exists.

```powershell
python -m unittest discover -s docs/features/electrical-userdata-write -p 'test_*.py' -v
cargo test --test electrical_userdata_write --locked
cargo test --test electrical_userdata_write --locked complete_native_key_and_slot -- --ignored --nocapture
```

`api-local-verification.json` and `api-process-verification.json` retain source,
log and output hashes. Whole-suite validation for this new source remains pending;
the completed 13931532 gate excludes this writer and the later direct field maps.

## Completed experiment

The investigation used the four frozen original Electrical template archives
and all 34 source-bound drawing keys. It replaced each of the twenty
`use_data0`–`use_data19` cells at every key: 680 committed in-memory edits.
Replacement strings included a key/slot marker, embedded NUL and Unicode text.
Original archive files were rehashed after the experiment and remained unchanged.
No original SQLite file or native application was written or executed.

Every original database contains 48 ordinary tables, 52 indexes and 22 views,
with no triggers and no foreign-key violations. The test retained all schema
rows, including names, SQL definitions and root pages, and encoded every stored
ordinary-table value with distinct NULL, integer, real-bit, text-byte and blob
tags. Each output was committed, serialized, reopened and checked independently.

All 680 edits passed full SQLite integrity checks, foreign-key checks and exact
page-extent checks. Each changed exactly the requested cell. All 122 schema
objects and every other stored cell stayed equal, including row ordering in the
selected table scans. The edit comparisons checked 10,340,720 cells, of which
10,340,040 were required to stay unchanged. Views were preserved as schema;
their computed query results were not mistaken for stored table values.

Fresh text edits back to the original empty strings restored every logical
table and schema in all 680 cases. None restored the original database bytes.
The forward edits changed between 55 and 431 byte positions without growing any
image. These observations do not identify every physical rewrite or establish
an allowed byte-difference whitelist. A retained source checkpoint is required
for an exact wire reversal.

Four separate committed engine no-ops, one per archive, did preserve original
bytes. The production API should still detect unchanged TEXT before mutation
to guarantee this behavior across its selected engine/profile and inputs.
The corpus observation alone does not establish every no-op path.

Python SQLite 3.50.4 completed the experiment in 76.08 seconds on the recorded
Windows host. This is research timing, not a release benchmark or Rust result.
`native-write-investigation.json` records source/log hashes and links four
authenticated per-archive reports. Those retain every key/slot, output hash,
extent, logical check count and fresh-reversal observation. The throwaway
experiment implementation was removed after measurement.

## Operation checklist and remaining acceptance

1. Parse the signed two-part key and bounded 0–19 slot once. Require exactly
   one match under SQLite's actual affinity/NULL comparison; ambiguity fails.
2. Validate the source image and schema before writing. Use an operation-local
   writable memory connection, strict execution/allocation limits, defensive
   configuration and foreign-key enforcement. Bound page growth before UPDATE.
3. Reject unsupported trigger, virtual/generated-column or other side-effect
   paths explicitly. Preserve original dynamic values without provider guesses.
4. Commit the isolated update and reparse its output. Require exactly the
   requested UTF-8 TEXT bytes and validate complete integrity, foreign keys,
   every ordinary table and schema, and application header metadata. Budget
   whole-operation comparisons, including both source and result snapshots.
5. Publish only after all required checks complete. Keep every unrelated SQL
   value unchanged and report the full SQLite engine rewrite scope separately
   from logical row/cell provenance. Carry the exact immutable source checkpoint.
6. Replace only the selected ZIP occurrence and compose the existing carrier
   history. Preserve encoded unrelated ZIP members. Confirm no-op byte identity,
   input isolation, exact checkpoint reversal, process determinism and concurrency.

The owned cases above cover NULL/dynamic values, affinity, duplicate/absent keys,
constraints, unsupported schema effects, page growth, exhausted budgets and input
isolation. General schema mutation, trigger/generated-table editing, arbitrary
long-value producer compatibility and native application acceptance remain outside
this operation. The original Python investigation does not substitute for the
production Rust or whole-suite gates. There is no public writer stub or fallback.

Native provider conversions, migrations, Electrical loader acceptance, EWG
contents and geometry remain separate. SQL validity alone will not be reported
as native application validity. The humanizer skill is unavailable; its prose
pass remains for maintainer review.
