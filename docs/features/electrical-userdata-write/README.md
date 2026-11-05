# Existing userdata text-cell writes (#238)

Status: native-copy investigation completed; the production Rust writer is not
implemented. This work extends the read-only userdata views toward Bessemer's
required read/write interoperability. It does not establish overall schema parity.

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

## Production operation to prove

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

Owned acceptance still needs NULL/dynamic-type inputs, affinity, duplicate and
absent keys, constraints, malformed schemas, large values/page growth, budget
exhaustion and source/result isolation. The 680 native short-string observations
do not substitute for those cases or for the production Rust gate. There is no
public writer stub or fallback implementation.

Native provider conversions, migrations, Electrical loader acceptance, EWG
contents and geometry remain separate. SQL validity alone will not be reported
as native application validity. The humanizer skill is unavailable; its prose
pass remains for maintainer review.
