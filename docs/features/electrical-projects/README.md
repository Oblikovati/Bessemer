# Electrical project interoperability (#231)

The specification documents Electrical project archives, lexical Archive.ini
records, SQLite schemas, project/drawing relationships and userdata queries in
14.24–14.28. Bessemer has no public Electrical API yet. This directory retains
the independent fixtures and codec-reuse evidence needed to build it.

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
library and reference readers. The temporary adapter implementation was removed
after verification; the reusable independent fixture oracles remain.

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
```

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

- Add the typed Electrical carrier and lexical edit operations with caller
  limits, immutable snapshots, occurrence identities, coverage and history.
- Implement the documented SQLite catalog and record views, including NULL,
  unknown columns and the distinct stored version fields.
- Bind project/drawing relationships, typed file-load mappings and delegated
  userdata/query selection without extending their documented native scope.
- Preserve opaque EWG bodies and qualify native loader acceptance, migrations,
  SQL/application validity and geometry separately.

Carrier CRCs do not establish SQLite or drawing semantics. No original native
Electrical application was executed. The humanizer pass remains for maintainer
review because that skill is unavailable in this environment.
