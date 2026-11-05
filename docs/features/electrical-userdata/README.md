# Electrical userdata SQL, lexical selection and identity gate (#231)

`ElectricalArchive::userdata_by_key` implements specification 14.28's fixed
22-column projection from `tew_userdata`, binding both signed 32-bit object ID
and object number as SQLite parameters. SQLite performs the equality, affinity
and NULL comparisons. The result retains every matching row, the original key,
the checked image and the source carrier/member revision.

```rust,ignore
let key = ElectricalUserdataKey { object_id: 26, object_no: -1 };
let query = archive.userdata_by_key(&member, key, limits)?;
let exists = query.exists();
let rows = query.rows();
```

`exists` reports only whether the complete query matched a row. Missing rows
complete with an empty 22-column result. SQL failures, corrupt images, foreign
member identities and exhausted limits produce typed errors. They do not become
an absent-row result. Duplicate matches remain visible; their scan order does
not establish the native provider's first-row priority.

`ElectricalProjectDatabase::userdata_for_object` separately implements the
reference helper's lexical numeric-ID comparison. It includes integral REAL
matches and all object numbers, retains unknown columns, and does not apply SQL
TEXT affinity. Returned values and column names are owned immutable snapshots.
Its output accounting is recomputed using the existing checked source bound.

`electrical_userdata_id_transition` models specification 14.27's selected gate
after provider conversion to signed long. A newly loaded zero retains any
nonzero current ID and rejects the delegated success branch. Otherwise negative
IDs become -1 and reject; nonnegative IDs replace the stored value and accept.
An existing SQL row with a negative ID can therefore make `exists` true while
the ID gate rejects it. This function does not model provider failures, NULL
conversion, later field-read failures, load-flag mutation or twenty-string
clearing behavior of the complete native delegate.

Each SQL operation first checks complete image integrity/schema metadata through
the [SQLite reader](../sqlite-images/README.md), then executes the fixed bounded
query on a separate local read-only connection. There is no public arbitrary-SQL
entry point, filesystem database, global callback state or unsafe code. All
existing image/query row, byte, column, output and VM limits apply. Publication
requires a complete result; no prefix of matching rows is returned on exhaustion.
Image coverage and native-application/foreign-key/priority exclusions accompany
the successful query. Schema validation, native migration/loader acceptance and
geometry remain separate requirements.

## Independent evidence

The EUQ1 packet contains three committed owned databases and all four original
Electrical template archives. Its 78 queries include all 34 independently
recorded native drawing keys, 17 absent native object-number queries and 27 owned
queries. Owned cases exercise INTEGER/TEXT/no affinity, NULLs, integer/REAL
comparison, numeric-looking strings, blob keys, signed boundaries, duplicate
matches, unknown columns, NULL/blob/REAL userdata and row-limit exhaustion.
The unchanged reference functions generate complete SQL and lexical results.
The same packet pins 81 current/loaded signed-ID combinations from the original
identity helper. Hashes bind every carrier and complete expected projection in
`tests/fixtures/electrical-userdata/integrity.json`.

The focused gate passed eleven userdata tests, twelve project-record regressions,
eleven SQLite regressions, two Python fixture tests and Clippy with warnings
denied. It includes eight-worker mixed reads and source-buffer mutation, exact
22-column empty results, foreign identities, required-check interruption and
query row-budget exhaustion. Two fresh processes reproduced every carrier,
SQL/lexical result and ID transition byte for byte. `local-verification.json`
and `process-verification.json` retain the exact source/log/output hashes.
Full-suite validation passed on source 13931532: 4,416 tests passed, zero failed
and 233 were explicitly ignored across 349 result summaries. The proof is retained
in `full-pr-gate-verification.json`. That source excludes the later direct field
maps and [userdata writer](../electrical-userdata-write/README.md).

```powershell
python -m unittest discover -s docs/features/electrical-userdata -p 'test_*.py' -v
cargo test --test electrical_userdata --test electrical_records --test sqlite_images --locked
```

The [direct file-field map](../electrical-file-fields/README.md) now retains the
38 verified reader/destination bindings. Full userdata delegate/provider state,
native file-loader conversion effects, database mutation and broader Electrical/application schema parity remain
incomplete. The humanizer skill is unavailable; its prose pass remains for review.

The reader evidence supplement in `reader-native-byte-verification.json`
corrects the coverage gap tracked in #240. The upstream packet retains only
15 bytes at `0x1804a3580`; those bytes cover the initial unwind region.
Three chained regions extend the logical reader through `0x1804a3661`.
The supplement authenticates all 225 bytes against the installed DLL and
retains all 65 decoded instructions. `test_native_reader.py` checks contiguous
region coverage, the instruction byte stream, and the installed artifact hash.

This is offline PE evidence, with no new REA or native execution credit.
It confirms the null-input return, rejected-ID string clearing, and accepted-ID
load flag and field reads. Provider conversions, the context helper's meaning,
and application acceptance remain unverified. The source packet is preserved.
The unavailable humanizer pass remains deferred to maintainer review.
