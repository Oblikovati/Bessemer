# Electrical project interoperability (#231)

The specification documents Electrical project archives, lexical Archive.ini
records, SQLite schemas, project/drawing relationships and userdata queries in
14.24–14.28. `bessemer::electrical` now provides complete ZIP32 carrier operations
and lossless ASCII Archive.ini views and value edits. SQLite catalogs/rows,
project relationships and userdata semantics remain unfinished.

## Typed operations

`ElectricalArchive::from_bytes` checks the complete physical partition and all
member termination, size and CRC invariants before publication. Limits bound
encoded bytes, member occurrences, individual/aggregate decoded bytes and
requested INI records. Reported wire sizes and offsets refer to the original
plain ZIP, excluding the adapter's internal prefix.

`members` preserves directory order, duplicate occurrences and literal names.
Each `ElectricalMemberId` belongs to its immutable archive revision.
`member_named` requires exactly one match; callers select duplicate occurrences
through enumeration. `member_metadata` exposes retained flags, local-header
offsets, extra bytes and comments. No member is extracted to a filesystem path.

`edit_member` replaces decoded contents on an isolated snapshot. `archive_ini`
returns the exact ASCII lexical view or opaque `None` for a non-ASCII payload.
`edit_ini_value` selects one raw lexical record ordinal within an exact
Archive.ini occurrence. It accepts length changes and reparses the whole view,
requiring the same record count, kinds, sections, keys and all other raw lines.
Duplicate fields, empty fields, whitespace and source line endings are retained.

```rust,ignore
let archive = ElectricalArchive::from_bytes(&bytes, limits)?;
let id = archive.member_named("Archive.ini")?;
let edit = archive.edit_ini_value(&id, field_ordinal, b"value")?;
let output = edit.result().to_bytes();
```

Each edit returns source/result snapshots, changed/unchanged completion,
coverage, achieved reparse error, complete occurrence history and diagnostics.
INI edits also return parent/result byte intervals for the decoded payload,
including differing source/result lengths. The source checkpoint retains the
original compression bytes. Freshly reopened reversals restore decoded member
contents; exact original wire restoration uses the retained source checkpoint.

## Complete carrier evidence

Four authenticated installed templates contain 49 members: four Archive.ini
files, 24 SQLite images, seventeen EWG drawings and four empty locators.
`native-sources.json` binds every original archive to the supplied schema report.
No installed source file was changed.

The existing Composer ZIP codec uses offsets relative to the ZIP body after
its three-byte prefix. A temporary adapter supplied that prefix in memory and
removed it from serialized results. All four native archives reconstructed
exactly. Every native member was then replaced, including length changes, and
the complete result reopened successfully through the same checked codec.

An independent ZIP32 oracle checked the actual end record, complete physical
partition, raw-deflate termination, decoded sizes and CRCs. It also compared
every local and central header, allowing only the selected member's payload
words and required relocated offsets. Across four native and four owned cases,
56 edits passed and 566 untouched encoded member representations stayed exact.
The archive comments, unknown metadata, duplicate names and literal path-like
names were preserved. No member name was used as a filesystem destination.

`zip-reuse-verification.json` records every output hash and check count.
`source-and-codec-bindings.json` binds the temporary executable source, linked
library and reference readers from the initial investigation. That probe was
removed after verification. The public API uses the same verified coordinate
conversion and shared ZIP codec; the independent fixture oracles remain.

## Fixtures and lexical records

`archive-cases.bin.gz` contains eight complete original files and independent
expected members, ZIP metadata and lexical INI records. Four cases are native.
The owned cases cover an empty archive, duplicate Archive.ini occurrences,
unknown path-like names, UTF-8 names, non-ASCII INI bytes, extra carriage returns
and every ASCII splitline delimiter used by the reference lexer.

The lexical oracle preserves duplicate assignments, empty keys/sections,
whitespace, comments, NUL bytes and raw line endings. It matched the original
reference function on eight fixture payloads and 257 generated ASCII payloads.
Non-ASCII payloads retain their bytes without an ASCII lexical interpretation.

```powershell
python -m unittest discover -s docs/features/electrical-projects -p 'test_*.py' -v
cargo test --test electrical_projects
```

The focused API gate passed sixteen Rust tests, eleven shared-Composer
regression tests and eight reference-fixture tests. It covered all 56 member
edits, 223 lexical field edits with decoded-content reversals, 223 carrier
no-ops, exact unselected encoded records, budget failures, stale identities and
eight-worker input/result isolation. Two fresh processes produced identical
55,464,770-byte output containing eight reconstructions and all 279 edits.
`api-local-verification.json` and `api-process-verification.json` retain the
source/log hashes and acceptance exclusions.

## Reference-reader discrepancy (#232)

On the installed Python runtime, the original reference ZIP reader reports
zero members and an empty comment for the owned one-member archive whose
comment contains an end-record-like signature. The independently constructed
fixture declares one member and preserves the complete comment. The existing
Rust codec and explicit-comment ZIP oracle both validate that complete file.
`reference-comment-selection-discrepancy.json` retains the original observation.
The owned expectations come from construction inputs, including this case,
rather than the affected readback. The supplied reference code was not changed.

## Remaining work

- The separate [SQLite image reader](../sqlite-images/README.md) now checks
  complete database images and retains schema metadata, NULLs, unknown columns
  and all named-table rows. Application catalog/record projections remain next.
- Bind project/drawing relationships, typed file-load mappings and delegated
  userdata/query selection without extending their documented native scope.
- Preserve opaque EWG bodies and qualify native loader acceptance, migrations,
  SQL/application validity and geometry separately.

Carrier CRCs do not establish SQLite or drawing semantics. No original native
Electrical application was executed. The humanizer pass remains for maintainer
review because that skill is unavailable in this environment.
